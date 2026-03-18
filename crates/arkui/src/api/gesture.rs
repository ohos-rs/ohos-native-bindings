use std::{
    cell::{LazyCell, RefCell},
    collections::HashMap,
    ffi::CString,
    os::raw::c_void,
    ptr::NonNull,
    rc::Rc,
    sync::{Mutex, OnceLock},
};

use ohos_arkui_input_binding::sys::ArkUI_NodeHandle;
use ohos_arkui_input_binding::ArkUIErrorCode;
#[cfg(feature = "api-20")]
use ohos_arkui_sys::OH_ArkUI_PreventGestureRecognizerBegin;
use ohos_arkui_sys::{
    ArkUI_GestureEvent, ArkUI_GestureEventActionTypeMask, ArkUI_GestureEventTargetInfo,
    ArkUI_GestureInterruptInfo, ArkUI_GestureInterruptResult, ArkUI_GestureRecognizer,
    ArkUI_GestureRecognizerHandle, ArkUI_GestureRecognizerHandleArray,
    ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_GESTURE, ArkUI_NativeGestureAPI_1,
    ArkUI_ParallelInnerGestureEvent, OH_ArkUI_GestureEventTargetInfo_IsScrollBegin,
    OH_ArkUI_GestureEventTargetInfo_IsScrollEnd, OH_ArkUI_GestureEvent_GetActionType,
    OH_ArkUI_GestureEvent_GetNode, OH_ArkUI_GestureEvent_GetRawInputEvent,
    OH_ArkUI_GestureInterruptInfo_GetGestureEvent, OH_ArkUI_GestureInterruptInfo_GetRecognizer,
    OH_ArkUI_GestureInterruptInfo_GetSystemFlag,
    OH_ArkUI_GestureInterruptInfo_GetSystemRecognizerType, OH_ArkUI_LongPress_GetRepeatCount,
    OH_ArkUI_PanGesture_GetOffsetX, OH_ArkUI_PanGesture_GetOffsetY,
    OH_ArkUI_PanGesture_GetVelocity, OH_ArkUI_PanGesture_GetVelocityX,
    OH_ArkUI_PanGesture_GetVelocityY, OH_ArkUI_ParallelInnerGestureEvent_GetConflictRecognizers,
    OH_ArkUI_ParallelInnerGestureEvent_GetCurrentRecognizer,
    OH_ArkUI_ParallelInnerGestureEvent_GetUserData, OH_ArkUI_PinchGesture_GetCenterX,
    OH_ArkUI_PinchGesture_GetCenterY, OH_ArkUI_PinchGesture_GetScale,
    OH_ArkUI_QueryModuleInterfaceByName, OH_ArkUI_RotationGesture_GetAngle,
    OH_ArkUI_SwipeGesture_GetAngle, OH_ArkUI_SwipeGesture_GetVelocity,
};
#[cfg(feature = "api-18")]
use ohos_arkui_sys::{
    ArkUI_NativeGestureAPI_2, OH_ArkUI_GestureInterrupter_GetUserData,
    OH_ArkUI_GetGestureParam_DirectMask, OH_ArkUI_GetGestureParam_FingerCount,
    OH_ArkUI_GetGestureParam_angle, OH_ArkUI_GetGestureParam_distance,
    OH_ArkUI_GetGestureParam_distanceThreshold, OH_ArkUI_GetGestureParam_duration,
    OH_ArkUI_GetGestureParam_limitFingerCount, OH_ArkUI_GetGestureParam_repeat,
    OH_ArkUI_GetGestureParam_speed,
};
#[cfg(feature = "api-15")]
use ohos_arkui_sys::{
    ArkUI_TouchRecognizerHandle, ArkUI_TouchRecognizerHandleArray,
    OH_ArkUI_GestureInterruptInfo_GetTouchRecognizers, OH_ArkUI_TouchRecognizer_CancelTouch,
    OH_ArkUI_TouchRecognizer_GetNodeHandle,
};
#[cfg(feature = "api-22")]
use ohos_arkui_sys::{
    OH_ArkUI_LongPressGesture_GetAllowableMovement, OH_ArkUI_LongPressGesture_SetAllowableMovement,
};
#[cfg(feature = "api-19")]
use ohos_arkui_sys::{
    OH_ArkUI_PanGesture_GetDistanceByToolType, OH_ArkUI_PanGesture_SetDistanceMap,
};

use crate::{
    check_arkui_status, ArkUIError, ArkUIResult, GestureData, GestureEventAction, GestureEventData,
    GestureInterruptResult, GestureRecognizerState, GestureRecognizerType, InnerGestureData,
    LongPressGestureData, PanGestureData, PinchGestureData, RotationGestureData, SwipeGestureData,
};

thread_local! {
    /// ArkUINativeGestureAPI1 struct
    /// Only can be used in main thread
    pub(crate) static ARK_UI_NATIVE_GESTURE_API_1: LazyCell<ArkUINativeGestureAPI1> =
    LazyCell::new(ArkUINativeGestureAPI1::new);

    #[cfg(feature = "api-18")]
    pub(crate) static ARK_UI_NATIVE_GESTURE_API_2: LazyCell<ArkUINativeGestureAPI2> =
    LazyCell::new(ArkUINativeGestureAPI2::new);
}

struct InnerGestureParallelCallbackContext {
    callback: Box<dyn Fn(ParallelInnerGestureEventRef) -> Option<GestureRecognizerRef>>,
}

struct GestureEventTargetCallbackContext {
    callback: Box<dyn Fn(GestureEventRef)>,
}

struct GestureInterrupterCallbackContext {
    callback: Box<dyn Fn(GestureInterruptInfoRef) -> GestureInterruptResult>,
}

#[cfg(feature = "api-20")]
struct TouchTestDoneCallbackContext {
    callback: Box<dyn Fn(GestureEventRef, Vec<GestureRecognizerRef>)>,
}

struct GestureDisposeNotifyCallbackContext {
    callback: Box<dyn Fn()>,
}

type InnerGestureParallelCallbackMap = HashMap<usize, usize>;
type GestureEventTargetCallbackMap = HashMap<usize, usize>;
type GestureInterrupterCallbackMap = HashMap<usize, usize>;
#[cfg(feature = "api-20")]
type TouchTestDoneCallbackMap = HashMap<usize, usize>;
type GestureDisposeNotifyCallbackMap = HashMap<usize, usize>;

