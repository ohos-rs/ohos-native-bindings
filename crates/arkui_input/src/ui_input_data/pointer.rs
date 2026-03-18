#[cfg(feature = "api-17")]
use ohos_arkui_input_sys::OH_ArkUI_PointerEvent_GetRollAngle;
#[cfg(feature = "api-15")]
use ohos_arkui_input_sys::{
    ArkUI_ErrorCode_ARKUI_ERROR_CODE_PARAM_INVALID, ArkUI_InteractionHand_ARKUI_EVENT_HAND_NONE,
    ArkUI_NodeHandle, OH_ArkUI_MouseEvent_GetPressedButtons, OH_ArkUI_MouseEvent_GetRawDeltaX,
    OH_ArkUI_MouseEvent_GetRawDeltaY, OH_ArkUI_PointerEvent_CreateClonedEvent,
    OH_ArkUI_PointerEvent_DestroyClonedEvent, OH_ArkUI_PointerEvent_GetInteractionHand,
    OH_ArkUI_PointerEvent_GetInteractionHandByIndex, OH_ArkUI_PointerEvent_GetPressedTimeByIndex,
    OH_ArkUI_PointerEvent_PostClonedEvent, OH_ArkUI_PointerEvent_SetClonedEventActionType,
    OH_ArkUI_PointerEvent_SetClonedEventChangedFingerId,
    OH_ArkUI_PointerEvent_SetClonedEventFingerIdByIndex,
    OH_ArkUI_PointerEvent_SetClonedEventLocalPosition,
    OH_ArkUI_PointerEvent_SetClonedEventLocalPositionByIndex,
};
use ohos_arkui_input_sys::{
    HitTestMode, OH_ArkUI_MouseEvent_GetMouseAction, OH_ArkUI_MouseEvent_GetMouseButton,
    OH_ArkUI_PointerEvent_GetDisplayX, OH_ArkUI_PointerEvent_GetDisplayXByIndex,
    OH_ArkUI_PointerEvent_GetDisplayY, OH_ArkUI_PointerEvent_GetDisplayYByIndex,
    OH_ArkUI_PointerEvent_GetHistoryDisplayX, OH_ArkUI_PointerEvent_GetHistoryDisplayY,
    OH_ArkUI_PointerEvent_GetHistoryEventTime, OH_ArkUI_PointerEvent_GetHistoryPointerCount,
    OH_ArkUI_PointerEvent_GetHistoryPointerId, OH_ArkUI_PointerEvent_GetHistoryPressure,
    OH_ArkUI_PointerEvent_GetHistorySize, OH_ArkUI_PointerEvent_GetHistoryTiltX,
    OH_ArkUI_PointerEvent_GetHistoryTiltY, OH_ArkUI_PointerEvent_GetHistoryTouchAreaHeight,
    OH_ArkUI_PointerEvent_GetHistoryTouchAreaWidth, OH_ArkUI_PointerEvent_GetHistoryWindowX,
    OH_ArkUI_PointerEvent_GetHistoryWindowY, OH_ArkUI_PointerEvent_GetHistoryX,
    OH_ArkUI_PointerEvent_GetHistoryY, OH_ArkUI_PointerEvent_GetPressure,
    OH_ArkUI_PointerEvent_GetTiltX, OH_ArkUI_PointerEvent_GetTiltY,
    OH_ArkUI_PointerEvent_GetTouchAreaHeight, OH_ArkUI_PointerEvent_GetTouchAreaWidth,
    OH_ArkUI_PointerEvent_GetWindowX, OH_ArkUI_PointerEvent_GetWindowXByIndex,
    OH_ArkUI_PointerEvent_GetWindowY, OH_ArkUI_PointerEvent_GetWindowYByIndex,
    OH_ArkUI_PointerEvent_GetX, OH_ArkUI_PointerEvent_GetXByIndex, OH_ArkUI_PointerEvent_GetY,
    OH_ArkUI_PointerEvent_GetYByIndex, OH_ArkUI_PointerEvent_SetInterceptHitTestMode,
    OH_ArkUI_PointerEvent_SetStopPropagation,
};
#[cfg(feature = "api-20")]
use ohos_arkui_input_sys::{
    OH_ArkUI_PointerEvent_GetGlobalDisplayX, OH_ArkUI_PointerEvent_GetGlobalDisplayXByIndex,
    OH_ArkUI_PointerEvent_GetGlobalDisplayY, OH_ArkUI_PointerEvent_GetGlobalDisplayYByIndex,
    OH_ArkUI_PointerEvent_GetHistoryGlobalDisplayX, OH_ArkUI_PointerEvent_GetHistoryGlobalDisplayY,
};
#[cfg(feature = "api-15")]
use std::ptr::NonNull;

