//! Module animate::options::animate_option wrappers and related types.

use std::{cell::RefCell, ptr::NonNull, rc::Rc};

use ohos_arkui_sys::{
    ArkUI_AnimateOption, OH_ArkUI_AnimateOption_Create, OH_ArkUI_AnimateOption_Dispose,
    OH_ArkUI_AnimateOption_GetCurve, OH_ArkUI_AnimateOption_GetDelay,
    OH_ArkUI_AnimateOption_GetDuration, OH_ArkUI_AnimateOption_GetExpectedFrameRateRange,
    OH_ArkUI_AnimateOption_GetICurve, OH_ArkUI_AnimateOption_GetIterations,
    OH_ArkUI_AnimateOption_GetPlayMode, OH_ArkUI_AnimateOption_GetTempo,
    OH_ArkUI_AnimateOption_SetCurve, OH_ArkUI_AnimateOption_SetDelay,
    OH_ArkUI_AnimateOption_SetDuration, OH_ArkUI_AnimateOption_SetExpectedFrameRateRange,
    OH_ArkUI_AnimateOption_SetICurve, OH_ArkUI_AnimateOption_SetIterations,
    OH_ArkUI_AnimateOption_SetPlayMode, OH_ArkUI_AnimateOption_SetTempo,
};

use crate::api::ARK_UI_NATIVE_ANIMATE_API_1;
use crate::{AnimationFinishCallbackType, AnimationMode, ArkUIContext, ArkUIResult, Curve};

use super::AnimationFrameRateRange;
use crate::animate::context::{AnimationFinishContext, AnimationUpdateContext};
use crate::animate::curve::CurveHandle;

/// High-level wrapper for `ArkUI_AnimateOption`.
pub struct Animation {
    pub(crate) raw: Rc<RefCell<NonNull<ArkUI_AnimateOption>>>,
    pub(crate) update_ctx: Rc<RefCell<AnimationUpdateContext>>,
    pub(crate) finish_ctx: Rc<RefCell<AnimationFinishContext>>,
}

impl Animation {
    /// Creates default animation options.
    pub fn new() -> Self {
        let raw = unsafe { OH_ArkUI_AnimateOption_Create() };
        let raw = NonNull::new(raw).unwrap_or_else(|| {
            panic!("OH_ArkUI_AnimateOption_Create returned null");
        });
        Self {
            raw: Rc::new(RefCell::new(raw)),
            update_ctx: Default::default(),
            finish_ctx: Default::default(),
        }
    }

    /// Sets animation duration in milliseconds.
    pub fn duration(&self, duration: i32) {
        unsafe { OH_ArkUI_AnimateOption_SetDuration(self.raw(), duration) };
    }

    pub fn get_duration(&self) -> u32 {
        unsafe { OH_ArkUI_AnimateOption_GetDuration(self.raw()) }
    }

    /// Sets playback speed multiplier.
    pub fn tempo(&self, tempo: f32) {
        unsafe { OH_ArkUI_AnimateOption_SetTempo(self.raw(), tempo) };
    }

    pub fn get_tempo(&self) -> f32 {
        unsafe { OH_ArkUI_AnimateOption_GetTempo(self.raw()) }
    }

    /// Sets start delay in milliseconds.
    pub fn delay(&self, delay: i32) {
        unsafe { OH_ArkUI_AnimateOption_SetDelay(self.raw(), delay) };
    }

    pub fn get_delay(&self) -> i32 {
        unsafe { OH_ArkUI_AnimateOption_GetDelay(self.raw()) }
    }

    /// Sets iteration count (`-1` for infinite).
    pub fn iterations(&self, iterations: i32) {
        unsafe { OH_ArkUI_AnimateOption_SetIterations(self.raw(), iterations) };
    }

    pub fn get_iterations(&self) -> i32 {
        unsafe { OH_ArkUI_AnimateOption_GetIterations(self.raw()) }
    }

    /// Selects built-in easing curve.
    pub fn curve(&self, curve: Curve) {
        unsafe { OH_ArkUI_AnimateOption_SetCurve(self.raw(), curve.into()) };
    }

    pub fn get_curve(&self) -> Option<Curve> {
        let curve = unsafe { OH_ArkUI_AnimateOption_GetCurve(self.raw()) };
        Curve::try_from_raw(curve)
    }

    /// Selects custom curve handle.
    pub fn i_curve(&self, curve: &CurveHandle) {
        unsafe { OH_ArkUI_AnimateOption_SetICurve(self.raw(), curve.as_raw()) };
    }

    pub fn get_i_curve(&self) -> Option<CurveHandle> {
        let curve = unsafe { OH_ArkUI_AnimateOption_GetICurve(self.raw()) };
        CurveHandle::from_raw_borrowed(curve)
    }

    /// Sets animation play mode.
    pub fn mode(&self, mode: AnimationMode) {
        unsafe { OH_ArkUI_AnimateOption_SetPlayMode(self.raw(), mode.into()) };
    }

    pub fn get_mode(&self) -> Option<AnimationMode> {
        let mode = unsafe { OH_ArkUI_AnimateOption_GetPlayMode(self.raw()) };
        AnimationMode::try_from_raw(mode)
    }

    /// Sets expected frame-rate range.
    pub fn rate_range(&self, range: AnimationFrameRateRange) {
        let mut raw_range = range.raw();
        unsafe { OH_ArkUI_AnimateOption_SetExpectedFrameRateRange(self.raw(), &mut raw_range) };
    }

    pub fn get_rate_range(&self) -> AnimationFrameRateRange {
        let range_ptr = unsafe { OH_ArkUI_AnimateOption_GetExpectedFrameRateRange(self.raw()) };
        if range_ptr.is_null() {
            return AnimationFrameRateRange::new();
        }

        let range = unsafe { *range_ptr };
        AnimationFrameRateRange::from_raw(range)
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_AnimateOption {
        self.raw.borrow().as_ptr()
    }

    /// Registers update callback.
    pub fn update<T: Fn() + 'static>(&self, callback: T) {
        let ctx = self.update_ctx.borrow();
        ctx.callback(callback);
    }

    /// Registers finish callback with callback-type selector.
    pub fn finish<T: Fn() + 'static>(
        &self,
        callback_type: AnimationFinishCallbackType,
        callback: T,
    ) {
        let ctx = self.finish_ctx.borrow();
        ctx.callback(callback);
        ctx.callback_type(callback_type);
    }

    #[cfg(feature = "napi")]
    pub fn animate_to(&self, ctx: ArkUIContext) -> ArkUIResult<()> {
        let update_ctx_raw = self.update_ctx.borrow().raw();
        let finish_ctx_raw = self.finish_ctx.borrow().raw();
        ARK_UI_NATIVE_ANIMATE_API_1
            .with(|api| api.animate_to(ctx.raw(), self.raw(), update_ctx_raw, finish_ctx_raw))?;
        Ok(())
    }
}

impl Default for Animation {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Animation {
    fn drop(&mut self) {
        unsafe { OH_ArkUI_AnimateOption_Dispose(self.raw()) };
    }
}
