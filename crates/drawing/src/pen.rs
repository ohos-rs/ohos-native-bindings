use std::ptr::NonNull;

use ohos_native_drawing_sys::{
    OH_Drawing_Pen, OH_Drawing_PenCopy, OH_Drawing_PenCreate, OH_Drawing_PenDestroy,
    OH_Drawing_PenGetAlpha, OH_Drawing_PenGetCap, OH_Drawing_PenGetColor,
    OH_Drawing_PenGetFillPath, OH_Drawing_PenGetJoin, OH_Drawing_PenGetMiterLimit,
    OH_Drawing_PenGetWidth, OH_Drawing_PenIsAntiAlias, OH_Drawing_PenReset, OH_Drawing_PenSetAlpha,
    OH_Drawing_PenSetAntiAlias, OH_Drawing_PenSetBlendMode, OH_Drawing_PenSetCap,
    OH_Drawing_PenSetColor, OH_Drawing_PenSetFilter, OH_Drawing_PenSetJoin,
    OH_Drawing_PenSetMiterLimit, OH_Drawing_PenSetPathEffect, OH_Drawing_PenSetShaderEffect,
    OH_Drawing_PenSetShadowLayer, OH_Drawing_PenSetWidth,
};

use crate::{
    BlendMode, Filter, LineCap, LineJoin, Matrix, Path, PathEffect, Rect, ShaderEffect, ShadowLayer,
};

#[derive(Debug)]
pub struct Pen {
    raw: NonNull<OH_Drawing_Pen>,
}

impl Pen {
    pub fn new() -> Self {
        let raw = unsafe { OH_Drawing_PenCreate() };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_PenCreate returned null"),
        }
    }

    pub fn clone_pen(&self) -> Self {
        let raw = unsafe { OH_Drawing_PenCopy(self.raw.as_ptr()) };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_PenCopy returned null"),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut OH_Drawing_Pen {
        self.raw.as_ptr()
    }

    pub fn is_anti_alias(&self) -> bool {
        unsafe { OH_Drawing_PenIsAntiAlias(self.raw.as_ptr()) }
    }

    pub fn set_anti_alias(&mut self, anti_alias: bool) {
        unsafe { OH_Drawing_PenSetAntiAlias(self.raw.as_ptr(), anti_alias) };
    }

    pub fn color(&self) -> u32 {
        unsafe { OH_Drawing_PenGetColor(self.raw.as_ptr()) }
    }

    pub fn set_color(&mut self, color: u32) {
        unsafe { OH_Drawing_PenSetColor(self.raw.as_ptr(), color) };
    }

    pub fn alpha(&self) -> u8 {
        unsafe { OH_Drawing_PenGetAlpha(self.raw.as_ptr()) }
    }

    pub fn set_alpha(&mut self, alpha: u8) {
        unsafe { OH_Drawing_PenSetAlpha(self.raw.as_ptr(), alpha) };
    }

    pub fn width(&self) -> f32 {
        unsafe { OH_Drawing_PenGetWidth(self.raw.as_ptr()) }
    }

    pub fn set_width(&mut self, width: f32) {
        unsafe { OH_Drawing_PenSetWidth(self.raw.as_ptr(), width) };
    }

    pub fn miter_limit(&self) -> f32 {
        unsafe { OH_Drawing_PenGetMiterLimit(self.raw.as_ptr()) }
    }

    pub fn set_miter_limit(&mut self, miter_limit: f32) {
        unsafe { OH_Drawing_PenSetMiterLimit(self.raw.as_ptr(), miter_limit) };
    }

    pub fn cap(&self) -> LineCap {
        unsafe { OH_Drawing_PenGetCap(self.raw.as_ptr()) }.into()
    }

    pub fn set_cap(&mut self, cap: LineCap) {
        unsafe { OH_Drawing_PenSetCap(self.raw.as_ptr(), cap.into()) };
    }

    pub fn join(&self) -> LineJoin {
        unsafe { OH_Drawing_PenGetJoin(self.raw.as_ptr()) }.into()
    }

    pub fn set_join(&mut self, join: LineJoin) {
        unsafe { OH_Drawing_PenSetJoin(self.raw.as_ptr(), join.into()) };
    }

    pub fn set_blend_mode(&mut self, blend_mode: BlendMode) {
        unsafe { OH_Drawing_PenSetBlendMode(self.raw.as_ptr(), blend_mode.into()) };
    }

    pub fn set_shader_effect(&mut self, shader: Option<&ShaderEffect>) {
        unsafe {
            OH_Drawing_PenSetShaderEffect(
                self.raw.as_ptr(),
                shader.map_or(std::ptr::null_mut(), ShaderEffect::as_ptr),
            )
        };
    }

    pub fn set_shadow_layer(&mut self, shadow: Option<&ShadowLayer>) {
        unsafe {
            OH_Drawing_PenSetShadowLayer(
                self.raw.as_ptr(),
                shadow.map_or(std::ptr::null_mut(), ShadowLayer::as_ptr),
            )
        };
    }

    pub fn set_path_effect(&mut self, effect: Option<&PathEffect>) {
        unsafe {
            OH_Drawing_PenSetPathEffect(
                self.raw.as_ptr(),
                effect.map_or(std::ptr::null_mut(), PathEffect::as_ptr),
            )
        };
    }

    pub fn set_filter(&mut self, filter: Option<&Filter>) {
        unsafe {
            OH_Drawing_PenSetFilter(
                self.raw.as_ptr(),
                filter.map_or(std::ptr::null_mut(), Filter::as_ptr),
            )
        };
    }

    pub fn fill_path(
        &self,
        source: &Path,
        destination: &mut Path,
        bounds: Option<&Rect>,
        transform: Option<&Matrix>,
    ) -> bool {
        unsafe {
            OH_Drawing_PenGetFillPath(
                self.raw.as_ptr(),
                source.as_ptr(),
                destination.as_ptr(),
                bounds.map_or(std::ptr::null(), |bounds| bounds.as_ptr().cast_const()),
                transform.map_or(std::ptr::null(), |transform| {
                    transform.as_ptr().cast_const()
                }),
            )
        }
    }

    pub fn reset(&mut self) {
        unsafe { OH_Drawing_PenReset(self.raw.as_ptr()) };
    }
}

impl Default for Pen {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Pen {
    fn drop(&mut self) {
        unsafe { OH_Drawing_PenDestroy(self.raw.as_ptr()) };
    }
}
