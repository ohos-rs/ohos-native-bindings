use ohos_input_method_sys::{
    InputMethod_InputMethodProxy, OH_InputMethodController_Attach, OH_InputMethodController_Detach,
    OH_InputMethodProxy_HideKeyboard,
};
use std::{cell::RefCell, ptr, rc::Rc, sync::atomic::AtomicBool};

use crate::{proxy::IME_CALLBACKS, AttachOptions, TextEditor};

unsafe impl Send for IME {}
unsafe impl Sync for IME {}

static IS_SHOWING: AtomicBool = AtomicBool::new(false);

#[derive(Clone)]
pub struct IME {
    raw: Rc<RefCell<Option<*mut InputMethod_InputMethodProxy>>>,
    option: AttachOptions,
    text_editor: Rc<RefCell<Option<TextEditor>>>,
}

impl PartialEq for IME {
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}

impl Eq for IME {}

impl std::hash::Hash for IME {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.option.hash(state);
    }
}

impl IME {
    pub fn new(option: AttachOptions) -> Self {
        let ime = IME {
            raw: Rc::new(RefCell::new(None)),
            text_editor: Rc::new(RefCell::new(None)),
            option,
        };

        ime
    }

    pub fn insert_text<'a, T>(&self, callback: T)
    where
        T: Fn(String) -> () + 'a,
    {
        let mut guard = IME_CALLBACKS.write().expect("IME_CALLBACKS write failed");
        let cb = unsafe {
            std::mem::transmute::<Box<dyn Fn(String) + 'a>, Box<dyn Fn(String) + 'static>>(
                Box::new(callback),
            )
        };
        (*guard).insert_text = Some(cb);
    }

    pub fn show_keyboard(&self) {
        if IS_SHOWING.load(std::sync::atomic::Ordering::SeqCst) {
            return;
        }
        let editor = TextEditor::new();
        unsafe {
            let mut raw: *mut InputMethod_InputMethodProxy = ptr::null_mut();
            let ret = OH_InputMethodController_Attach(
                editor.raw,
                self.option.raw,
                &mut raw as *mut *mut InputMethod_InputMethodProxy,
            );
            self.text_editor.replace(Some(editor));
            self.raw.replace(Some(raw));
            #[cfg(debug_assertions)]
            assert!(ret == 0, "OH_InputMethodController_Attach failed");
            IS_SHOWING.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }

    pub fn hide_keyboard(&self) {
        if !IS_SHOWING.load(std::sync::atomic::Ordering::SeqCst) {
            return;
        }
        unsafe {
            if let Some(raw) = *self.raw.borrow() {
                let ret = OH_InputMethodProxy_HideKeyboard(raw);
                if ret == 0 {
                    let ret = OH_InputMethodController_Detach(raw);
                    #[cfg(debug_assertions)]
                    assert!(ret == 0, "OH_InputMethodController_Detach failed");
                    IS_SHOWING.store(false, std::sync::atomic::Ordering::SeqCst);
                }
            }
        }
    }
}
