//! Module animate::curve::native wrappers and related types.

use std::{os::raw::c_void, ptr::NonNull};

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::{ArkUI_Curve, OH_ArkUI_Curve_CreateCustomCurve, OH_ArkUI_Curve_DisposeCurve};

use crate::{ArkUIError, ArkUIResult};

struct CustomCurveCallbackContext {
    callback: Box<dyn Fn(f32) -> f32>,
}

type CustomCurveInterpolateCallback =
    unsafe extern "C" fn(input: f32, user_data: *mut c_void) -> f32;

pub(crate) struct CustomCurve {
    raw: NonNull<ArkUI_Curve>,
    callback: Option<NonNull<CustomCurveCallbackContext>>,
}

impl CustomCurve {
    pub(crate) fn new<T: Fn(f32) -> f32 + 'static>(interpolate: T) -> ArkUIResult<Self> {
        let callback = NonNull::new(Box::into_raw(Box::new(CustomCurveCallbackContext {
            callback: Box::new(interpolate),
        })))
        .expect("CustomCurveCallbackContext pointer is null");
        let raw = unsafe {
            OH_ArkUI_Curve_CreateCustomCurve(
                callback.as_ptr().cast(),
                Some(custom_curve_trampoline as CustomCurveInterpolateCallback),
            )
        };
        let raw = match NonNull::new(raw) {
            Some(raw) => raw,
            None => {
                unsafe {
                    drop(Box::from_raw(callback.as_ptr()));
                }
                return Err(ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_Curve_CreateCustomCurve returned null",
                ));
            }
        };
        Ok(Self {
            raw,
            callback: Some(callback),
        })
    }

    pub(crate) fn raw(&self) -> NonNull<ArkUI_Curve> {
        self.raw
    }

    pub(crate) fn dispose(mut self) {
        if let Some(callback) = self.callback.take() {
            unsafe {
                drop(Box::from_raw(callback.as_ptr()));
            }
        }
        unsafe { OH_ArkUI_Curve_DisposeCurve(self.raw.as_ptr()) }
    }
}

unsafe extern "C" fn custom_curve_trampoline(input: f32, user_data: *mut c_void) -> f32 {
    if user_data.is_null() {
        return input;
    }
    let callback = unsafe { &*(user_data as *mut CustomCurveCallbackContext) };
    (callback.callback)(input)
}
