use std::{os::raw::c_void, ptr::NonNull};

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::{
    ArkUI_Curve, ArkUI_CurveHandle, OH_ArkUI_Curve_CreateCubicBezierCurve,
    OH_ArkUI_Curve_CreateCurveByType, OH_ArkUI_Curve_CreateCustomCurve,
    OH_ArkUI_Curve_CreateInterpolatingSpring, OH_ArkUI_Curve_CreateResponsiveSpringMotion,
    OH_ArkUI_Curve_CreateSpringCurve, OH_ArkUI_Curve_CreateSpringMotion,
    OH_ArkUI_Curve_CreateStepsCurve, OH_ArkUI_Curve_DisposeCurve,
};

use crate::{ArkUIError, ArkUIResult, Curve};

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
        let curve = create_custom_curve_raw(callback.as_ptr().cast());
        match curve {
            Ok(raw) => Ok(Self {
                raw,
                callback: Some(callback),
            }),
            Err(err) => {
                unsafe {
                    drop(Box::from_raw(callback.as_ptr()));
                }
                Err(err)
            }
        }
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
        dispose_curve(self.raw);
    }
}

pub(crate) fn create_curve_by_type(curve: Curve) -> ArkUIResult<NonNull<ArkUI_Curve>> {
    let handle = unsafe { OH_ArkUI_Curve_CreateCurveByType(curve.into()) };
    handle_or_error(handle, "OH_ArkUI_Curve_CreateCurveByType")
}

pub(crate) fn create_steps_curve(count: i32, end: bool) -> ArkUIResult<NonNull<ArkUI_Curve>> {
    let handle = unsafe { OH_ArkUI_Curve_CreateStepsCurve(count, end) };
    handle_or_error(handle, "OH_ArkUI_Curve_CreateStepsCurve")
}

pub(crate) fn create_cubic_bezier_curve(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
) -> ArkUIResult<NonNull<ArkUI_Curve>> {
    let handle = unsafe { OH_ArkUI_Curve_CreateCubicBezierCurve(x1, y1, x2, y2) };
    handle_or_error(handle, "OH_ArkUI_Curve_CreateCubicBezierCurve")
}

pub(crate) fn create_spring_curve(
    velocity: f32,
    mass: f32,
    stiffness: f32,
    damping: f32,
) -> ArkUIResult<NonNull<ArkUI_Curve>> {
    let handle = unsafe { OH_ArkUI_Curve_CreateSpringCurve(velocity, mass, stiffness, damping) };
    handle_or_error(handle, "OH_ArkUI_Curve_CreateSpringCurve")
}

pub(crate) fn create_spring_motion(
    response: f32,
    damping_fraction: f32,
    overlap_duration: f32,
) -> ArkUIResult<NonNull<ArkUI_Curve>> {
    let handle =
        unsafe { OH_ArkUI_Curve_CreateSpringMotion(response, damping_fraction, overlap_duration) };
    handle_or_error(handle, "OH_ArkUI_Curve_CreateSpringMotion")
}

pub(crate) fn create_responsive_spring_motion(
    response: f32,
    damping_fraction: f32,
    overlap_duration: f32,
) -> ArkUIResult<NonNull<ArkUI_Curve>> {
    let handle = unsafe {
        OH_ArkUI_Curve_CreateResponsiveSpringMotion(response, damping_fraction, overlap_duration)
    };
    handle_or_error(handle, "OH_ArkUI_Curve_CreateResponsiveSpringMotion")
}

pub(crate) fn create_interpolating_spring(
    velocity: f32,
    mass: f32,
    stiffness: f32,
    damping: f32,
) -> ArkUIResult<NonNull<ArkUI_Curve>> {
    let handle =
        unsafe { OH_ArkUI_Curve_CreateInterpolatingSpring(velocity, mass, stiffness, damping) };
    handle_or_error(handle, "OH_ArkUI_Curve_CreateInterpolatingSpring")
}

pub(crate) fn create_custom_curve<T: Fn(f32) -> f32 + 'static>(
    interpolate: T,
) -> ArkUIResult<CustomCurve> {
    CustomCurve::new(interpolate)
}

pub(crate) fn dispose_curve(curve: NonNull<ArkUI_Curve>) {
    unsafe { OH_ArkUI_Curve_DisposeCurve(curve.as_ptr()) }
}

unsafe extern "C" fn custom_curve_trampoline(input: f32, user_data: *mut c_void) -> f32 {
    if user_data.is_null() {
        return input;
    }
    let callback = unsafe { &*(user_data as *mut CustomCurveCallbackContext) };
    (callback.callback)(input)
}

fn create_custom_curve_raw(user_data: *mut c_void) -> ArkUIResult<NonNull<ArkUI_Curve>> {
    let handle = unsafe {
        OH_ArkUI_Curve_CreateCustomCurve(
            user_data,
            Some(custom_curve_trampoline as CustomCurveInterpolateCallback),
        )
    };
    handle_or_error(handle, "OH_ArkUI_Curve_CreateCustomCurve")
}

fn handle_or_error(
    curve: ArkUI_CurveHandle,
    func: &'static str,
) -> ArkUIResult<NonNull<ArkUI_Curve>> {
    if let Some(curve) = NonNull::new(curve) {
        Ok(curve)
    } else {
        Err(ArkUIError::new(
            ArkUIErrorCode::ParamInvalid,
            format!("{func} returned null"),
        ))
    }
}
