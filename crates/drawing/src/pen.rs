use std::ptr::NonNull;

use ohos_drawing_sys::{
    OH_Drawing_BlendMode, OH_Drawing_Pen, OH_Drawing_PenCopy, OH_Drawing_PenCreate,
    OH_Drawing_PenDestroy, OH_Drawing_PenGetAlpha, OH_Drawing_PenGetColor, OH_Drawing_PenGetWidth,
    OH_Drawing_PenIsAntiAlias, OH_Drawing_PenReset, OH_Drawing_PenSetAlpha,
    OH_Drawing_PenSetAntiAlias, OH_Drawing_PenSetBlendMode, OH_Drawing_PenSetColor,
    OH_Drawing_PenSetWidth,
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

    pub fn as_ptr(&self) -> *mut OH_Drawing_Pen {
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

    pub fn set_blend_mode(&mut self, blend_mode: OH_Drawing_BlendMode) {
        unsafe { OH_Drawing_PenSetBlendMode(self.raw.as_ptr(), blend_mode) };
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
