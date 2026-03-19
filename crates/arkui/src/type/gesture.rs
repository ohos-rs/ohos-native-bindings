//! Module type::gesture wrappers and related types.

use ohos_arkui_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkUI_GestureRecognizerType, "ArkUI_GestureRecognizerType_")]
/// Recognizer categories supported by ArkUI.
pub enum GestureRecognizerType {
    TapGesture,
    LongPressGesture,
    PanGesture,
    PinchGesture,
    RotationGesture,
    SwipeGesture,
    GroupGesture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    ArkUI_GestureInterruptResult,
    "ArkUI_GestureInterruptResult_GESTURE_INTERRUPT_RESULT_"
)]
/// Decision returned by a gesture interrupter callback.
pub enum GestureInterruptResult {
    Continue,
    Reject,
}
