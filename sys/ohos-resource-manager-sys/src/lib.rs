/* automatically generated by rust-bindgen 0.65.1 */

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use ohos_raw_sys::*;

pub const ResourceManager_ErrorCode_SUCCESS: ResourceManager_ErrorCode = 0;
pub const ResourceManager_ErrorCode_ERROR_CODE_INVALID_INPUT_PARAMETER: ResourceManager_ErrorCode =
    401;
pub const ResourceManager_ErrorCode_ERROR_CODE_RES_ID_NOT_FOUND: ResourceManager_ErrorCode =
    9001001;
pub const ResourceManager_ErrorCode_ERROR_CODE_RES_NOT_FOUND_BY_ID: ResourceManager_ErrorCode =
    9001002;
pub const ResourceManager_ErrorCode_ERROR_CODE_RES_NAME_NOT_FOUND: ResourceManager_ErrorCode =
    9001003;
pub const ResourceManager_ErrorCode_ERROR_CODE_RES_NOT_FOUND_BY_NAME: ResourceManager_ErrorCode =
    9001004;
pub const ResourceManager_ErrorCode_ERROR_CODE_RES_PATH_INVALID: ResourceManager_ErrorCode =
    9001005;
pub const ResourceManager_ErrorCode_ERROR_CODE_RES_REF_TOO_MUCH: ResourceManager_ErrorCode =
    9001006;
pub const ResourceManager_ErrorCode_ERROR_CODE_RES_ID_FORMAT_ERROR: ResourceManager_ErrorCode =
    9001007;
pub const ResourceManager_ErrorCode_ERROR_CODE_RES_NAME_FORMAT_ERROR: ResourceManager_ErrorCode =
    9001008;
pub const ResourceManager_ErrorCode_ERROR_CODE_SYSTEM_RES_MANAGER_GET_FAILED:
    ResourceManager_ErrorCode = 9001009;
pub const ResourceManager_ErrorCode_ERROR_CODE_OVERLAY_RES_PATH_INVALID: ResourceManager_ErrorCode =
    9001010;
pub const ResourceManager_ErrorCode_ERROR_CODE_OUT_OF_MEMORY: ResourceManager_ErrorCode = 9001100;
pub type ResourceManager_ErrorCode = ::std::os::raw::c_uint;
pub const ScreenDensity_SCREEN_SDPI: ScreenDensity = 120;
pub const ScreenDensity_SCREEN_MDPI: ScreenDensity = 160;
pub const ScreenDensity_SCREEN_LDPI: ScreenDensity = 240;
pub const ScreenDensity_SCREEN_XLDPI: ScreenDensity = 320;
pub const ScreenDensity_SCREEN_XXLDPI: ScreenDensity = 480;
pub const ScreenDensity_SCREEN_XXXLDPI: ScreenDensity = 640;
pub type ScreenDensity = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ArkUI_DrawableDescriptor {
    _unused: [u8; 0],
}
extern "C" {
    pub fn OH_ResourceManager_GetMediaBase64(
        mgr: *const NativeResourceManager,
        resId: u32,
        resultValue: *mut *mut ::std::os::raw::c_char,
        resultLen: *mut u64,
        density: u32,
    ) -> ResourceManager_ErrorCode;
}
extern "C" {
    pub fn OH_ResourceManager_GetMediaBase64ByName(
        mgr: *const NativeResourceManager,
        resName: *const ::std::os::raw::c_char,
        resultValue: *mut *mut ::std::os::raw::c_char,
        resultLen: *mut u64,
        density: u32,
    ) -> ResourceManager_ErrorCode;
}
extern "C" {
    pub fn OH_ResourceManager_GetMedia(
        mgr: *const NativeResourceManager,
        resId: u32,
        resultValue: *mut *mut u8,
        resultLen: *mut u64,
        density: u32,
    ) -> ResourceManager_ErrorCode;
}
extern "C" {
    pub fn OH_ResourceManager_GetMediaByName(
        mgr: *const NativeResourceManager,
        resName: *const ::std::os::raw::c_char,
        resultValue: *mut *mut u8,
        resultLen: *mut u64,
        density: u32,
    ) -> ResourceManager_ErrorCode;
}
extern "C" {
    pub fn OH_ResourceManager_GetDrawableDescriptor(
        mgr: *const NativeResourceManager,
        resId: u32,
        drawableDescriptor: *mut *mut ArkUI_DrawableDescriptor,
        density: u32,
        type_: u32,
    ) -> ResourceManager_ErrorCode;
}
extern "C" {
    pub fn OH_ResourceManager_GetDrawableDescriptorByName(
        mgr: *const NativeResourceManager,
        resName: *const ::std::os::raw::c_char,
        drawableDescriptor: *mut *mut ArkUI_DrawableDescriptor,
        density: u32,
        type_: u32,
    ) -> ResourceManager_ErrorCode;
}
