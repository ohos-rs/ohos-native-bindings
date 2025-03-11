use std::{
    cell::{LazyCell, RefCell},
    ffi::CString,
    os::raw::c_void,
    ptr::NonNull,
    rc::Rc,
};

use ohos_arkui_sys::{
    ArkUI_GestureEvent, ArkUI_GestureEventActionTypeMask, ArkUI_GestureMask, ArkUI_GesturePriority,
    ArkUI_GestureRecognizerHandle, ArkUI_GroupGestureMode,
    ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_GESTURE, ArkUI_NativeGestureAPI_1, ArkUI_NodeHandle,
    OH_ArkUI_GestureEvent_GetActionType, OH_ArkUI_LongPress_GetRepeatCount,
    OH_ArkUI_PanGesture_GetOffsetX, OH_ArkUI_PanGesture_GetOffsetY,
    OH_ArkUI_PanGesture_GetVelocity, OH_ArkUI_PanGesture_GetVelocityX,
    OH_ArkUI_PanGesture_GetVelocityY, OH_ArkUI_PinchGesture_GetCenterX,
    OH_ArkUI_PinchGesture_GetCenterY, OH_ArkUI_PinchGesture_GetScale,
    OH_ArkUI_QueryModuleInterfaceByName, OH_ArkUI_RotationGesture_GetAngle,
    OH_ArkUI_SwipeGesture_GetAngle, OH_ArkUI_SwipeGesture_GetVelocity,
};

use crate::{
    ArkUIError, GestureData, GestureEventAction, GestureEventData, GestureRecognizerType,
    InnerGestureData, LongPressGestureData, PanGestureData, PinchGestureData, RotationGestureData,
    SwipeGestureData,
};

/// ArkUINativeGestureAPI1 struct
/// Only can be used in main thread
pub const ARK_UI_NATIVE_GESTURE_API_1: LazyCell<ArkUINativeGestureAPI1> = LazyCell::new(|| {
    let api = ArkUINativeGestureAPI1::new();
    api
});

pub struct ArkUINativeGestureAPI1 {
    pub(crate) raw: NonNull<ArkUI_NativeGestureAPI_1>,
}

impl ArkUINativeGestureAPI1 {
    /// allow us to get the pointer of ArkUI_NativeGestureAPI_1 and use it directly
    pub fn raw(&self) -> NonNull<ArkUI_NativeGestureAPI_1> {
        self.raw
    }

    pub fn new() -> Self {
        #[allow(unused_assignments)]
        let mut api: *mut ArkUI_NativeGestureAPI_1 = std::ptr::null_mut();
        let struct_name =
            CString::new("ArkUI_NativeGestureAPI_1").expect("Failed to create CString");
        let raw_ptr = unsafe {
            OH_ArkUI_QueryModuleInterfaceByName(
                ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_GESTURE,
                struct_name.as_ptr().cast(),
            )
        };
        #[cfg(debug_assertions)]
        assert!(!raw_ptr.is_null(), "ArkUI_NativeGestureAPI_1 is NULL");
        api = raw_ptr.cast();
        Self {
            raw: unsafe { NonNull::new_unchecked(api) },
        }
    }

