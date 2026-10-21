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
use ohos_native_drawing_sys::{
    OH_Drawing_ClearFontCaches, OH_Drawing_CreateFontCollection,
    OH_Drawing_CreateSharedFontCollection, OH_Drawing_CreateTextShadow, OH_Drawing_CreateTextStyle,
    OH_Drawing_CreateTypography, OH_Drawing_CreateTypographyHandler,
    OH_Drawing_CreateTypographyStyle, OH_Drawing_DestroyFontCollection,
    OH_Drawing_DestroyTextShadow, OH_Drawing_DestroyTextStyle, OH_Drawing_DestroyTypography,
    OH_Drawing_DestroyTypographyHandler, OH_Drawing_DestroyTypographyStyle,
    OH_Drawing_DisableFontCollectionFallback, OH_Drawing_DisableFontCollectionSystemFont,
    OH_Drawing_FontCollection, OH_Drawing_Font_Metrics,
    OH_Drawing_GetAffinityFromPositionAndAffinity, OH_Drawing_GetPositionFromPositionAndAffinity,
    OH_Drawing_LineMetrics, OH_Drawing_PlaceholderSpan, OH_Drawing_PositionAndAffinity,
    OH_Drawing_RectHeightStyle,
    OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_INCLUDELINESPACEBOTTOM,
    OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_INCLUDELINESPACEMIDDLE,
    OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_INCLUDELINESPACETOP,
    OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_MAX,
    OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_STRUCT,
    OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_TIGHT, OH_Drawing_RectWidthStyle,
    OH_Drawing_RectWidthStyle_RECT_WIDTH_STYLE_MAX,
    OH_Drawing_RectWidthStyle_RECT_WIDTH_STYLE_TIGHT, OH_Drawing_RegisterFont,
    OH_Drawing_RegisterFontBuffer, OH_Drawing_SetTextShadow, OH_Drawing_SetTextStyleColor,
    OH_Drawing_SetTextStyleFontFamilies, OH_Drawing_SetTextStyleFontSize,
    OH_Drawing_SetTextStyleFontStyleStruct, OH_Drawing_SetTextStyleForegroundBrush,
    OH_Drawing_SetTextStyleForegroundPen, OH_Drawing_SetTextStyleLetterSpacing,
    OH_Drawing_SetTextStyleLocale, OH_Drawing_SetTextStyleWordSpacing,
    OH_Drawing_SetTypographyTextAlign, OH_Drawing_SetTypographyTextDirection,
    OH_Drawing_SetTypographyTextMaxLines, OH_Drawing_TextBox, OH_Drawing_TextShadow,
    OH_Drawing_TextStyle, OH_Drawing_TextStyleAddFontFeature, OH_Drawing_TextStyleAddFontVariation,
    OH_Drawing_TextStyleAddShadow, OH_Drawing_Typography, OH_Drawing_TypographyCreate,
    OH_Drawing_TypographyGetHeight, OH_Drawing_TypographyGetLineMetricsAt,
    OH_Drawing_TypographyGetLongestLine, OH_Drawing_TypographyGetMaxWidth,
    OH_Drawing_TypographyHandlerPopTextStyle, OH_Drawing_TypographyHandlerPushTextStyle,
    OH_Drawing_TypographyLayout, OH_Drawing_TypographyPaint, OH_Drawing_TypographyStyle,
};
#[cfg(feature = "api-20")]
use ohos_native_drawing_sys::{
    OH_Drawing_TypographyHandlerAddEncodedText, OH_Drawing_UnregisterFont,
};

