use std::{
    ffi::CString,
    os::raw::{c_char, c_void},
    ptr::NonNull,
};

use ohos_enum_derive::EnumFrom;
#[cfg(feature = "api-14")]
use ohos_native_drawing_sys::OH_Drawing_GetFontCollectionGlobalInstance;
#[cfg(not(feature = "api-20"))]
use ohos_native_drawing_sys::OH_Drawing_TypographyHandlerAddText;
#[cfg(feature = "api-18")]
use ohos_native_drawing_sys::{
    OH_Drawing_Array, OH_Drawing_DestroyRunGlyphs, OH_Drawing_DestroyRunPositions,
    OH_Drawing_DestroyRunStringIndices, OH_Drawing_DestroyRuns, OH_Drawing_DestroyTextLines,
    OH_Drawing_GetDrawingArraySize, OH_Drawing_GetRunByIndex, OH_Drawing_GetRunGlyphCount,
    OH_Drawing_GetRunGlyphs, OH_Drawing_GetRunGlyphsByIndex, OH_Drawing_GetRunPositions,
    OH_Drawing_GetRunPositionsByIndex, OH_Drawing_GetRunStringIndices,
    OH_Drawing_GetRunStringIndicesByIndex, OH_Drawing_GetRunStringRange,
    OH_Drawing_GetRunTypographicBounds, OH_Drawing_GetTextLineByIndex, OH_Drawing_PointGetX,
    OH_Drawing_PointGetY, OH_Drawing_Run, OH_Drawing_RunPaint, OH_Drawing_TextLine,
    OH_Drawing_TextLineGetAlignmentOffset, OH_Drawing_TextLineGetGlyphCount,
    OH_Drawing_TextLineGetGlyphRuns, OH_Drawing_TextLineGetOffsetForStringIndex,
    OH_Drawing_TextLineGetStringIndexForPosition, OH_Drawing_TextLineGetTextRange,
    OH_Drawing_TextLineGetTrailingSpaceWidth, OH_Drawing_TextLineGetTypographicBounds,
    OH_Drawing_TextLinePaint, OH_Drawing_TypographyGetTextLines,
};
use ohos_native_drawing_sys::{
    OH_Drawing_ClearFontCaches, OH_Drawing_CreateFontCollection,
    OH_Drawing_CreateSharedFontCollection, OH_Drawing_CreateTextShadow, OH_Drawing_CreateTextStyle,
    OH_Drawing_CreateTypography, OH_Drawing_CreateTypographyHandler,
    OH_Drawing_CreateTypographyStyle, OH_Drawing_DestroyFontCollection,
    OH_Drawing_DestroyTextShadow, OH_Drawing_DestroyTextStyle, OH_Drawing_DestroyTypography,
    OH_Drawing_DestroyTypographyHandler, OH_Drawing_DestroyTypographyStyle,
    OH_Drawing_DisableFontCollectionFallback, OH_Drawing_DisableFontCollectionSystemFont,
    OH_Drawing_FontCollection, OH_Drawing_Font_Metrics,
    OH_Drawing_GetAffinityFromPositionAndAffinity, OH_Drawing_GetBottomFromTextBox,
    OH_Drawing_GetLeftFromTextBox, OH_Drawing_GetPositionFromPositionAndAffinity,
    OH_Drawing_GetRightFromTextBox, OH_Drawing_GetSizeOfTextBox,
    OH_Drawing_GetTextDirectionFromTextBox, OH_Drawing_GetTopFromTextBox, OH_Drawing_LineMetrics,
    OH_Drawing_PlaceholderSpan, OH_Drawing_PositionAndAffinity, OH_Drawing_RectHeightStyle,
    OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_INCLUDELINESPACEBOTTOM,
    OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_INCLUDELINESPACEMIDDLE,
    OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_INCLUDELINESPACETOP,
    OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_MAX,
    OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_STRUCT,
    OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_TIGHT, OH_Drawing_RectWidthStyle,
    OH_Drawing_RectWidthStyle_RECT_WIDTH_STYLE_MAX,
    OH_Drawing_RectWidthStyle_RECT_WIDTH_STYLE_TIGHT, OH_Drawing_RegisterFont,
    OH_Drawing_RegisterFontBuffer, OH_Drawing_SetTextShadow, OH_Drawing_SetTextStyleColor,
    OH_Drawing_SetTextStyleDecoration, OH_Drawing_SetTextStyleDecorationColor,
    OH_Drawing_SetTextStyleDecorationStyle, OH_Drawing_SetTextStyleFontFamilies,
    OH_Drawing_SetTextStyleFontHeight, OH_Drawing_SetTextStyleFontSize,
    OH_Drawing_SetTextStyleFontStyleStruct, OH_Drawing_SetTextStyleForegroundBrush,
    OH_Drawing_SetTextStyleForegroundPen, OH_Drawing_SetTextStyleLetterSpacing,
    OH_Drawing_SetTextStyleLocale, OH_Drawing_SetTextStyleWordSpacing,
    OH_Drawing_SetTypographyTextAlign, OH_Drawing_SetTypographyTextDirection,
    OH_Drawing_SetTypographyTextEllipsis, OH_Drawing_SetTypographyTextEllipsisModal,
    OH_Drawing_SetTypographyTextMaxLines, OH_Drawing_TextBox,
    OH_Drawing_TextDecoration_TEXT_DECORATION_LINE_THROUGH,
    OH_Drawing_TextDecoration_TEXT_DECORATION_NONE,
    OH_Drawing_TextDecoration_TEXT_DECORATION_OVERLINE,
    OH_Drawing_TextDecoration_TEXT_DECORATION_UNDERLINE,
    OH_Drawing_TextDirection_TEXT_DIRECTION_RTL, OH_Drawing_TextShadow, OH_Drawing_TextStyle,
    OH_Drawing_TextStyleAddFontFeature, OH_Drawing_TextStyleAddFontVariation,
    OH_Drawing_TextStyleAddShadow, OH_Drawing_TextStyleGetFontMetrics, OH_Drawing_Typography,
    OH_Drawing_TypographyCreate, OH_Drawing_TypographyDestroyTextBox,
    OH_Drawing_TypographyGetHeight, OH_Drawing_TypographyGetLineCount,
    OH_Drawing_TypographyGetLineMetricsAt, OH_Drawing_TypographyGetLongestLine,
    OH_Drawing_TypographyGetMaxWidth, OH_Drawing_TypographyGetRectsForPlaceholders,
    OH_Drawing_TypographyGetRectsForRange, OH_Drawing_TypographyHandlerAddPlaceholder,
    OH_Drawing_TypographyHandlerPopTextStyle, OH_Drawing_TypographyHandlerPushTextStyle,
    OH_Drawing_TypographyLayout, OH_Drawing_TypographyPaint, OH_Drawing_TypographyStyle,
};
#[cfg(feature = "api-20")]
use ohos_native_drawing_sys::{
    OH_Drawing_DestroyRunGlyphAdvances, OH_Drawing_GetRunFont,
    OH_Drawing_GetRunGlyphAdvanceByIndex, OH_Drawing_GetRunGlyphAdvances,
    OH_Drawing_GetRunTextDirection, OH_Drawing_TypographyHandlerAddEncodedText,
    OH_Drawing_UnregisterFont,
};

