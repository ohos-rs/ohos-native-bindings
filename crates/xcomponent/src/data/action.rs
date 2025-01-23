use ohos_xcomponent_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    OH_NativeXComponent_KeyAction,
    "OH_NativeXComponent_KeyAction_OH_NATIVEXCOMPONENT_KEY_ACTION_"
)]
pub enum Action {
    Unknown,
    Down,
    Up,
}
