//! Module animate::curve::handle wrappers and related types.

use std::ptr::NonNull;

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::{
    ArkUI_Curve, ArkUI_CurveHandle as ArkUISysCurveHandle, OH_ArkUI_Curve_CreateCubicBezierCurve,
    OH_ArkUI_Curve_CreateCurveByType, OH_ArkUI_Curve_CreateInterpolatingSpring,
    OH_ArkUI_Curve_CreateResponsiveSpringMotion, OH_ArkUI_Curve_CreateSpringCurve,
    OH_ArkUI_Curve_CreateSpringMotion, OH_ArkUI_Curve_CreateStepsCurve,
    OH_ArkUI_Curve_DisposeCurve,
};

use crate::{ArkUIError, ArkUIResult, Curve};

use super::native;

enum CurveHandleOwner {
    System,
    Custom(native::CustomCurve),
}

/// Owning curve wrapper used by animation and transition options.
pub struct CurveHandle {
    raw: NonNull<ArkUI_Curve>,
    owner: Option<CurveHandleOwner>,
}

impl CurveHandle {
    pub fn from_curve_type(curve: Curve) -> ArkUIResult<Self> {
        let handle = unsafe { OH_ArkUI_Curve_CreateCurveByType(curve.into()) };
        let handle = NonNull::new(handle).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_Curve_CreateCurveByType returned null",
            )
        })?;
        Ok(Self::owned_system(handle))
    }

    pub fn steps(count: i32, end: bool) -> ArkUIResult<Self> {
        let handle = unsafe { OH_ArkUI_Curve_CreateStepsCurve(count, end) };
        let handle = NonNull::new(handle).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_Curve_CreateStepsCurve returned null",
            )
        })?;
        Ok(Self::owned_system(handle))
    }

    pub fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32) -> ArkUIResult<Self> {
        let handle = unsafe { OH_ArkUI_Curve_CreateCubicBezierCurve(x1, y1, x2, y2) };
        let handle = NonNull::new(handle).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_Curve_CreateCubicBezierCurve returned null",
            )
        })?;
        Ok(Self::owned_system(handle))
    }

    pub fn spring_curve(
        velocity: f32,
        mass: f32,
        stiffness: f32,
        damping: f32,
    ) -> ArkUIResult<Self> {
        let handle =
            unsafe { OH_ArkUI_Curve_CreateSpringCurve(velocity, mass, stiffness, damping) };
        let handle = NonNull::new(handle).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_Curve_CreateSpringCurve returned null",
            )
        })?;
        Ok(Self::owned_system(handle))
    }

    pub fn spring_motion(
        response: f32,
        damping_fraction: f32,
        overlap_duration: f32,
    ) -> ArkUIResult<Self> {
        let handle = unsafe {
            OH_ArkUI_Curve_CreateSpringMotion(response, damping_fraction, overlap_duration)
        };
        let handle = NonNull::new(handle).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_Curve_CreateSpringMotion returned null",
            )
        })?;
        Ok(Self::owned_system(handle))
    }

    pub fn responsive_spring_motion(
        response: f32,
        damping_fraction: f32,
        overlap_duration: f32,
    ) -> ArkUIResult<Self> {
        let handle = unsafe {
            OH_ArkUI_Curve_CreateResponsiveSpringMotion(
                response,
                damping_fraction,
                overlap_duration,
            )
        };
        let handle = NonNull::new(handle).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_Curve_CreateResponsiveSpringMotion returned null",
            )
        })?;
        Ok(Self::owned_system(handle))
    }

    pub fn interpolating_spring(
        velocity: f32,
        mass: f32,
        stiffness: f32,
        damping: f32,
    ) -> ArkUIResult<Self> {
        let handle =
            unsafe { OH_ArkUI_Curve_CreateInterpolatingSpring(velocity, mass, stiffness, damping) };
        let handle = NonNull::new(handle).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_Curve_CreateInterpolatingSpring returned null",
            )
        })?;
        Ok(Self::owned_system(handle))
    }

    pub fn custom<T: Fn(f32) -> f32 + 'static>(interpolate: T) -> ArkUIResult<Self> {
        let custom = native::CustomCurve::new(interpolate)?;
        Ok(Self {
            raw: custom.raw(),
            owner: Some(CurveHandleOwner::Custom(custom)),
        })
    }

    pub(crate) fn as_raw(&self) -> ArkUISysCurveHandle {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw_borrowed(raw: ArkUISysCurveHandle) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw, owner: None })
    }

    fn owned_system(raw: NonNull<ArkUI_Curve>) -> Self {
        Self {
            raw,
            owner: Some(CurveHandleOwner::System),
        }
    }

    fn dispose_curve(curve: NonNull<ArkUI_Curve>) {
        unsafe { OH_ArkUI_Curve_DisposeCurve(curve.as_ptr()) }
    }
}

impl Drop for CurveHandle {
    fn drop(&mut self) {
        match self.owner.take() {
            Some(CurveHandleOwner::System) => Self::dispose_curve(self.raw),
            Some(CurveHandleOwner::Custom(custom)) => custom.dispose(),
            None => {}
        }
    }
}