static INNER_GESTURE_PARALLEL_CALLBACK_CONTEXTS: OnceLock<Mutex<InnerGestureParallelCallbackMap>> =
    OnceLock::new();
static GESTURE_EVENT_TARGET_CALLBACK_CONTEXTS: OnceLock<Mutex<GestureEventTargetCallbackMap>> =
    OnceLock::new();
static GESTURE_INTERRUPTER_CALLBACK_CONTEXTS: OnceLock<Mutex<GestureInterrupterCallbackMap>> =
    OnceLock::new();
#[cfg(feature = "api-20")]
static TOUCH_TEST_DONE_CALLBACK_CONTEXTS: OnceLock<Mutex<TouchTestDoneCallbackMap>> =
    OnceLock::new();
static GESTURE_DISPOSE_NOTIFY_CALLBACK_CONTEXTS: OnceLock<Mutex<GestureDisposeNotifyCallbackMap>> =
    OnceLock::new();

fn inner_gesture_parallel_callback_contexts() -> &'static Mutex<InnerGestureParallelCallbackMap> {
    INNER_GESTURE_PARALLEL_CALLBACK_CONTEXTS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn gesture_event_target_callback_contexts() -> &'static Mutex<GestureEventTargetCallbackMap> {
    GESTURE_EVENT_TARGET_CALLBACK_CONTEXTS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn gesture_interrupter_callback_contexts() -> &'static Mutex<GestureInterrupterCallbackMap> {
    GESTURE_INTERRUPTER_CALLBACK_CONTEXTS.get_or_init(|| Mutex::new(HashMap::new()))
}

#[cfg(feature = "api-20")]
fn touch_test_done_callback_contexts() -> &'static Mutex<TouchTestDoneCallbackMap> {
    TOUCH_TEST_DONE_CALLBACK_CONTEXTS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn gesture_dispose_notify_callback_contexts() -> &'static Mutex<GestureDisposeNotifyCallbackMap> {
    GESTURE_DISPOSE_NOTIFY_CALLBACK_CONTEXTS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(crate) struct ArkUINativeGestureAPI1(pub(crate) NonNull<ArkUI_NativeGestureAPI_1>);

impl ArkUINativeGestureAPI1 {
    /// allow us to get the pointer of ArkUI_NativeGestureAPI_1 and use it directly
    pub(crate) fn raw(&self) -> *mut ArkUI_NativeGestureAPI_1 {
        self.0.as_ptr()
    }

    pub fn new() -> Self {
        let struct_name = CString::new("ArkUI_NativeGestureAPI_1").unwrap();
        let raw_ptr = unsafe {
            OH_ArkUI_QueryModuleInterfaceByName(
                ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_GESTURE,
                struct_name.as_ptr().cast(),
            )
        };
        let api = NonNull::new(raw_ptr.cast())
            .unwrap_or_else(|| panic!("ArkUI_NativeGestureAPI_1 is NULL"));
        Self(api)
    }

