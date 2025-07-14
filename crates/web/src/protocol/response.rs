use std::{
    ffi::CString,
    ptr::{self, NonNull},
};

use ohos_web_sys::{
    ArkWeb_Response, OH_ArkWebResponse_GetCharset, OH_ArkWebResponse_GetError,
    OH_ArkWebResponse_GetHeaderByName, OH_ArkWebResponse_GetMimeType, OH_ArkWebResponse_GetStatus,
    OH_ArkWebResponse_GetStatusText, OH_ArkWebResponse_GetUrl, OH_ArkWebResponse_SetCharset,
    OH_ArkWebResponse_SetError, OH_ArkWebResponse_SetHeaderByName, OH_ArkWebResponse_SetMimeType,
    OH_ArkWebResponse_SetStatus, OH_ArkWebResponse_SetStatusText, OH_ArkWebResponse_SetUrl,
    OH_ArkWeb_CreateResponse,
};

use crate::{
    NetError, ResponseCharset, ResponseHeaderValue, ResponseMimeType, ResponseStatusText, Url,
};

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

    pub fn url(&self) -> Url {
        let mut url = ptr::null_mut();
        unsafe { OH_ArkWebResponse_GetUrl(self.raw.as_ptr(), &mut url) };

        Url { raw: url }
    }

    pub fn error(&self) -> NetError {
        let err = unsafe { OH_ArkWebResponse_GetError(self.raw.as_ptr()) };

        NetError::from(err)
    }

    pub fn set_charset<S: Into<String>>(&self, charset: S) {
        let chars: String = charset.into();
        let charset = CString::new(chars).unwrap();

        unsafe {
            OH_ArkWebResponse_SetCharset(self.raw.as_ptr(), charset.as_ptr().cast());
        };
    }

    pub fn set_header<S: Into<String>>(&self, name: S, value: S, overwrite: bool) {
        let name: String = name.into();
        let name = CString::new(name).unwrap();

        let value: String = value.into();
        let value = CString::new(value).unwrap();

        unsafe {
            OH_ArkWebResponse_SetHeaderByName(
                self.raw.as_ptr(),
                name.as_ptr().cast(),
                value.as_ptr().cast(),
                overwrite,
            );
        };
    }

    pub fn set_net_error(&self, error: NetError) {
        unsafe {
            OH_ArkWebResponse_SetError(self.raw.as_ptr(), error.into());
        };
    }

    pub fn set_status(&self, status: i32) {
        unsafe {
            OH_ArkWebResponse_SetStatus(self.raw.as_ptr(), status);
        };
    }

    pub fn set_status_text<S: Into<String>>(&self, status_text: S) {
        let status_text: String = status_text.into();
        let status_text = CString::new(status_text).unwrap();

        unsafe {
            OH_ArkWebResponse_SetStatusText(self.raw.as_ptr(), status_text.as_ptr().cast());
        };
    }

    pub fn set_url<S: Into<String>>(&self, url: S) {
        let url: String = url.into();
        let url = CString::new(url).unwrap();

        unsafe {
            OH_ArkWebResponse_SetUrl(self.raw.as_ptr(), url.as_ptr().cast());
        };
    }

    pub fn set_mime_type<S: Into<String>>(&self, mime_type: S) {
        let mime_type: String = mime_type.into();
        let mime_type = CString::new(mime_type).unwrap();

        unsafe {
            OH_ArkWebResponse_SetMimeType(self.raw.as_ptr(), mime_type.as_ptr().cast());
        };
    }
}
