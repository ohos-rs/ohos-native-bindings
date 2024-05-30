use std::ffi::CStr;

use ohos_bundle_sys::{
    OH_NativeBundle_GetAppId, OH_NativeBundle_GetAppIdentifier,
    OH_NativeBundle_GetCurrentApplicationInfo,
};

#[derive(Debug, Clone, Copy)]
pub struct BundleInfo<'a> {
    pub bundle_name: &'a str,
    pub fingerprint: &'a str,
}

/// get current application info
pub fn get_bundle_info() -> BundleInfo<'static> {
    let info = unsafe { OH_NativeBundle_GetCurrentApplicationInfo() };
    BundleInfo {
        bundle_name: unsafe { CStr::from_ptr(info.bundleName).to_str().unwrap_or("") },
        fingerprint: unsafe { CStr::from_ptr(info.fingerprint).to_str().unwrap_or("") },
    }
}

/// get appid
pub fn get_appid() -> &'static str {
    unsafe {
        let id = OH_NativeBundle_GetAppId();
        CStr::from_ptr(id).to_str().unwrap_or("")
    }
}

/// get identifier which can't be changed
pub fn get_app_identifier() -> &'static str {
    unsafe {
        let id = OH_NativeBundle_GetAppIdentifier();
        CStr::from_ptr(id).to_str().unwrap_or("")
    }
}
