use std::ptr::NonNull;

use ohos_native_drawing_sys::{
    OH_Drawing_TextBlob, OH_Drawing_TextBlobCreateFromText, OH_Drawing_TextBlobDestroy,
};

use crate::{Font, TextEncoding};

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

    pub(crate) fn as_ptr(&self) -> *mut OH_Drawing_TextBlob {
        self.raw.as_ptr()
    }
}

impl Drop for TextBlob {
    fn drop(&mut self) {
        unsafe { OH_Drawing_TextBlobDestroy(self.raw.as_ptr()) };
    }
}