use crate::ui_input_data::check_status;
use crate::{ArkUIInputError, ArkUIInputEvent, HitTest, UIMouseEventAction, UIMouseEventButton};
#[cfg(feature = "api-15")]
use crate::{InteractionHand, UIInputAction};

impl ArkUIInputEvent {
    pub fn pointer_x(&self) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetX(self.raw()) }
    }

    pub fn pointer_x_by_index(&self, pointer_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetXByIndex(self.raw(), pointer_index) }
    }

    pub fn pointer_y(&self) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetY(self.raw()) }
    }

    pub fn pointer_y_by_index(&self, pointer_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetYByIndex(self.raw(), pointer_index) }
    }

    pub fn pointer_window_x(&self) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetWindowX(self.raw()) }
    }

    pub fn pointer_window_x_by_index(&self, pointer_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetWindowXByIndex(self.raw(), pointer_index) }
    }

    pub fn pointer_window_y(&self) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetWindowY(self.raw()) }
    }

    pub fn pointer_window_y_by_index(&self, pointer_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetWindowYByIndex(self.raw(), pointer_index) }
    }

    pub fn pointer_display_x(&self) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetDisplayX(self.raw()) }
    }

    pub fn pointer_display_x_by_index(&self, pointer_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetDisplayXByIndex(self.raw(), pointer_index) }
    }

    pub fn pointer_display_y(&self) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetDisplayY(self.raw()) }
    }

    pub fn pointer_display_y_by_index(&self, pointer_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetDisplayYByIndex(self.raw(), pointer_index) }
    }

    #[cfg(feature = "api-20")]
    pub fn pointer_global_display_x(&self) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetGlobalDisplayX(self.raw()) }
    }

    #[cfg(feature = "api-20")]
    pub fn pointer_global_display_x_by_index(&self, pointer_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetGlobalDisplayXByIndex(self.raw(), pointer_index) }
    }

    #[cfg(feature = "api-20")]
    pub fn pointer_global_display_y(&self) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetGlobalDisplayY(self.raw()) }
    }

    #[cfg(feature = "api-20")]
    pub fn pointer_global_display_y_by_index(&self, pointer_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetGlobalDisplayYByIndex(self.raw(), pointer_index) }
    }

    pub fn pointer_pressure(&self, pointer_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetPressure(self.raw(), pointer_index) }
    }

    pub fn pointer_tilt_x(&self, pointer_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetTiltX(self.raw(), pointer_index) }
    }

    pub fn pointer_tilt_y(&self, pointer_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetTiltY(self.raw(), pointer_index) }
    }

    #[cfg(feature = "api-17")]
    pub fn pointer_roll_angle(&self) -> Result<f64, ArkUIInputError> {
        let mut roll_angle = 0.0f64;
        check_status(unsafe { OH_ArkUI_PointerEvent_GetRollAngle(self.raw(), &mut roll_angle) })?;
        Ok(roll_angle)
    }

    pub fn pointer_touch_area_width(&self, pointer_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetTouchAreaWidth(self.raw(), pointer_index) }
    }

    pub fn pointer_touch_area_height(&self, pointer_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetTouchAreaHeight(self.raw(), pointer_index) }
    }

    #[cfg(feature = "api-15")]
    pub fn pointer_interaction_hand(&self) -> Result<InteractionHand, ArkUIInputError> {
        let mut hand = ArkUI_InteractionHand_ARKUI_EVENT_HAND_NONE;
        check_status(unsafe { OH_ArkUI_PointerEvent_GetInteractionHand(self.raw(), &mut hand) })?;
        Ok(InteractionHand::from(hand))
    }

    #[cfg(feature = "api-15")]
    pub fn pointer_interaction_hand_by_index(
        &self,
        pointer_index: i32,
    ) -> Result<InteractionHand, ArkUIInputError> {
        let mut hand = ArkUI_InteractionHand_ARKUI_EVENT_HAND_NONE;
        check_status(unsafe {
            OH_ArkUI_PointerEvent_GetInteractionHandByIndex(self.raw(), pointer_index, &mut hand)
        })?;
        Ok(InteractionHand::from(hand))
    }

    pub fn pointer_history_size(&self) -> u32 {
        unsafe { OH_ArkUI_PointerEvent_GetHistorySize(self.raw()) }
    }

    pub fn pointer_history_event_time(&self, history_index: u32) -> i64 {
        unsafe { OH_ArkUI_PointerEvent_GetHistoryEventTime(self.raw(), history_index) }
    }

    pub fn pointer_history_pointer_count(&self, history_index: u32) -> u32 {
        unsafe { OH_ArkUI_PointerEvent_GetHistoryPointerCount(self.raw(), history_index) }
    }

    pub fn pointer_history_pointer_id(&self, pointer_index: u32, history_index: u32) -> i32 {
        unsafe {
            OH_ArkUI_PointerEvent_GetHistoryPointerId(self.raw(), pointer_index, history_index)
        }
    }

    pub fn pointer_history_x(&self, pointer_index: u32, history_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetHistoryX(self.raw(), pointer_index, history_index) }
    }

    pub fn pointer_history_y(&self, pointer_index: u32, history_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetHistoryY(self.raw(), pointer_index, history_index) }
    }

    pub fn pointer_history_window_x(&self, pointer_index: u32, history_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetHistoryWindowX(self.raw(), pointer_index, history_index) }
    }

    pub fn pointer_history_window_y(&self, pointer_index: u32, history_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetHistoryWindowY(self.raw(), pointer_index, history_index) }
    }

    pub fn pointer_history_display_x(&self, pointer_index: u32, history_index: u32) -> f32 {
        unsafe {
            OH_ArkUI_PointerEvent_GetHistoryDisplayX(self.raw(), pointer_index, history_index)
        }
    }

    pub fn pointer_history_display_y(&self, pointer_index: u32, history_index: u32) -> f32 {
        unsafe {
            OH_ArkUI_PointerEvent_GetHistoryDisplayY(self.raw(), pointer_index, history_index)
        }
    }

    #[cfg(feature = "api-20")]
    pub fn pointer_history_global_display_x(&self, pointer_index: u32, history_index: u32) -> f32 {
        unsafe {
            OH_ArkUI_PointerEvent_GetHistoryGlobalDisplayX(self.raw(), pointer_index, history_index)
        }
    }

    #[cfg(feature = "api-20")]
    pub fn pointer_history_global_display_y(&self, pointer_index: u32, history_index: u32) -> f32 {
        unsafe {
            OH_ArkUI_PointerEvent_GetHistoryGlobalDisplayY(self.raw(), pointer_index, history_index)
        }
    }

    pub fn pointer_history_pressure(&self, pointer_index: u32, history_index: u32) -> f32 {
        unsafe {
            OH_ArkUI_PointerEvent_GetHistoryPressure(self.raw(), pointer_index, history_index)
        }
    }

    pub fn pointer_history_tilt_x(&self, pointer_index: u32, history_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetHistoryTiltX(self.raw(), pointer_index, history_index) }
    }

    pub fn pointer_history_tilt_y(&self, pointer_index: u32, history_index: u32) -> f32 {
        unsafe { OH_ArkUI_PointerEvent_GetHistoryTiltY(self.raw(), pointer_index, history_index) }
    }

    pub fn pointer_history_touch_area_width(&self, pointer_index: u32, history_index: u32) -> f32 {
        unsafe {
            OH_ArkUI_PointerEvent_GetHistoryTouchAreaWidth(self.raw(), pointer_index, history_index)
        }
    }

    pub fn pointer_history_touch_area_height(&self, pointer_index: u32, history_index: u32) -> f32 {
        unsafe {
            OH_ArkUI_PointerEvent_GetHistoryTouchAreaHeight(
                self.raw(),
                pointer_index,
                history_index,
            )
        }
    }

    pub fn pointer_set_intercept_hit_test_mode(
        &self,
        mode: HitTest,
    ) -> Result<(), ArkUIInputError> {
        let mode: HitTestMode = mode.into();
        check_status(unsafe { OH_ArkUI_PointerEvent_SetInterceptHitTestMode(self.raw(), mode) })
    }

    pub fn mouse_button(&self) -> UIMouseEventButton {
        UIMouseEventButton::from(unsafe { OH_ArkUI_MouseEvent_GetMouseButton(self.raw()) } as u32)
    }

    pub fn mouse_action(&self) -> UIMouseEventAction {
        UIMouseEventAction::from(unsafe { OH_ArkUI_MouseEvent_GetMouseAction(self.raw()) } as u32)
    }

    pub fn pointer_set_stop_propagation(
        &self,
        stop_propagation: bool,
    ) -> Result<(), ArkUIInputError> {
        check_status(unsafe {
            OH_ArkUI_PointerEvent_SetStopPropagation(self.raw(), stop_propagation)
        })
    }

    #[cfg(feature = "api-15")]
    pub fn pointer_pressed_time_by_index(&self, pointer_index: u32) -> i64 {
        unsafe { OH_ArkUI_PointerEvent_GetPressedTimeByIndex(self.raw(), pointer_index) }
    }

    #[cfg(feature = "api-15")]
    pub fn mouse_raw_delta_x(&self) -> f32 {
        unsafe { OH_ArkUI_MouseEvent_GetRawDeltaX(self.raw()) }
    }

    #[cfg(feature = "api-15")]
    pub fn mouse_raw_delta_y(&self) -> f32 {
        unsafe { OH_ArkUI_MouseEvent_GetRawDeltaY(self.raw()) }
    }

    #[cfg(feature = "api-15")]
    pub fn mouse_pressed_buttons(
        &self,
        pressed_buttons: &mut [i32],
    ) -> Result<usize, ArkUIInputError> {
        let mut length = pressed_buttons.len() as i32;
        check_status(unsafe {
            OH_ArkUI_MouseEvent_GetPressedButtons(
                self.raw(),
                pressed_buttons.as_mut_ptr(),
                &mut length,
            )
        })?;
        Ok(length.max(0) as usize)
    }

    #[cfg(feature = "api-15")]
    pub fn create_cloned_event(&self) -> Result<ArkUIInputClonedEvent, ArkUIInputError> {
        let mut cloned_event: *mut ohos_arkui_input_sys::ArkUI_UIInputEvent = std::ptr::null_mut();
        check_status(unsafe {
            OH_ArkUI_PointerEvent_CreateClonedEvent(self.raw(), &mut cloned_event)
        })?;
        let event = NonNull::new(cloned_event).ok_or(ArkUIInputError::InternalError(
            ArkUI_ErrorCode_ARKUI_ERROR_CODE_PARAM_INVALID as i32,
        ))?;
        Ok(ArkUIInputClonedEvent { event })
    }
}

