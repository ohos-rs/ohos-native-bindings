use ohos_arkui_sys::{ArkUI_GestureRecognizerHandle, ArkUI_GroupGestureMode};

use crate::{ArkUIResult, GestureGroupMode, ARK_UI_NATIVE_GESTURE_API_1};

use super::Gesture;

pub struct GestureGroup {
    pub(crate) raw: ArkUI_GestureRecognizerHandle,
    pub(crate) gestures: Vec<Gesture>,
}

impl GestureGroup {
    pub fn new(mode: GestureGroupMode) -> ArkUIResult<Self> {
        let mode: ArkUI_GroupGestureMode = mode.into();
        let handle = ARK_UI_NATIVE_GESTURE_API_1.with(|api| api.create_gesture_group(mode))?;
        Ok(GestureGroup {
            raw: handle,
            gestures: vec![],
        })
    }

    pub fn add_gesture(&mut self, gesture: Gesture) -> ArkUIResult<()> {
        let child_raw = *gesture.raw.borrow();
        self.gestures.push(gesture);
        ARK_UI_NATIVE_GESTURE_API_1.with(|api| api.add_child_gesture(self.raw, child_raw))?;
        Ok(())
    }
}