use crate::{
    check_error, Brush, Canvas, DecorationPattern, DrawingError, EllipsisMode, FontStyle, Pen,
    Point, Result, TextAlign, TextDirection,
};
#[cfg(feature = "api-20")]
use crate::{Font, TextEncoding};

#[derive(Debug)]
pub struct FontCollection {
    raw: NonNull<OH_Drawing_FontCollection>,
    owned: bool,
}

impl FontCollection {
    /// Creates a collection for one typography builder. Use [`Self::shared`]
    /// when preparing more than one paragraph with the same collection.
    pub fn new() -> Self {
        let raw = unsafe { OH_Drawing_CreateFontCollection() };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_CreateFontCollection returned null"),
            owned: true,
        }
    }

    /// Creates a collection that can be reused by multiple typography builders.
    /// This does not make the wrapper safe to share between threads.
    pub fn shared() -> Self {
        let raw = unsafe { OH_Drawing_CreateSharedFontCollection() };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_CreateSharedFontCollection returned null"),
            owned: true,
        }
    }

    #[cfg(feature = "api-14")]
    pub fn global_instance() -> Option<Self> {
        let raw = unsafe { OH_Drawing_GetFontCollectionGlobalInstance() };
        NonNull::new(raw).map(|raw| Self { raw, owned: false })
    }

    pub fn as_ptr(&self) -> *mut OH_Drawing_FontCollection {
        self.raw.as_ptr()
    }

    #[allow(deprecated)]
    pub fn disable_fallback(&mut self) {
        unsafe { OH_Drawing_DisableFontCollectionFallback(self.raw.as_ptr()) };
    }

    pub fn disable_system_font(&mut self) {
        unsafe { OH_Drawing_DisableFontCollectionSystemFont(self.raw.as_ptr()) };
    }

    pub fn clear_caches(&mut self) {
        unsafe { OH_Drawing_ClearFontCaches(self.raw.as_ptr()) };
    }

    pub fn register_font(&mut self, family: &str, src_path: &str) -> Result<()> {
        let family = CString::new(family).expect("font family contains interior NUL");
        let src_path = CString::new(src_path).expect("font path contains interior NUL");
        let code = unsafe {
            OH_Drawing_RegisterFont(self.raw.as_ptr(), family.as_ptr(), src_path.as_ptr())
        };
        check_error(code)
    }

    pub fn register_font_buffer(&mut self, family: &str, font_buffer: &mut [u8]) -> Result<()> {
        let family = CString::new(family).expect("font family contains interior NUL");
        let code = unsafe {
            OH_Drawing_RegisterFontBuffer(
                self.raw.as_ptr(),
                family.as_ptr(),
                font_buffer.as_mut_ptr(),
                font_buffer.len(),
            )
        };
        check_error(code)
    }

    #[cfg(feature = "api-20")]
    pub fn unregister_font(&mut self, family: &str) -> Result<()> {
        let family = CString::new(family).expect("font family contains interior NUL");
        let code = unsafe { OH_Drawing_UnregisterFont(self.raw.as_ptr(), family.as_ptr()) };
        check_error(code)
    }
}

impl Default for FontCollection {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for FontCollection {
    fn drop(&mut self) {
        if self.owned {
            unsafe { OH_Drawing_DestroyFontCollection(self.raw.as_ptr()) };
        }
    }
}

#[derive(Debug)]
pub struct TypographyStyle {
    raw: NonNull<OH_Drawing_TypographyStyle>,
}

impl TypographyStyle {
    pub fn new() -> Self {
        let raw = unsafe { OH_Drawing_CreateTypographyStyle() };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_CreateTypographyStyle returned null"),
        }
    }

    pub fn as_ptr(&self) -> *mut OH_Drawing_TypographyStyle {
        self.raw.as_ptr()
    }

