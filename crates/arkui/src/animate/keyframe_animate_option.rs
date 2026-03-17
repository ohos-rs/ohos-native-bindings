use std::{cell::RefCell, rc::Rc};

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::{
    ArkUI_CurveHandle, ArkUI_KeyframeAnimateOption, OH_ArkUI_KeyframeAnimateOption_Create,
    OH_ArkUI_KeyframeAnimateOption_Dispose, OH_ArkUI_KeyframeAnimateOption_GetCurve,
    OH_ArkUI_KeyframeAnimateOption_GetDelay, OH_ArkUI_KeyframeAnimateOption_GetDuration,
    OH_ArkUI_KeyframeAnimateOption_GetIterations, OH_ArkUI_KeyframeAnimateOption_SetCurve,
    OH_ArkUI_KeyframeAnimateOption_SetDelay, OH_ArkUI_KeyframeAnimateOption_SetDuration,
    OH_ArkUI_KeyframeAnimateOption_SetIterations,
};

#[cfg(feature = "api-19")]
use ohos_arkui_sys::{
    OH_ArkUI_KeyframeAnimateOption_GetExpectedFrameRate,
    OH_ArkUI_KeyframeAnimateOption_SetExpectedFrameRate,
};

use crate::{
    check_arkui_status, ArkUIContext, ArkUIError, ArkUIResult, ARK_UI_NATIVE_ANIMATE_API_1,
};

#[cfg(feature = "api-19")]
use super::AnimationFrameRateRange;

pub struct KeyframeAnimation {
    raw: Rc<RefCell<*mut ArkUI_KeyframeAnimateOption>>,
}

impl KeyframeAnimation {
    pub fn new(size: i32) -> ArkUIResult<Self> {
        let option = unsafe { OH_ArkUI_KeyframeAnimateOption_Create(size) };
        if option.is_null() {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_KeyframeAnimateOption_Create returned null",
            ));
        }

        Ok(Self {
            raw: Rc::new(RefCell::new(option)),
        })
    }

    pub fn raw(&self) -> *mut ArkUI_KeyframeAnimateOption {
        *self.raw.borrow()
    }

    pub fn delay(&self, delay: i32) -> ArkUIResult<()> {
        let option = self.raw();
        check_arkui_status!(unsafe { OH_ArkUI_KeyframeAnimateOption_SetDelay(option, delay) })
    }

    pub fn get_delay(&self) -> i32 {
        let option = self.raw();
        unsafe { OH_ArkUI_KeyframeAnimateOption_GetDelay(option) }
    }

    pub fn iterations(&self, iterations: i32) -> ArkUIResult<()> {
        let option = self.raw();
        check_arkui_status!(unsafe {
            OH_ArkUI_KeyframeAnimateOption_SetIterations(option, iterations)
        })
    }

    pub fn get_iterations(&self) -> i32 {
        let option = self.raw();
        unsafe { OH_ArkUI_KeyframeAnimateOption_GetIterations(option) }
    }

    pub fn duration(&self, duration: i32, index: i32) -> ArkUIResult<()> {
        let option = self.raw();
        check_arkui_status!(unsafe {
            OH_ArkUI_KeyframeAnimateOption_SetDuration(option, duration, index)
        })
    }

    pub fn get_duration(&self, index: i32) -> i32 {
        let option = self.raw();
        unsafe { OH_ArkUI_KeyframeAnimateOption_GetDuration(option, index) }
    }

    pub fn curve(&self, curve: ArkUI_CurveHandle, index: i32) -> ArkUIResult<()> {
        let option = self.raw();
        check_arkui_status!(unsafe {
            OH_ArkUI_KeyframeAnimateOption_SetCurve(option, curve, index)
        })
    }

    pub fn get_curve(&self, index: i32) -> ArkUI_CurveHandle {
        let option = self.raw();
        unsafe { OH_ArkUI_KeyframeAnimateOption_GetCurve(option, index) }
    }

    #[cfg(feature = "api-19")]
    pub fn rate_range(&self, range: AnimationFrameRateRange) -> ArkUIResult<()> {
        let option = self.raw();
        check_arkui_status!(unsafe {
            OH_ArkUI_KeyframeAnimateOption_SetExpectedFrameRate(option, range.0.as_ref().as_ptr())
        })
    }

    #[cfg(feature = "api-19")]
    pub fn get_rate_range(&self) -> AnimationFrameRateRange {
        let option = self.raw();
        let range_ptr = unsafe { OH_ArkUI_KeyframeAnimateOption_GetExpectedFrameRate(option) };
        if range_ptr.is_null() {
            return AnimationFrameRateRange::new();
        }

        let range = unsafe { *range_ptr };
        AnimationFrameRateRange::from_raw(range)
    }

    #[cfg(feature = "napi")]
    pub fn animate_to(&self, ctx: ArkUIContext) -> ArkUIResult<()> {
        ARK_UI_NATIVE_ANIMATE_API_1.with(|api| api.keyframe_animate_to(ctx.raw(), self.raw()))?;
        Ok(())
    }
}

impl Drop for KeyframeAnimation {
    fn drop(&mut self) {
        let raw = *self.raw.borrow();
        if !raw.is_null() {
            unsafe { OH_ArkUI_KeyframeAnimateOption_Dispose(raw) };
        }
    }
}
