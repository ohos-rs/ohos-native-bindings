use std::cell::{Cell, RefCell};
use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};

use ohos_input_method_sys::{
    InputMethod_ErrorCode_IME_ERR_DETACHED, InputMethod_ErrorCode_IME_ERR_NULL_POINTER,
    InputMethod_InputMethodProxy, OH_InputMethodController_Attach, OH_InputMethodController_Detach,
    OH_InputMethodProxy_HideKeyboard, OH_InputMethodProxy_NotifyConfigurationChange,
    OH_InputMethodProxy_NotifyCursorUpdate, OH_InputMethodProxy_NotifySelectionChange,
    OH_InputMethodProxy_ShowKeyboard,
};

use crate::proxy::{register_callbacks, unregister_callbacks, IMECallbacks, SharedCallbacks};
use crate::session::NativeSessions;
use crate::{
    Action, AttachOptions, Cursor, Direction, EnterKey, ImeError, ImeResult, InputType,
    KeyboardStatus, Rect, Selection, TextEditor, TextState,
};

static NEXT_SESSION_ID: AtomicU64 = AtomicU64::new(1);

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
    needs_attach: Cell<bool>,
    editor_generation: Cell<u64>,
    configuration: Arc<RwLock<EditorConfiguration>>,
    text_state: Arc<RwLock<TextState>>,
    text_notified: Cell<bool>,
    #[cfg(feature = "api-22")]
    callbacks_in_main_thread: bool,
}

