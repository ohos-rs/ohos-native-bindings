use std::cell::RefCell;
use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use ohos_input_method_sys::{
    InputMethod_ErrorCode_IME_ERR_NULL_POINTER, InputMethod_InputMethodProxy,
    OH_InputMethodController_Attach, OH_InputMethodController_Detach,
    OH_InputMethodProxy_HideKeyboard, OH_InputMethodProxy_ShowKeyboard,
};

use crate::proxy::{register_callbacks, unregister_callbacks, IMECallbacks, SharedCallbacks};
use crate::{AttachOptions, EnterKey, ImeError, ImeResult, KeyboardStatus, TextEditor};

static NEXT_SESSION_ID: AtomicU64 = AtomicU64::new(1);
static ACTIVE_SESSION_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
pub struct IME {
    inner: Rc<IMEInner>,
}

struct IMEInner {
    id: u64,
    raw: RefCell<Option<NonNull<InputMethod_InputMethodProxy>>>,
    option: AttachOptions,
    text_editor: RefCell<Option<TextEditor>>,
    callbacks: SharedCallbacks,
    #[cfg(feature = "api-22")]
    callbacks_in_main_thread: bool,
}

impl PartialEq for IME {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for IME {}

impl std::hash::Hash for IME {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(Rc::as_ptr(&self.inner), state);
    }
}

impl IME {
    pub fn new(option: AttachOptions) -> Self {
        Self::with_callback_thread(option, false)
    }

    /// Create an IME whose TextEditor callbacks are dispatched on the main
    /// thread instead of the platform-default IPC thread.
    #[cfg(feature = "api-22")]
    pub fn new_with_main_thread_callbacks(option: AttachOptions) -> Self {
        Self::with_callback_thread(option, true)
    }

    fn with_callback_thread(option: AttachOptions, callbacks_in_main_thread: bool) -> Self {
        #[cfg(not(feature = "api-22"))]
        let _ = callbacks_in_main_thread;
        Self {
            inner: Rc::new(IMEInner {
                id: next_session_id(),
                raw: RefCell::new(None),
                option,
                text_editor: RefCell::new(None),
                callbacks: Arc::new(RwLock::new(IMECallbacks::default())),
                #[cfg(feature = "api-22")]
                callbacks_in_main_thread,
            }),
        }
    }

