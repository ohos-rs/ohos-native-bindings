use ohos_input_method_sys::{
    InputMethod_InputMethodProxy, OH_InputMethodController_Attach, OH_InputMethodController_Detach,
    OH_InputMethodProxy_HideKeyboard, OH_InputMethodProxy_ShowKeyboard,
};
use std::{
    cell::RefCell,
    ptr::{self, NonNull},
    rc::Rc,
};

use crate::{proxy::OHOS_RS_IME_CALLBACKS, AttachOptions, TextEditor};

unsafe impl Send for IME {}
unsafe impl Sync for IME {}

#[derive(Clone)]
pub struct IME {
    raw: Rc<RefCell<Option<NonNull<InputMethod_InputMethodProxy>>>>,
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
        let mut guard = OHOS_RS_IME_CALLBACKS
            .write()
            .expect("OHOS_RS_IME_CALLBACKS write failed");
        let cb = unsafe {
            std::mem::transmute::<Box<dyn Fn(String) + 'a>, Box<dyn Fn(String) + 'static>>(
                Box::new(callback),
            )
        };
        (*guard).insert_text = Some(cb);
    }

    pub fn show_keyboard(&self) {
        if let Some(ime_proxy) = *self.raw.borrow() {
            unsafe {
                let ret = OH_InputMethodProxy_ShowKeyboard(ime_proxy.as_ptr());
                #[cfg(debug_assertions)]
                assert!(ret == 0, "OH_InputMethodProxy_ShowKeyboard failed");
            }
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
            self.raw.replace(Some(NonNull::new_unchecked(raw)));
            #[cfg(debug_assertions)]
            assert!(ret == 0, "OH_InputMethodController_Attach failed");
        }
    }

    pub fn hide_keyboard(&self) {
        unsafe {
            if let Some(raw) = *self.raw.borrow() {
                let ret = OH_InputMethodProxy_HideKeyboard(raw.as_ptr());

                #[cfg(debug_assertions)]
                assert!(ret == 0, "OH_InputMethodProxy_HideKeyboard failed");
            }
        }
    }
}

impl Drop for IME {
    fn drop(&mut self) {
        if let Some(raw) = *self.raw.borrow() {
            unsafe {
                let ret = OH_InputMethodController_Detach(raw.as_ptr());
                #[cfg(debug_assertions)]
                assert!(ret == 0, "OH_InputMethodController_Detach failed");
            }
        }
    }
}
