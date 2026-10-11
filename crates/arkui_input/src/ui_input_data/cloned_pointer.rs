use std::ptr::NonNull;

use crate::{
    sys, ArkUIInputError, InteractionHand, UIEventCompetitionStrategy, UIInputSourceType,
    UIInputToolType, UIMouseEventButton, UIPointerEventAction, UIPointerEventType,
};

use super::check_status;

/// An owned API 24 touch, mouse, or axis event.
///
/// Creation, mutation, posting, and destruction must take place on the ArkUI UI
/// thread. This type is neither `Send` nor `Sync`. It always uses the API 24
/// destructor, independently of the legacy API 15 `ArkUIInputClonedEvent`.
/// Native errors (including unsupported event categories) are preserved.
///
/// Indexed setters take a contact's array index, not its pointer ID. Changed-point
/// setters update the triggering point; they do not update every indexed point.
///
/// ```no_run
/// use ohos_arkui_input_binding::{
///     ArkUIOwnedPointerEvent, UIPointerEventType, UIPointerEventAction,
///     UIInputAction, UIInputSourceType, UIInputToolType, ArkUIInputError,
/// };
/// # fn main() -> Result<(), ArkUIInputError> {
/// let mut event = ArkUIOwnedPointerEvent::new(UIPointerEventType::Touch)?;
/// event.set_source_type(UIInputSourceType::TouchScreen)?;
/// event.set_tool_type(UIInputToolType::Pen)?;
/// event.set_action(UIPointerEventAction::Touch(UIInputAction::Move))?;
/// event.set_pressure(0.75)?;
/// event.set_tilt_angle(25.0, -15.0)?;
/// event.set_roll_angle(45.0)?;
/// // Post to a live ArkUI node on its UI thread, then drop the event normally.
/// # Ok(())
/// # }
/// ```
///
/// Native ownership cannot cross threads:
/// ```compile_fail
/// use ohos_arkui_input_binding::ArkUIOwnedPointerEvent;
/// fn require_send<T: Send>() {}
/// require_send::<ArkUIOwnedPointerEvent>();
/// ```
///
/// An owner cannot be used after it is destroyed:
/// ```compile_fail
/// use ohos_arkui_input_binding::{ArkUIOwnedPointerEvent, UIPointerEventType};
/// let mut event = ArkUIOwnedPointerEvent::new(UIPointerEventType::Touch).unwrap();
/// drop(event);
/// event.set_pressure(0.5).unwrap();
/// ```
#[derive(Debug)]
pub struct ArkUIOwnedPointerEvent {
    event: NonNull<sys::ArkUI_UIInputEvent>,
    event_type: UIPointerEventType,
}

fn invalid_parameter() -> ArkUIInputError {
    ArkUIInputError::InternalError(sys::ArkUI_ErrorCode_ARKUI_ERROR_CODE_PARAM_INVALID as i32)
}

fn native_index(index: usize) -> Result<i32, ArkUIInputError> {
    i32::try_from(index).map_err(|_| invalid_parameter())
}

fn ensure_native_node_api() -> Result<(), ArkUIInputError> {
    // The native clone and destroy implementations require NodeModel::GetFullImpl.
    // ArkTS rendering alone does not initialize that native interface. Query it
    // before allocating, so a successful constructor also has a usable destructor.
    let api = unsafe {
        sys::OH_ArkUI_QueryModuleInterfaceByName(
            sys::ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_NODE,
            c"ArkUI_NativeNodeAPI_1".as_ptr(),
        )
    };
    if api.is_null() {
        Err(invalid_parameter())
    } else {
        Ok(())
    }
}

