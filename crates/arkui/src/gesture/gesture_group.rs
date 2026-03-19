//! Module gesture::gesture_group wrappers and related types.

use ohos_arkui_sys::ArkUI_GestureRecognizerHandle;

use crate::{ArkUIResult, GestureGroupMode, ARK_UI_NATIVE_GESTURE_API_1};

use super::Gesture;

/// Group of gestures combined with a [`GestureGroupMode`].
pub struct GestureGroup {
    pub(crate) raw: ArkUI_GestureRecognizerHandle,
    pub(crate) gestures: Vec<Gesture>,
}

impl GestureGroup {
    /// Create a new gesture group.
    pub fn new(mode: GestureGroupMode) -> ArkUIResult<Self> {
        let handle = ARK_UI_NATIVE_GESTURE_API_1.with(|api| api.create_gesture_group(mode))?;
        Ok(GestureGroup {
            raw: handle,
            gestures: vec![],
        })
    }

    /// Add a child gesture to the group.
    pub fn add_gesture(&mut self, gesture: Gesture) -> ArkUIResult<()> {
        let child_raw = *gesture.raw.borrow();
        self.gestures.push(gesture);
        ARK_UI_NATIVE_GESTURE_API_1.with(|api| api.add_child_gesture(self.raw, child_raw))?;
        Ok(())
    }

    /// Remove child gesture by index.
    pub fn remove_gesture(&mut self, index: usize) -> ArkUIResult<Option<Gesture>> {
        if index >= self.gestures.len() {
            return Ok(None);
        }
        let gesture = self.gestures.remove(index);
        let child_raw = *gesture.raw.borrow();
        ARK_UI_NATIVE_GESTURE_API_1.with(|api| api.remove_child_gesture(self.raw, child_raw))?;
        Ok(Some(gesture))
    }

    /// Dispose the native gesture group and detach all child gestures.
    pub fn dispose(&mut self) -> ArkUIResult<()> {
        if self.raw.is_null() {
            self.gestures.clear();
            return Ok(());
        }

        for gesture in self.gestures.iter() {
            let child_raw = *gesture.raw.borrow();
            if !child_raw.is_null() {
                ARK_UI_NATIVE_GESTURE_API_1
                    .with(|api| api.remove_child_gesture(self.raw, child_raw))?;
            }
        }
        self.gestures.clear();

        ARK_UI_NATIVE_GESTURE_API_1.with(|api| api.dispose_gesture(self.raw))?;
        self.raw = std::ptr::null_mut();
        Ok(())
    }
}
