use std::{
    ffi::{CStr, CString},
    ptr::NonNull,
    str::FromStr,
};

use ohos_udmf_sys::{
    OH_UdsPlainText, OH_UdsPlainText_Create, OH_UdsPlainText_Destroy, OH_UdsPlainText_GetContent,
    OH_UdsPlainText_GetType, OH_UdsPlainText_SetContent,
};

use crate::{UdmfError, UdmfMeta};

use super::{Uds, UdsValue};

pub struct UdsPlainText {
    pub(crate) raw: NonNull<OH_UdsPlainText>,
}

impl UdsPlainText {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { OH_UdsPlainText_Create() })
            .expect("OH_UdsPlainTextCreate failed");
        Self { raw }
    }

    pub fn from_raw(raw: *mut OH_UdsPlainText) -> Self {
        Self {
            raw: NonNull::new(raw).expect("OH_UdsPlainTextCreate create from a raw pointer failed"),
        }
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
            return Err(UdmfError::InternalError(ret));
        }
        Ok(())
    }

    pub fn get_content(&self) -> Result<String, UdmfError> {
        let ret = unsafe { OH_UdsPlainText_GetContent(self.raw.as_ptr()) };
        if ret.is_null() {
            return Err(UdmfError::UdsInitError(String::from(
                "OH_UdsPlainText_GetContent call failed",
            )));
        }
        let c_str = unsafe { CStr::from_ptr(ret) };
        let text = c_str
            .to_str()
            .map_err(|e| UdmfError::CommonError(e.to_string()))?
            .to_string();
        Ok(text)
    }
}

impl Drop for UdsPlainText {
    fn drop(&mut self) {
        unsafe { OH_UdsPlainText_Destroy(self.raw.as_ptr()) }
    }
}

impl From<UdsPlainText> for Uds {
    fn from(value: UdsPlainText) -> Self {
        Uds::PlainText(value)
    }
}

impl UdsValue for UdsPlainText {
    fn get_type(&self) -> Result<UdmfMeta, UdmfError> {
        let ret = unsafe { OH_UdsPlainText_GetType(self.raw.as_ptr()) };
        let s = unsafe { CStr::from_ptr(ret) }
            .to_str()
            .map_err(|e| UdmfError::CommonError(e.to_string()))?;
        UdmfMeta::from_str(s)
            .map_err(|_| UdmfError::CommonError(String::from("UdmfMeta::from_str failed")))
    }
}

impl Default for UdsPlainText {
    fn default() -> Self {
        Self::new()
    }
}
