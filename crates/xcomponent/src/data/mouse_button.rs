use ohos_xcomponent_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    OH_NativeXComponent_MouseEventButton,
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
