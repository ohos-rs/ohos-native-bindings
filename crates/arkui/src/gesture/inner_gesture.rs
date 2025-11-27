use std::{cell::RefCell, os::raw::c_void, rc::Rc};

use ohos_arkui_sys::{
    ArkUI_GestureDirectionMask, ArkUI_GestureEventActionType, ArkUI_GestureRecognizerHandle,
};

use crate::{
    ArkUIResult, GestureDirection, GestureEventAction, GestureRecognizerType,
    ARK_UI_NATIVE_GESTURE_API_1,
};

use super::GestureEventData;

pub(crate) struct InnerGestureData {
    pub gesture_type: GestureRecognizerType,
    pub gesture_callback: Option<fn(GestureEventData) -> ()>,
    pub user_data: Option<*mut c_void>,
}

pub struct Gesture {
    pub(crate) raw: Rc<RefCell<ArkUI_GestureRecognizerHandle>>,
    pub(crate) inner_gesture_data: Rc<RefCell<InnerGestureData>>,
}

impl Gesture {
    /// create long gesture
    pub fn create_long_gesture(finger: i32, repeat: bool, duration: i32) -> ArkUIResult<Self> {
        let handle = ARK_UI_NATIVE_GESTURE_API_1
            .with(|api| api.create_long_gesture(finger, repeat, duration))?;
        Ok(Self {
            raw: Rc::new(RefCell::new(handle)),
            inner_gesture_data: Rc::new(RefCell::new(InnerGestureData {
                gesture_type: GestureRecognizerType::LongPressGesture,
                gesture_callback: None,
                user_data: None,
            })),
        })
    }

    pub fn create_pan_gesture(
        finger: i32,
        direction: GestureDirection,
        distance: f64,
    ) -> ArkUIResult<Self> {
        let direction: ArkUI_GestureDirectionMask = direction.into();
        let handle = ARK_UI_NATIVE_GESTURE_API_1
            .with(|api| api.create_pan_gesture(finger, direction, distance))?;
        Ok(Self {
            raw: Rc::new(RefCell::new(handle)),
            inner_gesture_data: Rc::new(RefCell::new(InnerGestureData {
                gesture_type: GestureRecognizerType::PanGesture,
                gesture_callback: None,
                user_data: None,
            })),
        })
    }

    pub fn create_tap_gesture(finger: i32, count: i32) -> ArkUIResult<Self> {
        let handle =
            ARK_UI_NATIVE_GESTURE_API_1.with(|api| api.create_tap_gesture(count, finger))?;
        Ok(Self {
            raw: Rc::new(RefCell::new(handle)),
            inner_gesture_data: Rc::new(RefCell::new(InnerGestureData {
                gesture_type: GestureRecognizerType::TapGesture,
                gesture_callback: None,
                user_data: None,
            })),
        })
    }

    pub fn create_pinch_gesture(finger: i32, distance: f64) -> ArkUIResult<Self> {
        let handle =
            ARK_UI_NATIVE_GESTURE_API_1.with(|api| api.create_pinch_gesture(finger, distance))?;
        Ok(Self {
            raw: Rc::new(RefCell::new(handle)),
            inner_gesture_data: Rc::new(RefCell::new(InnerGestureData {
                gesture_type: GestureRecognizerType::PinchGesture,
                gesture_callback: None,
                user_data: None,
            })),
        })
    }

    pub fn create_rotation_gesture(finger: i32, angle: f64) -> ArkUIResult<Self> {
        let handle =
            ARK_UI_NATIVE_GESTURE_API_1.with(|api| api.create_rotation_gesture(finger, angle))?;
        Ok(Self {
            raw: Rc::new(RefCell::new(handle)),
            inner_gesture_data: Rc::new(RefCell::new(InnerGestureData {
                gesture_type: GestureRecognizerType::RotationGesture,
                gesture_callback: None,
                user_data: None,
            })),
        })
    }

    pub fn create_swipe_gesture(
        finger: i32,
        direction: GestureDirection,
        speed: f64,
    ) -> ArkUIResult<Self> {
        let direction: ArkUI_GestureDirectionMask = direction.into();
        let handle = ARK_UI_NATIVE_GESTURE_API_1
            .with(|api| api.create_swipe_gesture(finger, direction, speed))?;
        Ok(Self {
            raw: Rc::new(RefCell::new(handle)),
            inner_gesture_data: Rc::new(RefCell::new(InnerGestureData {
                gesture_type: GestureRecognizerType::SwipeGesture,
                gesture_callback: None,
                user_data: None,
            })),
        })
    }

    pub fn on_gesture(
        &self,
        action_type: GestureEventAction,
        callback: fn(GestureEventData) -> (),
    ) -> ArkUIResult<()> {
        self.inner_gesture_data.borrow_mut().gesture_callback = Some(callback);
        self.inner_gesture_data.borrow_mut().user_data = None;

        let event_action_type: ArkUI_GestureEventActionType = action_type.into();

        let raw = *self.raw.borrow();

        ARK_UI_NATIVE_GESTURE_API_1.with(|api| {
            api.set_gesture_event_to_target(raw, event_action_type, self.inner_gesture_data.clone())
        })?;
        Ok(())
    }

    pub fn on_gesture_with_data(
        &self,
        action_type: GestureEventAction,
        data: *mut c_void,
        callback: fn(GestureEventData) -> (),
    ) -> ArkUIResult<()> {
        self.inner_gesture_data.borrow_mut().gesture_callback = Some(callback);
        self.inner_gesture_data.borrow_mut().user_data = Some(data);

        let raw = *self.raw.borrow();

        let event_action_type: ArkUI_GestureEventActionType = action_type.into();
        ARK_UI_NATIVE_GESTURE_API_1.with(|api| {
            api.set_gesture_event_to_target(raw, event_action_type, self.inner_gesture_data.clone())
        })?;
        Ok(())
    }
}
