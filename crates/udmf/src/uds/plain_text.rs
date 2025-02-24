use std::{
    ffi::{CStr, CString},
    ptr::NonNull,
};

use ohos_udmf_sys::{
    OH_UdsPlainText, OH_UdsPlainText_Create, OH_UdsPlainText_Destroy, OH_UdsPlainText_GetContent,
    OH_UdsPlainText_SetContent,
};

use crate::UdmfError;

pub struct UdsPlainText {
    pub(crate) raw: NonNull<OH_UdsPlainText>,
}

impl UdsPlainText {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { OH_UdsPlainText_Create() })
            .expect("OH_UdsPlainTextCreate failed");
        Self { raw }
    }

    pub fn init_with_text<T: AsRef<str>>(&mut self, text: T) -> Result<Self, UdmfError> {
        let ret = UdsPlainText::new();
        ret.set_content(text)?;
        Ok(ret)
    }

    pub fn set_content<T: AsRef<str>>(&self, content: T) -> Result<(), UdmfError> {
        let s = CString::new(content.as_ref()).expect("CString::new failed");
        let ret = unsafe { OH_UdsPlainText_SetContent(self.raw.as_ptr(), s.as_ptr().cast()) };
        if ret != 0 {
            return Err(UdmfError::IntervalError(ret));
        }
        Ok(())
    }

    pub fn get_content(&self) -> Result<String, UdmfError> {
        let ret = unsafe { OH_UdsPlainText_GetContent(self.raw.as_ptr()) };
        if ret.is_null() {
            return Err(UdmfError::IntervalError(-1));
        }
        let c_str = unsafe { CStr::from_ptr(ret) };
        let text = c_str
            .to_str()
            .map_err(|_| UdmfError::IntervalError(-1))?
            .to_string();
        Ok(text)
    }
}

impl Drop for UdsPlainText {
    fn drop(&mut self) {
        unsafe { OH_UdsPlainText_Destroy(self.raw.as_ptr()) }
    }
}