    pub fn create_long_gesture(
        &self,
        finger_number: i32,
        repeat: bool,
        duration: i32,
    ) -> ArkUIResult<ArkUI_GestureRecognizerHandle> {
        unsafe {
            if let Some(create_long_press_gesture) = (*self.raw()).createLongPressGesture {
                let ret = create_long_press_gesture(finger_number, repeat, duration);
                if ret.is_null() {
                    Err(ArkUIError::new(
                        ArkUIErrorCode::AttributeOrEventNotSupported,
                        "ArkUINativeGestureAPI1::createLongPressGesture is None",
                    ))
                } else {
                    Ok(ret)
                }
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::createLongPressGesture is None",
                ))
            }
        }
    }

    pub fn create_tap_gesture(
        &self,
        count: i32,
        finger: i32,
    ) -> ArkUIResult<ArkUI_GestureRecognizerHandle> {
        unsafe {
            if let Some(create_tap_gesture) = (*self.raw()).createTapGesture {
                let ret = create_tap_gesture(count, finger);
                if ret.is_null() {
                    Err(ArkUIError::new(
                        ArkUIErrorCode::AttributeOrEventNotSupported,
                        "ArkUINativeGestureAPI1::createTapGesture is None",
                    ))
                } else {
                    Ok(ret)
                }
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::createTapGesture is None",
                ))
            }
        }
    }

    pub fn create_tap_gesture_with_distance_threshold(
        &self,
        count: i32,
        finger: i32,
        distance_threshold: f64,
    ) -> ArkUIResult<ArkUI_GestureRecognizerHandle> {
        unsafe {
            if let Some(create_tap_gesture) = (*self.raw()).createTapGestureWithDistanceThreshold {
                let ret = create_tap_gesture(count, finger, distance_threshold);
                if ret.is_null() {
                    Err(ArkUIError::new(
                        ArkUIErrorCode::AttributeOrEventNotSupported,
                        "ArkUINativeGestureAPI1::createTapGestureWithDistanceThreshold is None",
                    ))
                } else {
                    Ok(ret)
                }
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::createTapGestureWithDistanceThreshold is None",
                ))
            }
        }
    }

    pub fn create_pan_gesture(
        &self,
        finger: i32,
        direction: crate::GestureDirection,
        distance: f64,
    ) -> ArkUIResult<ArkUI_GestureRecognizerHandle> {
        unsafe {
            if let Some(create_pan_gesture) = (*self.raw()).createPanGesture {
                let ret = create_pan_gesture(finger, direction.into(), distance);
                if ret.is_null() {
                    Err(ArkUIError::new(
                        ArkUIErrorCode::AttributeOrEventNotSupported,
                        "ArkUINativeGestureAPI1::createPanGesture is None",
                    ))
                } else {
                    Ok(ret)
                }
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::createPanGesture is None",
                ))
            }
        }
    }

    pub fn create_pinch_gesture(
        &self,
        finger: i32,
        distance: f64,
    ) -> ArkUIResult<ArkUI_GestureRecognizerHandle> {
        unsafe {
            if let Some(create_pinch_gesture) = (*self.raw()).createPinchGesture {
                let ret = create_pinch_gesture(finger, distance);
                if ret.is_null() {
                    Err(ArkUIError::new(
                        ArkUIErrorCode::AttributeOrEventNotSupported,
                        "ArkUINativeGestureAPI1::createPinchGesture is None",
                    ))
                } else {
                    Ok(ret)
                }
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::createPinchGesture is None",
                ))
            }
        }
    }

    pub fn create_rotation_gesture(
        &self,
        finger: i32,
        angle: f64,
    ) -> ArkUIResult<ArkUI_GestureRecognizerHandle> {
        unsafe {
            if let Some(create_rotation_gesture) = (*self.raw()).createRotationGesture {
                let ret = create_rotation_gesture(finger, angle);
                if ret.is_null() {
                    Err(ArkUIError::new(
                        ArkUIErrorCode::AttributeOrEventNotSupported,
                        "ArkUINativeGestureAPI1::createRotationGesture is None",
                    ))
                } else {
                    Ok(ret)
                }
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::createRotationGesture is None",
                ))
            }
        }
    }

    pub fn create_swipe_gesture(
        &self,
        finger: i32,
        direction: crate::GestureDirection,
        speed: f64,
    ) -> ArkUIResult<ArkUI_GestureRecognizerHandle> {
        unsafe {
            if let Some(create_swipe_gesture) = (*self.raw()).createSwipeGesture {
                let ret = create_swipe_gesture(finger, direction.into(), speed);
                if ret.is_null() {
                    Err(ArkUIError::new(
                        ArkUIErrorCode::AttributeOrEventNotSupported,
                        "ArkUINativeGestureAPI1::createSwipeGesture is None",
                    ))
                } else {
                    Ok(ret)
                }
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::createSwipeGesture is None",
                ))
            }
        }
    }

    pub fn create_gesture_group(
        &self,
        mode: crate::GestureGroupMode,
    ) -> ArkUIResult<ArkUI_GestureRecognizerHandle> {
        unsafe {
            if let Some(create_group_gesture) = (*self.raw()).createGroupGesture {
                let ret = create_group_gesture(mode.into());
                if ret.is_null() {
                    Err(ArkUIError::new(
                        ArkUIErrorCode::AttributeOrEventNotSupported,
                        "ArkUINativeGestureAPI1::createGroupGesture is None",
                    ))
                } else {
                    Ok(ret)
                }
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::createGestureGroup is None",
                ))
            }
        }
    }

    pub fn add_child_gesture(
        &self,
        group: ArkUI_GestureRecognizerHandle,
        child: ArkUI_GestureRecognizerHandle,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(add_child_gesture) = (*self.raw()).addChildGesture {
                check_arkui_status!(add_child_gesture(group, child))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::addChildGesture is None",
                ))
            }
        }
    }

    pub fn remove_child_gesture(
        &self,
        group: ArkUI_GestureRecognizerHandle,
        child: ArkUI_GestureRecognizerHandle,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(remove_child_gesture) = (*self.raw()).removeChildGesture {
                check_arkui_status!(remove_child_gesture(group, child))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::removeChildGesture is None",
                ))
            }
        }
    }

    pub fn add_gesture(
        &self,
        gesture: ArkUI_GestureRecognizerHandle,
        node: ArkUI_NodeHandle,
        mode: crate::GesturePriority,
        mask: crate::GestureMask,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(add_gesture_to_node) = (*self.raw()).addGestureToNode {
                check_arkui_status!(add_gesture_to_node(node, gesture, mode.into(), mask.into()))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::addGestureToNode is None",
                ))
            }
        }
    }

    pub fn remove_gesture(
        &self,
        gesture: ArkUI_GestureRecognizerHandle,
        node: ArkUI_NodeHandle,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(remove_gesture_from_node) = (*self.raw()).removeGestureFromNode {
                check_arkui_status!(remove_gesture_from_node(node, gesture))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::removeGestureFromNode is None",
                ))
            }
        }
    }

    pub(crate) fn dispose_gesture(
        &self,
        gesture: ArkUI_GestureRecognizerHandle,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(dispose) = (*self.raw()).dispose {
                dispose(gesture);
                Ok(())
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::dispose is None",
                ))
            }
        }
    }

    pub fn get_gesture_type(
        &self,
        gesture: ArkUI_GestureRecognizerHandle,
    ) -> ArkUIResult<GestureRecognizerType> {
        unsafe {
            if let Some(get_gesture_type) = (*self.raw()).getGestureType {
                Ok(get_gesture_type(gesture).into())
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::getGestureType is None",
                ))
            }
        }
    }

    fn set_gesture_interrupter_to_node_raw(
        &self,
        node: ArkUI_NodeHandle,
        interrupter: Option<
            unsafe extern "C" fn(
                info: *mut ArkUI_GestureInterruptInfo,
            ) -> ArkUI_GestureInterruptResult,
        >,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(set_gesture_interrupter_to_node) = (*self.raw()).setGestureInterrupterToNode
            {
                check_arkui_status!(set_gesture_interrupter_to_node(node, interrupter))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::setGestureInterrupterToNode is None",
                ))
            }
        }
    }

    pub fn set_gesture_interrupter_to_node<
        T: Fn(GestureInterruptInfoRef) -> GestureInterruptResult + 'static,
    >(
        &self,
        node: ArkUI_NodeHandle,
        interrupter: T,
    ) -> ArkUIResult<()> {
        let callback = Box::into_raw(Box::new(GestureInterrupterCallbackContext {
            callback: Box::new(interrupter),
        }));
        let result = self.set_gesture_interrupter_to_node_raw(
            node,
            Some(gesture_interrupter_callback_trampoline),
        );
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        let mut callbacks = match gesture_interrupter_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(old) = callbacks.insert(node as usize, callback as usize) {
            unsafe {
                drop(Box::from_raw(old as *mut GestureInterrupterCallbackContext));
            }
        }
        Ok(())
    }

    pub(crate) fn clear_gesture_interrupter_to_node(
        &self,
        node: ArkUI_NodeHandle,
    ) -> ArkUIResult<()> {
        self.set_gesture_interrupter_to_node_raw(node, None)?;
        let mut callbacks = match gesture_interrupter_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(callback) = callbacks.remove(&(node as usize)) {
            unsafe {
                drop(Box::from_raw(
                    callback as *mut GestureInterrupterCallbackContext,
                ));
            }
        }
        Ok(())
    }

    fn set_inner_gesture_parallel_to_raw(
        &self,
        node: ArkUI_NodeHandle,
        user_data: *mut c_void,
        parallel_inner_gesture: Option<
            unsafe extern "C" fn(
                event: *mut ArkUI_ParallelInnerGestureEvent,
            ) -> *mut ArkUI_GestureRecognizer,
        >,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(set_inner_gesture_parallel_to) = (*self.raw()).setInnerGestureParallelTo {
                check_arkui_status!(set_inner_gesture_parallel_to(
                    node,
                    user_data,
                    parallel_inner_gesture
                ))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::setInnerGestureParallelTo is None",
                ))
            }
        }
    }

    pub fn set_inner_gesture_parallel_to<
        T: Fn(ParallelInnerGestureEventRef) -> Option<GestureRecognizerRef> + 'static,
    >(
        &self,
        node: ArkUI_NodeHandle,
        parallel_inner_gesture: T,
    ) -> ArkUIResult<()> {
        let callback = Box::into_raw(Box::new(InnerGestureParallelCallbackContext {
            callback: Box::new(parallel_inner_gesture),
        }));
        let result = self.set_inner_gesture_parallel_to_raw(
            node,
            callback.cast(),
            Some(inner_gesture_parallel_callback_trampoline),
        );
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        let mut callbacks = match inner_gesture_parallel_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(old) = callbacks.insert(node as usize, callback as usize) {
            unsafe {
                drop(Box::from_raw(
                    old as *mut InnerGestureParallelCallbackContext,
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn clear_inner_gesture_parallel_to(
        &self,
        node: ArkUI_NodeHandle,
    ) -> ArkUIResult<()> {
        self.set_inner_gesture_parallel_to_raw(node, std::ptr::null_mut(), None)?;
        let mut callbacks = match inner_gesture_parallel_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(callback) = callbacks.remove(&(node as usize)) {
            unsafe {
                drop(Box::from_raw(
                    callback as *mut InnerGestureParallelCallbackContext,
                ));
            }
        }
        Ok(())
    }

    #[cfg(feature = "api-20")]
    pub fn prevent_gesture_recognizer_begin(
        &self,
        recognizer: ArkUI_GestureRecognizerHandle,
    ) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_PreventGestureRecognizerBegin(recognizer)) }
    }

    pub(crate) fn set_gesture_event_to_target(
        &self,
        gesture: ArkUI_GestureRecognizerHandle,
        action_type: ArkUI_GestureEventActionTypeMask,
        extra_params: Rc<RefCell<InnerGestureData>>,
    ) -> ArkUIResult<()> {
        self.set_gesture_event_target_raw(
            gesture,
            action_type,
            Box::into_raw(Box::new(extra_params)) as *mut c_void,
            Some(target_receiver),
        )
    }

    fn set_gesture_event_target_raw(
        &self,
        gesture: ArkUI_GestureRecognizerHandle,
        action_type: ArkUI_GestureEventActionTypeMask,
        extra_params: *mut c_void,
        target_receiver: Option<
            unsafe extern "C" fn(event: *mut ArkUI_GestureEvent, extra_params: *mut c_void),
        >,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(set_gesture_event_to_target) = (*self.raw()).setGestureEventTarget {
                check_arkui_status!(set_gesture_event_to_target(
                    gesture,
                    action_type,
                    extra_params,
                    target_receiver
                ))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::setGestureEventTarget is None",
                ))
            }
        }
    }

    pub(crate) fn set_gesture_event_target<T: Fn(GestureEventRef) + 'static>(
        &self,
        gesture: ArkUI_GestureRecognizerHandle,
        action_type: ArkUI_GestureEventActionTypeMask,
        target_receiver: T,
    ) -> ArkUIResult<()> {
        let callback = Box::into_raw(Box::new(GestureEventTargetCallbackContext {
            callback: Box::new(target_receiver),
        }));
        let result = self.set_gesture_event_target_raw(
            gesture,
            action_type,
            callback.cast(),
            Some(gesture_event_target_callback_trampoline),
        );
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        let mut callbacks = match gesture_event_target_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(old) = callbacks.insert(gesture as usize, callback as usize) {
            unsafe {
                drop(Box::from_raw(old as *mut GestureEventTargetCallbackContext));
            }
        }
        Ok(())
    }

    pub fn clear_gesture_event_target(
        &self,
        gesture: ArkUI_GestureRecognizerHandle,
        action_type: ArkUI_GestureEventActionTypeMask,
    ) -> ArkUIResult<()> {
        self.set_gesture_event_target_raw(gesture, action_type, std::ptr::null_mut(), None)?;
        let mut callbacks = match gesture_event_target_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(callback) = callbacks.remove(&(gesture as usize)) {
            unsafe {
                drop(Box::from_raw(
                    callback as *mut GestureEventTargetCallbackContext,
                ));
            }
        }
        Ok(())
    }
}