    pub fn set_text_align(&mut self, align: TextAlign) {
        unsafe { OH_Drawing_SetTypographyTextAlign(self.raw.as_ptr(), u32::from(align) as i32) };
    }

    pub fn set_text_direction(&mut self, direction: TextDirection) {
        unsafe {
            OH_Drawing_SetTypographyTextDirection(self.raw.as_ptr(), u32::from(direction) as i32)
        };
    }

    pub fn set_max_lines(&mut self, line_number: i32) {
        unsafe { OH_Drawing_SetTypographyTextMaxLines(self.raw.as_ptr(), line_number) };
    }

    /// Sets the ellipsis glyph and its placement (head, middle or tail).
    pub fn set_ellipsis(&mut self, text: &str, mode: EllipsisMode) -> Result<()> {
        let text = CString::new(text).map_err(|_| DrawingError::invalid_parameter())?;
        // SAFETY: the style is owned and native copies the temporary C string.
        unsafe {
            OH_Drawing_SetTypographyTextEllipsis(self.raw.as_ptr(), text.as_ptr());
            OH_Drawing_SetTypographyTextEllipsisModal(self.raw.as_ptr(), mode as i32);
        }
        Ok(())
    }
}

impl Default for TypographyStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TypographyStyle {
    fn drop(&mut self) {
        unsafe { OH_Drawing_DestroyTypographyStyle(self.raw.as_ptr()) };
    }
}

/// Native text-decoration flags, combinable with `|` (API 11).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TextDecoration(i32);

impl TextDecoration {
    /// No decoration.
    pub const NONE: Self = Self(OH_Drawing_TextDecoration_TEXT_DECORATION_NONE as i32);
    /// Underline.
    pub const UNDERLINE: Self = Self(OH_Drawing_TextDecoration_TEXT_DECORATION_UNDERLINE as i32);
    /// Overline.
    pub const OVERLINE: Self = Self(OH_Drawing_TextDecoration_TEXT_DECORATION_OVERLINE as i32);
    /// Line through the text.
    pub const LINE_THROUGH: Self =
        Self(OH_Drawing_TextDecoration_TEXT_DECORATION_LINE_THROUGH as i32);

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

#[derive(Debug)]
pub struct TextStyle {
    raw: NonNull<OH_Drawing_TextStyle>,
}

impl TextStyle {
    pub fn new() -> Self {
        let raw = unsafe { OH_Drawing_CreateTextStyle() };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_CreateTextStyle returned null"),
        }
    }

    pub fn as_ptr(&self) -> *mut OH_Drawing_TextStyle {
        self.raw.as_ptr()
    }

    pub fn set_color(&mut self, color: u32) {
        unsafe { OH_Drawing_SetTextStyleColor(self.raw.as_ptr(), color) };
    }

    /// Sets the caller's native decoration mask, color and pattern.
    pub fn set_decoration(
        &mut self,
        decoration: TextDecoration,
        color: u32,
        pattern: DecorationPattern,
    ) {
        // SAFETY: the style is owned; these functions copy scalar values.
        unsafe {
            OH_Drawing_SetTextStyleDecoration(self.raw.as_ptr(), decoration.bits());
            OH_Drawing_SetTextStyleDecorationColor(self.raw.as_ptr(), color);
            OH_Drawing_SetTextStyleDecorationStyle(self.raw.as_ptr(), pattern as i32);
        }
    }

    pub fn set_font_size(&mut self, font_size: f64) {
        unsafe { OH_Drawing_SetTextStyleFontSize(self.raw.as_ptr(), font_size) };
    }

    /// Sets line height as a multiple of font size. Zero restores font metrics.
    /// Non-finite and negative multipliers are rejected before calling native code.
    pub fn set_font_height(&mut self, multiplier: f64) -> Result<()> {
        if !multiplier.is_finite() || multiplier < 0.0 {
            return Err(DrawingError::invalid_parameter());
        }
        // SAFETY: this wrapper owns the live native style for this call.
        unsafe { OH_Drawing_SetTextStyleFontHeight(self.raw.as_ptr(), multiplier) };
        Ok(())
    }

    pub fn set_font_style(&mut self, style: FontStyle) {
        unsafe { OH_Drawing_SetTextStyleFontStyleStruct(self.raw.as_ptr(), style.into_raw()) };
    }

    pub fn set_font_families(&mut self, families: &[&str]) {
        let c_families: Vec<CString> = families
            .iter()
            .map(|s| CString::new(*s).expect("font family contains interior NUL"))
            .collect();
        let mut ptrs: Vec<*const c_char> = c_families.iter().map(|s| s.as_ptr()).collect();
        unsafe {
            OH_Drawing_SetTextStyleFontFamilies(
                self.raw.as_ptr(),
                ptrs.len() as i32,
                ptrs.as_mut_ptr(),
            );
        }
    }

    pub fn set_letter_spacing(&mut self, spacing: f64) {
        unsafe { OH_Drawing_SetTextStyleLetterSpacing(self.raw.as_ptr(), spacing) };
    }

    pub fn set_word_spacing(&mut self, spacing: f64) {
        unsafe { OH_Drawing_SetTextStyleWordSpacing(self.raw.as_ptr(), spacing) };
    }

    pub fn set_locale(&mut self, locale: &str) {
        if let Ok(locale) = CString::new(locale) {
            unsafe { OH_Drawing_SetTextStyleLocale(self.raw.as_ptr(), locale.as_ptr()) };
        }
    }

    pub fn add_font_feature(&mut self, tag: &str, value: i32) -> bool {
        let Ok(tag) = CString::new(tag) else {
            return false;
        };
        unsafe { OH_Drawing_TextStyleAddFontFeature(self.raw.as_ptr(), tag.as_ptr(), value) };
        true
    }

