use ohos_native_drawing_sys::{
    OH_Drawing_GetBottomFromTextBox, OH_Drawing_GetLeftFromTextBox, OH_Drawing_GetRightFromTextBox,
    OH_Drawing_GetSizeOfTextBox, OH_Drawing_GetTextDirectionFromTextBox,
    OH_Drawing_GetTopFromTextBox, OH_Drawing_TextBox, OH_Drawing_TextDirection_TEXT_DIRECTION_RTL,
    OH_Drawing_TypographyDestroyTextBox,
};
use std::ptr::NonNull;

/// An owned visual range rectangle; it holds no native handles.
#[derive(Clone, Copy, Debug)]
pub struct RangeBox {
    /// Left, top, right and bottom in paragraph layout units.
    pub bounds: [f32; 4],
    /// Whether the visual run reads from right to left.
    pub rtl: bool,
}

pub(super) struct TextBoxes(NonNull<OH_Drawing_TextBox>);
impl TextBoxes {
    pub(super) fn collect(raw: *mut OH_Drawing_TextBox) -> Vec<RangeBox> {
        let Some(raw) = NonNull::new(raw) else {
            return Vec::new();
        };
        let _boxes = Self(raw);
        // SAFETY: native returned this owned array; getters only receive indices
        // within its count and the API-11 guard releases it after all copies.
        unsafe {
            let count = OH_Drawing_GetSizeOfTextBox(raw.as_ptr()).min(i32::MAX as usize);
            (0..count as i32)
                .map(|index| RangeBox {
                    bounds: [
                        OH_Drawing_GetLeftFromTextBox(raw.as_ptr(), index),
                        OH_Drawing_GetTopFromTextBox(raw.as_ptr(), index),
                        OH_Drawing_GetRightFromTextBox(raw.as_ptr(), index),
                        OH_Drawing_GetBottomFromTextBox(raw.as_ptr(), index),
                    ],
                    rtl: OH_Drawing_GetTextDirectionFromTextBox(raw.as_ptr(), index)
                        == OH_Drawing_TextDirection_TEXT_DIRECTION_RTL as i32,
                })
                .collect()
        }
    }
}
impl Drop for TextBoxes {
    fn drop(&mut self) {
        // SAFETY: this unique native array owner is released exactly once.
        unsafe { OH_Drawing_TypographyDestroyTextBox(self.0.as_ptr()) };
    }
}
