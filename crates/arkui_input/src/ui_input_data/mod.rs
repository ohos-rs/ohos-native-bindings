mod axis;
#[cfg(feature = "api-22")]
mod coasting;
mod pointer;

#[cfg(feature = "api-22")]
pub use coasting::ArkUICoastingAxisEvent;
#[cfg(feature = "api-15")]
pub use pointer::ArkUIInputClonedEvent;

use ohos_arkui_input_sys::{
    ArkUI_ErrorCode_ARKUI_ERROR_CODE_NO_ERROR, ArkUI_ModifierKeyName, ArkUI_UIInputEvent,
    OH_ArkUI_HoverEvent_IsHovered, OH_ArkUI_PointerEvent_GetPointerCount,
    OH_ArkUI_PointerEvent_GetPointerId, OH_ArkUI_UIInputEvent_GetAction,
    OH_ArkUI_UIInputEvent_GetEventTime, OH_ArkUI_UIInputEvent_GetSourceType,
    OH_ArkUI_UIInputEvent_GetToolType, OH_ArkUI_UIInputEvent_GetType,
};
#[cfg(feature = "api-15")]
use ohos_arkui_input_sys::{
    OH_ArkUI_AxisEvent_GetAxisAction, OH_ArkUI_PointerEvent_GetChangedPointerId,
    OH_ArkUI_UIInputEvent_GetTargetDisplayId,
};
#[cfg(feature = "api-14")]
use ohos_arkui_input_sys::{
    OH_ArkUI_UIInputEvent_GetDeviceId, OH_ArkUI_UIInputEvent_GetPressedKeys,
};
#[cfg(feature = "api-17")]
use ohos_arkui_input_sys::{
    OH_ArkUI_UIInputEvent_GetEventTargetGlobalPositionX,
    OH_ArkUI_UIInputEvent_GetEventTargetGlobalPositionY,
    OH_ArkUI_UIInputEvent_GetEventTargetHeight, OH_ArkUI_UIInputEvent_GetEventTargetPositionX,
    OH_ArkUI_UIInputEvent_GetEventTargetPositionY, OH_ArkUI_UIInputEvent_GetEventTargetWidth,
    OH_ArkUI_UIInputEvent_GetModifierKeyStates,
};
use std::ptr::NonNull;

use crate::{
    ArkUIInputError, ModifierKey, UIInputAction, UIInputEvent, UIInputSourceType, UIInputToolType,
};

/// Wrapper for ArkUI_UIInputEvent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArkUIInputEvent {
    event: NonNull<ArkUI_UIInputEvent>,

    pub event_type: UIInputEvent,
    pub action: UIInputAction,
    pub source_type: UIInputSourceType,
    pub tool_type: UIInputToolType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModifierKeyStates(u64);

impl ModifierKeyStates {
    pub fn bits(self) -> u64 {
        self.0
    }

    pub fn contains(self, key: ModifierKey) -> bool {
        self.0 & modifier_key_mask(key) != 0
    }
}

pub(crate) fn check_status(status: i32) -> Result<(), ArkUIInputError> {
    if status == ArkUI_ErrorCode_ARKUI_ERROR_CODE_NO_ERROR as i32 {
        Ok(())
    } else {
        Err(ArkUIInputError::InternalError(status))
    }
}

fn modifier_key_mask(key: ModifierKey) -> u64 {
    let raw: ArkUI_ModifierKeyName = key.into();
    raw as u64
}

impl ArkUIInputEvent {
    pub fn raw(&self) -> *const ArkUI_UIInputEvent {
        self.event.as_ptr()
    }

    #[cfg(feature = "api-22")]
    pub(crate) fn raw_mut(&self) -> *mut ArkUI_UIInputEvent {
        self.event.as_ptr()
    }

    pub fn from_raw(event: *const ArkUI_UIInputEvent) -> Self {
        let event = NonNull::new(event.cast_mut()).expect("ArkUI_UIInputEvent pointer is null");
        let raw_event_type = unsafe { OH_ArkUI_UIInputEvent_GetType(event.as_ptr()) };
        let event_type =
            UIInputEvent::try_from_raw(raw_event_type as u32).unwrap_or(UIInputEvent::Unknown);
        let action = match event_type {
            // The generic action getter explicitly excludes axis events. Use
            // the category-specific API whenever the target SDK exposes it.
            #[cfg(feature = "api-15")]
            UIInputEvent::Axis => unsafe { OH_ArkUI_AxisEvent_GetAxisAction(event.as_ptr()) },
            _ => unsafe { OH_ArkUI_UIInputEvent_GetAction(event.as_ptr()) },
        };
        let source_type = unsafe { OH_ArkUI_UIInputEvent_GetSourceType(event.as_ptr()) };
        let tool_type = unsafe { OH_ArkUI_UIInputEvent_GetToolType(event.as_ptr()) };
        Self {
            event,
            event_type,
            action: normalized_action(event_type, action),
            source_type: UIInputSourceType::try_from_raw(source_type as u32)
                .unwrap_or(UIInputSourceType::Unknown),
            tool_type: UIInputToolType::try_from_raw(tool_type as u32)
                .unwrap_or(UIInputToolType::Unknown),
        }
    }

