/* automatically generated by rust-bindgen 0.65.1 */

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OH_NativeBundle_ApplicationInfo {
    pub bundleName: *mut ::std::os::raw::c_char,
    pub fingerprint: *mut ::std::os::raw::c_char,
}
extern "C" {
    pub fn OH_NativeBundle_GetCurrentApplicationInfo() -> OH_NativeBundle_ApplicationInfo;
}
extern "C" {
    pub fn OH_NativeBundle_GetAppId() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn OH_NativeBundle_GetAppIdentifier() -> *mut ::std::os::raw::c_char;
}
