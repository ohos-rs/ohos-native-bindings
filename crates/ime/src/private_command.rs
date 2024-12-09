use std::ffi::CString;

use ohos_input_method_sys::{
    InputMethod_PrivateCommand, OH_PrivateCommand_Create, OH_PrivateCommand_Destroy,
};

pub struct PrivateCommand {
    pub(crate) raw: *mut InputMethod_PrivateCommand,
}

impl PrivateCommand {
    pub fn new<T: AsRef<str>>(key: T) -> Self {
        let k = key.as_ref();
        let c_string = CString::new(k).expect("CString::new failed");
        let raw = unsafe { OH_PrivateCommand_Create(c_string.into_raw(), k.len()) };
        PrivateCommand { raw }
    }
}

impl Drop for PrivateCommand {
    fn drop(&mut self) {
        unsafe {
            OH_PrivateCommand_Destroy(self.raw);
        }
    }
}
