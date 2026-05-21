use ohos_xcomponent_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    OH_NativeXComponent_KeyAction,
    "OH_NativeXComponent_KeyAction_OH_NATIVEXCOMPONENT_KEY_ACTION_"
)]
pub enum Action {
    Unknown,
    Down,
    Up,
}
