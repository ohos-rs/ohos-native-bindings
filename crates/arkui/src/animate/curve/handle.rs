use std::ptr::NonNull;

use ohos_arkui_sys::{ArkUI_Curve, ArkUI_CurveHandle as ArkUISysCurveHandle};

use crate::{ArkUIResult, Curve};

use super::native;

enum CurveHandleOwner {
    System,
    Custom(native::CustomCurve),
}

pub struct CurveHandle {
    raw: NonNull<ArkUI_Curve>,
    owner: Option<CurveHandleOwner>,
}

impl CurveHandle {
    pub fn from_curve_type(curve: Curve) -> ArkUIResult<Self> {
        native::create_curve_by_type(curve).map(Self::owned_system)
    }

    pub fn steps(count: i32, end: bool) -> ArkUIResult<Self> {
        native::create_steps_curve(count, end).map(Self::owned_system)
    }

    pub fn cubic_bezier(x1: f32, y1: f32, x2: f32, y2: f32) -> ArkUIResult<Self> {
        native::create_cubic_bezier_curve(x1, y1, x2, y2).map(Self::owned_system)
    }

    pub fn spring_curve(
        velocity: f32,
        mass: f32,
        stiffness: f32,
        damping: f32,
    ) -> ArkUIResult<Self> {
        native::create_spring_curve(velocity, mass, stiffness, damping).map(Self::owned_system)
    }

    pub fn spring_motion(
        response: f32,
        damping_fraction: f32,
        overlap_duration: f32,
    ) -> ArkUIResult<Self> {
        native::create_spring_motion(response, damping_fraction, overlap_duration)
            .map(Self::owned_system)
    }

    pub fn responsive_spring_motion(
        response: f32,
        damping_fraction: f32,
        overlap_duration: f32,
    ) -> ArkUIResult<Self> {
        native::create_responsive_spring_motion(response, damping_fraction, overlap_duration)
            .map(Self::owned_system)
    }

    pub fn interpolating_spring(
        velocity: f32,
        mass: f32,
        stiffness: f32,
        damping: f32,
    ) -> ArkUIResult<Self> {
        native::create_interpolating_spring(velocity, mass, stiffness, damping)
            .map(Self::owned_system)
    }

    pub fn custom<T: Fn(f32) -> f32 + 'static>(interpolate: T) -> ArkUIResult<Self> {
        let custom = native::create_custom_curve(interpolate)?;
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
}

impl Drop for CurveHandle {
    fn drop(&mut self) {
        match self.owner.take() {
            Some(CurveHandleOwner::System) => native::dispose_curve(self.raw),
            Some(CurveHandleOwner::Custom(custom)) => custom.dispose(),
            None => {}
        }
    }
}
