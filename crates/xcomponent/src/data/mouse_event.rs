use ohos_xcomponent_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    OH_NativeXComponent_MouseEventAction,
    "OH_NativeXComponent_MouseEventButton_OH_NATIVEXCOMPONENT_"
)]
pub enum MouseButton {
    NoneButton,
    LeftButton,
    RightButton,
    MiddleButton,
    BackButton,
    ForwardButton,
}