    pub fn create_long_gesture(
        &self,
        finger_number: i32,
        repeat: bool,
        duration: i32,
    ) -> Result<ArkUI_GestureRecognizerHandle, ArkUIError> {
        unsafe {
            if let Some(create_long_press_gesture) = (*self.raw.as_ptr()).createLongPressGesture {
                let ret = create_long_press_gesture(finger_number, repeat, duration);
                if ret.is_null() {
                    Err(ArkUIError::NullError(String::from(
                        "api is ArkUINativeGestureAPI1::createLongPressGesture",
                    )))
                } else {
                    Ok(ret)
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUINativeGestureAPI1::createLongPressGesture is None",
                )))
            }
        }
    }

    pub fn create_tap_gesture(
        &self,
        count: i32,
        finger: i32,
    ) -> Result<ArkUI_GestureRecognizerHandle, ArkUIError> {
        unsafe {
            if let Some(create_tap_gesture) = (*self.raw.as_ptr()).createTapGesture {
                let ret = create_tap_gesture(count, finger);
                if ret.is_null() {
                    Err(ArkUIError::NullError(String::from(
                        "api is ArkUINativeGestureAPI1::createTapGesture",
                    )))
                } else {
                    Ok(ret)
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUINativeGestureAPI1::createTapGesture is None",
                )))
            }
        }
    }

    pub fn create_pan_gesture(
        &self,
        finger: i32,
        direction: u32,
        distance: f64,
    ) -> Result<ArkUI_GestureRecognizerHandle, ArkUIError> {
        unsafe {
            if let Some(create_pan_gesture) = (*self.raw.as_ptr()).createPanGesture {
                let ret = create_pan_gesture(finger, direction, distance);
                if ret.is_null() {
                    Err(ArkUIError::NullError(String::from(
                        "api is ArkUINativeGestureAPI1::createPanGesture",
                    )))
                } else {
                    Ok(ret)
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUINativeGestureAPI1::createPanGesture is None",
                )))
            }
        }
    }

    pub fn create_pinch_gesture(
        &self,
        finger: i32,
        distance: f64,
    ) -> Result<ArkUI_GestureRecognizerHandle, ArkUIError> {
        unsafe {
            if let Some(create_pinch_gesture) = (*self.raw.as_ptr()).createPinchGesture {
                let ret = create_pinch_gesture(finger, distance);
                if ret.is_null() {
                    Err(ArkUIError::NullError(String::from(
                        "api is ArkUINativeGestureAPI1::createPinchGesture",
                    )))
                } else {
                    Ok(ret)
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUINativeGestureAPI1::createPinchGesture is None",
                )))
            }
        }
    }

    pub fn create_rotation_gesture(
        &self,
        finger: i32,
        angle: f64,
    ) -> Result<ArkUI_GestureRecognizerHandle, ArkUIError> {
        unsafe {
            if let Some(create_rotation_gesture) = (*self.raw.as_ptr()).createRotationGesture {
                let ret = create_rotation_gesture(finger, angle);
                if ret.is_null() {
                    Err(ArkUIError::NullError(String::from(
                        "api is ArkUINativeGestureAPI1::createRotationGesture",
                    )))
                } else {
                    Ok(ret)
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUINativeGestureAPI1::createRotationGesture is None",
                )))
            }
        }
    }

    pub fn create_swipe_gesture(
        &self,
        finger: i32,
        direction: u32,
        speed: f64,
    ) -> Result<ArkUI_GestureRecognizerHandle, ArkUIError> {
        unsafe {
            if let Some(create_swipe_gesture) = (*self.raw.as_ptr()).createSwipeGesture {
                let ret = create_swipe_gesture(finger, direction, speed);
                if ret.is_null() {
                    Err(ArkUIError::NullError(String::from(
                        "api is ArkUINativeGestureAPI1::createSwipeGesture",
                    )))
                } else {
                    Ok(ret)
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUINativeGestureAPI1::createSwipeGesture is None",
                )))
            }
        }
    }

    pub fn create_gesture_group(
        &self,
        mode: ArkUI_GroupGestureMode,
    ) -> Result<ArkUI_GestureRecognizerHandle, ArkUIError> {
        unsafe {
            if let Some(create_group_gesture) = (*self.raw.as_ptr()).createGroupGesture {
                let ret = create_group_gesture(mode);
                if ret.is_null() {
                    Err(ArkUIError::NullError(String::from(
                        "api is ArkUINativeGestureAPI1::createGroupGesture",
                    )))
                } else {
                    Ok(ret)
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUINativeGestureAPI1::createGestureGroup is None",
                )))
            }
        }
    }

    pub fn add_child_gesture(
        &self,
        group: ArkUI_GestureRecognizerHandle,
        child: ArkUI_GestureRecognizerHandle,
    ) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(add_child_gesture) = (*self.raw.as_ptr()).addChildGesture {
                let ret = add_child_gesture(group, child);
                if ret != 0 {
                    Err(ArkUIError::InternalError(String::from(
                        "api is ArkUINativeGestureAPI1::addChildGesture",
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUINativeGestureAPI1::addChildGesture is None",
                )))
            }
        }
    }

    pub fn add_gesture(
        &self,
        gesture: ArkUI_GestureRecognizerHandle,
        node: ArkUI_NodeHandle,
        mode: ArkUI_GesturePriority,
        mask: ArkUI_GestureMask,
    ) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(add_gesture_to_node) = (*self.raw.as_ptr()).addGestureToNode {
                let ret = add_gesture_to_node(node, gesture, mode, mask);
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is ArkUINativeGestureAPI1::addGestureToNode, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUINativeGestureAPI1::addGestureToNode is None",
                )))
            }
        }
    }

    pub(crate) fn set_gesture_event_to_target(
        &self,
        gesture: ArkUI_GestureRecognizerHandle,
        action_type: ArkUI_GestureEventActionTypeMask,
        extra_params: Rc<RefCell<InnerGestureData>>,
    ) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(set_gesture_event_to_target) = (*self.raw.as_ptr()).setGestureEventTarget {
                let ret = set_gesture_event_to_target(
                    gesture,
                    action_type,
                    Box::into_raw(Box::new(extra_params)) as *mut c_void,
                    Some(target_receiver),
                );
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is ArkUINativeGestureAPI1::setGestureEventTarget, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(String::from(
                    "ArkUINativeGestureAPI1::setGestureEventTarget is None",
                )))
            }
        }
    }
}

