use ohos_xcomponent_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    OH_NativeXComponent_MouseEventButton,
    "OH_NativeXComponent_MouseEventAction_OH_NATIVEXCOMPONENT_MOUSE_"
)]
pub enum MouseEvent {
    None,
    Press,
    Release,
    Move,
}
