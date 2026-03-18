#[cfg(feature = "api-18")]
use crate::ARK_UI_NATIVE_GESTURE_API_2;
use crate::{
    ArkUIResult, Gesture, GestureInterruptInfoRef, GestureInterruptResult, GestureMask,
    GesturePriority, GestureRecognizerRef, ParallelInnerGestureEventRef,
    ARK_UI_NATIVE_GESTURE_API_1,
};

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
        let mode = mode.unwrap_or(GesturePriority::Parallel);
        let mask = mask.unwrap_or(GestureMask::NormalGestureMask);
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

    fn set_gesture_interrupter<
        T: Fn(GestureInterruptInfoRef) -> GestureInterruptResult + 'static,
    >(
        &self,
        interrupter: T,
    ) -> ArkUIResult<()> {
        #[cfg(feature = "api-18")]
        ARK_UI_NATIVE_GESTURE_API_2
            .with(|api| api.set_gesture_interrupter_to_node(self.raw().raw(), interrupter))?;
        #[cfg(not(feature = "api-18"))]
        ARK_UI_NATIVE_GESTURE_API_1
            .with(|api| api.set_gesture_interrupter_to_node(self.raw().raw(), interrupter))?;
        Ok(())
    }

    fn clear_gesture_interrupter(&self) -> ArkUIResult<()> {
        #[cfg(feature = "api-18")]
        ARK_UI_NATIVE_GESTURE_API_2
            .with(|api| api.clear_gesture_interrupter_to_node(self.raw().raw()))?;
        #[cfg(not(feature = "api-18"))]
        ARK_UI_NATIVE_GESTURE_API_1
            .with(|api| api.clear_gesture_interrupter_to_node(self.raw().raw()))?;
        Ok(())
    }

    fn set_inner_gesture_parallel_to<
        T: Fn(ParallelInnerGestureEventRef) -> Option<GestureRecognizerRef> + 'static,
    >(
        &self,
        parallel_inner_gesture: T,
    ) -> ArkUIResult<()> {
        ARK_UI_NATIVE_GESTURE_API_1.with(|api| {
            api.set_inner_gesture_parallel_to(self.raw().raw(), parallel_inner_gesture)
        })?;
        Ok(())
    }

    fn clear_inner_gesture_parallel_to(&self) -> ArkUIResult<()> {
        ARK_UI_NATIVE_GESTURE_API_1
            .with(|api| api.clear_inner_gesture_parallel_to(self.raw().raw()))?;
        Ok(())
    }
}
