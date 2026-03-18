use std::{cell::RefCell, collections::HashMap, os::raw::c_void, ptr::NonNull, rc::Rc};

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::{
    ArkUI_KeyframeAnimateOption, OH_ArkUI_KeyframeAnimateOption_Create,
    OH_ArkUI_KeyframeAnimateOption_Dispose, OH_ArkUI_KeyframeAnimateOption_GetCurve,
    OH_ArkUI_KeyframeAnimateOption_GetDelay, OH_ArkUI_KeyframeAnimateOption_GetDuration,
    OH_ArkUI_KeyframeAnimateOption_GetIterations,
    OH_ArkUI_KeyframeAnimateOption_RegisterOnEventCallback,
    OH_ArkUI_KeyframeAnimateOption_RegisterOnFinishCallback,
    OH_ArkUI_KeyframeAnimateOption_SetCurve, OH_ArkUI_KeyframeAnimateOption_SetDelay,
    OH_ArkUI_KeyframeAnimateOption_SetDuration, OH_ArkUI_KeyframeAnimateOption_SetIterations,
};

#[cfg(feature = "api-19")]
use ohos_arkui_sys::{
    OH_ArkUI_KeyframeAnimateOption_GetExpectedFrameRate,
    OH_ArkUI_KeyframeAnimateOption_SetExpectedFrameRate,
};

use crate::animate::curve::CurveHandle;
use crate::api::ARK_UI_NATIVE_ANIMATE_API_1;
use crate::{check_arkui_status, ArkUIContext, ArkUIError, ArkUIResult};

#[cfg(feature = "api-19")]
use super::AnimationFrameRateRange;

pub struct KeyframeAnimation {
    raw: Rc<RefCell<NonNull<ArkUI_KeyframeAnimateOption>>>,
    finish_callback: RefCell<Option<*mut KeyframeCallbackContext>>,
    event_callbacks: RefCell<HashMap<i32, *mut KeyframeCallbackContext>>,
}

struct KeyframeCallbackContext {
    callback: Box<dyn Fn()>,
}

impl KeyframeAnimation {
    pub fn new(size: i32) -> ArkUIResult<Self> {
        let option = unsafe { OH_ArkUI_KeyframeAnimateOption_Create(size) };
        let option = NonNull::new(option).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_KeyframeAnimateOption_Create returned null",
            )
        })?;

        Ok(Self {
            raw: Rc::new(RefCell::new(option)),
            finish_callback: RefCell::new(None),
            event_callbacks: RefCell::new(HashMap::new()),
        })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_KeyframeAnimateOption {
        self.raw.borrow().as_ptr()
    }

    pub fn delay(&self, delay: i32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe { OH_ArkUI_KeyframeAnimateOption_SetDelay(self.raw(), delay) })
    }

    pub fn get_delay(&self) -> i32 {
        unsafe { OH_ArkUI_KeyframeAnimateOption_GetDelay(self.raw()) }
    }

    pub fn iterations(&self, iterations: i32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            OH_ArkUI_KeyframeAnimateOption_SetIterations(self.raw(), iterations)
        })
    }

    pub fn get_iterations(&self) -> i32 {
        unsafe { OH_ArkUI_KeyframeAnimateOption_GetIterations(self.raw()) }
    }

    pub fn on_finish_callback<T: Fn() + 'static>(&self, on_finish: T) -> ArkUIResult<()> {
        let callback = Box::into_raw(Box::new(KeyframeCallbackContext {
            callback: Box::new(on_finish),
        }));
        let result = check_arkui_status!(unsafe {
            OH_ArkUI_KeyframeAnimateOption_RegisterOnFinishCallback(
                self.raw(),
                callback.cast(),
                Some(keyframe_callback_trampoline),
            )
        });

        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }

        if let Some(old) = self.finish_callback.borrow_mut().replace(callback) {
            unsafe {
                drop(Box::from_raw(old));
            }
        }

        Ok(())
    }

    pub fn clear_on_finish_callback(&self) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            OH_ArkUI_KeyframeAnimateOption_RegisterOnFinishCallback(
                self.raw(),
                std::ptr::null_mut(),
                None,
            )
        })?;

        if let Some(old) = self.finish_callback.borrow_mut().take() {
            unsafe {
                drop(Box::from_raw(old));
            }
        }

        Ok(())
    }

    pub fn on_event_callback<T: Fn() + 'static>(&self, index: i32, event: T) -> ArkUIResult<()> {
        let callback = Box::into_raw(Box::new(KeyframeCallbackContext {
            callback: Box::new(event),
        }));
        let result = check_arkui_status!(unsafe {
            OH_ArkUI_KeyframeAnimateOption_RegisterOnEventCallback(
                self.raw(),
                callback.cast(),
                Some(keyframe_callback_trampoline),
                index,
            )
        });

        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }

        if let Some(old) = self.event_callbacks.borrow_mut().insert(index, callback) {
            unsafe {
                drop(Box::from_raw(old));
            }
        }

        Ok(())
    }

    pub fn clear_on_event_callback(&self, index: i32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            OH_ArkUI_KeyframeAnimateOption_RegisterOnEventCallback(
                self.raw(),
                std::ptr::null_mut(),
                None,
                index,
            )
        })?;

        if let Some(old) = self.event_callbacks.borrow_mut().remove(&index) {
            unsafe {
                drop(Box::from_raw(old));
            }
        }

        Ok(())
    }

    pub fn duration(&self, duration: i32, index: i32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            OH_ArkUI_KeyframeAnimateOption_SetDuration(self.raw(), duration, index)
        })
    }

    pub fn get_duration(&self, index: i32) -> i32 {
        unsafe { OH_ArkUI_KeyframeAnimateOption_GetDuration(self.raw(), index) }
    }

    pub fn curve(&self, curve: &CurveHandle, index: i32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            OH_ArkUI_KeyframeAnimateOption_SetCurve(self.raw(), curve.as_raw(), index)
        })
    }

    pub fn get_curve(&self, index: i32) -> Option<CurveHandle> {
        let curve = unsafe { OH_ArkUI_KeyframeAnimateOption_GetCurve(self.raw(), index) };
        CurveHandle::from_raw_borrowed(curve)
    }

    #[cfg(feature = "api-19")]
    pub fn rate_range(&self, range: AnimationFrameRateRange) -> ArkUIResult<()> {
        let mut raw_range = range.raw();
        check_arkui_status!(unsafe {
            OH_ArkUI_KeyframeAnimateOption_SetExpectedFrameRate(self.raw(), &mut raw_range)
        })
    }

    #[cfg(feature = "api-19")]
    pub fn get_rate_range(&self) -> AnimationFrameRateRange {
        let range_ptr = unsafe { OH_ArkUI_KeyframeAnimateOption_GetExpectedFrameRate(self.raw()) };
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
        if let Some(callback) = self.finish_callback.borrow_mut().take() {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }

        for (_, callback) in self.event_callbacks.borrow_mut().drain() {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }

        unsafe { OH_ArkUI_KeyframeAnimateOption_Dispose(self.raw()) };
    }
}

unsafe extern "C" fn keyframe_callback_trampoline(data: *mut c_void) {
    if data.is_null() {
        return;
    }

    let callback = unsafe { &*(data as *mut KeyframeCallbackContext) };
    (callback.callback)();
}
