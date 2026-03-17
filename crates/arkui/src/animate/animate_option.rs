use std::{cell::RefCell, os::raw::c_void, rc::Rc};

use ohos_arkui_sys::{
    ArkUI_AnimateOption, ArkUI_CurveHandle, OH_ArkUI_AnimateOption_Create,
    OH_ArkUI_AnimateOption_Dispose, OH_ArkUI_AnimateOption_GetCurve,
    OH_ArkUI_AnimateOption_GetDelay, OH_ArkUI_AnimateOption_GetDuration,
    OH_ArkUI_AnimateOption_GetExpectedFrameRateRange, OH_ArkUI_AnimateOption_GetICurve,
    OH_ArkUI_AnimateOption_GetIterations, OH_ArkUI_AnimateOption_GetPlayMode,
    OH_ArkUI_AnimateOption_GetTempo, OH_ArkUI_AnimateOption_SetCurve,
    OH_ArkUI_AnimateOption_SetDelay, OH_ArkUI_AnimateOption_SetDuration,
    OH_ArkUI_AnimateOption_SetExpectedFrameRateRange, OH_ArkUI_AnimateOption_SetICurve,
    OH_ArkUI_AnimateOption_SetIterations, OH_ArkUI_AnimateOption_SetPlayMode,
    OH_ArkUI_AnimateOption_SetTempo,
};

use crate::{
    AnimationFinishCallbackType, AnimationMode, ArkUIContext, ArkUIResult, Curve,
    ARK_UI_NATIVE_ANIMATE_API_1,
};

use super::{AnimationFinishContext, AnimationFrameRateRange, AnimationUpdateContext};

pub struct Animation {
    pub(crate) raw: Rc<RefCell<*mut ArkUI_AnimateOption>>,
    pub(crate) update_ctx: Rc<RefCell<AnimationUpdateContext>>,
    pub(crate) finish_ctx: Rc<RefCell<AnimationFinishContext>>,
}

impl Animation {
    pub fn new() -> Self {
        let ret = unsafe { OH_ArkUI_AnimateOption_Create() };
        Animation {
            raw: Rc::new(RefCell::new(ret)),
            update_ctx: Default::default(),
            finish_ctx: Default::default(),
        }
    }

    pub fn duration(&self, duration: i32) {
        let options = self.raw.borrow();
        unsafe { OH_ArkUI_AnimateOption_SetDuration(*options, duration) };
    }

    pub fn get_duration(&self) -> u32 {
        let options = self.raw.borrow();
        unsafe { OH_ArkUI_AnimateOption_GetDuration(*options) }
    }

    pub fn tempo(&self, tempo: f32) {
        let options = self.raw.borrow();
        unsafe { OH_ArkUI_AnimateOption_SetTempo(*options, tempo) };
    }

    pub fn get_tempo(&self) -> f32 {
        let options = self.raw.borrow();
        unsafe { OH_ArkUI_AnimateOption_GetTempo(*options) }
    }

    pub fn delay(&self, delay: i32) {
        let options = self.raw.borrow();
        unsafe { OH_ArkUI_AnimateOption_SetDelay(*options, delay) };
    }

    pub fn get_delay(&self) -> i32 {
        let options = self.raw.borrow();
        unsafe { OH_ArkUI_AnimateOption_GetDelay(*options) }
    }

    pub fn iterations(&self, iterations: i32) {
        let options = self.raw.borrow();
        unsafe { OH_ArkUI_AnimateOption_SetIterations(*options, iterations) };
    }

    pub fn get_iterations(&self) -> i32 {
        let options = self.raw.borrow();
        unsafe { OH_ArkUI_AnimateOption_GetIterations(*options) }
    }

    pub fn curve(&self, curve: Curve) {
        let options = self.raw.borrow();
        unsafe { OH_ArkUI_AnimateOption_SetCurve(*options, curve.into()) };
    }

    pub fn get_curve(&self) -> Option<Curve> {
        let options = self.raw.borrow();
        let curve = unsafe { OH_ArkUI_AnimateOption_GetCurve(*options) };
        Curve::try_from_raw(curve)
    }

    pub fn i_curve(&self, curve: ArkUI_CurveHandle) {
        let options = self.raw.borrow();
        unsafe { OH_ArkUI_AnimateOption_SetICurve(*options, curve) };
    }

    pub fn get_i_curve(&self) -> ArkUI_CurveHandle {
        let options = self.raw.borrow();
        unsafe { OH_ArkUI_AnimateOption_GetICurve(*options) }
    }

    pub fn mode(&self, mode: AnimationMode) {
        let options = self.raw.borrow();
        unsafe { OH_ArkUI_AnimateOption_SetPlayMode(*options, mode.into()) };
    }

    pub fn get_mode(&self) -> Option<AnimationMode> {
        let options = self.raw.borrow();
        let mode = unsafe { OH_ArkUI_AnimateOption_GetPlayMode(*options) };
        AnimationMode::try_from_raw(mode)
    }

    pub fn rate_range(&self, range: AnimationFrameRateRange) {
        let options = self.raw.borrow();
        unsafe {
            OH_ArkUI_AnimateOption_SetExpectedFrameRateRange(*options, range.0.as_ref().as_ptr())
        };
    }

    pub fn get_rate_range(&self) -> AnimationFrameRateRange {
        let options = self.raw.borrow();
        let range_ptr = unsafe { OH_ArkUI_AnimateOption_GetExpectedFrameRateRange(*options) };
        if range_ptr.is_null() {
            return AnimationFrameRateRange::new();
        }

        let range = unsafe { *range_ptr };
        AnimationFrameRateRange::from_raw(range)
    }

    pub fn raw(&self) -> *mut ArkUI_AnimateOption {
        *self.raw.borrow()
    }

    pub fn update<T: Fn(*mut c_void) + 'static>(&self, data: *mut c_void, callback: T) {
        let ctx = self.update_ctx.borrow_mut();
        ctx.data(data);
        ctx.callback(callback);
    }

    pub fn finish<T: Fn(*mut c_void) + 'static>(
        &self,
        callback_type: AnimationFinishCallbackType,
        data: *mut c_void,
        callback: T,
    ) {
        let ctx = self.finish_ctx.borrow_mut();
        ctx.data(data);
        ctx.callback(callback);
        ctx.callback_type(callback_type);
    }

    #[cfg(feature = "napi")]
    pub fn animate_to(&self, ctx: ArkUIContext) -> ArkUIResult<()> {
        let option = self.raw.borrow();
        let update_ctx_raw = self.update_ctx.borrow().raw();
        let finish_ctx_raw = self.finish_ctx.borrow().raw();
        ARK_UI_NATIVE_ANIMATE_API_1
            .with(|api| api.animate_to(ctx.raw(), *option, update_ctx_raw, finish_ctx_raw))?;
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
        let raw = *self.raw.borrow();
        if !raw.is_null() {
            unsafe { OH_ArkUI_AnimateOption_Dispose(raw) };
        }
    }
}
