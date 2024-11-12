use std::{cell::RefCell, os::raw::c_void, rc::Rc};

use ohos_arkui_sys::{ArkUI_GestureEventActionType, ArkUI_GestureRecognizerHandle};

use crate::{ArkUIResult, GestureEventAction, ARK_UI_NATIVE_GESTURE_API_1};

pub(crate) struct InnerGestureData {
    pub gesture_callback: Option<fn(GestureEventData) -> ()>,
    pub user_data: Option<*mut c_void>,
}

pub struct GestureEventData {
    pub data: Option<*mut c_void>,
}

pub struct Gesture {
    pub(crate) raw: ArkUI_GestureRecognizerHandle,
    pub(crate) inner_gesture_data: Rc<RefCell<InnerGestureData>>,
}

impl Gesture {
    /// create long gesture
    pub fn create_long_gesture(finger: i32, repeat: bool, duration: i32) -> ArkUIResult<Self> {
        let handle = ARK_UI_NATIVE_GESTURE_API_1.create_long_gesture(finger, repeat, duration)?;
        Ok(Self {
            raw: handle,
            inner_gesture_data: Rc::new(RefCell::new(InnerGestureData {
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

        ARK_UI_NATIVE_GESTURE_API_1.set_gesture_event_to_target(
            self.raw,
            event_action_type,
            self.inner_gesture_data.clone(),
        )?;
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

        let event_action_type: ArkUI_GestureEventActionType = action_type.into();
        ARK_UI_NATIVE_GESTURE_API_1.set_gesture_event_to_target(
            self.raw,
            event_action_type,
            self.inner_gesture_data.clone(),
        )?;
        Ok(())
    }
}
