use ohos_xcomponent_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    OH_NativeXComponent_TouchEventType,
    "OH_NativeXComponent_TouchEventType_OH_NATIVEXCOMPONENT_"
)]
pub enum TouchEvent {
    Down,
    Up,
    Move,
    Cancel,
    Unknown,
}
