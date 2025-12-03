#![allow(non_snake_case)]
#![allow(dead_code)]

use napi_sys_ohos::{napi_env, napi_value};
use std::ffi::c_void;

#[repr(C)]
#[derive(Default)]
pub struct OhosPixelMapInfo {
    /// width
    pub width: u32,
    /// height
    pub height: u32,
    /// 每行的bytes数
    pub rowSize: u32,
    /// Pixel的格式
    pub pixelFormat: PixelFormat,
}

#[repr(C)]
#[derive(PartialEq, Debug, Default)]
pub enum PixelFormat {
    #[default]
    OhosPixelMapFormatNone = 0,
    OhosPixelMapFormatRgba8888 = 3,
    OhosPixelMapFormatRgb565 = 2,
}

#[repr(C)]
#[derive(PartialEq)]
pub enum PixelResult {
    OhosImageResultBadParameter = -1,
    OhosImageResultSuccess = 0,
}

extern "C" {
    pub fn OH_GetImageInfo(
        env: napi_env,
        value: napi_value,
        info: &OhosPixelMapInfo,
    ) -> PixelResult;
    pub fn OH_AccessPixels(
        env: napi_env,
        value: napi_value,
        addr_ptr: *mut *mut c_void,
    ) -> PixelResult;
    pub fn OH_UnAccessPixels(env: napi_env, value: napi_value) -> PixelResult;
}
