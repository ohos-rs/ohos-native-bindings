use ohos_input_method_sys::{
    InputMethod_InputMethodProxy, OH_InputMethodController_Attach,
    OH_InputMethodProxy_HideKeyboard, OH_InputMethodProxy_ShowKeyboard,
};
use std::{cell::RefCell, ptr, rc::Rc};

use crate::{common::IME_INSTANCE, AttachOptions, TextEditor};

#[derive(Clone)]
pub struct IME {
    raw: *mut InputMethod_InputMethodProxy,
    text_editor: TextEditor,
    pub(crate) delete_backward: Rc<RefCell<Option<Box<dyn Fn(i32) -> ()>>>>,
    pub(crate) insert_text: Rc<RefCell<Option<Box<dyn Fn(String) -> ()>>>>,
}

unsafe impl Send for IME {}
unsafe impl Sync for IME {}

impl IME {
    pub fn new(option: AttachOptions) -> Self {
        let editor = TextEditor::new();

        let mut raw: *mut InputMethod_InputMethodProxy = ptr::null_mut();
        let _ret = unsafe {
            OH_InputMethodController_Attach(
                editor.raw,
                option.raw,
                &mut raw as *mut *mut InputMethod_InputMethodProxy,
            )
        };

        let ime = IME {
            raw,
            text_editor: editor,
            delete_backward: Rc::new(RefCell::new(None)),
            insert_text: Rc::new(RefCell::new(None)),
        };

        let mut instance = IME_INSTANCE.write().unwrap();
        instance.insert(ime.text_editor.raw as usize, Box::new(ime.clone()));

        ime
    }

    pub fn on_delete_backward<F>(&self, f: F)
    where
        F: Fn(i32) -> () + 'static,
    {
        *self.delete_backward.borrow_mut() = Some(Box::new(f));

        self.text_editor.set_delete_backward();
    }

    pub fn on_insert_text<F>(&self, f: F)
    where
        F: Fn(String) -> () + 'static,
    {
        *self.insert_text.borrow_mut() = Some(Box::new(f));

        self.text_editor.set_insert_text();
    }

    pub fn show_keyboard(&self) {
        unsafe {
            OH_InputMethodProxy_ShowKeyboard(self.raw);
        }
    }

    pub fn hide_keyboard(&self) {
        unsafe {
            OH_InputMethodProxy_HideKeyboard(self.raw);
        }
    }
}
