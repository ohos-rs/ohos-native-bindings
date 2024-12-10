use ohos_xcomponent_sys::*;

use enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    OH_NativeXComponent_EventSourceType,
    "OH_NativeXComponent_EventSourceType_OH_NATIVEXCOMPONENT_SOURCE_TYPE_"
)]
pub enum EventSource {
    Unknown,
    Mouse,
    Touchscreen,
    Touchpad,
    Joystick,
    Keyboard,
}
