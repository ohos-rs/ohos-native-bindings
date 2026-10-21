/// Native text-decoration flags, combinable with `|` (API 11).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextDecoration(i32);

impl TextDecoration {
    /// No decoration.
    pub const NONE: Self =
        Self(ohos_native_drawing_sys::OH_Drawing_TextDecoration_TEXT_DECORATION_NONE as i32);
    /// Underline.
    pub const UNDERLINE: Self =
        Self(ohos_native_drawing_sys::OH_Drawing_TextDecoration_TEXT_DECORATION_UNDERLINE as i32);
    /// Overline.
    pub const OVERLINE: Self =
        Self(ohos_native_drawing_sys::OH_Drawing_TextDecoration_TEXT_DECORATION_OVERLINE as i32);
    /// Line through the text.
    pub const LINE_THROUGH: Self = Self(
        ohos_native_drawing_sys::OH_Drawing_TextDecoration_TEXT_DECORATION_LINE_THROUGH as i32,
    );

    /// Returns the SDK's decoration mask.
    pub const fn bits(self) -> i32 {
        self.0
    }
}

impl std::ops::BitOr for TextDecoration {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for TextDecoration {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