impl Default for ArkUINativeGestureAPI1 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "api-18")]
pub(crate) struct ArkUINativeGestureAPI2(pub(crate) NonNull<ArkUI_NativeGestureAPI_2>);

#[cfg(feature = "api-18")]
impl ArkUINativeGestureAPI2 {
    pub(crate) fn raw(&self) -> *mut ArkUI_NativeGestureAPI_2 {
        self.0.as_ptr()
    }

    pub fn new() -> Self {
        let struct_name = CString::new("ArkUI_NativeGestureAPI_2").unwrap();
        let raw_ptr = unsafe {
            OH_ArkUI_QueryModuleInterfaceByName(
                ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_GESTURE,
                struct_name.as_ptr().cast(),
            )
        };
        let api = NonNull::new(raw_ptr.cast())
            .unwrap_or_else(|| panic!("ArkUI_NativeGestureAPI_2 is NULL"));
        Self(api)
    }

    fn set_gesture_interrupter_to_node_raw(
        &self,
        node: ArkUI_NodeHandle,
        user_data: *mut c_void,
        interrupter: Option<
            unsafe extern "C" fn(
                info: *mut ArkUI_GestureInterruptInfo,
            ) -> ArkUI_GestureInterruptResult,
        >,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(set_gesture_interrupter_to_node) = (*self.raw()).setGestureInterrupterToNode
            {
                check_arkui_status!(set_gesture_interrupter_to_node(
                    node,
                    user_data,
                    interrupter
                ))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI2::setGestureInterrupterToNode is None",
                ))
            }
        }
    }

    pub fn set_gesture_interrupter_to_node<
        T: Fn(GestureInterruptInfoRef) -> GestureInterruptResult + 'static,
    >(
        &self,
        node: ArkUI_NodeHandle,
        interrupter: T,
    ) -> ArkUIResult<()> {
        let callback = Box::into_raw(Box::new(GestureInterrupterCallbackContext {
            callback: Box::new(interrupter),
        }));
        let result = self.set_gesture_interrupter_to_node_raw(
            node,
            callback.cast(),
            Some(gesture_interrupter_callback_trampoline),
        );
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        let mut callbacks = match gesture_interrupter_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(old) = callbacks.insert(node as usize, callback as usize) {
            unsafe {
                drop(Box::from_raw(old as *mut GestureInterrupterCallbackContext));
            }
        }
        Ok(())
    }

    pub(crate) fn clear_gesture_interrupter_to_node(
        &self,
        node: ArkUI_NodeHandle,
    ) -> ArkUIResult<()> {
        self.set_gesture_interrupter_to_node_raw(node, std::ptr::null_mut(), None)?;
        let mut callbacks = match gesture_interrupter_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(callback) = callbacks.remove(&(node as usize)) {
            unsafe {
                drop(Box::from_raw(
                    callback as *mut GestureInterrupterCallbackContext,
                ));
            }
        }
        Ok(())
    }
}

