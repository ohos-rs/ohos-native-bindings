use ohos_arkui_input_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    ArkUI_UIInputEvent_Type,
    "ArkUI_UIInputEvent_Type_ARKUI_UIINPUTEVENT_TYPE_"
)]
pub enum UIInputEvent {
    Unknown,
    Touch,
    Axis,
    Mouse,
    #[cfg(feature = "api-20")]
    Key,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(_bindgen_ty_1, "UI_TOUCH_EVENT_ACTION_")]
pub enum UIInputAction {
    Cancel,
    Down,
    Move,
    Up,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(_bindgen_ty_2, "UI_INPUT_EVENT_TOOL_TYPE_")]
pub enum UIInputToolType {
    Unknown,
    Finger,
    Pen,
    Mouse,
    Touchpad,
    Joystick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(_bindgen_ty_3, "UI_INPUT_EVENT_SOURCE_TYPE_")]
pub enum UIInputSourceType {
    Unknown,
    Mouse,
    TouchScreen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(_bindgen_ty_4, "UI_MOUSE_EVENT_ACTION_")]
pub enum UIMouseEventAction {
    Unknown,
    Press,
    Release,
    Move,
    #[cfg(feature = "api-20")]
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(_bindgen_ty_5, "UI_MOUSE_EVENT_BUTTON_")]
pub enum UIMouseEventButton {
    None,
    Left,
    Right,
    Middle,
    Back,
    Forward,
}

#[cfg(feature = "api-15")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(_bindgen_ty_6, "UI_FOCUS_AXIS_EVENT_ABS_")]
pub enum UIFocusAxisEventAbs {
    X,
    Y,
    Z,
    RZ,
    BRAKE,
    HAT0X,
    HAT0Y,
}

#[cfg(feature = "api-15")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(_bindgen_ty_7, "UI_AXIS_EVENT_ACTION_")]
pub enum UIAxisEventAction {
    None,
    Begin,
    Update,
    End,
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(HitTestMode, "HitTestMode_HTM_")]
pub enum HitTest {
    Default,
    Block,
    Transparent,
    None,
    #[cfg(feature = "api-20")]
    BlockHierarchy,
    #[cfg(feature = "api-20")]
    BlockDescendants,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkUI_ModifierKeyName, "ArkUI_ModifierKeyName_ARKUI_MODIFIER_KEY_")]
pub enum ModifierKey {
    Ctrl,
    Shift,
    Alt,
    Fn,
}

#[cfg(feature = "api-15")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkUI_InteractionHand, "ArkUI_InteractionHand_ARKUI_EVENT_HAND_")]
pub enum InteractionHand {
    None,
    Left,
    Right,
}
