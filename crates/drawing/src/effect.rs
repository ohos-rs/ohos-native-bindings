use std::ptr::NonNull;

#[cfg(feature = "api-20")]
use ohos_native_drawing_sys::OH_Drawing_ShaderEffectCreateSweepGradientWithLocalMatrix;
use ohos_native_drawing_sys::{
    OH_Drawing_CreateDashPathEffect, OH_Drawing_PathEffect, OH_Drawing_PathEffectDestroy,
    OH_Drawing_ShaderEffect, OH_Drawing_ShaderEffectCreateImageShader,
    OH_Drawing_ShaderEffectCreateLinearGradient,
    OH_Drawing_ShaderEffectCreateLinearGradientWithLocalMatrix,
    OH_Drawing_ShaderEffectCreateRadialGradient,
    OH_Drawing_ShaderEffectCreateRadialGradientWithLocalMatrix,
    OH_Drawing_ShaderEffectCreateSweepGradient,
    OH_Drawing_ShaderEffectCreateTwoPointConicalGradient, OH_Drawing_ShaderEffectDestroy,
    OH_Drawing_ShadowLayer, OH_Drawing_ShadowLayerCreate, OH_Drawing_ShadowLayerDestroy,
};

use crate::{Image, Matrix, Point, SamplingOptions, TileMode};

#[derive(Debug)]
pub struct PathEffect {
    raw: NonNull<OH_Drawing_PathEffect>,
}

impl PathEffect {
    pub fn dash(intervals: &[f32], phase: f32) -> Option<Self> {
        if intervals.is_empty() {
            return None;
        }
        let mut intervals = intervals.to_vec();
        let raw = unsafe {
            OH_Drawing_CreateDashPathEffect(intervals.as_mut_ptr(), intervals.len() as i32, phase)
        };
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) fn as_ptr(&self) -> *mut OH_Drawing_PathEffect {
        self.raw.as_ptr()
    }
}