#[cfg(feature = "api-18")]
impl Default for ArkUINativeGestureAPI2 {
    fn default() -> Self {
        Self::new()
    }
}

fn read_buffer_string<F>(mut reader: F) -> ArkUIResult<String>
where
    F: FnMut(*mut std::os::raw::c_char, i32, *mut i32) -> i32,
{
    let mut write_length = 0;
    let mut buffer = vec![0u8; 256];
    let mut status = reader(
        buffer.as_mut_ptr().cast(),
        buffer.len() as i32,
        &mut write_length,
    );
    if write_length > buffer.len() as i32 {
        buffer.resize(write_length as usize, 0);
        status = reader(
            buffer.as_mut_ptr().cast(),
            buffer.len() as i32,
            &mut write_length,
        );
    }
    check_arkui_status!(status)?;
    let mut end = (write_length as usize).min(buffer.len());
    if end == 0 {
        end = buffer.iter().position(|v| *v == 0).unwrap_or(0);
    } else if buffer.get(end.saturating_sub(1)).copied() == Some(0) {
        end -= 1;
    }
    Ok(String::from_utf8_lossy(&buffer[..end]).into_owned())
}

fn collect_gesture_recognizers(
    recognizers: ArkUI_GestureRecognizerHandleArray,
    count: i32,
) -> Vec<GestureRecognizerRef> {
    if recognizers.is_null() || count <= 0 {
        return Vec::new();
    }
    let handles = unsafe { std::slice::from_raw_parts(recognizers, count as usize) };
    handles
        .iter()
        .filter_map(|handle| GestureRecognizerRef::from_handle(*handle))
        .collect()
}

#[derive(Clone, Copy, Debug)]
pub struct GestureEventRef {
    raw: NonNull<ArkUI_GestureEvent>,
}