impl ArkUIOwnedPointerEvent {
    /// Allocates a new event. Requires an API 24 or newer runtime.
    pub fn new(event_type: UIPointerEventType) -> Result<Self, ArkUIInputError> {
        ensure_native_node_api()?;
        let mut event = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_ArkUI_PointerEvent_CreatePointerEvent(&mut event, event_type.into()) as i32
        })?;
        Ok(Self {
            event: NonNull::new(event).ok_or_else(invalid_parameter)?,
            event_type,
        })
    }

    /// Copies a native touch, mouse, or axis event without taking its ownership.
    ///
    /// # Safety
    /// `event` must point to a live ArkUI input event for the duration of this call.
    /// Call on its ArkUI UI thread, for example inside its input callback. The
    /// returned copy is independently owned and can outlive the callback.
    pub unsafe fn clone_from_raw(
        event: *const sys::ArkUI_UIInputEvent,
    ) -> Result<Self, ArkUIInputError> {
        if event.is_null() {
            return Err(invalid_parameter());
        }
        let event_type = UIPointerEventType::try_from_raw(unsafe {
            sys::OH_ArkUI_UIInputEvent_GetType(event) as u32
        })
        .ok_or_else(invalid_parameter)?;
        Self::clone_pointer(event, event_type)
    }

    /// Makes an independent native copy. Allocation or native errors are returned.
    pub fn try_clone(&self) -> Result<Self, ArkUIInputError> {
        Self::clone_pointer(self.raw(), self.event_type)
    }

    fn clone_pointer(
        source: *const sys::ArkUI_UIInputEvent,
        event_type: UIPointerEventType,
    ) -> Result<Self, ArkUIInputError> {
        ensure_native_node_api()?;
        let mut event = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_ArkUI_PointerEvent_CreateClonedPointerEvent(source, &mut event) as i32
        })?;
        Ok(Self {
            event: NonNull::new(event).ok_or_else(invalid_parameter)?,
            event_type,
        })
    }

    pub fn event_type(&self) -> UIPointerEventType {
        self.event_type
    }

    /// Borrows the native pointer. It must not be destroyed or used after `self`
    /// is dropped. Passing it to native code does not transfer ownership.
    pub fn raw(&self) -> *const sys::ArkUI_UIInputEvent {
        self.event.as_ptr()
    }

    /// Sets an action after checking that its category matches this event.
    pub fn set_action(&mut self, action: UIPointerEventAction) -> Result<(), ArkUIInputError> {
        let raw = match (self.event_type, action) {
            (UIPointerEventType::Touch, UIPointerEventAction::Touch(action)) => i32::from(action),
            (UIPointerEventType::Mouse, UIPointerEventAction::Mouse(action)) => {
                u32::from(action) as i32
            }
            (UIPointerEventType::Axis, UIPointerEventAction::Axis(action)) => {
                u32::from(action) as i32
            }
            _ => return Err(invalid_parameter()),
        };
        check_status(unsafe { sys::OH_ArkUI_ClonedEvent_SetActionType(self.raw(), raw) as i32 })
    }

    /// Sets the input source category.
    pub fn set_source_type(&mut self, source: UIInputSourceType) -> Result<(), ArkUIInputError> {
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetSourceType(self.raw(), u32::from(source) as i32) as i32
        })
    }

    /// Sets the input tool, including Pen.
    pub fn set_tool_type(&mut self, tool: UIInputToolType) -> Result<(), ArkUIInputError> {
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetToolType(self.raw(), u32::from(tool) as i32) as i32
        })
    }

    /// Sets changed-point pressure. Values greater than 1 are preserved for devices that support them.
    pub fn set_pressure(&mut self, pressure: f32) -> Result<(), ArkUIInputError> {
        if !pressure.is_finite() {
            return Err(invalid_parameter());
        }
        if pressure < 0.0 {
            return Err(invalid_parameter());
        }
        check_status(unsafe { sys::OH_ArkUI_ClonedEvent_SetPressure(self.raw(), pressure) as i32 })
    }

    /// Sets pressure for an indexed touch point; values greater than 1 are preserved.
    pub fn set_pressure_by_index(
        &mut self,
        pressure: f32,
        pointer_index: usize,
    ) -> Result<(), ArkUIInputError> {
        if !pressure.is_finite() {
            return Err(invalid_parameter());
        }
        let pointer_index = native_index(pointer_index)?;
        if pressure < 0.0 {
            return Err(invalid_parameter());
        }
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetPressureByIndex(self.raw(), pressure, pointer_index) as i32
        })
    }

    /// Sets the event timestamp in nanoseconds.
    pub fn set_event_time(&mut self, timestamp: i64) -> Result<(), ArkUIInputError> {
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetEventTime(self.raw(), timestamp) as i32
        })
    }

    /// Sets the originating device ID.
    pub fn set_device_id(&mut self, device_id: i32) -> Result<(), ArkUIInputError> {
        check_status(unsafe { sys::OH_ArkUI_ClonedEvent_SetDeviceId(self.raw(), device_id) as i32 })
    }

    /// Sets the target display ID.
    pub fn set_target_display_id(&mut self, display_id: i32) -> Result<(), ArkUIInputError> {
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetTargetDisplayId(self.raw(), display_id) as i32
        })
    }

    /// Sets the triggering touch point ID.
    pub fn set_changed_finger_id(&mut self, finger_id: i32) -> Result<(), ArkUIInputError> {
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetChangedFingerId(self.raw(), finger_id) as i32
        })
    }

    /// Sets the ID of an indexed touch point.
    pub fn set_finger_id_by_index(
        &mut self,
        finger_id: i32,
        pointer_index: usize,
    ) -> Result<(), ArkUIInputError> {
        let pointer_index = native_index(pointer_index)?;
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetFingerIdByIndex(self.raw(), finger_id, pointer_index)
                as i32
        })
    }

    /// Sets the triggering position relative to the window, in pixels.
    pub fn set_changed_window_position(&mut self, x: f32, y: f32) -> Result<(), ArkUIInputError> {
        if !x.is_finite() {
            return Err(invalid_parameter());
        }
        if !y.is_finite() {
            return Err(invalid_parameter());
        }
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetChangedWindowPosition(self.raw(), x, y) as i32
        })
    }

    /// Sets an indexed touch position relative to the window, in pixels.
    pub fn set_window_position_by_index(
        &mut self,
        x: f32,
        y: f32,
        pointer_index: usize,
    ) -> Result<(), ArkUIInputError> {
        if !x.is_finite() {
            return Err(invalid_parameter());
        }
        if !y.is_finite() {
            return Err(invalid_parameter());
        }
        let pointer_index = native_index(pointer_index)?;
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetWindowPositionByIndex(self.raw(), x, y, pointer_index)
                as i32
        })
    }

    /// Sets the triggering position relative to the screen, in pixels.
    pub fn set_changed_screen_position(&mut self, x: f32, y: f32) -> Result<(), ArkUIInputError> {
        if !x.is_finite() {
            return Err(invalid_parameter());
        }
        if !y.is_finite() {
            return Err(invalid_parameter());
        }
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetChangedScreenPosition(self.raw(), x, y) as i32
        })
    }

    /// Sets an indexed touch position relative to the screen, in pixels.
    pub fn set_screen_position_by_index(
        &mut self,
        x: f32,
        y: f32,
        pointer_index: usize,
    ) -> Result<(), ArkUIInputError> {
        if !x.is_finite() {
            return Err(invalid_parameter());
        }
        if !y.is_finite() {
            return Err(invalid_parameter());
        }
        let pointer_index = native_index(pointer_index)?;
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetScreenPositionByIndex(self.raw(), x, y, pointer_index)
                as i32
        })
    }

    /// Sets the changed touch point in global display coordinates, in pixels.
    pub fn set_changed_global_display_position(
        &mut self,
        x: f32,
        y: f32,
    ) -> Result<(), ArkUIInputError> {
        if !x.is_finite() {
            return Err(invalid_parameter());
        }
        if !y.is_finite() {
            return Err(invalid_parameter());
        }
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetChangedGlobalDisplayPosition(self.raw(), x, y) as i32
        })
    }

    /// Sets an indexed touch point in global display coordinates, in pixels.
    pub fn set_global_display_position_by_index(
        &mut self,
        x: f32,
        y: f32,
        pointer_index: usize,
    ) -> Result<(), ArkUIInputError> {
        if !x.is_finite() {
            return Err(invalid_parameter());
        }
        if !y.is_finite() {
            return Err(invalid_parameter());
        }
        let pointer_index = native_index(pointer_index)?;
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetGlobalDisplayPositionByIndex(
                self.raw(),
                x,
                y,
                pointer_index,
            ) as i32
        })
    }

    /// Sets the event handle ID.
    pub fn set_handle_id(&mut self, handle_id: i32) -> Result<(), ArkUIInputError> {
        check_status(unsafe { sys::OH_ArkUI_ClonedEvent_SetHandleId(self.raw(), handle_id) as i32 })
    }

    /// Sets changed-point pen tilt in degrees. Both angles must be within [-90, 90].
    pub fn set_tilt_angle(&mut self, tilt_x: f32, tilt_y: f32) -> Result<(), ArkUIInputError> {
        if !tilt_x.is_finite() {
            return Err(invalid_parameter());
        }
        if !tilt_y.is_finite() {
            return Err(invalid_parameter());
        }
        if !(-90.0..=90.0).contains(&tilt_x) {
            return Err(invalid_parameter());
        }
        if !(-90.0..=90.0).contains(&tilt_y) {
            return Err(invalid_parameter());
        }
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetTiltAngle(self.raw(), tilt_x, tilt_y) as i32
        })
    }

    /// Sets pen rotation around its Z axis in degrees, without normalizing the angle.
    pub fn set_roll_angle(&mut self, angle: f32) -> Result<(), ArkUIInputError> {
        if !angle.is_finite() {
            return Err(invalid_parameter());
        }
        check_status(unsafe { sys::OH_ArkUI_ClonedEvent_SetRollAngle(self.raw(), angle) as i32 })
    }

    /// Sets the changed touch contact dimensions, in pixels.
    pub fn set_changed_touch_area(
        &mut self,
        width: f32,
        height: f32,
    ) -> Result<(), ArkUIInputError> {
        if !width.is_finite() {
            return Err(invalid_parameter());
        }
        if !height.is_finite() {
            return Err(invalid_parameter());
        }
        if width < 0.0 {
            return Err(invalid_parameter());
        }
        if height < 0.0 {
            return Err(invalid_parameter());
        }
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetChangedTouchArea(self.raw(), width, height) as i32
        })
    }

    /// Sets indexed touch contact dimensions, in pixels.
    pub fn set_touch_area_by_index(
        &mut self,
        width: f32,
        height: f32,
        pointer_index: usize,
    ) -> Result<(), ArkUIInputError> {
        if !width.is_finite() {
            return Err(invalid_parameter());
        }
        if !height.is_finite() {
            return Err(invalid_parameter());
        }
        let pointer_index = native_index(pointer_index)?;
        if width < 0.0 {
            return Err(invalid_parameter());
        }
        if height < 0.0 {
            return Err(invalid_parameter());
        }
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetTouchAreaByIndex(self.raw(), width, height, pointer_index)
                as i32
        })
    }

    /// Sets the hand of the changed touch point.
    pub fn set_changed_interaction_hand(
        &mut self,
        hand: InteractionHand,
    ) -> Result<(), ArkUIInputError> {
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetChangedInteractionHand(self.raw(), u32::from(hand) as i32)
                as i32
        })
    }

    /// Sets the hand of an indexed touch point.
    pub fn set_interaction_hand_by_index(
        &mut self,
        hand: InteractionHand,
        pointer_index: usize,
    ) -> Result<(), ArkUIInputError> {
        let pointer_index = native_index(pointer_index)?;
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetInteractionHandByIndex(
                self.raw(),
                u32::from(hand) as i32,
                pointer_index,
            ) as i32
        })
    }

    /// Sets the press timestamp of an indexed touch point, in nanoseconds.
    pub fn set_pressed_time_by_index(
        &mut self,
        timestamp: i64,
        pointer_index: usize,
    ) -> Result<(), ArkUIInputError> {
        let pointer_index = native_index(pointer_index)?;
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetPressedTimeByIndex(self.raw(), timestamp, pointer_index)
                as i32
        })
    }

    /// Sets the nonnegative pinch scale for an axis event.
    pub fn set_pinch_axis_scale_value(&mut self, scale: f64) -> Result<(), ArkUIInputError> {
        if !scale.is_finite() {
            return Err(invalid_parameter());
        }
        if scale < 0.0 {
            return Err(invalid_parameter());
        }
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetPinchAxisScaleValue(self.raw(), scale) as i32
        })
    }

    /// Sets the horizontal axis value.
    pub fn set_horizontal_axis_scale_value(&mut self, value: f64) -> Result<(), ArkUIInputError> {
        if !value.is_finite() {
            return Err(invalid_parameter());
        }
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetHorizontalAxisScaleValue(self.raw(), value) as i32
        })
    }

    /// Sets the vertical axis value.
    pub fn set_vertical_axis_scale_value(&mut self, value: f64) -> Result<(), ArkUIInputError> {
        if !value.is_finite() {
            return Err(invalid_parameter());
        }
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetVerticalAxisScaleValue(self.raw(), value) as i32
        })
    }

    /// Sets the scrolling step coefficient of an axis event.
    pub fn set_scroll_step(&mut self, step: i32) -> Result<(), ArkUIInputError> {
        check_status(unsafe { sys::OH_ArkUI_ClonedEvent_SetScrollStep(self.raw(), step) as i32 })
    }

    /// Sets the changed button for a mouse event.
    pub fn set_mouse_button(&mut self, button: UIMouseEventButton) -> Result<(), ArkUIInputError> {
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetMouseButton(self.raw(), u32::from(button) as i32) as i32
        })
    }

    /// Sets the unaccelerated horizontal mouse movement.
    pub fn set_raw_delta_x(&mut self, delta: f32) -> Result<(), ArkUIInputError> {
        if !delta.is_finite() {
            return Err(invalid_parameter());
        }
        check_status(unsafe { sys::OH_ArkUI_ClonedEvent_SetRawDeltaX(self.raw(), delta) as i32 })
    }

    /// Sets the unaccelerated vertical mouse movement.
    pub fn set_raw_delta_y(&mut self, delta: f32) -> Result<(), ArkUIInputError> {
        if !delta.is_finite() {
            return Err(invalid_parameter());
        }
        check_status(unsafe { sys::OH_ArkUI_ClonedEvent_SetRawDeltaY(self.raw(), delta) as i32 })
    }

    /// Sets pressed key codes. ArkUI copies the array during this call.
    /// An empty array is rejected by the native API; it does not clear the keys.
    pub fn set_pressed_keys(&mut self, keys: &[i32]) -> Result<(), ArkUIInputError> {
        let length = native_index(keys.len())?;
        if keys.is_empty() {
            return Err(invalid_parameter());
        }
        // The C signature takes int32_t*, so never cast a shared Rust slice to
        // mutable. The native implementation copies these values synchronously.
        let mut keys = keys.to_vec();
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetPressedKeys(self.raw(), keys.as_mut_ptr(), length) as i32
        })
    }

    /// Sets the pressed mouse buttons. ArkUI copies the array during this call.
    /// An empty array is rejected by the native API.
    pub fn set_pressed_buttons(
        &mut self,
        buttons: &[UIMouseEventButton],
    ) -> Result<(), ArkUIInputError> {
        let length = native_index(buttons.len())?;
        if buttons.is_empty() {
            return Err(invalid_parameter());
        }
        let buttons: Vec<i32> = buttons
            .iter()
            .map(|button| u32::from(*button) as i32)
            .collect();
        check_status(unsafe {
            sys::OH_ArkUI_ClonedEvent_SetPressedButtons(self.raw(), buttons.as_ptr(), length) as i32
        })
    }

    /// Dispatches this event to a node with the specified gesture competition policy.
    /// This is application-local ArkUI dispatch, not system-wide input injection.
    /// Ownership stays with this object, including when dispatch returns an error.
    ///
    /// # Safety
    /// `node` must be a live ArkUI node on the current UI thread. Callers must obey
    /// the native event dispatch and callback reentrancy requirements for that node.
    pub unsafe fn post_to_node(
        &self,
        node: sys::ArkUI_NodeHandle,
        strategy: UIEventCompetitionStrategy,
    ) -> Result<(), ArkUIInputError> {
        check_status(unsafe {
            sys::OH_ArkUI_PointerEvent_PostClonedEventWithStrategy(
                node,
                self.raw(),
                strategy.into(),
            ) as i32
        })
    }
}

impl Drop for ArkUIOwnedPointerEvent {
    fn drop(&mut self) {
        unsafe {
            sys::OH_ArkUI_PointerEvent_DestroyClonedPointerEvent(self.raw());
        }
    }
}
