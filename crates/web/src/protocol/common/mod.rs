use std::ffi::c_char;

use ohos_web_sys::{OH_ArkWeb_ReleaseByteArray, OH_ArkWeb_ReleaseString};

/// Response Charset
pub struct CommonString {
    pub(crate) raw: *mut c_char,
}

impl Drop for CommonString {
    fn drop(&mut self) {
        unsafe {
            OH_ArkWeb_ReleaseString(self.raw);
        }
    }
}

impl Into<String> for CommonString {
    fn into(self) -> String {
        unsafe {
            let s = std::ffi::CStr::from_ptr(self.raw);
            s.to_string_lossy().to_string()
        }
    }
}

/// Common Bytes
pub struct CommonBytes {
    pub(crate) raw: *mut u8,
}

impl Drop for CommonBytes {
    fn drop(&mut self) {
        unsafe {
            OH_ArkWeb_ReleaseByteArray(self.raw);
        }
    }
}

// Define response related types
/// Response Charset
pub type ResponseCharset = CommonString;
/// Response Header Key
pub type ResponseHeaderKey = CommonString;
/// Response Header Value
pub type ResponseHeaderValue = CommonString;
/// Response Mime Type
pub type ResponseMimeType = CommonString;
/// Response Status Text
pub type ResponseStatusText = CommonString;
/// Response Url
pub type Url = CommonString;
/// Response Method
pub type Method = CommonString;
/// Response Referrer
pub type Referrer = CommonString;
