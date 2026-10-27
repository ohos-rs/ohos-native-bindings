use std::ptr::NonNull;

use ohos_native_drawing_sys::{
    OH_Drawing_Point2D, OH_Drawing_TextBlob, OH_Drawing_TextBlobBuilder,
    OH_Drawing_TextBlobBuilderAllocRunPos, OH_Drawing_TextBlobBuilderCreate,
    OH_Drawing_TextBlobBuilderDestroy, OH_Drawing_TextBlobBuilderMake,
    OH_Drawing_TextBlobCreateFromPosText, OH_Drawing_TextBlobCreateFromText,
    OH_Drawing_TextBlobDestroy, OH_Drawing_TextBlobGetBounds, OH_Drawing_TextBlobUniqueID,
};

use crate::{Font, Rect, TextEncoding};

/// Immutable glyph run created from encoded text and a native font.
#[derive(Debug)]
pub struct TextBlob {
    raw: NonNull<OH_Drawing_TextBlob>,
}

impl TextBlob {
    pub fn from_text(text: &[u8], font: &Font, encoding: TextEncoding) -> Option<Self> {
        if text.is_empty() {
            return None;
        }
        let raw = unsafe {
            OH_Drawing_TextBlobCreateFromText(
                text.as_ptr().cast(),
                text.len(),
                font.as_ptr(),
                encoding.into(),
            )
        };
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub fn from_utf8(text: &str, font: &Font) -> Option<Self> {
        Self::from_text(text.as_bytes(), font, TextEncoding::Utf8)
    }

    pub fn from_pos_text(
        text: &[u8],
        positions: &[(f32, f32)],
        font: &Font,
        encoding: TextEncoding,
    ) -> Option<Self> {
        if text.is_empty() || positions.is_empty() {
            return None;
        }
        let mut points: Vec<OH_Drawing_Point2D> = positions
            .iter()
            .map(|&(x, y)| OH_Drawing_Point2D { x, y })
            .collect();
        let raw = unsafe {
            OH_Drawing_TextBlobCreateFromPosText(
                text.as_ptr().cast(),
                text.len(),
                points.as_mut_ptr(),
                font.as_ptr(),
                encoding.into(),
            )
        };
        NonNull::new(raw).map(|raw| Self { raw })
    }

    /// Creates a `TextBlob` from positioned glyph IDs using `TextBlobBuilder`.
    pub fn from_glyph_positions(
        glyphs: &[u16],
        positions: &[(f32, f32)],
        font: &Font,
        bounds: Option<&Rect>,
    ) -> Option<Self> {
        let mut builder = TextBlobBuilder::new()?;
        if !builder.alloc_run_pos(font, glyphs, positions, bounds) {
            return None;
        }
        builder.build()
    }

    pub fn bounds(&self) -> Rect {
        let rect = Rect::new(0.0, 0.0, 0.0, 0.0);
        unsafe { OH_Drawing_TextBlobGetBounds(self.raw.as_ptr(), rect.as_ptr()) };
        rect
    }

    pub fn unique_id(&self) -> u32 {
        unsafe { OH_Drawing_TextBlobUniqueID(self.raw.as_ptr()) }
    }

    pub(crate) fn as_ptr(&self) -> *mut OH_Drawing_TextBlob {
        self.raw.as_ptr()
    }
}

impl Drop for TextBlob {
    fn drop(&mut self) {
        unsafe { OH_Drawing_TextBlobDestroy(self.raw.as_ptr()) };
    }
}

/// Builder for positioned glyph runs (`OH_Drawing_TextBlobBuilder`).
#[derive(Debug)]
pub struct TextBlobBuilder {
    raw: NonNull<OH_Drawing_TextBlobBuilder>,
}

impl TextBlobBuilder {
    pub fn new() -> Option<Self> {
        let raw = unsafe { OH_Drawing_TextBlobBuilderCreate() };
        NonNull::new(raw).map(|raw| Self { raw })
    }

    /// Allocates and populates a positioned glyph run (`OH_Drawing_TextBlobBuilderAllocRunPos`).
    pub fn alloc_run_pos(
        &mut self,
        font: &Font,
        glyphs: &[u16],
        positions: &[(f32, f32)],
        bounds: Option<&Rect>,
    ) -> bool {
        if glyphs.is_empty() || glyphs.len() != positions.len() || glyphs.len() > i32::MAX as usize
        {
            return false;
        }
        let count = glyphs.len();
        let rect_ptr = bounds.map_or(std::ptr::null(), |rect| rect.as_ptr().cast_const());
        unsafe {
            let run_buf = OH_Drawing_TextBlobBuilderAllocRunPos(
                self.raw.as_ptr(),
                font.as_ptr(),
                count as i32,
                rect_ptr,
            );
            if run_buf.is_null() {
                return false;
            }
            let buf = &*run_buf;
            if buf.glyphs.is_null() || buf.pos.is_null() {
                return false;
            }
            std::ptr::copy_nonoverlapping(glyphs.as_ptr(), buf.glyphs, count);
            let out_pos = std::slice::from_raw_parts_mut(buf.pos, count * 2);
            for (index, &(x, y)) in positions.iter().enumerate() {
                out_pos[index * 2] = x;
                out_pos[index * 2 + 1] = y;
            }
            true
        }
    }

    pub fn build(self) -> Option<TextBlob> {
        let raw = unsafe { OH_Drawing_TextBlobBuilderMake(self.raw.as_ptr()) };
        NonNull::new(raw).map(|raw| TextBlob { raw })
    }
}

impl Drop for TextBlobBuilder {
    fn drop(&mut self) {
        unsafe { OH_Drawing_TextBlobBuilderDestroy(self.raw.as_ptr()) };
    }
}
