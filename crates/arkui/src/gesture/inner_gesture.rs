use std::{cell::RefCell, os::raw::c_void, rc::Rc};

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::{ArkUI_GestureEventActionType, ArkUI_GestureRecognizerHandle};

#[cfg(any(feature = "api-19", feature = "api-22"))]
use crate::GestureRecognizerRef;
use crate::{
    ArkUIError, ArkUIResult, GestureDirection, GestureEventAction, GestureRecognizerType,
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
    fn raw_handle(&self) -> ArkUIResult<ArkUI_GestureRecognizerHandle> {
        let raw = *self.raw.borrow();
        if raw.is_null() {
            Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "Gesture handle is null",
            ))
        } else {
            Ok(raw)
        }
    }

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

    pub fn create_tap_gesture_with_distance_threshold(
        finger: i32,
        count: i32,
        distance_threshold: f64,
    ) -> ArkUIResult<Self> {
        let handle = ARK_UI_NATIVE_GESTURE_API_1.with(|api| {
            api.create_tap_gesture_with_distance_threshold(count, finger, distance_threshold)
        })?;
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

        let raw = self.raw_handle()?;

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

        let raw = self.raw_handle()?;

        let event_action_type: ArkUI_GestureEventActionType = action_type.into();
        ARK_UI_NATIVE_GESTURE_API_1.with(|api| {
            api.set_gesture_event_to_target(raw, event_action_type, self.inner_gesture_data.clone())
        })?;
        Ok(())
    }

    pub fn gesture_type(&self) -> ArkUIResult<GestureRecognizerType> {
        let raw = self.raw_handle()?;
        ARK_UI_NATIVE_GESTURE_API_1.with(|api| api.get_gesture_type(raw))
    }

    #[cfg(feature = "api-19")]
    pub fn set_distance_map(
        &self,
        tool_type_array: &mut [i32],
        distance_array: &mut [f64],
    ) -> ArkUIResult<()> {
        if self.inner_gesture_data.borrow().gesture_type != GestureRecognizerType::PanGesture {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "set_distance_map is only available for pan gesture",
            ));
        }
        let raw = self.raw_handle()?;
        let recognizer = GestureRecognizerRef::from_handle(raw).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "Gesture recognizer handle is null",
            )
        })?;
        recognizer.set_distance_map(tool_type_array, distance_array)
    }

    #[cfg(feature = "api-19")]
    pub fn distance_by_tool_type(&self, tool_type: i32) -> ArkUIResult<f64> {
        if self.inner_gesture_data.borrow().gesture_type != GestureRecognizerType::PanGesture {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "distance_by_tool_type is only available for pan gesture",
            ));
        }
        let raw = self.raw_handle()?;
        let recognizer = GestureRecognizerRef::from_handle(raw).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "Gesture recognizer handle is null",
            )
        })?;
        recognizer.distance_by_tool_type(tool_type)
    }

    #[cfg(feature = "api-22")]
    pub fn set_allowable_movement(&self, allowable_movement: f64) -> ArkUIResult<()> {
        if self.inner_gesture_data.borrow().gesture_type != GestureRecognizerType::LongPressGesture
        {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "set_allowable_movement is only available for long press gesture",
            ));
        }
        let raw = self.raw_handle()?;
        let recognizer = GestureRecognizerRef::from_handle(raw).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "Gesture recognizer handle is null",
            )
        })?;
        recognizer.set_allowable_movement(allowable_movement)
    }

    #[cfg(feature = "api-22")]
    pub fn allowable_movement(&self) -> ArkUIResult<f64> {
        if self.inner_gesture_data.borrow().gesture_type != GestureRecognizerType::LongPressGesture
        {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "allowable_movement is only available for long press gesture",
            ));
        }
        let raw = self.raw_handle()?;
        let recognizer = GestureRecognizerRef::from_handle(raw).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "Gesture recognizer handle is null",
            )
        })?;
        recognizer.allowable_movement()
    }

    pub fn dispose(&self) -> ArkUIResult<()> {
        let mut raw = self.raw.borrow_mut();
        if raw.is_null() {
            return Ok(());
        }
        ARK_UI_NATIVE_GESTURE_API_1.with(|api| api.dispose_gesture(*raw))?;
        *raw = std::ptr::null_mut();
        Ok(())
    }
}
