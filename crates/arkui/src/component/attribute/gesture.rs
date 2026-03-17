use std::os::raw::c_void;

use ohos_arkui_sys::{
    ArkUI_GestureInterruptInfo, ArkUI_GestureInterruptResult, ArkUI_GestureMask,
    ArkUI_GesturePriority, ArkUI_GestureRecognizer, ArkUI_ParallelInnerGestureEvent,
};

use crate::{ArkUIResult, Gesture, GestureMask, GesturePriority, ARK_UI_NATIVE_GESTURE_API_1};

use super::ArkUIAttributeBasic;

pub trait ArkUIGesture: ArkUIAttributeBasic {
    fn add_gesture(
        &self,
        gesture: Gesture,
        mode: Option<GesturePriority>,
        mask: Option<GestureMask>,
    ) -> ArkUIResult<()> {
        self.add_gesture_ref(&gesture, mode, mask)
    }

    fn add_gesture_ref(
        &self,
        gesture: &Gesture,
        mode: Option<GesturePriority>,
        mask: Option<GestureMask>,
    ) -> ArkUIResult<()> {
        let mode: ArkUI_GesturePriority = mode.unwrap_or(GesturePriority::Parallel).into();
        let mask: ArkUI_GestureMask = mask.unwrap_or(GestureMask::NormalGestureMask).into();
        let raw = *gesture.raw.borrow();
        ARK_UI_NATIVE_GESTURE_API_1
            .with(|api| api.add_gesture(raw, self.raw().raw(), mode, mask))?;
        Ok(())
    }

    fn remove_gesture(&self, gesture: &Gesture) -> ArkUIResult<()> {
        let raw = *gesture.raw.borrow();
        ARK_UI_NATIVE_GESTURE_API_1.with(|api| api.remove_gesture(raw, self.raw().raw()))?;
        Ok(())
    }

    fn set_gesture_interrupter(
        &self,
        interrupter: Option<
            unsafe extern "C" fn(
                info: *mut ArkUI_GestureInterruptInfo,
            ) -> ArkUI_GestureInterruptResult,
        >,
    ) -> ArkUIResult<()> {
        ARK_UI_NATIVE_GESTURE_API_1
            .with(|api| api.set_gesture_interrupter_to_node(self.raw().raw(), interrupter))?;
        Ok(())
    }

    fn set_inner_gesture_parallel_to(
        &self,
        user_data: *mut c_void,
        parallel_inner_gesture: Option<
            unsafe extern "C" fn(
                event: *mut ArkUI_ParallelInnerGestureEvent,
            ) -> *mut ArkUI_GestureRecognizer,
        >,
    ) -> ArkUIResult<()> {
        ARK_UI_NATIVE_GESTURE_API_1.with(|api| {
            api.set_inner_gesture_parallel_to(self.raw().raw(), user_data, parallel_inner_gesture)
        })?;
        Ok(())
    }
}
