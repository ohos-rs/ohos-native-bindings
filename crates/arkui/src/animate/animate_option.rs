use std::{cell::RefCell, os::raw::c_void, rc::Rc};

use ohos_arkui_sys::{
    ArkUI_AnimateOption, OH_ArkUI_AnimateOption_Create, OH_ArkUI_AnimateOption_SetCurve,
    OH_ArkUI_AnimateOption_SetDelay, OH_ArkUI_AnimateOption_SetDuration,
    OH_ArkUI_AnimateOption_SetExpectedFrameRateRange, OH_ArkUI_AnimateOption_SetIterations,
    OH_ArkUI_AnimateOption_SetPlayMode, OH_ArkUI_AnimateOption_SetTempo,
};

use crate::{AnimationMode, ArkUIContext, Curve, ARK_UI_NATIVE_ANIMATE_API_1};

use super::{AnimationFrameRateRange, AnimationUpdateContext};

pub struct Animation {
    pub(crate) raw: Rc<RefCell<*mut ArkUI_AnimateOption>>,
    pub(crate) update_ctx: Rc<RefCell<AnimationUpdateContext>>,
}

impl Animation {
    pub fn new() -> Self {
        let ret = unsafe { OH_ArkUI_AnimateOption_Create() };
        Animation {
            raw: Rc::new(RefCell::new(ret)),
            update_ctx: Rc::new(RefCell::new(AnimationUpdateContext {
                callback: Rc::new(RefCell::new(None)),
                data: Rc::new(RefCell::new(None)),
            })),
        }
    }

    pub fn duration(&self, duration: i32) {
        let options = self.raw.borrow();
        unsafe { OH_ArkUI_AnimateOption_SetDuration(*options, duration) };
    }

    pub fn tempo(&self, tempo: f32) {
        let options = self.raw.borrow();
        unsafe { OH_ArkUI_AnimateOption_SetTempo(*options, tempo) };
    }

    pub fn delay(&self, delay: i32) {
        let options = self.raw.borrow();
        unsafe { OH_ArkUI_AnimateOption_SetDelay(*options, delay) };
    }

    pub fn iterations(&self, iterations: i32) {
        let options = self.raw.borrow();
        unsafe { OH_ArkUI_AnimateOption_SetIterations(*options, iterations) };
    }

    pub fn curve(&self, curve: Curve) {
        let options = self.raw.borrow();
        unsafe { OH_ArkUI_AnimateOption_SetCurve(*options, curve.into()) };
    }

    pub fn mode(&self, mode: AnimationMode) {
        let options = self.raw.borrow();
        unsafe { OH_ArkUI_AnimateOption_SetPlayMode(*options, mode.into()) };
    }

    pub fn rate_range(&self, range: AnimationFrameRateRange) {
        let options = self.raw.borrow();
        unsafe {
            OH_ArkUI_AnimateOption_SetExpectedFrameRateRange(*options, range.0.as_ref().as_ptr())
        };
    }

    pub fn update<T: Fn(*mut c_void) -> () + 'static>(&self, data: *mut c_void, callback: T) {
        let mut ctx = self.update_ctx.borrow_mut();
        ctx.data = Rc::new(RefCell::new(Some(data)));
        ctx.callback = Rc::new(RefCell::new(Some(Box::new(callback))))
    }

    // #[cfg(feature = "napi")]
    // pub fn animate_to(&self, ctx: ArkUIContext) {
    //     unsafe {
    //         ARK_UI_NATIVE_ANIMATE_API_1.
    //     };
    // }
}
