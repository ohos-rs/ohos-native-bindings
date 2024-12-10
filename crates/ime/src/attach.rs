use ohos_input_method_sys::{
    InputMethod_AttachOptions, OH_AttachOptions_Create, OH_AttachOptions_Destroy,
    OH_AttachOptions_IsShowKeyboard,
};

pub struct AttachOptions {
    pub(crate) raw: *mut InputMethod_AttachOptions,
}

impl AttachOptions {
    pub fn new(show_keyboard: bool) -> Self {
        let raw = unsafe { OH_AttachOptions_Create(show_keyboard) };
        AttachOptions { raw }
    }

    pub fn is_showing_keyboard(&self) -> bool {
        let mut is_show = false;
        unsafe { OH_AttachOptions_IsShowKeyboard(self.raw, &mut is_show) };
        is_show
    }
}

impl Default for AttachOptions {
    fn default() -> Self {
        Self::new(true)
    }
}

impl Drop for AttachOptions {
    fn drop(&mut self) {
        unsafe {
            OH_AttachOptions_Destroy(self.raw);
        }
    }
}
