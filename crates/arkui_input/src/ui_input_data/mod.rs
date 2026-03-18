mod axis;
#[cfg(feature = "api-22")]
mod coasting;
mod pointer;

#[cfg(feature = "api-22")]
pub use coasting::ArkUICoastingAxisEvent;
#[cfg(feature = "api-15")]
pub use pointer::ArkUIInputClonedEvent;

use ohos_arkui_input_sys::{
    ArkUI_ErrorCode_ARKUI_ERROR_CODE_NO_ERROR, ArkUI_ModifierKeyName_ARKUI_MODIFIER_KEY_ALT,
    ArkUI_ModifierKeyName_ARKUI_MODIFIER_KEY_CTRL, ArkUI_ModifierKeyName_ARKUI_MODIFIER_KEY_FN,
    ArkUI_ModifierKeyName_ARKUI_MODIFIER_KEY_SHIFT, ArkUI_UIInputEvent,
    OH_ArkUI_PointerEvent_GetPointerCount, OH_ArkUI_PointerEvent_GetPointerId,
    OH_ArkUI_UIInputEvent_GetAction, OH_ArkUI_UIInputEvent_GetEventTime,
    OH_ArkUI_UIInputEvent_GetSourceType, OH_ArkUI_UIInputEvent_GetToolType,
    OH_ArkUI_UIInputEvent_GetType,
};
#[cfg(feature = "api-15")]
use ohos_arkui_input_sys::{
    OH_ArkUI_PointerEvent_GetChangedPointerId, OH_ArkUI_UIInputEvent_GetTargetDisplayId,
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
    match key {
        ModifierKey::Ctrl => ArkUI_ModifierKeyName_ARKUI_MODIFIER_KEY_CTRL as u64,
        ModifierKey::Shift => ArkUI_ModifierKeyName_ARKUI_MODIFIER_KEY_SHIFT as u64,
        ModifierKey::Alt => ArkUI_ModifierKeyName_ARKUI_MODIFIER_KEY_ALT as u64,
        ModifierKey::Fn => ArkUI_ModifierKeyName_ARKUI_MODIFIER_KEY_FN as u64,
    }
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
        let event_type = unsafe { OH_ArkUI_UIInputEvent_GetType(event.as_ptr()) };
        let action = unsafe { OH_ArkUI_UIInputEvent_GetAction(event.as_ptr()) };
        let source_type = unsafe { OH_ArkUI_UIInputEvent_GetSourceType(event.as_ptr()) };
        let tool_type = unsafe { OH_ArkUI_UIInputEvent_GetToolType(event.as_ptr()) };
        Self {
            event,
            event_type: UIInputEvent::from(event_type as u32),
            action: UIInputAction::from(action as u32),
            source_type: UIInputSourceType::from(source_type as u32),
            tool_type: UIInputToolType::from(tool_type as u32),
        }
    }

    pub fn event_time(&self) -> i64 {
        unsafe { OH_ArkUI_UIInputEvent_GetEventTime(self.raw()) }
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
