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
#[enum_from_config(UI_TOUCH_EVENT_ACTION, "UI_TOUCH_EVENT_ACTION_")]
pub enum UIInputAction {
    Cancel,
    Down,
    Move,
    Up,
}

impl From<UIInputAction> for i32 {
    fn from(value: UIInputAction) -> Self {
        match value {
            UIInputAction::Cancel => UI_TOUCH_EVENT_ACTION_CANCEL as i32,
            UIInputAction::Down => UI_TOUCH_EVENT_ACTION_DOWN as i32,
            UIInputAction::Move => UI_TOUCH_EVENT_ACTION_MOVE as i32,
            UIInputAction::Up => UI_TOUCH_EVENT_ACTION_UP as i32,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(UI_INPUT_EVENT_TOOL_TYPE, "UI_INPUT_EVENT_TOOL_TYPE_")]
pub enum UIInputToolType {
    Unknown,
    Finger,
    Pen,
    Mouse,
    Touchpad,
    Joystick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(UI_INPUT_EVENT_SOURCE_TYPE, "UI_INPUT_EVENT_SOURCE_TYPE_")]
pub enum UIInputSourceType {
    Unknown,
    Mouse,
    TouchScreen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(UI_MOUSE_EVENT_ACTION, "UI_MOUSE_EVENT_ACTION_")]
pub enum UIMouseEventAction {
    Unknown,
    Press,
    Release,
    Move,
    #[cfg(feature = "api-20")]
    Cancel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(UI_MOUSE_EVENT_BUTTON, "UI_MOUSE_EVENT_BUTTON_")]
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
#[enum_from_config(UI_FOCUS_AXIS_EVENT_ABS, "UI_FOCUS_AXIS_EVENT_ABS_")]
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
#[enum_from_config(UI_AXIS_EVENT_ACTION, "UI_AXIS_EVENT_ACTION_")]
pub enum UIAxisEventAction {
    None,
    Begin,
    Update,
    End,
    Cancel,
}

#[cfg(feature = "api-22")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    ArkUI_CoastingAxisEventPhase,
    "ArkUI_CoastingAxisEventPhase_ARKUI_COASTING_AXIS_EVENT_PHASE_"
)]
pub enum UICoastingAxisEventPhase {
    None,
    Begin,
    Update,
    End,
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
