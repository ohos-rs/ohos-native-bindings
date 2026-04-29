#[cfg(feature = "api-22")]
use ohos_arkui_input_sys::OH_ArkUI_AxisEvent_HasAxis;
#[cfg(feature = "api-15")]
use ohos_arkui_input_sys::{
    OH_ArkUI_AxisEvent_GetAxisAction, OH_ArkUI_FocusAxisEvent_GetAxisValue,
    OH_ArkUI_FocusAxisEvent_SetStopPropagation, UI_FOCUS_AXIS_EVENT_ABS,
};
use ohos_arkui_input_sys::{
    OH_ArkUI_AxisEvent_GetHorizontalAxisValue, OH_ArkUI_AxisEvent_GetPinchAxisScaleValue,
    OH_ArkUI_AxisEvent_GetVerticalAxisValue,
};
#[cfg(feature = "api-17")]
use ohos_arkui_input_sys::{
    OH_ArkUI_AxisEvent_GetScrollStep, OH_ArkUI_AxisEvent_SetPropagation,
    OH_ArkUI_HoverEvent_IsHovered,
};

#[cfg(feature = "api-15")]
use crate::ui_input_data::check_status;
#[cfg(any(feature = "api-15", feature = "api-17"))]
use crate::ArkUIInputError;
use crate::ArkUIInputEvent;
#[cfg(feature = "api-15")]
use crate::{UIAxisEventAction, UIFocusAxisEventAbs};

impl ArkUIInputEvent {
    pub fn axis_vertical_value(&self) -> f64 {
        unsafe { OH_ArkUI_AxisEvent_GetVerticalAxisValue(self.raw()) }
    }

    pub fn axis_horizontal_value(&self) -> f64 {
        unsafe { OH_ArkUI_AxisEvent_GetHorizontalAxisValue(self.raw()) }
    }

    pub fn axis_pinch_scale_value(&self) -> f64 {
        unsafe { OH_ArkUI_AxisEvent_GetPinchAxisScaleValue(self.raw()) }
    }

    #[cfg(feature = "api-15")]
    pub fn axis_action(&self) -> UIAxisEventAction {
        UIAxisEventAction::from(unsafe { OH_ArkUI_AxisEvent_GetAxisAction(self.raw()) } as u32)
    }

    #[cfg(feature = "api-22")]
    pub fn axis_has(&self, axis: i32) -> bool {
        unsafe { OH_ArkUI_AxisEvent_HasAxis(self.raw(), axis) != 0 }
    }

    #[cfg(feature = "api-15")]
    pub fn focus_axis_value(&self, axis: UIFocusAxisEventAbs) -> f64 {
        let axis: UI_FOCUS_AXIS_EVENT_ABS = axis.into();
        unsafe { OH_ArkUI_FocusAxisEvent_GetAxisValue(self.raw(), axis as i32) }
    }

    #[cfg(feature = "api-15")]
    pub fn focus_axis_set_stop_propagation(
        &self,
        stop_propagation: bool,
    ) -> Result<(), ArkUIInputError> {
        check_status(unsafe {
            OH_ArkUI_FocusAxisEvent_SetStopPropagation(self.raw(), stop_propagation)
        })
    }

    #[cfg(feature = "api-17")]
    pub fn hover_is_hovered(&self) -> bool {
        unsafe { OH_ArkUI_HoverEvent_IsHovered(self.raw()) }
    }

    #[cfg(feature = "api-17")]
    pub fn axis_set_propagation(&self, propagation: bool) -> Result<(), ArkUIInputError> {
        check_status(unsafe { OH_ArkUI_AxisEvent_SetPropagation(self.raw(), propagation) })
    }

    #[cfg(feature = "api-17")]
    pub fn axis_scroll_step(&self) -> i32 {
        unsafe { OH_ArkUI_AxisEvent_GetScrollStep(self.raw()) }
    }
}
