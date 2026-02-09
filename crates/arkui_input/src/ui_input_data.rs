use ohos_arkui_input_sys::{
    ArkUI_UIInputEvent, OH_ArkUI_AxisEvent_GetHorizontalAxisValue,
    OH_ArkUI_AxisEvent_GetScrollStep, OH_ArkUI_AxisEvent_GetVerticalAxisValue,
    OH_ArkUI_PointerEvent_GetChangedPointerId, OH_ArkUI_PointerEvent_GetPointerCount,
    OH_ArkUI_PointerEvent_GetPointerId, OH_ArkUI_UIInputEvent_GetAction,
    OH_ArkUI_UIInputEvent_GetSourceType, OH_ArkUI_UIInputEvent_GetToolType,
    OH_ArkUI_UIInputEvent_GetType,
};
use std::ptr::NonNull;

use crate::{ArkUIInputError, UIInputAction, UIInputEvent, UIInputSourceType, UIInputToolType};

/// Wrapper for ArkUI_UIInputEvent
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArkUIInputEvent {
    event: NonNull<ArkUI_UIInputEvent>,

    pub event_type: UIInputEvent,
    pub action: UIInputAction,
    pub source_type: UIInputSourceType,
    pub tool_type: UIInputToolType,
}

impl ArkUIInputEvent {
    pub fn raw(&self) -> *const ArkUI_UIInputEvent {
        self.event.as_ptr()
    }

    pub fn from_raw(event: *const ArkUI_UIInputEvent) -> Self {
        let event_type = unsafe { OH_ArkUI_UIInputEvent_GetType(event) };
        let action = unsafe { OH_ArkUI_UIInputEvent_GetAction(event) };
        let source_type = unsafe { OH_ArkUI_UIInputEvent_GetSourceType(event) };
        let tool_type = unsafe { OH_ArkUI_UIInputEvent_GetToolType(event) };
        Self {
            event: unsafe { NonNull::new_unchecked(event as *mut ArkUI_UIInputEvent) },
            event_type: UIInputEvent::from(event_type as u32),
            action: UIInputAction::from(action as u32),
            source_type: UIInputSourceType::from(source_type as u32),
            tool_type: UIInputToolType::from(tool_type as u32),
        }
    }

    /// Get the number of contact points from a pointer event (such as a touch, mouse, or axis event).
    pub fn pointer_count(&self) -> u32 {
        unsafe { OH_ArkUI_PointerEvent_GetPointerCount(self.event.as_ptr()) }
    }

    /// Get the unique ID of a contact point from a pointer event (such as a touch, mouse, or axis event).
    pub fn pointer_id(&self, pointer_index: u32) -> i32 {
        unsafe { OH_ArkUI_PointerEvent_GetPointerId(self.event.as_ptr(), pointer_index) }
    }

    /// Get the ID of the touch pointer that triggers the current touch event.
    #[cfg(feature = "api-15")]
    pub fn get_changed_pointer_id(&self) -> Result<u32, ArkUIInputError> {
        let mut pointer_index = Box::new(0);
        let ret = unsafe {
            OH_ArkUI_PointerEvent_GetChangedPointerId(self.event.as_ptr(), pointer_index.as_mut())
        };

        if ret == 0 {
            Ok(*pointer_index)
        } else {
            Err(ArkUIInputError::InternalError(ret))
        }
    }

    /// Get the vertical scroll delta of an axis event.
    /// Only works for touch board and mouse wheel.
    pub fn get_scroll_delta_y(&self) -> Result<f64, ArkUIInputError> {
        match self.event_type {
            UIInputEvent::Axis => {
                let scroll_delta_y =
                    unsafe { OH_ArkUI_AxisEvent_GetVerticalAxisValue(self.event.as_ptr()) };
                Ok(scroll_delta_y)
            }
            #[cfg(feature = "api-17")]
            UIInputEvent::Mouse => {
                let scroll_delta_y =
                    unsafe { OH_ArkUI_AxisEvent_GetVerticalAxisValue(self.event.as_ptr()) };
                // need api-17
                let scroll_step = unsafe { OH_ArkUI_AxisEvent_GetScrollStep(self.event.as_ptr()) };
                Ok(scroll_delta_y * scroll_step as f64)
            }
            _ => Err(ArkUIInputError::DeviceTypeNotSupported(
                "get_scroll_delta_y".to_string(),
                self.source_type.into(),
            )),
        }
    }

    /// Get the horizontal scroll delta of an axis event.
    /// Only works for touch board.
    pub fn get_scroll_delta_x(&self) -> Result<f64, ArkUIInputError> {
        match self.event_type {
            UIInputEvent::Axis => {
                let scroll_delta_x =
                    unsafe { OH_ArkUI_AxisEvent_GetHorizontalAxisValue(self.event.as_ptr()) };
                Ok(scroll_delta_x)
            }
            _ => Err(ArkUIInputError::DeviceTypeNotSupported(
                "get_scroll_delta_x".to_string(),
                self.source_type.into(),
            )),
        }
    }
}
