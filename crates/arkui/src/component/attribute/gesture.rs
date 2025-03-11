use ohos_arkui_sys::{ArkUI_GestureMask, ArkUI_GesturePriority};

use crate::{ArkUIError, Gesture, GestureMask, GesturePriority, ARK_UI_NATIVE_GESTURE_API_1};

use super::ArkUIAttributeBasic;

pub trait ArkUIGesture: ArkUIAttributeBasic {
    fn add_gesture(
        &self,
        gesture: Gesture,
        mode: Option<GesturePriority>,
        mask: Option<GestureMask>,
    ) -> Result<(), ArkUIError> {
        let mode: ArkUI_GesturePriority = mode.unwrap_or(GesturePriority::Parallel).into();
        let mask: ArkUI_GestureMask = mask.unwrap_or(GestureMask::NormalGestureMask).into();
        let raw = gesture.raw.borrow().clone();
        ARK_UI_NATIVE_GESTURE_API_1.add_gesture(raw, self.raw().raw(), mode, mask)?;
        Ok(())
    }
}
