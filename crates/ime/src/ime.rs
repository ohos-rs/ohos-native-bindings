use ohos_input_method_sys::{
    InputMethod_InputMethodProxy, OH_InputMethodController_Attach,
    OH_InputMethodProxy_HideKeyboard, OH_InputMethodProxy_ShowKeyboard,
};
use std::{cell::RefCell, ptr, rc::Rc};

use crate::{
    common::IME_INSTANCE, private_command::PrivateCommand, Action, AttachOptions, Direction,
    EnterKey, KeyboardStatus, Selection, TextConfig, TextEditor,
};

#[derive(Clone)]
pub struct IME {
    raw: *mut InputMethod_InputMethodProxy,
    text_editor: TextEditor,
    pub(crate) delete_backward: Rc<RefCell<Option<Box<dyn Fn(i32) -> ()>>>>,
    pub(crate) insert_text: Rc<RefCell<Option<Box<dyn Fn(String) -> ()>>>>,
    pub(crate) delete_forward: Rc<RefCell<Option<Box<dyn Fn(i32) -> ()>>>>,
    pub(crate) finish_text_preview: Rc<RefCell<Option<Box<dyn Fn() -> ()>>>>,
    pub(crate) get_left_text_of_cursor: Rc<RefCell<Option<Box<dyn Fn(i32) -> String>>>>,
    pub(crate) get_right_text_of_cursor: Rc<RefCell<Option<Box<dyn Fn(i32) -> String>>>>,
    pub(crate) get_text_config: Rc<RefCell<Option<Box<dyn Fn(TextConfig) -> ()>>>>,
    pub(crate) get_text_index_at_cursor: Rc<RefCell<Option<Box<dyn Fn() -> i32>>>>,
    pub(crate) handle_extend_action: Rc<RefCell<Option<Box<dyn Fn(Action) -> ()>>>>,
    pub(crate) handle_set_selection: Rc<RefCell<Option<Box<dyn Fn(Selection) -> ()>>>>,
    pub(crate) move_cursor: Rc<RefCell<Option<Box<dyn Fn(Direction) -> ()>>>>,
    pub(crate) receive_private_command: Rc<RefCell<Option<Box<dyn Fn(Vec<PrivateCommand>) -> ()>>>>,
    pub(crate) send_enter_key: Rc<RefCell<Option<Box<dyn Fn(EnterKey) -> ()>>>>,
    pub(crate) send_keyboard_status: Rc<RefCell<Option<Box<dyn Fn(KeyboardStatus) -> ()>>>>,
    pub(crate) set_preview_text: Rc<RefCell<Option<Box<dyn Fn(String, i32, i32) -> ()>>>>,
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
            delete_forward: Rc::new(RefCell::new(None)),
            finish_text_preview: Rc::new(RefCell::new(None)),
            get_left_text_of_cursor: Rc::new(RefCell::new(None)),
            get_right_text_of_cursor: Rc::new(RefCell::new(None)),
            get_text_config: Rc::new(RefCell::new(None)),
            get_text_index_at_cursor: Rc::new(RefCell::new(None)),
            handle_extend_action: Rc::new(RefCell::new(None)),
            handle_set_selection: Rc::new(RefCell::new(None)),
            move_cursor: Rc::new(RefCell::new(None)),
            receive_private_command: Rc::new(RefCell::new(None)),
            send_enter_key: Rc::new(RefCell::new(None)),
            send_keyboard_status: Rc::new(RefCell::new(None)),
            set_preview_text: Rc::new(RefCell::new(None)),
        };

        let mut instance = IME_INSTANCE.write().unwrap();
        instance.insert(ime.text_editor.raw as usize, Box::new(ime.clone()));

        ime
    }

    pub fn insert_text<T>(&self, callback: T)
    where
        T: Fn(String) -> () + 'static,
    {
        *self.insert_text.borrow_mut() = Some(Box::new(callback));
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
