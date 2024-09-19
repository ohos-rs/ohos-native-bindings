/* automatically generated by rust-bindgen 0.65.1 */

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ArkUI_UIInputEvent {
    _unused: [u8; 0],
}
pub const ArkUI_UIInputEvent_Type_ARKUI_UIINPUTEVENT_TYPE_UNKNOWN: ArkUI_UIInputEvent_Type = 0;
pub const ArkUI_UIInputEvent_Type_ARKUI_UIINPUTEVENT_TYPE_TOUCH: ArkUI_UIInputEvent_Type = 1;
pub const ArkUI_UIInputEvent_Type_ARKUI_UIINPUTEVENT_TYPE_AXIS: ArkUI_UIInputEvent_Type = 2;
pub const ArkUI_UIInputEvent_Type_ARKUI_UIINPUTEVENT_TYPE_MOUSE: ArkUI_UIInputEvent_Type = 3;
pub type ArkUI_UIInputEvent_Type = ::std::os::raw::c_uint;
pub const UI_TOUCH_EVENT_ACTION_CANCEL: _bindgen_ty_1 = 0;
pub const UI_TOUCH_EVENT_ACTION_DOWN: _bindgen_ty_1 = 1;
pub const UI_TOUCH_EVENT_ACTION_MOVE: _bindgen_ty_1 = 2;
pub const UI_TOUCH_EVENT_ACTION_UP: _bindgen_ty_1 = 3;
pub type _bindgen_ty_1 = ::std::os::raw::c_uint;
pub const UI_INPUT_EVENT_TOOL_TYPE_UNKNOWN: _bindgen_ty_2 = 0;
pub const UI_INPUT_EVENT_TOOL_TYPE_FINGER: _bindgen_ty_2 = 1;
pub const UI_INPUT_EVENT_TOOL_TYPE_PEN: _bindgen_ty_2 = 2;
pub const UI_INPUT_EVENT_TOOL_TYPE_MOUSE: _bindgen_ty_2 = 3;
pub const UI_INPUT_EVENT_TOOL_TYPE_TOUCHPAD: _bindgen_ty_2 = 4;
pub const UI_INPUT_EVENT_TOOL_TYPE_JOYSTICK: _bindgen_ty_2 = 5;
pub type _bindgen_ty_2 = ::std::os::raw::c_uint;
pub const UI_INPUT_EVENT_SOURCE_TYPE_UNKNOWN: _bindgen_ty_3 = 0;
pub const UI_INPUT_EVENT_SOURCE_TYPE_MOUSE: _bindgen_ty_3 = 1;
pub const UI_INPUT_EVENT_SOURCE_TYPE_TOUCH_SCREEN: _bindgen_ty_3 = 2;
pub type _bindgen_ty_3 = ::std::os::raw::c_uint;
pub const HitTestMode_HTM_DEFAULT: HitTestMode = 0;
pub const HitTestMode_HTM_BLOCK: HitTestMode = 1;
pub const HitTestMode_HTM_TRANSPARENT: HitTestMode = 2;
pub const HitTestMode_HTM_NONE: HitTestMode = 3;
pub type HitTestMode = ::std::os::raw::c_uint;
pub const UI_MOUSE_EVENT_ACTION_UNKNOWN: _bindgen_ty_4 = 0;
pub const UI_MOUSE_EVENT_ACTION_PRESS: _bindgen_ty_4 = 1;
pub const UI_MOUSE_EVENT_ACTION_RELEASE: _bindgen_ty_4 = 2;
pub const UI_MOUSE_EVENT_ACTION_MOVE: _bindgen_ty_4 = 3;
pub type _bindgen_ty_4 = ::std::os::raw::c_uint;
pub const UI_MOUSE_EVENT_BUTTON_NONE: _bindgen_ty_5 = 0;
pub const UI_MOUSE_EVENT_BUTTON_LEFT: _bindgen_ty_5 = 1;
pub const UI_MOUSE_EVENT_BUTTON_RIGHT: _bindgen_ty_5 = 2;
pub const UI_MOUSE_EVENT_BUTTON_MIDDLE: _bindgen_ty_5 = 3;
pub const UI_MOUSE_EVENT_BUTTON_BACK: _bindgen_ty_5 = 4;
pub const UI_MOUSE_EVENT_BUTTON_FORWARD: _bindgen_ty_5 = 5;
pub type _bindgen_ty_5 = ::std::os::raw::c_uint;
pub const ArkUI_ModifierKeyName_ARKUI_MODIFIER_KEY_CTRL: ArkUI_ModifierKeyName = 1;
pub const ArkUI_ModifierKeyName_ARKUI_MODIFIER_KEY_SHIFT: ArkUI_ModifierKeyName = 2;
pub const ArkUI_ModifierKeyName_ARKUI_MODIFIER_KEY_ALT: ArkUI_ModifierKeyName = 4;
pub const ArkUI_ModifierKeyName_ARKUI_MODIFIER_KEY_FN: ArkUI_ModifierKeyName = 8;
pub type ArkUI_ModifierKeyName = ::std::os::raw::c_uint;
extern "C" {
    pub fn OH_ArkUI_UIInputEvent_GetType(event: *const ArkUI_UIInputEvent) -> i32;
}
extern "C" {
    pub fn OH_ArkUI_UIInputEvent_GetAction(event: *const ArkUI_UIInputEvent) -> i32;
}
extern "C" {
    pub fn OH_ArkUI_UIInputEvent_GetSourceType(event: *const ArkUI_UIInputEvent) -> i32;
}
extern "C" {
    pub fn OH_ArkUI_UIInputEvent_GetToolType(event: *const ArkUI_UIInputEvent) -> i32;
}
extern "C" {
    pub fn OH_ArkUI_UIInputEvent_GetEventTime(event: *const ArkUI_UIInputEvent) -> i64;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetPointerCount(event: *const ArkUI_UIInputEvent) -> u32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetPointerId(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> i32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetX(event: *const ArkUI_UIInputEvent) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetXByIndex(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetY(event: *const ArkUI_UIInputEvent) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetYByIndex(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetWindowX(event: *const ArkUI_UIInputEvent) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetWindowXByIndex(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetWindowY(event: *const ArkUI_UIInputEvent) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetWindowYByIndex(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetDisplayX(event: *const ArkUI_UIInputEvent) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetDisplayXByIndex(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetDisplayY(event: *const ArkUI_UIInputEvent) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetDisplayYByIndex(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetPressure(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetTiltX(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetTiltY(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetTouchAreaWidth(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetTouchAreaHeight(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetHistorySize(event: *const ArkUI_UIInputEvent) -> u32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetHistoryEventTime(
        event: *const ArkUI_UIInputEvent,
        historyIndex: u32,
    ) -> i64;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetHistoryPointerCount(
        event: *const ArkUI_UIInputEvent,
        historyIndex: u32,
    ) -> u32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetHistoryPointerId(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> i32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetHistoryX(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetHistoryY(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetHistoryWindowX(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetHistoryWindowY(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetHistoryDisplayX(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetHistoryDisplayY(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetHistoryPressure(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetHistoryTiltX(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetHistoryTiltY(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetHistoryTouchAreaWidth(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_GetHistoryTouchAreaHeight(
        event: *const ArkUI_UIInputEvent,
        pointerIndex: u32,
        historyIndex: u32,
    ) -> f32;
}
extern "C" {
    pub fn OH_ArkUI_AxisEvent_GetVerticalAxisValue(event: *const ArkUI_UIInputEvent) -> f64;
}
extern "C" {
    pub fn OH_ArkUI_AxisEvent_GetHorizontalAxisValue(event: *const ArkUI_UIInputEvent) -> f64;
}
extern "C" {
    pub fn OH_ArkUI_AxisEvent_GetPinchAxisScaleValue(event: *const ArkUI_UIInputEvent) -> f64;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_SetInterceptHitTestMode(
        event: *const ArkUI_UIInputEvent,
        mode: HitTestMode,
    ) -> i32;
}
extern "C" {
    pub fn OH_ArkUI_MouseEvent_GetMouseButton(event: *const ArkUI_UIInputEvent) -> i32;
}
extern "C" {
    pub fn OH_ArkUI_MouseEvent_GetMouseAction(event: *const ArkUI_UIInputEvent) -> i32;
}
extern "C" {
    pub fn OH_ArkUI_PointerEvent_SetStopPropagation(
        event: *const ArkUI_UIInputEvent,
        stopPropagation: bool,
    ) -> i32;
}