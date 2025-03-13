use std::{
    ffi::{CStr, CString},
    ptr::NonNull,
};

use ohos_udmf_sys::{
    OH_UdsHtml, OH_UdsHtml_Create, OH_UdsHtml_Destroy, OH_UdsHtml_GetContent,
    OH_UdsHtml_GetPlainContent, OH_UdsHtml_SetContent, OH_UdsHtml_SetPlainContent,
};

use crate::UdmfError;

use super::Uds;

pub struct UdsHtml {
    pub(crate) raw: NonNull<OH_UdsHtml>,
}

impl UdsHtml {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { OH_UdsHtml_Create() }).expect("OH_UdsHtml_Create failed");
        Self { raw }
    }

    pub fn from_raw(raw: *mut OH_UdsHtml) -> Self {
        Self {
            raw: NonNull::new(raw).expect("OH_UdsHtml_Create create from a raw pointer failed"),
        }
    }

    pub fn init_with_primary_content<T: AsRef<str>>(
        &mut self,
        content: T,
    ) -> Result<Self, UdmfError> {
        let ret = UdsHtml::new();
        ret.set_primary_content(content)?;
        Ok(ret)
    }

    pub fn init_with_html<T: AsRef<str>>(&mut self, html: T) -> Result<Self, UdmfError> {
        let ret = UdsHtml::new();
        ret.set_html(html)?;
        Ok(ret)
    }

    pub fn set_primary_content<T: AsRef<str>>(&self, content: T) -> Result<(), UdmfError> {
        let s = CString::new(content.as_ref()).expect("CString::new failed");
        let ret = unsafe { OH_UdsHtml_SetPlainContent(self.raw.as_ptr(), s.as_ptr().cast()) };
        if ret != 0 {
            return Err(UdmfError::InternalError(ret));
        }
        Ok(())
    }

    pub fn set_html<T: AsRef<str>>(&self, content: T) -> Result<(), UdmfError> {
        let s = CString::new(content.as_ref()).expect("CString::new failed");
        let ret = unsafe { OH_UdsHtml_SetContent(self.raw.as_ptr(), s.as_ptr().cast()) };
        if ret != 0 {
            return Err(UdmfError::InternalError(ret));
        }
        Ok(())
    }

    pub fn get_primary_content(&self) -> Result<String, UdmfError> {
        let ret = unsafe { OH_UdsHtml_GetPlainContent(self.raw.as_ptr()) };
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

    pub fn get_html(&self) -> Result<String, UdmfError> {
        let ret = unsafe { OH_UdsHtml_GetContent(self.raw.as_ptr()) };
        if ret.is_null() {
            return Err(UdmfError::UdsInitError(String::from(
                "OH_UdsHtml_GetContent call failed",
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

impl Default for UdsHtml {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for UdsHtml {
    fn drop(&mut self) {
        unsafe { OH_UdsHtml_Destroy(self.raw.as_ptr()) }
    }
}

impl From<UdsHtml> for Uds {
    fn from(value: UdsHtml) -> Self {
        Uds::Html(value)
    }
}