impl Drop for PathEffect {
    fn drop(&mut self) {
        unsafe { OH_Drawing_PathEffectDestroy(self.raw.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct ShaderEffect {
    raw: NonNull<OH_Drawing_ShaderEffect>,
}

impl ShaderEffect {
    fn from_raw(raw: *mut OH_Drawing_ShaderEffect) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub fn linear_gradient(
        start: (f32, f32),
        end: (f32, f32),
        colors: &[u32],
        positions: &[f32],
        tile_mode: TileMode,
    ) -> Option<Self> {
        if colors.len() < 2 || colors.len() != positions.len() {
            return None;
        }
        let start = Point::new(start.0, start.1);
        let end = Point::new(end.0, end.1);
        Self::from_raw(unsafe {
            OH_Drawing_ShaderEffectCreateLinearGradient(
                start.as_ptr(),
                end.as_ptr(),
                colors.as_ptr(),
                positions.as_ptr(),
                colors.len() as u32,
                tile_mode.into(),
            )
        })
    }

    pub fn linear_gradient_with_local_matrix(
        start: (f32, f32),
        end: (f32, f32),
        colors: &[u32],
        positions: &[f32],
        tile_mode: TileMode,
        matrix: &Matrix,
    ) -> Option<Self> {
        if colors.len() < 2 || colors.len() != positions.len() {
            return None;
        }
        let start = ohos_native_drawing_sys::OH_Drawing_Point2D {
            x: start.0,
            y: start.1,
        };
        let end = ohos_native_drawing_sys::OH_Drawing_Point2D { x: end.0, y: end.1 };
        Self::from_raw(unsafe {
            OH_Drawing_ShaderEffectCreateLinearGradientWithLocalMatrix(
                &start,
                &end,
                colors.as_ptr(),
                positions.as_ptr(),
                colors.len() as u32,
                tile_mode.into(),
                matrix.as_ptr().cast_const(),
            )
        })
    }

    pub fn radial_gradient(
        center: (f32, f32),
        radius: f32,
        colors: &[u32],
        positions: &[f32],
        tile_mode: TileMode,
    ) -> Option<Self> {
        if colors.len() < 2 || colors.len() != positions.len() || radius < 0.0 {
            return None;
        }
        let center = Point::new(center.0, center.1);
        Self::from_raw(unsafe {
            OH_Drawing_ShaderEffectCreateRadialGradient(
                center.as_ptr(),
                radius,
                colors.as_ptr(),
                positions.as_ptr(),
                colors.len() as u32,
                tile_mode.into(),
            )
        })
    }

    pub fn radial_gradient_with_local_matrix(
        center: (f32, f32),
        radius: f32,
        colors: &[u32],
        positions: &[f32],
        tile_mode: TileMode,
        matrix: &Matrix,
    ) -> Option<Self> {
        if colors.len() < 2 || colors.len() != positions.len() || radius < 0.0 {
            return None;
        }
        let center = ohos_native_drawing_sys::OH_Drawing_Point2D {
            x: center.0,
            y: center.1,
        };
        Self::from_raw(unsafe {
            OH_Drawing_ShaderEffectCreateRadialGradientWithLocalMatrix(
                &center,
                radius,
                colors.as_ptr(),
                positions.as_ptr(),
                colors.len() as u32,
                tile_mode.into(),
                matrix.as_ptr().cast_const(),
            )
        })
    }

    pub fn conic_gradient(
        center: (f32, f32),
        colors: &[u32],
        positions: &[f32],
        tile_mode: TileMode,
    ) -> Option<Self> {
        if colors.len() < 2 || colors.len() != positions.len() {
            return None;
        }
        let center = Point::new(center.0, center.1);
        Self::from_raw(unsafe {
            OH_Drawing_ShaderEffectCreateSweepGradient(
                center.as_ptr(),
                colors.as_ptr(),
                positions.as_ptr(),
                colors.len() as u32,
                tile_mode.into(),
            )
        })
    }

    #[cfg(feature = "api-20")]
    pub fn conic_gradient_with_local_matrix(
        center: (f32, f32),
        colors: &[u32],
        positions: &[f32],
        tile_mode: TileMode,
        matrix: &Matrix,
    ) -> Option<Self> {
        if colors.len() < 2 || colors.len() != positions.len() {
            return None;
        }
        let center = Point::new(center.0, center.1);
        Self::from_raw(unsafe {
            OH_Drawing_ShaderEffectCreateSweepGradientWithLocalMatrix(
                center.as_ptr(),
                colors.as_ptr(),
                positions.as_ptr(),
                colors.len() as u32,
                tile_mode.into(),
                matrix.as_ptr().cast_const(),
            )
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn two_point_conical_gradient(
        start: (f32, f32),
        start_radius: f32,
        end: (f32, f32),
        end_radius: f32,
        colors: &[u32],
        positions: &[f32],
        tile_mode: TileMode,
        matrix: Option<&Matrix>,
    ) -> Option<Self> {
        if colors.len() < 2
            || colors.len() != positions.len()
            || start_radius < 0.0
            || end_radius < 0.0
        {
            return None;
        }
        let start = ohos_native_drawing_sys::OH_Drawing_Point2D {
            x: start.0,
            y: start.1,
        };
        let end = ohos_native_drawing_sys::OH_Drawing_Point2D { x: end.0, y: end.1 };
        Self::from_raw(unsafe {
            OH_Drawing_ShaderEffectCreateTwoPointConicalGradient(
                &start,
                start_radius,
                &end,
                end_radius,
                colors.as_ptr(),
                positions.as_ptr(),
                colors.len() as u32,
                tile_mode.into(),
                matrix.map_or(std::ptr::null(), |matrix| matrix.as_ptr().cast_const()),
            )
        })
    }

    pub fn image(
        image: &Image<'_>,
        tile_x: TileMode,
        tile_y: TileMode,
        sampling: &SamplingOptions,
        matrix: Option<&Matrix>,
    ) -> Option<Self> {
        Self::from_raw(unsafe {
            OH_Drawing_ShaderEffectCreateImageShader(
                image.as_ptr(),
                tile_x.into(),
                tile_y.into(),
                sampling.as_ptr(),
                matrix.map_or(std::ptr::null(), |matrix| matrix.as_ptr().cast_const()),
            )
        })
    }

    pub(crate) fn as_ptr(&self) -> *mut OH_Drawing_ShaderEffect {
        self.raw.as_ptr()
    }
}

impl Drop for ShaderEffect {
    fn drop(&mut self) {
        unsafe { OH_Drawing_ShaderEffectDestroy(self.raw.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct ShadowLayer {
    raw: NonNull<OH_Drawing_ShadowLayer>,
}

impl ShadowLayer {
    pub fn new(blur_radius: f32, offset_x: f32, offset_y: f32, color: u32) -> Option<Self> {
        let raw = unsafe { OH_Drawing_ShadowLayerCreate(blur_radius, offset_x, offset_y, color) };
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub(crate) fn as_ptr(&self) -> *mut OH_Drawing_ShadowLayer {
        self.raw.as_ptr()
    }
}

impl Drop for ShadowLayer {
    fn drop(&mut self) {
        unsafe { OH_Drawing_ShadowLayerDestroy(self.raw.as_ptr()) };
    }
}