    pub fn add_font_variation(&mut self, axis: &str, value: f32) -> bool {
        let Ok(axis) = CString::new(axis) else {
            return false;
        };
        unsafe { OH_Drawing_TextStyleAddFontVariation(self.raw.as_ptr(), axis.as_ptr(), value) };
        true
    }

    pub fn set_foreground_brush(&mut self, brush: &Brush) {
        unsafe { OH_Drawing_SetTextStyleForegroundBrush(self.raw.as_ptr(), brush.as_ptr()) };
    }

    pub fn set_foreground_pen(&mut self, pen: &Pen) {
        unsafe { OH_Drawing_SetTextStyleForegroundPen(self.raw.as_ptr(), pen.as_ptr()) };
    }

    /// Adds a copy of `shadow` to this text style.
    pub fn add_shadow(&mut self, shadow: &TextShadow) {
        unsafe { OH_Drawing_TextStyleAddShadow(self.raw.as_ptr(), shadow.raw.as_ptr()) };
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TextStyle {
    fn drop(&mut self) {
        unsafe { OH_Drawing_DestroyTextStyle(self.raw.as_ptr()) };
    }
}

/// Owned native text-shadow descriptor.
#[derive(Debug)]
pub struct TextShadow {
    raw: NonNull<OH_Drawing_TextShadow>,
}

impl TextShadow {
    pub fn new(color: u32, offset: &Point, blur_radius: f64) -> Self {
        let raw = unsafe { OH_Drawing_CreateTextShadow() };
        let raw = NonNull::new(raw).expect("OH_Drawing_CreateTextShadow returned null");
        unsafe { OH_Drawing_SetTextShadow(raw.as_ptr(), color, offset.as_ptr(), blur_radius) };
        Self { raw }
    }
}

impl Drop for TextShadow {
    fn drop(&mut self) {
        unsafe { OH_Drawing_DestroyTextShadow(self.raw.as_ptr()) };
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawingTypographyStyle {
    raw: NonNull<c_void>,
}

impl DrawingTypographyStyle {
    /// # Safety
    /// The pointer must be a valid `OH_Drawing_TypographyStyle`.
    pub unsafe fn from_raw(raw: *mut c_void) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> NonNull<c_void> {
        self.raw
    }

    pub fn raw(&self) -> *mut OH_Drawing_TypographyStyle {
        self.raw.as_ptr().cast()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawingFontCollection {
    raw: NonNull<c_void>,
}

impl DrawingFontCollection {
    /// # Safety
    /// The pointer must be a valid `OH_Drawing_FontCollection`.
    pub unsafe fn from_raw(raw: *mut c_void) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> NonNull<c_void> {
        self.raw
    }

    pub fn raw(&self) -> *mut OH_Drawing_FontCollection {
        self.raw.as_ptr().cast()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawingTextStyle {
    raw: NonNull<c_void>,
}

impl DrawingTextStyle {
    /// # Safety
    /// The pointer must be a valid `OH_Drawing_TextStyle`.
    pub unsafe fn from_raw(raw: *mut c_void) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> NonNull<c_void> {
        self.raw
    }

    pub fn raw(&self) -> *mut OH_Drawing_TextStyle {
        self.raw.as_ptr().cast()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawingPlaceholderSpan {
    raw: NonNull<c_void>,
}

impl DrawingPlaceholderSpan {
    /// # Safety
    /// The pointer must be a valid `OH_Drawing_PlaceholderSpan`.
    pub unsafe fn from_raw(raw: *mut c_void) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> NonNull<c_void> {
        self.raw
    }

    pub fn raw(&self) -> *mut OH_Drawing_PlaceholderSpan {
        self.raw.as_ptr().cast()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawingTypography {
    raw: NonNull<c_void>,
}

impl DrawingTypography {
    /// # Safety
    /// The pointer must be a valid `OH_Drawing_Typography`.
    pub unsafe fn from_raw(raw: *mut OH_Drawing_Typography) -> Option<Self> {
        NonNull::new(raw.cast()).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> NonNull<c_void> {
        self.raw
    }

    pub fn raw(&self) -> *mut OH_Drawing_Typography {
        self.raw.as_ptr().cast()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawingTextBox {
    raw: NonNull<c_void>,
}

impl DrawingTextBox {
    /// # Safety
    /// The pointer must be a valid `OH_Drawing_TextBox`.
    pub unsafe fn from_raw(raw: *mut OH_Drawing_TextBox) -> Option<Self> {
        NonNull::new(raw.cast()).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> NonNull<c_void> {
        self.raw
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrawingFontMetrics {
    pub flags: u32,
    pub top: f32,
    pub ascent: f32,
    pub descent: f32,
    pub bottom: f32,
    pub leading: f32,
    pub avg_char_width: f32,
    pub max_char_width: f32,
    pub x_min: f32,
    pub x_max: f32,
    pub x_height: f32,
    pub cap_height: f32,
    pub underline_thickness: f32,
    pub underline_position: f32,
    pub strikeout_thickness: f32,
    pub strikeout_position: f32,
}

impl From<OH_Drawing_Font_Metrics> for DrawingFontMetrics {
    fn from(value: OH_Drawing_Font_Metrics) -> Self {
        Self {
            flags: value.flags,
            top: value.top,
            ascent: value.ascent,
            descent: value.descent,
            bottom: value.bottom,
            leading: value.leading,
            avg_char_width: value.avgCharWidth,
            max_char_width: value.maxCharWidth,
            x_min: value.xMin,
            x_max: value.xMax,
            x_height: value.xHeight,
            cap_height: value.capHeight,
            underline_thickness: value.underlineThickness,
            underline_position: value.underlinePosition,
            strikeout_thickness: value.strikeoutThickness,
            strikeout_position: value.strikeoutPosition,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrawingLineMetrics {
    pub ascender: f64,
    pub descender: f64,
    pub cap_height: f64,
    pub x_height: f64,
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub start_index: usize,
    pub end_index: usize,
    pub first_char_metrics: DrawingFontMetrics,
}

pub type DrawingLineMetricsRaw = OH_Drawing_LineMetrics;

impl From<OH_Drawing_LineMetrics> for DrawingLineMetrics {
    fn from(value: OH_Drawing_LineMetrics) -> Self {
        Self {
            ascender: value.ascender,
            descender: value.descender,
            cap_height: value.capHeight,
            x_height: value.xHeight,
            width: value.width,
            height: value.height,
            x: value.x,
            y: value.y,
            start_index: value.startIndex,
            end_index: value.endIndex,
            first_char_metrics: value.firstCharMetrics.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    OH_Drawing_RectWidthStyle,
    "OH_Drawing_RectWidthStyle_RECT_WIDTH_STYLE_"
)]
pub enum TextRectWidthStyle {
    Tight,
    Max,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    OH_Drawing_RectHeightStyle,
    "OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_"
)]
pub enum TextRectHeightStyle {
    Tight,
    Max,
    #[suffix("INCLUDELINESPACEMIDDLE")]
    IncludeLineSpaceMiddle,
    #[suffix("INCLUDELINESPACETOP")]
    IncludeLineSpaceTop,
    #[suffix("INCLUDELINESPACEBOTTOM")]
    IncludeLineSpaceBottom,
    Struct,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PositionAndAffinity {
    raw: NonNull<OH_Drawing_PositionAndAffinity>,
}

impl PositionAndAffinity {
    /// Wraps a non-owned native position-and-affinity pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure `raw` remains valid for the returned wrapper's lifetime.
    pub unsafe fn from_raw_borrowed(raw: *mut OH_Drawing_PositionAndAffinity) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub fn as_ptr(&self) -> *mut OH_Drawing_PositionAndAffinity {
        self.raw.as_ptr()
    }

    pub fn position(&self) -> usize {
        unsafe { OH_Drawing_GetPositionFromPositionAndAffinity(self.raw.as_ptr()) }
    }

    pub fn affinity(&self) -> i32 {
        unsafe { OH_Drawing_GetAffinityFromPositionAndAffinity(self.raw.as_ptr()) }
    }
}

#[derive(Debug)]
pub struct TypographyBuilder {
    raw: NonNull<OH_Drawing_TypographyCreate>,
}

impl TypographyBuilder {
    pub fn new(style: &mut TypographyStyle, fonts: &mut FontCollection) -> Self {
        let raw = unsafe { OH_Drawing_CreateTypographyHandler(style.as_ptr(), fonts.as_ptr()) };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_CreateTypographyHandler returned null"),
        }
    }

    pub fn push_text_style(&mut self, style: &mut TextStyle) {
        unsafe { OH_Drawing_TypographyHandlerPushTextStyle(self.raw.as_ptr(), style.as_ptr()) };
    }

    pub fn pop_text_style(&mut self) {
        unsafe { OH_Drawing_TypographyHandlerPopTextStyle(self.raw.as_ptr()) };
    }

    /// Adds the caller's complete native inline-box descriptor.
    pub fn add_placeholder(&mut self, span: &OH_Drawing_PlaceholderSpan) -> Result<()> {
        if !span.width.is_finite()
            || !span.height.is_finite()
            || !span.baselineOffset.is_finite()
            || span.width < 0.0
            || span.height < 0.0
        {
            return Err(DrawingError::invalid_parameter());
        }
        let mut span = *span;
        // SAFETY: the builder is owned and native copies the stack descriptor.
        unsafe { OH_Drawing_TypographyHandlerAddPlaceholder(self.raw.as_ptr(), &mut span) };
        Ok(())
    }

    /// Adds UTF-8 text, preserving the existing infallible convenience API.
    ///
    /// # Panics
    ///
    /// Before API 20, interior NULs cannot be represented by the native C-string
    /// API. Use [`Self::try_add_text`] to receive an error instead of a panic.
    pub fn add_text(&mut self, text: &str) {
        self.try_add_text(text)
            .expect("native text API cannot represent an interior NUL");
    }

    /// Adds UTF-8 without modifying it. Before API 20, an interior NUL returns
    /// an invalid-parameter error before calling the native C-string API.
    pub fn try_add_text(&mut self, text: &str) -> Result<()> {
        #[cfg(feature = "api-20")]
        unsafe {
            OH_Drawing_TypographyHandlerAddEncodedText(
                self.raw.as_ptr(),
                text.as_ptr().cast(),
                text.len(),
                TextEncoding::Utf8.into(),
            );
        }

        #[cfg(not(feature = "api-20"))]
        {
            let text = CString::new(text).map_err(|_| DrawingError::invalid_parameter())?;
            // SAFETY: both the builder and NUL-terminated text remain live
            // throughout this synchronous call, which copies the text.
            unsafe { OH_Drawing_TypographyHandlerAddText(self.raw.as_ptr(), text.as_ptr()) };
        }
        Ok(())
    }

    pub fn build(self) -> Typography {
        let raw = unsafe { OH_Drawing_CreateTypography(self.raw.as_ptr()) };
        Typography {
            raw: NonNull::new(raw).expect("OH_Drawing_CreateTypography returned null"),
        }
    }
}

impl Drop for TypographyBuilder {
    fn drop(&mut self) {
        unsafe { OH_Drawing_DestroyTypographyHandler(self.raw.as_ptr()) };
    }
}

/// An owned visual range rectangle; it holds no native handles.
#[derive(Clone, Copy, Debug)]
pub struct RangeBox {
    /// Left, top, right and bottom in paragraph layout units.
    pub bounds: [f32; 4],
    /// Whether the visual run reads from right to left.
    pub rtl: bool,
}

struct TextBoxes(NonNull<OH_Drawing_TextBox>);

impl TextBoxes {
    fn collect(raw: *mut OH_Drawing_TextBox) -> Vec<RangeBox> {
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

#[derive(Debug)]
pub struct Typography {
    raw: NonNull<OH_Drawing_Typography>,
}

impl Typography {
    /// Tight UTF-16 range boxes with the native direction of each visual run.
    /// Coordinates use the paragraph's layout units (API 11).
    pub fn range_boxes(&self, start: usize, end: usize) -> Vec<RangeBox> {
        // SAFETY: native clamps the range to this owned, live paragraph.
        let raw = unsafe {
            OH_Drawing_TypographyGetRectsForRange(
                self.raw.as_ptr(),
                start,
                end,
                OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_TIGHT,
                OH_Drawing_RectWidthStyle_RECT_WIDTH_STYLE_TIGHT,
            )
        };
        TextBoxes::collect(raw)
    }

    /// Resolves the font metrics of a text style in this paragraph (API 12).
    pub fn font_metrics(&self, style: &TextStyle) -> Option<DrawingFontMetrics> {
        let mut metrics = std::mem::MaybeUninit::<OH_Drawing_Font_Metrics>::uninit();
        // SAFETY: both owned objects are live and native initializes metrics on success.
        unsafe {
            OH_Drawing_TextStyleGetFontMetrics(
                self.raw.as_ptr(),
                style.raw.as_ptr(),
                metrics.as_mut_ptr(),
            )
            .then(|| metrics.assume_init().into())
        }
    }

    /// Number of lines after layout.
    pub fn line_count(&self) -> usize {
        // SAFETY: this wrapper owns the live paragraph.
        unsafe { OH_Drawing_TypographyGetLineCount(self.raw.as_ptr()) }
    }

    /// Tight rectangles for a UTF-16 range, as `[left, top, right, bottom]`.
    pub fn range_rectangles(&self, start: usize, end: usize) -> Vec<[f32; 4]> {
        self.range_boxes(start, end)
            .into_iter()
            .map(|range| range.bounds)
            .collect()
    }

    /// Laid-out inline boxes, as `[left, top, right, bottom]`.
    pub fn placeholder_rectangles(&self) -> Vec<[f32; 4]> {
        // SAFETY: the live paragraph owns its placeholder layout.
        let raw = unsafe { OH_Drawing_TypographyGetRectsForPlaceholders(self.raw.as_ptr()) };
        TextBoxes::collect(raw)
            .into_iter()
            .map(|range| range.bounds)
            .collect()
    }

    pub fn as_ptr(&self) -> *mut OH_Drawing_Typography {
        self.raw.as_ptr()
    }

    pub fn layout(&mut self, max_width: f64) {
        unsafe { OH_Drawing_TypographyLayout(self.raw.as_ptr(), max_width) };
    }

    pub fn paint(&mut self, canvas: &Canvas, x: f64, y: f64) {
        unsafe { OH_Drawing_TypographyPaint(self.raw.as_ptr(), canvas.as_ptr(), x, y) };
    }

    pub fn max_width(&self) -> f64 {
        unsafe { OH_Drawing_TypographyGetMaxWidth(self.raw.as_ptr()) }
    }

    pub fn height(&self) -> f64 {
        unsafe { OH_Drawing_TypographyGetHeight(self.raw.as_ptr()) }
    }

    pub fn longest_line(&self) -> f64 {
        unsafe { OH_Drawing_TypographyGetLongestLine(self.raw.as_ptr()) }
    }

    pub fn line_metrics(&self, line: i32) -> Option<DrawingLineMetrics> {
        let mut metrics = std::mem::MaybeUninit::<OH_Drawing_LineMetrics>::uninit();
        unsafe {
            OH_Drawing_TypographyGetLineMetricsAt(self.raw.as_ptr(), line, metrics.as_mut_ptr())
                .then(|| metrics.assume_init().into())
        }
    }

    /// Returns the laid-out text lines of this paragraph (`OH_Drawing_TypographyGetTextLines`, API 18).
    #[cfg(feature = "api-18")]
    pub fn text_lines(&self) -> TextLines<'_> {
        let raw = unsafe { OH_Drawing_TypographyGetTextLines(self.raw.as_ptr()) };
        TextLines {
            raw: NonNull::new(raw),
            _owner: std::marker::PhantomData,
        }
    }
}

impl Drop for Typography {
    fn drop(&mut self) {
        unsafe { OH_Drawing_DestroyTypography(self.raw.as_ptr()) };
    }
}

/// Typographic metrics of a laid-out [`TextLine`] (API 18).
#[cfg(feature = "api-18")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineTypographicBounds {
    pub width: f64,
    pub ascent: f64,
    pub descent: f64,
    pub leading: f64,
}

/// Typographic metrics of a laid-out [`GlyphRun`] (API 18).
#[cfg(feature = "api-18")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RunTypographicBounds {
    pub width: f32,
    pub ascent: f32,
    pub descent: f32,
    pub leading: f32,
}

/// Owned array of [`TextLine`] items returned by [`Typography::text_lines`] (API 18).
#[cfg(feature = "api-18")]
#[derive(Debug)]
pub struct TextLines<'a> {
    raw: Option<NonNull<OH_Drawing_Array>>,
    _owner: std::marker::PhantomData<&'a Typography>,
}

#[cfg(feature = "api-18")]
impl<'a> TextLines<'a> {
    pub fn len(&self) -> usize {
        self.raw.map_or(0, |raw| unsafe {
            OH_Drawing_GetDrawingArraySize(raw.as_ptr())
        })
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<TextLine<'_>> {
        let raw = self.raw?;
        let line = unsafe { OH_Drawing_GetTextLineByIndex(raw.as_ptr(), index) };
        NonNull::new(line).map(|raw| TextLine {
            raw,
            _owner: std::marker::PhantomData,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = TextLine<'_>> + '_ {
        (0..self.len()).filter_map(|index| self.get(index))
    }
}

#[cfg(feature = "api-18")]
impl Drop for TextLines<'_> {
    fn drop(&mut self) {
        if let Some(raw) = self.raw {
            unsafe { OH_Drawing_DestroyTextLines(raw.as_ptr()) };
        }
    }
}

/// A borrowed text line within a [`TextLines`] array (API 18).
#[cfg(feature = "api-18")]
#[derive(Clone, Copy, Debug)]
pub struct TextLine<'a> {
    raw: NonNull<OH_Drawing_TextLine>,
    _owner: std::marker::PhantomData<&'a TextLines<'a>>,
}

#[cfg(feature = "api-18")]
impl<'a> TextLine<'a> {
    pub fn as_ptr(&self) -> *mut OH_Drawing_TextLine {
        self.raw.as_ptr()
    }

    pub fn glyph_count(&self) -> usize {
        let count = unsafe { OH_Drawing_TextLineGetGlyphCount(self.raw.as_ptr()) };
        if count.is_finite() && count > 0.0 {
            count as usize
        } else {
            0
        }
    }

    pub fn text_range(&self) -> (usize, usize) {
        let mut start = 0usize;
        let mut end = 0usize;
        unsafe { OH_Drawing_TextLineGetTextRange(self.raw.as_ptr(), &mut start, &mut end) };
        (start, end)
    }

    pub fn typographic_bounds(&self) -> LineTypographicBounds {
        let mut ascent = 0.0;
        let mut descent = 0.0;
        let mut leading = 0.0;
        let width = unsafe {
            OH_Drawing_TextLineGetTypographicBounds(
                self.raw.as_ptr(),
                &mut ascent,
                &mut descent,
                &mut leading,
            )
        };
        LineTypographicBounds {
            width,
            ascent,
            descent,
            leading,
        }
    }

    pub fn trailing_space_width(&self) -> f64 {
        unsafe { OH_Drawing_TextLineGetTrailingSpaceWidth(self.raw.as_ptr()) }
    }

    pub fn alignment_offset(&self, factor: f64, width: f64) -> f64 {
        unsafe { OH_Drawing_TextLineGetAlignmentOffset(self.raw.as_ptr(), factor, width) }
    }

    pub fn string_index_for_position(&self, x: f32, y: f32) -> i32 {
        let point = Point::new(x, y);
        unsafe { OH_Drawing_TextLineGetStringIndexForPosition(self.raw.as_ptr(), point.as_ptr()) }
    }

    pub fn offset_for_string_index(&self, index: i32) -> f64 {
        unsafe { OH_Drawing_TextLineGetOffsetForStringIndex(self.raw.as_ptr(), index) }
    }

    pub fn paint(&self, canvas: &Canvas, x: f64, y: f64) {
        unsafe { OH_Drawing_TextLinePaint(self.raw.as_ptr(), canvas.as_ptr(), x, y) };
    }

    pub fn glyph_runs(&self) -> GlyphRuns<'_> {
        let raw = unsafe { OH_Drawing_TextLineGetGlyphRuns(self.raw.as_ptr()) };
        GlyphRuns {
            raw: NonNull::new(raw),
            _owner: std::marker::PhantomData,
        }
    }
}

/// Owned array of [`GlyphRun`] items returned by [`TextLine::glyph_runs`] (API 18).
#[cfg(feature = "api-18")]
#[derive(Debug)]
pub struct GlyphRuns<'a> {
    raw: Option<NonNull<OH_Drawing_Array>>,
    _owner: std::marker::PhantomData<&'a TextLine<'a>>,
}

#[cfg(feature = "api-18")]
impl<'a> GlyphRuns<'a> {
    pub fn len(&self) -> usize {
        self.raw.map_or(0, |raw| unsafe {
            OH_Drawing_GetDrawingArraySize(raw.as_ptr())
        })
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<GlyphRun<'_>> {
        let raw = self.raw?;
        let run = unsafe { OH_Drawing_GetRunByIndex(raw.as_ptr(), index) };
        NonNull::new(run).map(|raw| GlyphRun {
            raw,
            _owner: std::marker::PhantomData,
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = GlyphRun<'_>> + '_ {
        (0..self.len()).filter_map(|index| self.get(index))
    }
}

#[cfg(feature = "api-18")]
impl Drop for GlyphRuns<'_> {
    fn drop(&mut self) {
        if let Some(raw) = self.raw {
            unsafe { OH_Drawing_DestroyRuns(raw.as_ptr()) };
        }
    }
}

/// A borrowed shaped glyph run within a [`GlyphRuns`] array (API 18).
#[cfg(feature = "api-18")]
#[derive(Clone, Copy, Debug)]
pub struct GlyphRun<'a> {
    raw: NonNull<OH_Drawing_Run>,
    _owner: std::marker::PhantomData<&'a GlyphRuns<'a>>,
}

#[cfg(feature = "api-18")]
impl<'a> GlyphRun<'a> {
    pub fn as_ptr(&self) -> *mut OH_Drawing_Run {
        self.raw.as_ptr()
    }

    pub fn glyph_count(&self) -> u32 {
        unsafe { OH_Drawing_GetRunGlyphCount(self.raw.as_ptr()) }
    }

    pub fn string_range(&self) -> (u64, u64) {
        let mut location = 0u64;
        let mut length = 0u64;
        unsafe { OH_Drawing_GetRunStringRange(self.raw.as_ptr(), &mut location, &mut length) };
        (location, length)
    }

    pub fn typographic_bounds(&self) -> RunTypographicBounds {
        let mut ascent = 0.0;
        let mut descent = 0.0;
        let mut leading = 0.0;
        let width = unsafe {
            OH_Drawing_GetRunTypographicBounds(
                self.raw.as_ptr(),
                &mut ascent,
                &mut descent,
                &mut leading,
            )
        };
        RunTypographicBounds {
            width,
            ascent,
            descent,
            leading,
        }
    }

    pub fn glyphs(&self) -> Vec<u16> {
        let count = self.glyph_count();
        if count == 0 {
            return Vec::new();
        }
        unsafe {
            let arr = OH_Drawing_GetRunGlyphs(self.raw.as_ptr(), 0, count as i64);
            if arr.is_null() {
                return Vec::new();
            }
            let len = OH_Drawing_GetDrawingArraySize(arr);
            let mut out = Vec::with_capacity(len);
            for index in 0..len {
                out.push(OH_Drawing_GetRunGlyphsByIndex(arr, index));
            }
            OH_Drawing_DestroyRunGlyphs(arr);
            out
        }
    }

    pub fn positions(&self) -> Vec<(f32, f32)> {
        let count = self.glyph_count();
        if count == 0 {
            return Vec::new();
        }
        unsafe {
            let arr = OH_Drawing_GetRunPositions(self.raw.as_ptr(), 0, count as i64);
            if arr.is_null() {
                return Vec::new();
            }
            let len = OH_Drawing_GetDrawingArraySize(arr);
            let mut out = Vec::with_capacity(len);
            for index in 0..len {
                let pt = OH_Drawing_GetRunPositionsByIndex(arr, index);
                let mut x = 0.0f32;
                let mut y = 0.0f32;
                if !pt.is_null() {
                    OH_Drawing_PointGetX(pt, &mut x);
                    OH_Drawing_PointGetY(pt, &mut y);
                }
                out.push((x, y));
            }
            OH_Drawing_DestroyRunPositions(arr);
            out
        }
    }

    pub fn string_indices(&self) -> Vec<u64> {
        let count = self.glyph_count();
        if count == 0 {
            return Vec::new();
        }
        unsafe {
            let arr = OH_Drawing_GetRunStringIndices(self.raw.as_ptr(), 0, count as i64);
            if arr.is_null() {
                return Vec::new();
            }
            let len = OH_Drawing_GetDrawingArraySize(arr);
            let mut out = Vec::with_capacity(len);
            for index in 0..len {
                out.push(OH_Drawing_GetRunStringIndicesByIndex(arr, index));
            }
            OH_Drawing_DestroyRunStringIndices(arr);
            out
        }
    }

    pub fn paint(&self, canvas: &Canvas, x: f64, y: f64) {
        unsafe { OH_Drawing_RunPaint(canvas.as_ptr(), self.raw.as_ptr(), x, y) };
    }

    /// Returns an owned [`Font`] copy for this glyph run (`OH_Drawing_GetRunFont`, API 20).
    #[cfg(feature = "api-20")]
    pub fn font(&self) -> Option<Font> {
        unsafe { Font::from_raw_owned(OH_Drawing_GetRunFont(self.raw.as_ptr())) }
    }

    /// Returns the text direction of this glyph run (`OH_Drawing_GetRunTextDirection`, API 20).
    #[cfg(feature = "api-20")]
    pub fn text_direction(&self) -> TextDirection {
        let raw = unsafe { OH_Drawing_GetRunTextDirection(self.raw.as_ptr()) };
        if raw == OH_Drawing_TextDirection_TEXT_DIRECTION_RTL {
            TextDirection::Rtl
        } else {
            TextDirection::Ltr
        }
    }

    /// Returns per-glyph horizontal advances for this glyph run (`OH_Drawing_GetRunGlyphAdvances`, API 20).
    #[cfg(feature = "api-20")]
    pub fn glyph_advances(&self) -> Vec<f32> {
        let count = self.glyph_count();
        if count == 0 {
            return Vec::new();
        }
        unsafe {
            let arr = OH_Drawing_GetRunGlyphAdvances(self.raw.as_ptr(), 0, count);
            if arr.is_null() {
                return Vec::new();
            }
            let len = OH_Drawing_GetDrawingArraySize(arr);
            let mut out = Vec::with_capacity(len);
            for index in 0..len {
                let pt = OH_Drawing_GetRunGlyphAdvanceByIndex(arr, index);
                let mut advance = 0.0f32;
                if !pt.is_null() {
                    OH_Drawing_PointGetX(pt, &mut advance);
                }
                out.push(advance);
            }
            OH_Drawing_DestroyRunGlyphAdvances(arr);
            out
        }
    }
}