impl GestureEventRef {
    pub(crate) fn from_raw(raw: *mut ArkUI_GestureEvent) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) fn from_const_raw(raw: *const ArkUI_GestureEvent) -> Option<Self> {
        Self::from_raw(raw.cast_mut())
    }

    fn raw(&self) -> *mut ArkUI_GestureEvent {
        self.raw.as_ptr()
    }

    pub fn raw_input_event(&self) -> Option<ohos_arkui_input_binding::ArkUIInputEvent> {
        let raw_input_event = unsafe { OH_ArkUI_GestureEvent_GetRawInputEvent(self.raw()) };
        if raw_input_event.is_null() {
            None
        } else {
            Some(ohos_arkui_input_binding::ArkUIInputEvent::from_raw(
                raw_input_event.cast_mut(),
            ))
        }
    }

    pub fn node(&self) -> Option<crate::ArkUINode> {
        let node = unsafe { OH_ArkUI_GestureEvent_GetNode(self.raw()) };
        crate::ArkUINode::from_raw_handle(node)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GestureEventTargetInfoRef {
    raw: NonNull<ArkUI_GestureEventTargetInfo>,
}

impl GestureEventTargetInfoRef {
    fn from_raw(raw: *mut ArkUI_GestureEventTargetInfo) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub fn raw(&self) -> *mut ArkUI_GestureEventTargetInfo {
        self.raw.as_ptr()
    }

    pub fn is_scroll_begin(&self) -> ArkUIResult<bool> {
        let mut ret = false;
        check_arkui_status!(unsafe {
            OH_ArkUI_GestureEventTargetInfo_IsScrollBegin(self.raw(), &mut ret)
        })?;
        Ok(ret)
    }

    pub fn is_scroll_end(&self) -> ArkUIResult<bool> {
        let mut ret = false;
        check_arkui_status!(unsafe {
            OH_ArkUI_GestureEventTargetInfo_IsScrollEnd(self.raw(), &mut ret)
        })?;
        Ok(ret)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GestureRecognizerRef {
    raw: NonNull<ArkUI_GestureRecognizer>,
}

impl GestureRecognizerRef {
    pub(crate) fn from_raw(raw: *mut ArkUI_GestureRecognizer) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) fn from_handle(handle: ArkUI_GestureRecognizerHandle) -> Option<Self> {
        Self::from_raw(handle)
    }

    pub fn raw(&self) -> *mut ArkUI_GestureRecognizer {
        self.raw.as_ptr()
    }

    pub fn as_handle(&self) -> ArkUI_GestureRecognizerHandle {
        self.raw()
    }

    pub fn set_enabled(&self, enabled: bool) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(ohos_arkui_sys::OH_ArkUI_SetGestureRecognizerEnabled(
                self.raw(),
                enabled
            ))
        }
    }

    #[cfg(feature = "api-15")]
    pub fn set_limit_finger_count(&self, limit_finger_count: bool) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(
                ohos_arkui_sys::OH_ArkUI_SetGestureRecognizerLimitFingerCount(
                    self.raw(),
                    limit_finger_count
                )
            )
        }
    }

    pub fn enabled(&self) -> bool {
        unsafe { ohos_arkui_sys::OH_ArkUI_GetGestureRecognizerEnabled(self.raw()) }
    }

    pub fn state(&self) -> ArkUIResult<GestureRecognizerState> {
        let mut state =
            ohos_arkui_sys::ArkUI_GestureRecognizerState_ARKUI_GESTURE_RECOGNIZER_STATE_READY;
        unsafe {
            check_arkui_status!(ohos_arkui_sys::OH_ArkUI_GetGestureRecognizerState(
                self.raw(),
                &mut state
            ))
        }?;
        Ok(state.into())
    }

    pub fn event_target_info(&self) -> ArkUIResult<GestureEventTargetInfoRef> {
        let mut info = std::ptr::null_mut();
        unsafe {
            check_arkui_status!(ohos_arkui_sys::OH_ArkUI_GetGestureEventTargetInfo(
                self.raw(),
                &mut info
            ))
        }?;
        GestureEventTargetInfoRef::from_raw(info).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_GetGestureEventTargetInfo returned null",
            )
        })
    }

    pub fn pan_direction_mask(&self) -> ArkUIResult<crate::GestureDirection> {
        let mut direction_mask: ohos_arkui_sys::ArkUI_GestureDirectionMask = 0;
        unsafe {
            check_arkui_status!(ohos_arkui_sys::OH_ArkUI_GetPanGestureDirectionMask(
                self.raw(),
                &mut direction_mask
            ))
        }?;
        Ok(direction_mask.into())
    }

    pub fn is_built_in(&self) -> bool {
        unsafe { ohos_arkui_sys::OH_ArkUI_IsBuiltInGesture(self.raw()) }
    }

    pub fn tag(&self) -> ArkUIResult<String> {
        read_buffer_string(|buffer, buffer_size, write_length| unsafe {
            ohos_arkui_sys::OH_ArkUI_GetGestureTag(self.raw(), buffer, buffer_size, write_length)
        })
    }

    pub fn bind_node_id(&self) -> ArkUIResult<String> {
        read_buffer_string(|buffer, buffer_size, write_length| unsafe {
            ohos_arkui_sys::OH_ArkUI_GetGestureBindNodeId(
                self.raw(),
                buffer,
                buffer_size,
                write_length,
            )
        })
    }

    pub fn is_valid(&self) -> bool {
        unsafe { ohos_arkui_sys::OH_ArkUI_IsGestureRecognizerValid(self.raw()) }
    }

    pub fn set_dispose_notify<T: Fn() + 'static>(&self, callback: T) -> ArkUIResult<()> {
        let callback = Box::into_raw(Box::new(GestureDisposeNotifyCallbackContext {
            callback: Box::new(callback),
        }));
        let result = unsafe {
            check_arkui_status!(
                ohos_arkui_sys::OH_ArkUI_SetArkUIGestureRecognizerDisposeNotify(
                    self.raw(),
                    Some(gesture_dispose_notify_callback_trampoline),
                    callback.cast(),
                )
            )
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        let mut callbacks = match gesture_dispose_notify_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(old) = callbacks.insert(self.as_handle() as usize, callback as usize) {
            unsafe {
                drop(Box::from_raw(
                    old as *mut GestureDisposeNotifyCallbackContext,
                ));
            }
        }
        Ok(())
    }

    pub fn clear_dispose_notify(&self) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(
                ohos_arkui_sys::OH_ArkUI_SetArkUIGestureRecognizerDisposeNotify(
                    self.raw(),
                    None,
                    std::ptr::null_mut(),
                )
            )
        }?;
        let mut callbacks = match gesture_dispose_notify_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        if let Some(callback) = callbacks.remove(&(self.as_handle() as usize)) {
            unsafe {
                drop(Box::from_raw(
                    callback as *mut GestureDisposeNotifyCallbackContext,
                ));
            }
        }
        Ok(())
    }

    #[cfg(feature = "api-19")]
    pub fn set_distance_map(
        &self,
        tool_type_array: &mut [i32],
        distance_array: &mut [f64],
    ) -> ArkUIResult<()> {
        if tool_type_array.len() != distance_array.len() {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "tool_type_array and distance_array must have the same length",
            ));
        }
        check_arkui_status!(unsafe {
            OH_ArkUI_PanGesture_SetDistanceMap(
                self.as_handle(),
                tool_type_array.len() as i32,
                tool_type_array.as_mut_ptr(),
                distance_array.as_mut_ptr(),
            )
        })
    }

    #[cfg(feature = "api-19")]
    pub fn distance_by_tool_type(&self, tool_type: i32) -> ArkUIResult<f64> {
        let mut distance = 0f64;
        check_arkui_status!(unsafe {
            OH_ArkUI_PanGesture_GetDistanceByToolType(self.as_handle(), tool_type, &mut distance)
        })?;
        Ok(distance)
    }

    #[cfg(feature = "api-18")]
    pub fn param_direct_mask(&self) -> ArkUIResult<crate::GestureDirection> {
        let mut direction_mask = 0;
        check_arkui_status!(unsafe {
            OH_ArkUI_GetGestureParam_DirectMask(self.as_handle(), &mut direction_mask)
        })?;
        Ok(direction_mask.into())
    }

    #[cfg(feature = "api-18")]
    pub fn param_finger_count(&self) -> ArkUIResult<i32> {
        let mut finger = 0;
        check_arkui_status!(unsafe {
            OH_ArkUI_GetGestureParam_FingerCount(self.as_handle(), &mut finger)
        })?;
        Ok(finger)
    }

    #[cfg(feature = "api-18")]
    pub fn param_limit_finger_count(&self) -> ArkUIResult<bool> {
        let mut limited = false;
        check_arkui_status!(unsafe {
            OH_ArkUI_GetGestureParam_limitFingerCount(self.as_handle(), &mut limited)
        })?;
        Ok(limited)
    }

    #[cfg(feature = "api-18")]
    pub fn param_repeat(&self) -> ArkUIResult<bool> {
        let mut repeat = false;
        check_arkui_status!(unsafe {
            OH_ArkUI_GetGestureParam_repeat(self.as_handle(), &mut repeat)
        })?;
        Ok(repeat)
    }

    #[cfg(feature = "api-18")]
    pub fn param_distance(&self) -> ArkUIResult<f64> {
        let mut distance = 0f64;
        check_arkui_status!(unsafe {
            OH_ArkUI_GetGestureParam_distance(self.as_handle(), &mut distance)
        })?;
        Ok(distance)
    }

    #[cfg(feature = "api-18")]
    pub fn param_speed(&self) -> ArkUIResult<f64> {
        let mut speed = 0f64;
        check_arkui_status!(unsafe {
            OH_ArkUI_GetGestureParam_speed(self.as_handle(), &mut speed)
        })?;
        Ok(speed)
    }

    #[cfg(feature = "api-18")]
    pub fn param_duration(&self) -> ArkUIResult<i32> {
        let mut duration = 0;
        check_arkui_status!(unsafe {
            OH_ArkUI_GetGestureParam_duration(self.as_handle(), &mut duration)
        })?;
        Ok(duration)
    }

    #[cfg(feature = "api-18")]
    pub fn param_angle(&self) -> ArkUIResult<f64> {
        let mut angle = 0f64;
        check_arkui_status!(unsafe {
            OH_ArkUI_GetGestureParam_angle(self.as_handle(), &mut angle)
        })?;
        Ok(angle)
    }

    #[cfg(feature = "api-18")]
    pub fn param_distance_threshold(&self) -> ArkUIResult<f64> {
        let mut distance_threshold = 0f64;
        check_arkui_status!(unsafe {
            OH_ArkUI_GetGestureParam_distanceThreshold(self.as_handle(), &mut distance_threshold)
        })?;
        Ok(distance_threshold)
    }

    #[cfg(feature = "api-22")]
    pub fn set_allowable_movement(&self, allowable_movement: f64) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            OH_ArkUI_LongPressGesture_SetAllowableMovement(self.as_handle(), allowable_movement)
        })
    }

    #[cfg(feature = "api-22")]
    pub fn allowable_movement(&self) -> ArkUIResult<f64> {
        let mut allowable_movement = 0f64;
        check_arkui_status!(unsafe {
            OH_ArkUI_LongPressGesture_GetAllowableMovement(
                self.as_handle(),
                &mut allowable_movement,
            )
        })?;
        Ok(allowable_movement)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GestureInterruptInfoRef {
    raw: NonNull<ArkUI_GestureInterruptInfo>,
}

impl GestureInterruptInfoRef {
    pub(crate) fn from_raw(raw: *mut ArkUI_GestureInterruptInfo) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) fn from_const_raw(raw: *const ArkUI_GestureInterruptInfo) -> Option<Self> {
        Self::from_raw(raw.cast_mut())
    }

    fn raw(&self) -> *mut ArkUI_GestureInterruptInfo {
        self.raw.as_ptr()
    }

    pub fn system_flag(&self) -> bool {
        unsafe { OH_ArkUI_GestureInterruptInfo_GetSystemFlag(self.raw()) }
    }

    pub fn recognizer(&self) -> Option<GestureRecognizerRef> {
        let recognizer = unsafe { OH_ArkUI_GestureInterruptInfo_GetRecognizer(self.raw()) };
        GestureRecognizerRef::from_raw(recognizer)
    }

    pub fn gesture_event(&self) -> Option<GestureEventRef> {
        let gesture_event = unsafe { OH_ArkUI_GestureInterruptInfo_GetGestureEvent(self.raw()) };
        GestureEventRef::from_raw(gesture_event)
    }

    pub fn system_recognizer_type(&self) -> Option<crate::GestureRecognizerType> {
        let recognizer_type =
            unsafe { OH_ArkUI_GestureInterruptInfo_GetSystemRecognizerType(self.raw()) };
        if recognizer_type < 0 {
            None
        } else {
            crate::GestureRecognizerType::try_from_raw(
                recognizer_type as ohos_arkui_sys::ArkUI_GestureRecognizerType,
            )
        }
    }

    pub fn response_recognizers(&self) -> ArkUIResult<Vec<GestureRecognizerRef>> {
        let mut response_chain: ArkUI_GestureRecognizerHandleArray = std::ptr::null_mut();
        let mut count = 0;
        unsafe {
            check_arkui_status!(
                ohos_arkui_sys::OH_ArkUI_GetResponseRecognizersFromInterruptInfo(
                    self.raw(),
                    &mut response_chain,
                    &mut count
                )
            )
        }?;
        Ok(collect_gesture_recognizers(response_chain, count))
    }

    #[cfg(feature = "api-15")]
    pub fn touch_recognizers(&self) -> ArkUIResult<Vec<TouchRecognizerRef>> {
        let mut recognizers: ArkUI_TouchRecognizerHandleArray = std::ptr::null_mut();
        let mut size = 0;
        check_arkui_status!(unsafe {
            OH_ArkUI_GestureInterruptInfo_GetTouchRecognizers(
                self.raw(),
                &mut recognizers,
                &mut size,
            )
        })?;
        if recognizers.is_null() || size <= 0 {
            return Ok(Vec::new());
        }
        let handles = unsafe { std::slice::from_raw_parts(recognizers, size as usize) };
        Ok(handles
            .iter()
            .filter_map(|handle| TouchRecognizerRef::from_handle(*handle))
            .collect())
    }
}

