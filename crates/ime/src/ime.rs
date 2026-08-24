use ohos_input_method_sys::{
    InputMethod_InputMethodProxy, OH_InputMethodController_Attach, OH_InputMethodController_Detach,
    OH_InputMethodProxy_HideKeyboard, OH_InputMethodProxy_ShowKeyboard,
};
use std::{
    cell::RefCell,
    ptr::{self, NonNull},
    rc::Rc,
};

use crate::{proxy::OHOS_RS_IME_CALLBACKS, AttachOptions, EnterKey, KeyboardStatus, TextEditor};

/// Report a failed input-method NDK call instead of panicking: these calls
/// fail in environments without a live input method (e.g. a test runner
/// process), and a panic across the napi `extern "C"` boundary aborts the
/// whole app.
fn log_call_failure(call: &str, ret: u32) {
    // Intentionally no logging here: thread-local storage for stderr can
    // itself abort once the process has exhausted its pthread keys (many
    // napi .so modules loaded). The goal is only to be non-fatal.
    let _ = (call, ret);
}

#[derive(Clone)]
pub struct IME {
    inner: Rc<IMEInner>,
}

struct IMEInner {
    raw: RefCell<Option<NonNull<InputMethod_InputMethodProxy>>>,
    option: AttachOptions,
    text_editor: RefCell<Option<TextEditor>>,
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
        IME {
            inner: Rc::new(IMEInner {
                raw: RefCell::new(None),
                text_editor: RefCell::new(None),
                option,
                #[cfg(feature = "api-22")]
                callbacks_in_main_thread,
            }),
        }
    }

    pub fn insert_text<'a, T>(&self, callback: T)
    where
        T: Fn(String) + 'a,
    {
        let mut guard = OHOS_RS_IME_CALLBACKS
            .write()
            .expect("OHOS_RS_IME_CALLBACKS write failed");
        let cb = unsafe {
            std::mem::transmute::<Box<dyn Fn(String) + 'a>, Box<dyn Fn(String) + 'static>>(
                Box::new(callback),
            )
        };
        guard.insert_text = Some(cb);
    }

    pub fn pre_edit<'a, T>(&self, callback: T)
    where
        T: Fn(String, i32, i32) + 'a,
    {
        let mut guard = OHOS_RS_IME_CALLBACKS
            .write()
            .expect("OHOS_RS_IME_CALLBACKS write failed");
        let cb = unsafe {
            std::mem::transmute::<
                Box<dyn Fn(String, i32, i32) + 'a>,
                Box<dyn Fn(String, i32, i32) + 'static>,
            >(Box::new(callback))
        };
        guard.set_preview_text = Some(cb);
    }

    pub fn on_status_change<'a, T>(&self, callback: T)
    where
        T: Fn(KeyboardStatus) + 'a,
    {
        let mut guard = OHOS_RS_IME_CALLBACKS
            .write()
            .expect("OHOS_RS_IME_CALLBACKS write failed");
        let cb = unsafe {
            std::mem::transmute::<
                Box<dyn Fn(KeyboardStatus) + 'a>,
                Box<dyn Fn(KeyboardStatus) + 'static>,
            >(Box::new(callback))
        };
        guard.send_keyboard_status = Some(cb);
    }

    pub fn on_delete<'a, T>(&self, callback: T)
    where
        T: Fn(i32) + 'a,
    {
        let mut guard = OHOS_RS_IME_CALLBACKS
            .write()
            .expect("OHOS_RS_IME_CALLBACKS write failed");
        let cb = unsafe {
            std::mem::transmute::<Box<dyn Fn(i32) + 'a>, Box<dyn Fn(i32) + 'static>>(Box::new(
                callback,
            ))
        };
        guard.delete_backward = Some(cb);
    }

    pub fn attach(&self) {
        if self.inner.raw.borrow().is_some() {
            return;
        }

        let editor = TextEditor::new();
        #[cfg(feature = "api-22")]
        if self.inner.callbacks_in_main_thread && !editor.set_callback_in_main_thread(true) {
            return;
        }
        unsafe {
            let mut raw: *mut InputMethod_InputMethodProxy = ptr::null_mut();
            let ret = OH_InputMethodController_Attach(
                editor.raw,
                self.inner.option.raw,
                &mut raw as *mut *mut InputMethod_InputMethodProxy,
            );
            #[cfg(debug_assertions)]
            log_call_failure("OH_InputMethodController_Attach", ret);

            if let Some(raw) = NonNull::new(raw) {
                self.inner.text_editor.replace(Some(editor));
                self.inner.raw.replace(Some(raw));
            }
        }
    }

    pub fn show_keyboard(&self) {
        self.attach();

        // Drop the RefCell borrow before entering native code. The platform
        // may synchronously invoke a callback which requests another IME
        // operation.
        let ime_proxy = *self.inner.raw.borrow();
        if let Some(ime_proxy) = ime_proxy {
            unsafe {
                let ret = OH_InputMethodProxy_ShowKeyboard(ime_proxy.as_ptr());
                #[cfg(debug_assertions)]
                log_call_failure("OH_InputMethodProxy_ShowKeyboard", ret);
            }
        }
    }

    pub fn on_backspace<'a, T>(&self, callback: T)
    where
        T: Fn(i32) + 'a,
    {
        let mut guard = OHOS_RS_IME_CALLBACKS
            .write()
            .expect("OHOS_RS_IME_CALLBACKS write failed");
        let cb = unsafe {
            std::mem::transmute::<Box<dyn Fn(i32) + 'a>, Box<dyn Fn(i32) + 'static>>(Box::new(
                callback,
            ))
        };
        guard.delete_backward = Some(cb);
    }

    pub fn on_enter<'a, T>(&self, callback: T)
    where
        T: Fn(EnterKey) + 'a,
    {
        let mut guard = OHOS_RS_IME_CALLBACKS
            .write()
            .expect("OHOS_RS_IME_CALLBACKS write failed");
        let cb = unsafe {
            std::mem::transmute::<Box<dyn Fn(EnterKey) + 'a>, Box<dyn Fn(EnterKey) + 'static>>(
                Box::new(callback),
            )
        };
        guard.send_enter_key = Some(cb);
    }

    pub fn on_preview<'a, T>(&self, callback: T)
    where
        T: Fn(String, i32, i32) + 'a,
    {
        let mut guard = OHOS_RS_IME_CALLBACKS
            .write()
            .expect("OHOS_RS_IME_CALLBACKS write failed");
        let cb = unsafe {
            std::mem::transmute::<
                Box<dyn Fn(String, i32, i32) + 'a>,
                Box<dyn Fn(String, i32, i32) + 'static>,
            >(Box::new(callback))
        };
        guard.set_preview_text = Some(cb);
    }

    pub fn on_finish_preview<'a, T>(&self, callback: T)
    where
        T: Fn() + 'a,
    {
        let mut guard = OHOS_RS_IME_CALLBACKS
            .write()
            .expect("OHOS_RS_IME_CALLBACKS write failed");
        let cb = unsafe {
            std::mem::transmute::<Box<dyn Fn() + 'a>, Box<dyn Fn() + 'static>>(Box::new(callback))
        };
        guard.finish_text_preview = Some(cb);
    }

    pub fn hide_keyboard(&self) {
        let raw = *self.inner.raw.borrow();
        if let Some(raw) = raw {
            unsafe {
                let ret = OH_InputMethodProxy_HideKeyboard(raw.as_ptr());

                #[cfg(debug_assertions)]
                log_call_failure("OH_InputMethodProxy_HideKeyboard", ret);
            }
        }
    }

    pub fn detach(&self) {
        let raw = self.inner.raw.borrow_mut().take();
        if let Some(raw) = raw {
            unsafe {
                let ret = OH_InputMethodController_Detach(raw.as_ptr());
                #[cfg(debug_assertions)]
                log_call_failure("OH_InputMethodController_Detach", ret);
            }
        }
        self.inner.text_editor.borrow_mut().take();
    }
}

impl Drop for IMEInner {
    fn drop(&mut self) {
        let raw = self.raw.get_mut().take();
        if let Some(raw) = raw {
            unsafe {
                let ret = OH_InputMethodController_Detach(raw.as_ptr());
                #[cfg(debug_assertions)]
                log_call_failure("OH_InputMethodController_Detach", ret);
            }
        }
        self.text_editor.get_mut().take();
    }
}
