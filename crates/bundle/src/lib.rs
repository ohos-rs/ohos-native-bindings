use std::ffi::CStr;

use ohos_bundle_sys::{
    OH_NativeBundle_GetAppId, OH_NativeBundle_GetAppIdentifier,
    OH_NativeBundle_GetCurrentApplicationInfo,
};

#[cfg(feature = "api-13")]
use ohos_bundle_sys::OH_NativeBundle_GetMainElementName;

#[cfg(feature = "api-14")]
use ohos_bundle_sys::OH_NativeBundle_GetCompatibleDeviceType;

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

#[cfg(feature = "api-13")]
#[derive(Debug, Clone, Copy)]
pub struct MainElementName {
    pub bundle_name: &'static str,
    pub module_name: &'static str,
    pub ability_name: &'static str,
}

/// get main element name
#[cfg(feature = "api-13")]
pub fn get_main_element_name() -> MainElementName {
    unsafe {
        let name = OH_NativeBundle_GetMainElementName();
        MainElementName {
            bundle_name: CStr::from_ptr(name.bundleName).to_str().unwrap_or(""),
            module_name: CStr::from_ptr(name.moduleName).to_str().unwrap_or(""),
            ability_name: CStr::from_ptr(name.abilityName).to_str().unwrap_or(""),
        }
    }
}

/// get compatible device type
#[cfg(feature = "api-14")]
pub fn get_compatible_device_type() -> &'static str {
    unsafe {
        let t = OH_NativeBundle_GetCompatibleDeviceType();
        CStr::from_ptr(t).to_str().unwrap_or("")
    }
}