#[cfg(feature = "api-15")]
#[derive(Clone, Copy, Debug)]
pub struct TouchRecognizerRef {
    raw: NonNull<c_void>,
}

#[cfg(feature = "api-15")]
impl TouchRecognizerRef {
    pub(crate) fn from_handle(handle: ArkUI_TouchRecognizerHandle) -> Option<Self> {
        NonNull::new(handle.cast()).map(|raw| Self { raw })
    }

    fn raw(&self) -> ArkUI_TouchRecognizerHandle {
        self.raw.as_ptr().cast()
    }

    pub fn node(&self) -> Option<crate::ArkUINode> {
        let node = unsafe { OH_ArkUI_TouchRecognizer_GetNodeHandle(self.raw()) };
        crate::ArkUINode::from_raw_handle(node)
    }

    pub fn cancel_touch(&self, info: GestureInterruptInfoRef) -> ArkUIResult<()> {
        check_arkui_status!(unsafe { OH_ArkUI_TouchRecognizer_CancelTouch(self.raw(), info.raw()) })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ParallelInnerGestureEventRef {
    raw: NonNull<ArkUI_ParallelInnerGestureEvent>,
}

impl ParallelInnerGestureEventRef {
    pub(crate) fn from_raw(raw: *mut ArkUI_ParallelInnerGestureEvent) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    fn raw(&self) -> *mut ArkUI_ParallelInnerGestureEvent {
        self.raw.as_ptr()
    }

    pub fn user_data(&self) -> Option<NonNull<c_void>> {
        NonNull::new(unsafe { OH_ArkUI_ParallelInnerGestureEvent_GetUserData(self.raw()) })
    }

    pub fn current_recognizer(&self) -> Option<GestureRecognizerRef> {
        let recognizer =
            unsafe { OH_ArkUI_ParallelInnerGestureEvent_GetCurrentRecognizer(self.raw()) };
        GestureRecognizerRef::from_raw(recognizer)
    }

    pub fn conflict_recognizers(&self) -> ArkUIResult<Vec<GestureRecognizerRef>> {
        let mut array: ArkUI_GestureRecognizerHandleArray = std::ptr::null_mut();
        let mut size = 0;
        check_arkui_status!(unsafe {
            OH_ArkUI_ParallelInnerGestureEvent_GetConflictRecognizers(
                self.raw(),
                &mut array,
                &mut size,
            )
        })?;
        Ok(collect_gesture_recognizers(array, size))
    }
}

#[cfg(feature = "api-20")]
fn set_touch_test_done_callback_raw(
    node: ArkUI_NodeHandle,
    user_data: *mut c_void,
    touch_test_done: Option<
        unsafe extern "C" fn(
            event: *mut ArkUI_GestureEvent,
            recognizers: ArkUI_GestureRecognizerHandleArray,
            count: i32,
            user_data: *mut c_void,
        ),
    >,
) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(ohos_arkui_sys::OH_ArkUI_SetTouchTestDoneCallback(
            node,
            user_data,
            touch_test_done
        ))
    }
}

