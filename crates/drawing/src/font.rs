use std::{ffi::CString, ptr::NonNull};

use ohos_drawing_sys::{
    OH_Drawing_Font, OH_Drawing_FontCountText, OH_Drawing_FontCreate, OH_Drawing_FontDestroy,
    OH_Drawing_FontGetTextSize, OH_Drawing_FontMeasureText, OH_Drawing_FontSetTextSize,
    OH_Drawing_TextEncoding,
};

use crate::{check_error, Rect, Result};

#[derive(Debug)]
pub struct Font {
    raw: NonNull<OH_Drawing_Font>,
}

#[derive(Debug)]
pub struct TextMeasure {
    pub width: f32,
    pub bounds: Rect,
}

impl Font {
    pub fn new() -> Self {
        let raw = unsafe { OH_Drawing_FontCreate() };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_FontCreate returned null"),
        }
    }

    pub fn as_ptr(&self) -> *mut OH_Drawing_Font {
        self.raw.as_ptr()
    }

    pub fn set_text_size(&mut self, text_size: f32) {
        unsafe { OH_Drawing_FontSetTextSize(self.raw.as_ptr(), text_size) };
    }

    pub fn text_size(&self) -> f32 {
        unsafe { OH_Drawing_FontGetTextSize(self.raw.as_ptr()) }
    }

    pub fn count_text(&self, text: &str, encoding: OH_Drawing_TextEncoding) -> i32 {
        unsafe {
            OH_Drawing_FontCountText(
                self.raw.as_ptr(),
                text.as_ptr().cast(),
                text.len(),
                encoding,
            )
        }
    }

    pub fn measure_text(
        &self,
        text: &str,
        encoding: OH_Drawing_TextEncoding,
    ) -> Result<TextMeasure> {
        let bounds = Rect::new(0.0, 0.0, 0.0, 0.0);
        let mut width = 0.0;
        let code = unsafe {
            OH_Drawing_FontMeasureText(
                self.raw.as_ptr(),
                text.as_ptr().cast(),
                text.len(),
                encoding,
                bounds.as_ptr(),
                &mut width,
            )
        };
        check_error(code)?;
        Ok(TextMeasure { width, bounds })
    }

    pub fn measure_single_character(&self, ch: &str) -> Result<f32> {
        let c = CString::new(ch).expect("character contains interior NUL");
        let mut width = 0.0;
        let code = unsafe {
            ohos_drawing_sys::OH_Drawing_FontMeasureSingleCharacter(
                self.raw.as_ptr(),
                c.as_ptr(),
                &mut width,
            )
        };
        check_error(code)?;
        Ok(width)
    }
}

impl Default for Font {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe { OH_Drawing_FontDestroy(self.raw.as_ptr()) };
    }
}