#[cfg(feature = "api-15")]
#[derive(Debug)]
pub struct ArkUIInputClonedEvent {
    event: NonNull<ohos_arkui_input_sys::ArkUI_UIInputEvent>,
}

#[cfg(feature = "api-15")]
impl ArkUIInputClonedEvent {
    pub fn raw(&self) -> *const ohos_arkui_input_sys::ArkUI_UIInputEvent {
        self.event.as_ptr()
    }

    pub fn set_local_position(&self, x: f32, y: f32) -> Result<(), ArkUIInputError> {
        check_status(unsafe { OH_ArkUI_PointerEvent_SetClonedEventLocalPosition(self.raw(), x, y) })
    }

    pub fn set_local_position_by_index(
        &self,
        x: f32,
        y: f32,
        pointer_index: i32,
    ) -> Result<(), ArkUIInputError> {
        check_status(unsafe {
            OH_ArkUI_PointerEvent_SetClonedEventLocalPositionByIndex(
                self.raw(),
                x,
                y,
                pointer_index,
            )
        })
    }

    pub fn set_action_type(&self, action: UIInputAction) -> Result<(), ArkUIInputError> {
        check_status(unsafe {
            OH_ArkUI_PointerEvent_SetClonedEventActionType(self.raw(), action.into())
        })
    }

    pub fn set_changed_finger_id(&self, finger_id: i32) -> Result<(), ArkUIInputError> {
        check_status(unsafe {
            OH_ArkUI_PointerEvent_SetClonedEventChangedFingerId(self.raw(), finger_id)
        })
    }

    pub fn set_finger_id_by_index(
        &self,
        finger_id: i32,
        pointer_index: i32,
    ) -> Result<(), ArkUIInputError> {
        check_status(unsafe {
            OH_ArkUI_PointerEvent_SetClonedEventFingerIdByIndex(
                self.raw(),
                finger_id,
                pointer_index,
            )
        })
    }

    pub fn post_to_node(&self, node: ArkUI_NodeHandle) -> Result<(), ArkUIInputError> {
        check_status(unsafe { OH_ArkUI_PointerEvent_PostClonedEvent(node, self.raw()) })
    }
}

#[cfg(feature = "api-15")]
impl Drop for ArkUIInputClonedEvent {
    fn drop(&mut self) {
        unsafe {
            let _ = OH_ArkUI_PointerEvent_DestroyClonedEvent(self.raw());
        }
    }
}
