use ohos_input_method_sys::{
    InputMethod_TextEditorProxy, OH_TextEditorProxy_Create, OH_TextEditorProxy_Destroy,
    OH_TextEditorProxy_SetDeleteBackwardFunc, OH_TextEditorProxy_SetInsertTextFunc,
};

use crate::proxy::{delete_backward, insert_text};

#[derive(Clone)]
pub struct TextEditor {
    pub(crate) raw: *mut InputMethod_TextEditorProxy,
}

impl TextEditor {
    pub fn new() -> Self {
        let raw = unsafe { OH_TextEditorProxy_Create() };
        TextEditor { raw }
    }

    pub fn set_delete_backward(&self) {
        unsafe {
            OH_TextEditorProxy_SetDeleteBackwardFunc(self.raw, Some(delete_backward));
        }
    }

    pub fn set_insert_text(&self) {
        unsafe {
            OH_TextEditorProxy_SetInsertTextFunc(self.raw, Some(insert_text));
        }
    }
}

impl Drop for TextEditor {
    fn drop(&mut self) {
        unsafe {
            OH_TextEditorProxy_Destroy(self.raw);
        }
    }
}