    pub fn insert_text<T>(&self, callback: T)
    where
        T: Fn(String) + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| callbacks.insert_text = Some(Arc::new(callback)));
    }

    pub fn pre_edit<T>(&self, callback: T)
    where
        T: Fn(String, i32, i32) + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| callbacks.set_preview_text = Some(Arc::new(callback)));
    }

    pub fn on_status_change<T>(&self, callback: T)
    where
        T: Fn(KeyboardStatus) + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| {
            callbacks.send_keyboard_status = Some(Arc::new(callback));
        });
    }

    pub fn on_delete<T>(&self, callback: T)
    where
        T: Fn(i32) + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| callbacks.delete_backward = Some(Arc::new(callback)));
    }

    pub fn on_backspace<T>(&self, callback: T)
    where
        T: Fn(i32) + Send + Sync + 'static,
    {
        self.on_delete(callback);
    }

    pub fn on_enter<T>(&self, callback: T)
    where
        T: Fn(EnterKey) + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| callbacks.send_enter_key = Some(Arc::new(callback)));
    }

    pub fn on_preview<T>(&self, callback: T)
    where
        T: Fn(String, i32, i32) + Send + Sync + 'static,
    {
        self.pre_edit(callback);
    }

    pub fn on_finish_preview<T>(&self, callback: T)
    where
        T: Fn() + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| {
            callbacks.finish_text_preview = Some(Arc::new(callback));
        });
    }

    /// Attach this editor, replacing any other `IME` session previously
    /// attached through this binding.
    pub fn try_attach(&self) -> ImeResult<()> {
        if ACTIVE_SESSION_ID.load(Ordering::Acquire) == self.inner.id
            && self.inner.raw.borrow().is_some()
        {
            return Ok(());
        }

        // A later binding session may already have invalidated this proxy.
        // The NDK contract ends its lifetime at the next attach, so it must be
        // discarded locally without detaching the currently active editor.
        if self.inner.raw.borrow().is_some() {
            self.discard_local_session();
        }

        let editor = TextEditor::new();
        #[cfg(feature = "api-22")]
        if self.inner.callbacks_in_main_thread {
            editor.set_callback_in_main_thread(true)?;
        }
        register_callbacks(editor.raw, self.inner.callbacks.clone());

        let mut raw: *mut InputMethod_InputMethodProxy = ptr::null_mut();
        let code = unsafe {
            OH_InputMethodController_Attach(
                editor.raw,
                self.inner.option.raw,
                &mut raw as *mut *mut InputMethod_InputMethodProxy,
            )
        };
        if code != 0 {
            unregister_callbacks(editor.raw);
            return Err(ImeError::new("attach", code));
        }
        let Some(raw) = NonNull::new(raw) else {
            unregister_callbacks(editor.raw);
            return Err(ImeError::new(
                "attach",
                InputMethod_ErrorCode_IME_ERR_NULL_POINTER,
            ));
        };

        self.inner.text_editor.replace(Some(editor));
        self.inner.raw.replace(Some(raw));
        ACTIVE_SESSION_ID.store(self.inner.id, Ordering::Release);
        Ok(())
    }

    pub fn attach(&self) {
        let _ = self.try_attach();
    }

    /// Show the keyboard, recovering once if HarmonyOS invalidated the proxy
    /// because another editor was attached or the Ability left foreground.
    pub fn try_show_keyboard(&self) -> ImeResult<()> {
        self.try_attach()?;
        match self.show_attached() {
            Ok(()) => Ok(()),
            Err(error) if error.is_stale_session() => {
                self.discard_local_session();
                self.clear_active_session();
                self.try_attach()?;
                let result = self.show_attached();
                if result.as_ref().is_err_and(|error| error.is_stale_session()) {
                    self.discard_local_session();
                    self.clear_active_session();
                }
                result
            }
            Err(error) => Err(error),
        }
    }

    pub fn show_keyboard(&self) {
        let _ = self.try_show_keyboard();
    }

    /// Hide the keyboard without ending this editor session.
    pub fn try_hide_keyboard(&self) -> ImeResult<()> {
        let Some(raw) = *self.inner.raw.borrow() else {
            return Ok(());
        };
        let code = unsafe { OH_InputMethodProxy_HideKeyboard(raw.as_ptr()) };
        if code == 0 {
            Ok(())
        } else {
            let error = ImeError::new("hide-keyboard", code);
            if error.is_stale_session() {
                self.discard_local_session();
                self.clear_active_session();
            }
            Err(error)
        }
    }

    pub fn hide_keyboard(&self) {
        let _ = self.try_hide_keyboard();
    }

    /// Explicitly end this editor session. Stale sessions are only discarded
    /// locally so they cannot detach another editor which replaced them.
    pub fn try_detach(&self) -> ImeResult<()> {
        let is_active = ACTIVE_SESSION_ID
            .compare_exchange(self.inner.id, 0, Ordering::AcqRel, Ordering::Acquire)
            .is_ok();
        let raw = self.inner.raw.borrow_mut().take();
        let result = if is_active {
            raw.map_or(Ok(()), |raw| {
                let code = unsafe { OH_InputMethodController_Detach(raw.as_ptr()) };
                if code == 0 {
                    Ok(())
                } else {
                    Err(ImeError::new("detach", code))
                }
            })
        } else {
            Ok(())
        };
        self.drop_text_editor();
        result
    }

    pub fn detach(&self) {
        let _ = self.try_detach();
    }

    fn show_attached(&self) -> ImeResult<()> {
        let Some(raw) = *self.inner.raw.borrow() else {
            return Err(ImeError::new(
                "show-keyboard",
                InputMethod_ErrorCode_IME_ERR_NULL_POINTER,
            ));
        };
        let code = unsafe { OH_InputMethodProxy_ShowKeyboard(raw.as_ptr()) };
        if code == 0 {
            Ok(())
        } else {
            Err(ImeError::new("show-keyboard", code))
        }
    }

    fn update_callbacks(&self, update: impl FnOnce(&mut IMECallbacks)) {
        let mut callbacks = self
            .inner
            .callbacks
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        update(&mut callbacks);
    }

    fn discard_local_session(&self) {
        self.inner.raw.borrow_mut().take();
        self.drop_text_editor();
    }

    fn drop_text_editor(&self) {
        if let Some(editor) = self.inner.text_editor.borrow_mut().take() {
            unregister_callbacks(editor.raw);
        }
    }

    fn clear_active_session(&self) {
        let _ = ACTIVE_SESSION_ID.compare_exchange(
            self.inner.id,
            0,
            Ordering::AcqRel,
            Ordering::Acquire,
        );
    }
}

impl Drop for IMEInner {
    fn drop(&mut self) {
        let is_active = ACTIVE_SESSION_ID
            .compare_exchange(self.id, 0, Ordering::AcqRel, Ordering::Acquire)
            .is_ok();
        if is_active {
            if let Some(raw) = self.raw.get_mut().take() {
                unsafe {
                    OH_InputMethodController_Detach(raw.as_ptr());
                }
            }
        }
        if let Some(editor) = self.text_editor.get_mut().take() {
            unregister_callbacks(editor.raw);
        }
    }
}

fn next_session_id() -> u64 {
    loop {
        let id = NEXT_SESSION_ID.fetch_add(1, Ordering::Relaxed);
        if id != 0 {
            return id;
        }
    }
}