#[cfg(feature = "api-20")]
use crate::TextEncoding;
use crate::{check_error, Brush, Canvas, FontStyle, Pen, Point, Result, TextAlign, TextDirection};

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
    pub fn set_ellipsis(&mut self, text: &str, mode: crate::EllipsisMode) -> Result<()> {
        let text = CString::new(text).map_err(|_| crate::DrawingError::invalid_parameter())?;
        // SAFETY: the style is owned and native copies the temporary C string.
        unsafe {
            ohos_native_drawing_sys::OH_Drawing_SetTypographyTextEllipsis(
                self.raw.as_ptr(),
                text.as_ptr(),
            );
            ohos_native_drawing_sys::OH_Drawing_SetTypographyTextEllipsisModal(
                self.raw.as_ptr(),
                mode as i32,
            );
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

    /// Sets underline/strike-through lines, their color and pattern.
    pub fn set_decoration(
        &mut self,
        underline: bool,
        strike: bool,
        color: u32,
        pattern: crate::DecorationPattern,
    ) {
        // SAFETY: the style is owned; these functions copy scalar values.
        unsafe {
            ohos_native_drawing_sys::OH_Drawing_SetTextStyleDecoration(
                self.raw.as_ptr(),
                i32::from(underline) | (i32::from(strike) << 2),
            );
            ohos_native_drawing_sys::OH_Drawing_SetTextStyleDecorationColor(
                self.raw.as_ptr(),
                color,
            );
            ohos_native_drawing_sys::OH_Drawing_SetTextStyleDecorationStyle(
                self.raw.as_ptr(),
                pattern as i32,
            );
        }
    }

    pub fn set_font_size(&mut self, font_size: f64) {
        unsafe { OH_Drawing_SetTextStyleFontSize(self.raw.as_ptr(), font_size) };
    }

    /// Sets line height as a multiple of font size. Zero restores font metrics.
    /// Non-finite and negative multipliers are rejected before calling native code.
    pub fn set_font_height(&mut self, multiplier: f64) -> Result<()> {
        if !multiplier.is_finite() || multiplier < 0.0 {
            return Err(crate::DrawingError::invalid_parameter());
        }
        // SAFETY: this wrapper owns the live native style for this call.
        unsafe {
            ohos_native_drawing_sys::OH_Drawing_SetTextStyleFontHeight(
                self.raw.as_ptr(),
                multiplier,
            )
        };
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

    /// Adds an inline box entirely above the alphabetic baseline.
    pub fn add_placeholder(&mut self, width: f64, height: f64) -> Result<()> {
        if !width.is_finite() || !height.is_finite() || width < 0.0 || height < 0.0 {
            return Err(crate::DrawingError::invalid_parameter());
        }
        let mut span = OH_Drawing_PlaceholderSpan {
            width, height,
            alignment: ohos_native_drawing_sys::OH_Drawing_PlaceholderVerticalAlignment_ALIGNMENT_ABOVE_BASELINE,
            baseline: ohos_native_drawing_sys::OH_Drawing_TextBaseline_TEXT_BASELINE_ALPHABETIC,
            baselineOffset: 0.0,
        };
        // SAFETY: the builder is owned and native copies the stack descriptor.
        unsafe {
            ohos_native_drawing_sys::OH_Drawing_TypographyHandlerAddPlaceholder(
                self.raw.as_ptr(),
                &mut span,
            )
        };
        Ok(())
    }

    /// Adds UTF-8 text. Before API 20, embedded NUL characters are displayed as
    /// U+FFFD because the platform's text API requires a C string.
    pub fn add_text(&mut self, text: &str) {
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
            let text = crate::text_encoding::nul_terminated(text);
            // SAFETY: both the builder and NUL-terminated text remain live
            // throughout this synchronous call, which copies the text.
            unsafe { OH_Drawing_TypographyHandlerAddText(self.raw.as_ptr(), text.as_ptr()) };
        }
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

#[derive(Debug)]
pub struct Typography {
    raw: NonNull<OH_Drawing_Typography>,
}

impl Typography {
    /// Tight UTF-16 range boxes with the native direction of each visual run.
    /// Coordinates use the paragraph's layout units (API 11).
    pub fn range_boxes(&self, start: usize, end: usize) -> Vec<crate::RangeBox> {
        // SAFETY: native clamps the range to this owned, live paragraph.
        let raw = unsafe {
            ohos_native_drawing_sys::OH_Drawing_TypographyGetRectsForRange(
                self.raw.as_ptr(),
                start,
                end,
                OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_TIGHT,
                OH_Drawing_RectWidthStyle_RECT_WIDTH_STYLE_TIGHT,
            )
        };
        crate::text_boxes::TextBoxes::collect(raw)
    }
    /// Resolves the font metrics of a text style in this paragraph (API 12).
    pub fn font_metrics(&self, style: &TextStyle) -> Option<DrawingFontMetrics> {
        let mut metrics = std::mem::MaybeUninit::<OH_Drawing_Font_Metrics>::uninit();
        // SAFETY: both owned objects are live and native initializes metrics on success.
        unsafe {
            ohos_native_drawing_sys::OH_Drawing_TextStyleGetFontMetrics(
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
        unsafe { ohos_native_drawing_sys::OH_Drawing_TypographyGetLineCount(self.raw.as_ptr()) }
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
        let raw = unsafe {
            ohos_native_drawing_sys::OH_Drawing_TypographyGetRectsForPlaceholders(self.raw.as_ptr())
        };
        crate::text_boxes::TextBoxes::collect(raw)
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
}

impl Drop for Typography {
    fn drop(&mut self) {
        unsafe { OH_Drawing_DestroyTypography(self.raw.as_ptr()) };
    }
}
