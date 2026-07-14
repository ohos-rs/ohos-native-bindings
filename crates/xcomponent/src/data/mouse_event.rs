use ohos_xcomponent_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    OH_NativeXComponent_MouseEventAction,
    "OH_NativeXComponent_MouseEventAction_OH_NATIVEXCOMPONENT_MOUSE_"
)]
pub enum MouseAction {
    None,
    Press,
    Release,
    Move,
    #[cfg(feature = "api-18")]
    Cancel,
}