#[derive(Clone, Copy)]
struct EditorConfiguration {
    enter_key: EnterKey,
    input_type: InputType,
    cursor: Option<Rect>,
    preview_supported: bool,
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
        let configuration = Arc::new(RwLock::new(EditorConfiguration {
            enter_key: EnterKey::Unspecified,
            input_type: InputType::Text,
            cursor: None,
            preview_supported: false,
        }));
        let callback_configuration = configuration.clone();
        let text_state = Arc::new(RwLock::new(TextState::default()));
        let left_state = text_state.clone();
        let right_state = text_state.clone();
        let cursor_state = text_state.clone();
        let config_state = text_state.clone();
        let callbacks = IMECallbacks {
            get_left_text_of_cursor: Some(Arc::new(move |number| {
                left_state
                    .read()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .before_cursor(number)
            })),
            get_right_text_of_cursor: Some(Arc::new(move |number| {
                right_state
                    .read()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .after_cursor(number)
            })),
            get_text_index_at_cursor: Some(Arc::new(move || {
                cursor_state
                    .read()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .cursor()
                    .min(i32::MAX as usize) as i32
            })),
            get_text_config: Some(Arc::new(move |config| {
                let configuration = *callback_configuration
                    .read()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                config.set_enter_key(configuration.enter_key);
                config.set_input_type(configuration.input_type);
                config.set_preview_text_supported(configuration.preview_supported);
                let state = config_state
                    .read()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                config.set_selection(Selection {
                    start: state.selection.start.min(i32::MAX as usize) as i32,
                    end: state.selection.end.min(i32::MAX as usize) as i32,
                });
                if let Some(rect) = configuration.cursor {
                    config.cursor().set_rect(rect);
                }
            })),
            ..Default::default()
        };
        Self {
            inner: Rc::new(IMEInner {
                id: next_session_id(),
                raw: RefCell::new(None),
                option,
                text_editor: RefCell::new(None),
                callbacks: Arc::new(RwLock::new(callbacks)),
                needs_attach: Cell::new(true),
                editor_generation: Cell::new(0),
                configuration,
                text_state,
                text_notified: Cell::new(false),
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
        self.inner
            .configuration
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .preview_supported = true;
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

    /// Receive deletion toward the end of the document, in UTF-16 units.
    pub fn on_delete_forward<T>(&self, callback: T)
    where
        T: Fn(i32) + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| callbacks.delete_forward = Some(Arc::new(callback)));
    }

    /// Receive cursor movement requested by the system input method.
    pub fn on_move_cursor<T>(&self, callback: T)
    where
        T: Fn(Direction) + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| callbacks.move_cursor = Some(Arc::new(callback)));
    }

    /// Receive the system's requested selection in UTF-16 units.
    pub fn on_set_selection<T>(&self, callback: T)
    where
        T: Fn(Selection) + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| {
            callbacks.handle_set_selection = Some(Arc::new(callback))
        });
    }

    /// Receive system Select All, Cut, Copy and Paste commands.
    pub fn on_extend_action<T>(&self, callback: T)
    where
        T: Fn(Action) + Send + Sync + 'static,
    {
        self.update_callbacks(|callbacks| {
            callbacks.handle_extend_action = Some(Arc::new(callback))
        });
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

    /// Retain the editor attributes for the next attach and update this session
    /// immediately when it is active. An inactive editor never changes the
    /// attributes of another editor which replaced it.
    pub fn try_update_configuration(
        &self,
        enter_key: EnterKey,
        input_type: InputType,
    ) -> ImeResult<()> {
        {
            let mut configuration = self
                .inner
                .configuration
                .write()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            configuration.enter_key = enter_key;
            configuration.input_type = input_type;
        }
        let sessions = NativeSessions::acquire("notify")?;
        let Some(raw) = self.active_proxy(&sessions) else {
            return Ok(());
        };
        let code = unsafe {
            OH_InputMethodProxy_NotifyConfigurationChange(
                raw.as_ptr(),
                enter_key.into(),
                input_type.into(),
            )
        };
        self.notification_result("update-configuration", code)
    }

    /// Retain the cursor rectangle in absolute physical screen coordinates.
    /// The current session receives the update if this editor is active.
    pub fn try_update_cursor(&self, rect: Rect) -> ImeResult<()> {
        self.inner
            .configuration
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .cursor = Some(rect);
        let sessions = NativeSessions::acquire("notify")?;
        let Some(raw) = self.active_proxy(&sessions) else {
            return Ok(());
        };
        let cursor = Cursor::new(rect);
        let code = unsafe { OH_InputMethodProxy_NotifyCursorUpdate(raw.as_ptr(), cursor.raw) };
        self.notification_result("update-cursor", code)
    }

    /// Cache text for IPC-thread queries and notify the active native editor.
    /// The SDK accepts at most 8K UTF-16 units of whole input text. Larger or
    /// offset snapshots answer surrounding-text queries but are not passed as
    /// misleading whole-document selection notifications.
    pub fn try_update_text_state(&self, state: TextState) -> ImeResult<()> {
        let changed = {
            let mut previous = self
                .inner
                .text_state
                .write()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if *previous == state {
                false
            } else {
                *previous = state;
                true
            }
        };
        if !changed && self.inner.text_notified.get() {
            return Ok(());
        }
        let sessions = NativeSessions::acquire("notify")?;
        let Some(raw) = self.active_proxy(&sessions) else {
            return Ok(());
        };
        let state = self
            .inner
            .text_state
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone();
        let mut text: Vec<u16> = state.text.encode_utf16().collect();
        if state.offset != 0
            || text.len() > 8192
            || state.selection.end > text.len()
            || state.selection.start > state.selection.end
        {
            return Ok(());
        }
        let code = unsafe {
            OH_InputMethodProxy_NotifySelectionChange(
                raw.as_ptr(),
                text.as_mut_ptr(),
                text.len(),
                state.selection.start as i32,
                state.selection.end as i32,
            )
        };
        self.notification_result("update-selection", code)?;
        self.inner.text_notified.set(true);
        Ok(())
    }

    fn active_proxy(
        &self,
        sessions: &NativeSessions,
    ) -> Option<NonNull<InputMethod_InputMethodProxy>> {
        (sessions.active_id == self.inner.id && !self.inner.needs_attach.get())
            .then(|| *self.inner.raw.borrow())
            .flatten()
    }

    fn notification_result(&self, operation: &'static str, code: u32) -> ImeResult<()> {
        if code == 0 {
            return Ok(());
        }
        let error = ImeError::new(operation, code);
        if error.is_stale_session() {
            self.invalidate_session();
        }
        Err(error)
    }

    /// Attach this editor, replacing any other `IME` session previously
    /// attached through this binding.
    pub fn try_attach(&self) -> ImeResult<()> {
        let mut sessions = NativeSessions::acquire("attach")?;
        if self.active_proxy(&sessions).is_some() {
            return Ok(());
        }

        // Keep the same callback table across stale-session recovery. Neither
        // NOT_EDITABLE nor DETACHED proves that OnInputStop has finished with it.
        if self.inner.text_editor.borrow().is_none() {
            let editor = TextEditor::new();
            #[cfg(feature = "api-22")]
            if self.inner.callbacks_in_main_thread {
                editor.set_callback_in_main_thread(true)?;
            }
            register_callbacks(editor.raw, self.inner.callbacks.clone());
            self.inner.text_editor.replace(Some(editor));
        }
        let editor = self.inner.text_editor.borrow().as_ref().unwrap().raw;
        self.inner.raw.borrow_mut().take();
        self.invalidate_session();
        self.inner.editor_generation.set(sessions.generation);
        // Even a failed attach may replace the NDK's global proxy. Never let an
        // older Rust session use its previous handle after this attempt.
        sessions.active_id = 0;
        let mut raw: *mut InputMethod_InputMethodProxy = ptr::null_mut();
        let code =
            unsafe { OH_InputMethodController_Attach(editor, self.inner.option.raw, &mut raw) };
        if code != 0 {
            // The NDK can retain the callback table even when attach fails.
            return Err(ImeError::new("attach", code));
        }
        let Some(raw) = NonNull::new(raw) else {
            return Err(ImeError::new(
                "attach",
                InputMethod_ErrorCode_IME_ERR_NULL_POINTER,
            ));
        };

        sessions.complete_transition(self.inner.id);
        self.inner.editor_generation.set(sessions.generation);
        self.inner.raw.replace(Some(raw));
        self.inner.needs_attach.set(false);
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
                self.invalidate_session();
                self.try_attach()?;
                let result = self.show_attached();
                if result.as_ref().is_err_and(|error| error.is_stale_session()) {
                    self.invalidate_session();
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
        let sessions = NativeSessions::acquire("hide-keyboard")?;
        let Some(raw) = self.active_proxy(&sessions) else {
            // Inactive/stale editors must not call through an invalid proxy.
            self.invalidate_session();
            return Ok(());
        };
        let code = unsafe { OH_InputMethodProxy_HideKeyboard(raw.as_ptr()) };
        if code == 0 {
            Ok(())
        } else {
            let error = ImeError::new("hide-keyboard", code);
            if error.is_stale_session() {
                self.invalidate_session();
            }
            Err(error)
        }
    }

    pub fn hide_keyboard(&self) {
        let _ = self.try_hide_keyboard();
    }

    /// End this session without detaching an editor which replaced it. If the
    /// native cleanup fails, callbacks are disabled and their allocation is
    /// retained until a later successful native attach/detach.
    pub fn try_detach(&self) -> ImeResult<()> {
        let mut sessions = NativeSessions::acquire("detach")?;
        self.inner.detach(&mut sessions)
    }

    pub fn detach(&self) {
        let _ = self.try_detach();
    }

    fn show_attached(&self) -> ImeResult<()> {
        let sessions = NativeSessions::acquire("show-keyboard")?;
        let Some(raw) = self.active_proxy(&sessions) else {
            return Err(ImeError::new(
                "show-keyboard",
                InputMethod_ErrorCode_IME_ERR_DETACHED,
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

    fn invalidate_session(&self) {
        self.inner.needs_attach.set(true);
        self.inner.text_notified.set(false);
    }
}

impl IMEInner {
    fn detach(&self, sessions: &mut NativeSessions) -> ImeResult<()> {
        self.needs_attach.set(true);
        self.text_notified.set(false);
        let raw = self.raw.borrow_mut().take();
        let mut result = Ok(());
        if sessions.active_id == self.id {
            sessions.active_id = 0;
            if let Some(raw) = raw {
                let code = unsafe { OH_InputMethodController_Detach(raw.as_ptr()) };
                if code == 0 {
                    sessions.complete_transition(0);
                } else {
                    result = Err(ImeError::new("detach", code));
                }
            }
        }
        if let Some(editor) = self.text_editor.borrow_mut().take() {
            unregister_callbacks(editor.raw);
            if self.editor_generation.get() < sessions.generation {
                drop(editor);
            } else {
                NativeSessions::retire(editor);
            }
        }
        result
    }
}

impl Drop for IMEInner {
    fn drop(&mut self) {
        if let Ok(mut sessions) = NativeSessions::acquire("detach") {
            let _ = self.detach(&mut sessions);
        } else if let Some(editor) = self.text_editor.get_mut().take() {
            // A synchronous native callback may drop a different Rust session.
            // Its pointer cannot be freed until the in-flight transition ends.
            unregister_callbacks(editor.raw);
            NativeSessions::retire(editor);
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
