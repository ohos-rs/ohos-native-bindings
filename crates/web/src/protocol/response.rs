use std::{
    ffi::CString,
    ptr::{self, NonNull},
};

use ohos_web_sys::{
    ArkWeb_Response, OH_ArkWebResponse_GetCharset, OH_ArkWebResponse_GetError,
    OH_ArkWebResponse_GetHeaderByName, OH_ArkWebResponse_GetMimeType, OH_ArkWebResponse_GetStatus,
    OH_ArkWebResponse_GetStatusText, OH_ArkWebResponse_GetUrl, OH_ArkWeb_CreateResponse,
};

use crate::{NetError, ResponseCharset, ResponseHeaderValue, ResponseMimeType, ResponseStatusText};

pub struct ArkWebResponse {
    raw: NonNull<ArkWeb_Response>,
}

impl Default for ArkWebResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl ArkWebResponse {
    pub fn new() -> Self {
        let mut raw = ptr::null_mut();
        unsafe {
            OH_ArkWeb_CreateResponse(&mut raw);

            #[cfg(debug_assertions)]
            assert!(!raw.is_null(), "Failed to create response");

            Self {
                raw: NonNull::new_unchecked(raw),
            }
        }
    }

    pub fn raw(&self) -> *mut ArkWeb_Response {
        self.raw.as_ptr()
    }

    pub fn charset(&self) -> ResponseCharset {
        let mut charset = ptr::null_mut();
        unsafe { OH_ArkWebResponse_GetCharset(self.raw.as_ptr(), &mut charset) };

        ResponseCharset { raw: charset }
    }

    pub fn header<S: Into<String>>(&self, name: S) -> ResponseHeaderValue {
        let name: String = name.into();
        let name = CString::new(name).unwrap();

        let mut value = ptr::null_mut();
        unsafe {
            OH_ArkWebResponse_GetHeaderByName(
                self.raw.as_ptr(),
                name.as_ptr() as *const _,
                &mut value,
            );
        }
        ResponseHeaderValue { raw: value }
    }

    pub fn status(&self) -> i32 {
        unsafe { OH_ArkWebResponse_GetStatus(self.raw.as_ptr()) }
    }

    pub fn status_text(&self) -> ResponseStatusText {
        let mut status_text = ptr::null_mut();
        unsafe { OH_ArkWebResponse_GetStatusText(self.raw.as_ptr(), &mut status_text) };

        ResponseStatusText { raw: status_text }
    }

    pub fn mime_type(&self) -> ResponseMimeType {
        let mut mime_type = ptr::null_mut();
        unsafe { OH_ArkWebResponse_GetMimeType(self.raw.as_ptr(), &mut mime_type) };

        ResponseMimeType { raw: mime_type }
    }

    pub fn url(&self) -> String {
        let mut url = ptr::null_mut();
        unsafe { OH_ArkWebResponse_GetUrl(self.raw.as_ptr(), &mut url) };

        unsafe {
            let s = std::ffi::CStr::from_ptr(url);
            s.to_string_lossy().to_string()
        }
    }

    pub fn error(&self) -> NetError {
        let err = unsafe { OH_ArkWebResponse_GetError(self.raw.as_ptr()) };

        NetError::from(err)
    }
}
