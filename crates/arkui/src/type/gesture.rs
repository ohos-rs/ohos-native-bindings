use ohos_arkui_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkUI_GestureRecognizerType, "ArkUI_GestureRecognizerType_")]
pub enum GestureRecognizerType {
    TapGesture,
    LongPressGesture,
    PanGesture,
    PinchGesture,
    RotationGesture,
    SwipeGesture,
    GroupGesture,
}
