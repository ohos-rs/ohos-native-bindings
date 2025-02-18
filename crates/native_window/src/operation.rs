use ohos_native_window_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(NativeWindowOperation, "NativeWindowOperation_")]
pub enum Operation {
    SetBufferGeometry,
    GetBufferGeometry,
    GetFormat,
    SetFormat,
    GetUsage,
    SetUsage,
    GetStride,
    SetStride,
    SetSwapInterval,
    GetSwapInterval,
    SetTimeout,
    GetTimeout,
    SetColorGamut,
    GetColorGamut,
    SetTransform,
    GetTransform,
    SetUITimestamp,
    GetBufferqueueSize,
    SetSourceType,
    GetSourceType,
    SetAppFrameworkType,
    GetAppFrameworkType,
    SetHDRWhitePointBrightness,
    SetSDRWhitePointBrightness,
    SetDesiredPresentTimestamp,
}

impl Operation {
    pub fn into_i32(self) -> i32 {
        let raw: NativeWindowOperation = self.into();
        raw as i32
    }
}
