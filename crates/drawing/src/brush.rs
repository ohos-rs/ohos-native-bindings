use std::ptr::NonNull;

use ohos_drawing_sys::{
    OH_Drawing_BlendMode, OH_Drawing_Brush, OH_Drawing_BrushCopy, OH_Drawing_BrushCreate,
    OH_Drawing_BrushDestroy, OH_Drawing_BrushGetAlpha, OH_Drawing_BrushGetColor,
    OH_Drawing_BrushIsAntiAlias, OH_Drawing_BrushReset, OH_Drawing_BrushSetAlpha,
    OH_Drawing_BrushSetAntiAlias, OH_Drawing_BrushSetBlendMode, OH_Drawing_BrushSetColor,
};

#[derive(Debug)]
pub struct Brush {
    raw: NonNull<OH_Drawing_Brush>,
}

impl Brush {
    pub fn new() -> Self {
        let raw = unsafe { OH_Drawing_BrushCreate() };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_BrushCreate returned null"),
        }
    }

    pub fn clone_brush(&self) -> Self {
        let raw = unsafe { OH_Drawing_BrushCopy(self.raw.as_ptr()) };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_BrushCopy returned null"),
        }
    }

    pub fn as_ptr(&self) -> *mut OH_Drawing_Brush {
        self.raw.as_ptr()
    }

    pub fn is_anti_alias(&self) -> bool {
        unsafe { OH_Drawing_BrushIsAntiAlias(self.raw.as_ptr()) }
    }

    pub fn set_anti_alias(&mut self, anti_alias: bool) {
        unsafe { OH_Drawing_BrushSetAntiAlias(self.raw.as_ptr(), anti_alias) };
    }

    pub fn color(&self) -> u32 {
        unsafe { OH_Drawing_BrushGetColor(self.raw.as_ptr()) }
    }

    pub fn set_color(&mut self, color: u32) {
        unsafe { OH_Drawing_BrushSetColor(self.raw.as_ptr(), color) };
    }

    pub fn alpha(&self) -> u8 {
        unsafe { OH_Drawing_BrushGetAlpha(self.raw.as_ptr()) }
    }

    pub fn set_alpha(&mut self, alpha: u8) {
        unsafe { OH_Drawing_BrushSetAlpha(self.raw.as_ptr(), alpha) };
    }

    pub fn set_blend_mode(&mut self, blend_mode: OH_Drawing_BlendMode) {
        unsafe { OH_Drawing_BrushSetBlendMode(self.raw.as_ptr(), blend_mode) };
    }

    pub fn reset(&mut self) {
        unsafe { OH_Drawing_BrushReset(self.raw.as_ptr()) };
    }
}

impl Default for Brush {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Brush {
    fn drop(&mut self) {
        unsafe { OH_Drawing_BrushDestroy(self.raw.as_ptr()) };
    }
}
