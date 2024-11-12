use std::{
    cell::{LazyCell, RefCell},
    ffi::CString,
    os::raw::c_void,
    rc::Rc,
};

use ohos_arkui_sys::{
    ArkUI_GestureEvent, ArkUI_GestureEventActionTypeMask, ArkUI_GestureMask, ArkUI_GesturePriority,
    ArkUI_GestureRecognizerHandle, ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_GESTURE,
    ArkUI_NativeGestureAPI_1, ArkUI_NodeHandle, OH_ArkUI_QueryModuleInterfaceByName,
};

use crate::{
    check_arkui_status, ArkUIError, ArkUIErrorCode, ArkUIResult, GestureEventData, InnerGestureData,
};

/// ArkUINativeGestureAPI1 struct
/// Only can be used in main thread
pub const ARK_UI_NATIVE_GESTURE_API_1: LazyCell<ArkUINativeGestureAPI1> = LazyCell::new(|| {
    let api = ArkUINativeGestureAPI1::new();
    api
});

pub struct ArkUINativeGestureAPI1(pub(crate) *mut ArkUI_NativeGestureAPI_1);

impl ArkUINativeGestureAPI1 {
    /// allow us to get the pointer of ArkUI_NativeGestureAPI_1 and use it directly
    pub fn raw(&self) -> *mut ArkUI_NativeGestureAPI_1 {
        self.0
    }

    pub fn new() -> Self {
        #[allow(unused_assignments)]
        let mut api: *mut ArkUI_NativeGestureAPI_1 = std::ptr::null_mut();
        let struct_name = CString::new("ArkUI_NativeGestureAPI_1").unwrap();
        let raw_ptr = unsafe {
            OH_ArkUI_QueryModuleInterfaceByName(
                ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_GESTURE,
                struct_name.as_ptr().cast(),
            )
        };
        #[cfg(debug_assertions)]
        assert!(!raw_ptr.is_null(), "ArkUI_NativeGestureAPI_1 is NULL");
        api = raw_ptr.cast();
        Self(api)
    }

    pub fn create_long_gesture(
        &self,
        finger_number: i32,
        repeat: bool,
        duration: i32,
    ) -> ArkUIResult<ArkUI_GestureRecognizerHandle> {
        unsafe {
            if let Some(create_long_press_gesture) = (*self.0).createLongPressGesture {
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

    pub fn add_gesture(
        &self,
        gesture: ArkUI_GestureRecognizerHandle,
        node: ArkUI_NodeHandle,
        mode: ArkUI_GesturePriority,
        mask: ArkUI_GestureMask,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(add_gesture_to_node) = (*self.0).addGestureToNode {
                check_arkui_status!(add_gesture_to_node(node, gesture, mode, mask))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::addGestureToNode is None",
                ))
            }
        }
    }

    pub(crate) fn set_gesture_event_to_target(
        &self,
        gesture: ArkUI_GestureRecognizerHandle,
        action_type: ArkUI_GestureEventActionTypeMask,
        extra_params: Rc<RefCell<InnerGestureData>>,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(set_gesture_event_to_target) = (*self.0).setGestureEventTarget {
                check_arkui_status!(set_gesture_event_to_target(
                    gesture,
                    action_type,
                    Box::into_raw(Box::new(extra_params)) as *mut c_void,
                    Some(target_receiver)
                ))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUINativeGestureAPI1::setGestureEventTarget is None",
                ))
            }
        }
    }
}

unsafe extern "C" fn target_receiver(event: *mut ArkUI_GestureEvent, extra_params: *mut c_void) {
    let user_data: &Rc<RefCell<InnerGestureData>> =
        &*(extra_params as *const Rc<RefCell<InnerGestureData>>);

    let data = user_data.borrow_mut();

    if let Some(event) = data.gesture_callback.as_ref() {
        event(GestureEventData {
            data: data.user_data,
        });
    }
}