#[cfg(feature = "api-20")]
pub fn set_touch_test_done_callback<T: Fn(GestureEventRef, Vec<GestureRecognizerRef>) + 'static>(
    node: ArkUI_NodeHandle,
    touch_test_done: T,
) -> ArkUIResult<()> {
    let callback = Box::into_raw(Box::new(TouchTestDoneCallbackContext {
        callback: Box::new(touch_test_done),
    }));
    let result = set_touch_test_done_callback_raw(
        node,
        callback.cast(),
        Some(touch_test_done_callback_trampoline),
    );
    if let Err(err) = result {
        unsafe {
            drop(Box::from_raw(callback));
        }
        return Err(err);
    }
    let mut callbacks = match touch_test_done_callback_contexts().lock() {
        Ok(callbacks) => callbacks,
        Err(poisoned) => poisoned.into_inner(),
    };
    if let Some(old) = callbacks.insert(node as usize, callback as usize) {
        unsafe {
            drop(Box::from_raw(old as *mut TouchTestDoneCallbackContext));
        }
    }
    Ok(())
}

#[cfg(feature = "api-20")]
pub(crate) fn clear_touch_test_done_callback(node: ArkUI_NodeHandle) -> ArkUIResult<()> {
    set_touch_test_done_callback_raw(node, std::ptr::null_mut(), None)?;
    let mut callbacks = match touch_test_done_callback_contexts().lock() {
        Ok(callbacks) => callbacks,
        Err(poisoned) => poisoned.into_inner(),
    };
    if let Some(callback) = callbacks.remove(&(node as usize)) {
        unsafe {
            drop(Box::from_raw(callback as *mut TouchTestDoneCallbackContext));
        }
    }
    Ok(())
}

unsafe extern "C" fn inner_gesture_parallel_callback_trampoline(
    event: *mut ArkUI_ParallelInnerGestureEvent,
) -> *mut ArkUI_GestureRecognizer {
    let Some(event) = ParallelInnerGestureEventRef::from_raw(event) else {
        return std::ptr::null_mut();
    };
    let user_data = unsafe { OH_ArkUI_ParallelInnerGestureEvent_GetUserData(event.raw()) };
    if user_data.is_null() {
        return std::ptr::null_mut();
    }
    let callback = unsafe { &*(user_data as *mut InnerGestureParallelCallbackContext) };
    match (callback.callback)(event) {
        Some(recognizer) => recognizer.as_handle(),
        None => std::ptr::null_mut(),
    }
}

unsafe extern "C" fn gesture_event_target_callback_trampoline(
    event: *mut ArkUI_GestureEvent,
    extra_params: *mut c_void,
) {
    if extra_params.is_null() {
        return;
    }
    let Some(event) = GestureEventRef::from_raw(event) else {
        return;
    };
    let callback = unsafe { &*(extra_params as *mut GestureEventTargetCallbackContext) };
    (callback.callback)(event);
}

unsafe extern "C" fn gesture_interrupter_callback_trampoline(
    info: *mut ArkUI_GestureInterruptInfo,
) -> ArkUI_GestureInterruptResult {
    let Some(info) = GestureInterruptInfoRef::from_raw(info) else {
        return GestureInterruptResult::Continue.into();
    };

    #[cfg(feature = "api-18")]
    let user_data = unsafe { OH_ArkUI_GestureInterrupter_GetUserData(info.raw()) };
    #[cfg(feature = "api-18")]
    if !user_data.is_null() {
        let callback = unsafe { &*(user_data as *mut GestureInterrupterCallbackContext) };
        return (callback.callback)(info).into();
    }

    let Some(node_handle) = info
        .gesture_event()
        .and_then(|event| event.node())
        .map(|node| node.raw())
    else {
        return GestureInterruptResult::Continue.into();
    };

    let callback = {
        let callbacks = match gesture_interrupter_callback_contexts().lock() {
            Ok(callbacks) => callbacks,
            Err(poisoned) => poisoned.into_inner(),
        };
        callbacks.get(&(node_handle as usize)).copied()
    };
    let Some(callback) = callback else {
        return GestureInterruptResult::Continue.into();
    };
    let callback = unsafe { &*(callback as *const GestureInterrupterCallbackContext) };
    (callback.callback)(info).into()
}

#[cfg(feature = "api-20")]
unsafe extern "C" fn touch_test_done_callback_trampoline(
    event: *mut ArkUI_GestureEvent,
    recognizers: ArkUI_GestureRecognizerHandleArray,
    count: i32,
    user_data: *mut c_void,
) {
    if user_data.is_null() {
        return;
    }
    let Some(event) = GestureEventRef::from_raw(event) else {
        return;
    };
    let callback = unsafe { &*(user_data as *mut TouchTestDoneCallbackContext) };
    (callback.callback)(event, collect_gesture_recognizers(recognizers, count));
}

unsafe extern "C" fn gesture_dispose_notify_callback_trampoline(
    recognizer: *mut ArkUI_GestureRecognizer,
    user_data: *mut c_void,
) {
    if user_data.is_null() {
        return;
    }
    let callback = unsafe { Box::from_raw(user_data as *mut GestureDisposeNotifyCallbackContext) };
    (callback.callback)();
    let mut callbacks = match gesture_dispose_notify_callback_contexts().lock() {
        Ok(callbacks) => callbacks,
        Err(poisoned) => poisoned.into_inner(),
    };
    if !recognizer.is_null() {
        callbacks.remove(&(recognizer as usize));
    } else {
        callbacks.retain(|_, value| *value != user_data as usize);
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
            event_action_type,
            event_action_data: event_data,
        });
    }
}