    pub fn event_time(&self) -> i64 {
        unsafe { OH_ArkUI_UIInputEvent_GetEventTime(self.raw()) }
    }

    /// Whether the pointer is currently inside the target for a hover event.
    ///
    /// ArkUI exposes hover enter/leave through a dedicated accessor; the
    /// generic action field is not defined for this event category.
    pub fn is_hovered(&self) -> bool {
        unsafe { OH_ArkUI_HoverEvent_IsHovered(self.raw()) }
    }

    /// Get the number of contact points from a pointer event (such as a touch, mouse, or axis event).
    pub fn pointer_count(&self) -> u32 {
        unsafe { OH_ArkUI_PointerEvent_GetPointerCount(self.raw()) }
    }

    /// Get the unique ID of a contact point from a pointer event (such as a touch, mouse, or axis event).
    pub fn pointer_id(&self, pointer_index: u32) -> i32 {
        unsafe { OH_ArkUI_PointerEvent_GetPointerId(self.raw(), pointer_index) }
    }

    /// Get the ID of the touch pointer that triggers the current touch event.
    #[cfg(feature = "api-15")]
    pub fn get_changed_pointer_id(&self) -> Result<u32, ArkUIInputError> {
        let mut pointer_index = 0;
        check_status(unsafe {
            OH_ArkUI_PointerEvent_GetChangedPointerId(self.raw(), &mut pointer_index)
        })?;
        Ok(pointer_index)
    }

    /// Get the vertical scroll delta of an axis event.
    /// Only works for touch board and mouse wheel.
    pub fn get_scroll_delta_y(&self) -> Result<f64, ArkUIInputError> {
        match self.event_type {
            UIInputEvent::Axis => Ok(self.axis_vertical_value()),
            #[cfg(feature = "api-17")]
            UIInputEvent::Mouse => Ok(self.axis_vertical_value() * self.axis_scroll_step() as f64),
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
            UIInputEvent::Axis => Ok(self.axis_horizontal_value()),
            _ => Err(ArkUIInputError::DeviceTypeNotSupported(
                "get_scroll_delta_x".to_string(),
                self.source_type.into(),
            )),
        }
    }

    #[cfg(feature = "api-14")]
    pub fn device_id(&self) -> i32 {
        unsafe { OH_ArkUI_UIInputEvent_GetDeviceId(self.raw()) }
    }

    #[cfg(feature = "api-14")]
    pub fn pressed_keys(&self, pressed_key_codes: &mut [i32]) -> Result<usize, ArkUIInputError> {
        let mut length = pressed_key_codes.len() as i32;
        check_status(unsafe {
            OH_ArkUI_UIInputEvent_GetPressedKeys(
                self.raw(),
                pressed_key_codes.as_mut_ptr(),
                &mut length,
            )
        })?;
        Ok(length.max(0) as usize)
    }

    #[cfg(feature = "api-15")]
    pub fn target_display_id(&self) -> i32 {
        unsafe { OH_ArkUI_UIInputEvent_GetTargetDisplayId(self.raw()) }
    }

    #[cfg(feature = "api-17")]
    pub fn event_target_width(&self) -> f32 {
        unsafe { OH_ArkUI_UIInputEvent_GetEventTargetWidth(self.raw()) }
    }

    #[cfg(feature = "api-17")]
    pub fn event_target_height(&self) -> f32 {
        unsafe { OH_ArkUI_UIInputEvent_GetEventTargetHeight(self.raw()) }
    }

    #[cfg(feature = "api-17")]
    pub fn event_target_position_x(&self) -> f32 {
        unsafe { OH_ArkUI_UIInputEvent_GetEventTargetPositionX(self.raw()) }
    }

    #[cfg(feature = "api-17")]
    pub fn event_target_position_y(&self) -> f32 {
        unsafe { OH_ArkUI_UIInputEvent_GetEventTargetPositionY(self.raw()) }
    }

