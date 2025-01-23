use ohos_xcomponent_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
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