unsafe extern "C" fn target_receiver(event: *mut ArkUI_GestureEvent, extra_params: *mut c_void) {
    let user_data: &Rc<RefCell<InnerGestureData>> =
        &*(extra_params as *const Rc<RefCell<InnerGestureData>>);

    let data = user_data.borrow_mut();
    let event_action_type: GestureEventAction = OH_ArkUI_GestureEvent_GetActionType(event).into();

    let event_data: GestureData = match data.gesture_type {
        GestureRecognizerType::LongPressGesture => {
            let repeat = OH_ArkUI_LongPress_GetRepeatCount(event);
            GestureData::LongPress(LongPressGestureData { repeat })
        }
        GestureRecognizerType::TapGesture => GestureData::Tap,
        GestureRecognizerType::PanGesture => {
            let velocity = OH_ArkUI_PanGesture_GetVelocity(event);
            let velocity_x = OH_ArkUI_PanGesture_GetVelocityX(event);
            let velocity_y = OH_ArkUI_PanGesture_GetVelocityY(event);
            let offset_x = OH_ArkUI_PanGesture_GetOffsetX(event);
            let offset_y = OH_ArkUI_PanGesture_GetOffsetY(event);
            GestureData::Pan(PanGestureData {
                velocity,
                velocity_x,
                velocity_y,
                offset_x,
                offset_y,
            })
        }
        GestureRecognizerType::PinchGesture => {
            let scale = OH_ArkUI_PinchGesture_GetScale(event);
            let center_x = OH_ArkUI_PinchGesture_GetCenterX(event);
            let center_y = OH_ArkUI_PinchGesture_GetCenterY(event);
            GestureData::Pinch(PinchGestureData {
                scale,
                center_x,
                center_y,
            })
        }
        GestureRecognizerType::RotationGesture => {
            let angle = OH_ArkUI_RotationGesture_GetAngle(event);
            GestureData::Rotation(RotationGestureData { angle })
        }
        GestureRecognizerType::SwipeGesture => {
            let angle = OH_ArkUI_SwipeGesture_GetAngle(event);
            let velocity = OH_ArkUI_SwipeGesture_GetVelocity(event);
            GestureData::Swipe(SwipeGestureData { angle, velocity })
        }
        _ => unreachable!("Invalid gesture type"),
    };

    if let Some(event) = data.gesture_callback.as_ref() {
        event(GestureEventData {
            data: data.user_data,
            event_action_type: event_action_type,
            event_action_data: event_data,
        });
    }
}