    #[cfg(feature = "api-17")]
    pub fn event_target_global_position_x(&self) -> f32 {
        unsafe { OH_ArkUI_UIInputEvent_GetEventTargetGlobalPositionX(self.raw()) }
    }

    #[cfg(feature = "api-17")]
    pub fn event_target_global_position_y(&self) -> f32 {
        unsafe { OH_ArkUI_UIInputEvent_GetEventTargetGlobalPositionY(self.raw()) }
    }

    #[cfg(feature = "api-17")]
    pub fn modifier_key_states(&self) -> Result<ModifierKeyStates, ArkUIInputError> {
        let mut keys = 0u64;
        check_status(unsafe { OH_ArkUI_UIInputEvent_GetModifierKeyStates(self.raw(), &mut keys) })?;
        Ok(ModifierKeyStates(keys))
    }
}

/// Converts category-specific ArkUI action codes into the touch-style action
/// semantics exposed by the existing public API.
///
/// ArkUI reuses numeric values differently for touch, axis, and mouse events.
/// Keep this conversion total because native accessors may also return `-1`
/// for invalid input or values introduced by a newer SDK.
fn normalized_action(event_type: UIInputEvent, action: i32) -> UIInputAction {
    match event_type {
        UIInputEvent::Touch => match action {
            1 => UIInputAction::Down,
            2 => UIInputAction::Move,
            3 => UIInputAction::Up,
            0 => UIInputAction::Cancel,
            _ => UIInputAction::Cancel,
        },
        UIInputEvent::Axis => match action {
            // UI_AXIS_EVENT_ACTION_BEGIN / UPDATE / END / CANCEL
            1 => UIInputAction::Down,
            2 => UIInputAction::Move,
            3 => UIInputAction::Up,
            0 | 4 => UIInputAction::Cancel,
            _ => UIInputAction::Cancel,
        },
        UIInputEvent::Mouse => match action {
            // UI_MOUSE_EVENT_ACTION_PRESS / RELEASE / MOVE / CANCEL
            1 => UIInputAction::Down,
            2 => UIInputAction::Up,
            3 => UIInputAction::Move,
            0 | 13 => UIInputAction::Cancel,
            _ => UIInputAction::Cancel,
        },
        _ => UIInputAction::Cancel,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_touch_actions() {
        assert_eq!(
            normalized_action(UIInputEvent::Touch, 0),
            UIInputAction::Cancel
        );
        assert_eq!(
            normalized_action(UIInputEvent::Touch, 1),
            UIInputAction::Down
        );
        assert_eq!(
            normalized_action(UIInputEvent::Touch, 2),
            UIInputAction::Move
        );
        assert_eq!(normalized_action(UIInputEvent::Touch, 3), UIInputAction::Up);
    }

    #[test]
    fn normalizes_axis_actions_without_panicking_on_cancel() {
        assert_eq!(
            normalized_action(UIInputEvent::Axis, 0),
            UIInputAction::Cancel
        );
        assert_eq!(
            normalized_action(UIInputEvent::Axis, 1),
            UIInputAction::Down
        );
        assert_eq!(
            normalized_action(UIInputEvent::Axis, 2),
            UIInputAction::Move
        );
        assert_eq!(normalized_action(UIInputEvent::Axis, 3), UIInputAction::Up);
        assert_eq!(
            normalized_action(UIInputEvent::Axis, 4),
            UIInputAction::Cancel
        );
    }

    #[test]
    fn normalizes_mouse_actions_by_mouse_semantics() {
        assert_eq!(
            normalized_action(UIInputEvent::Mouse, 0),
            UIInputAction::Cancel
        );
        assert_eq!(
            normalized_action(UIInputEvent::Mouse, 1),
            UIInputAction::Down
        );
        assert_eq!(normalized_action(UIInputEvent::Mouse, 2), UIInputAction::Up);
        assert_eq!(
            normalized_action(UIInputEvent::Mouse, 3),
            UIInputAction::Move
        );
        assert_eq!(
            normalized_action(UIInputEvent::Mouse, 13),
            UIInputAction::Cancel
        );
    }

    #[test]
    fn invalid_and_future_values_fall_back_to_cancel() {
        assert_eq!(
            normalized_action(UIInputEvent::Touch, -1),
            UIInputAction::Cancel
        );
        assert_eq!(
            normalized_action(UIInputEvent::Axis, 99),
            UIInputAction::Cancel
        );
        assert_eq!(
            normalized_action(UIInputEvent::Unknown, 5),
            UIInputAction::Cancel
        );
    }
}
