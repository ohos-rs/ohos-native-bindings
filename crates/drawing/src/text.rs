use std::{
    ffi::CString,
    os::raw::{c_char, c_void},
    ptr::NonNull,
};

#[cfg(feature = "api-14")]
use ohos_native_drawing_sys::OH_Drawing_GetFontCollectionGlobalInstance;
#[cfg(feature = "api-20")]
use ohos_native_drawing_sys::OH_Drawing_UnregisterFont;
use ohos_native_drawing_sys::{
    OH_Drawing_ClearFontCaches, OH_Drawing_CreateFontCollection,
    OH_Drawing_CreateSharedFontCollection, OH_Drawing_CreateTextStyle, OH_Drawing_CreateTypography,
    OH_Drawing_CreateTypographyHandler, OH_Drawing_CreateTypographyStyle,
    OH_Drawing_DestroyFontCollection, OH_Drawing_DestroyTextStyle, OH_Drawing_DestroyTypography,
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
    OH_Drawing_RegisterFontBuffer, OH_Drawing_SetTextStyleColor,
    OH_Drawing_SetTextStyleFontFamilies, OH_Drawing_SetTextStyleFontSize,
    OH_Drawing_SetTextStyleFontStyle, OH_Drawing_SetTextStyleFontWeight,
    OH_Drawing_SetTypographyTextAlign, OH_Drawing_SetTypographyTextDirection,
    OH_Drawing_SetTypographyTextMaxLines, OH_Drawing_TextBox, OH_Drawing_TextStyle,
    OH_Drawing_Typography, OH_Drawing_TypographyCreate, OH_Drawing_TypographyGetHeight,
    OH_Drawing_TypographyGetLongestLine, OH_Drawing_TypographyGetMaxWidth,
    OH_Drawing_TypographyHandlerAddText, OH_Drawing_TypographyHandlerPopTextStyle,
    OH_Drawing_TypographyHandlerPushTextStyle, OH_Drawing_TypographyLayout,
    OH_Drawing_TypographyPaint, OH_Drawing_TypographyStyle,
};

use crate::{check_error, Canvas, Result};

#[derive(Debug)]
pub struct FontCollection {
    raw: NonNull<OH_Drawing_FontCollection>,
    owned: bool,
}

impl FontCollection {
    pub fn new() -> Self {
        let raw = unsafe { OH_Drawing_CreateFontCollection() };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_CreateFontCollection returned null"),
            owned: true,
        }
    }

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

    pub fn set_text_align(&mut self, align: i32) {
        unsafe { OH_Drawing_SetTypographyTextAlign(self.raw.as_ptr(), align) };
    }

    pub fn set_text_direction(&mut self, direction: i32) {
        unsafe { OH_Drawing_SetTypographyTextDirection(self.raw.as_ptr(), direction) };
    }

    pub fn set_max_lines(&mut self, line_number: i32) {
        unsafe { OH_Drawing_SetTypographyTextMaxLines(self.raw.as_ptr(), line_number) };
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

    pub fn set_font_size(&mut self, font_size: f64) {
        unsafe { OH_Drawing_SetTextStyleFontSize(self.raw.as_ptr(), font_size) };
    }

    pub fn set_font_weight(&mut self, weight: i32) {
        unsafe { OH_Drawing_SetTextStyleFontWeight(self.raw.as_ptr(), weight) };
    }

    pub fn set_font_style(&mut self, style: i32) {
        unsafe { OH_Drawing_SetTextStyleFontStyle(self.raw.as_ptr(), style) };
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextRectWidthStyle {
    Tight,
    Max,
}

impl From<TextRectWidthStyle> for OH_Drawing_RectWidthStyle {
    fn from(value: TextRectWidthStyle) -> Self {
        match value {
            TextRectWidthStyle::Tight => OH_Drawing_RectWidthStyle_RECT_WIDTH_STYLE_TIGHT,
            TextRectWidthStyle::Max => OH_Drawing_RectWidthStyle_RECT_WIDTH_STYLE_MAX,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextRectHeightStyle {
    Tight,
    Max,
    IncludeLineSpaceMiddle,
    IncludeLineSpaceTop,
    IncludeLineSpaceBottom,
    Struct,
}

impl From<TextRectHeightStyle> for OH_Drawing_RectHeightStyle {
    fn from(value: TextRectHeightStyle) -> Self {
        match value {
            TextRectHeightStyle::Tight => OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_TIGHT,
            TextRectHeightStyle::Max => OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_MAX,
            TextRectHeightStyle::IncludeLineSpaceMiddle => {
                OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_INCLUDELINESPACEMIDDLE
            }
            TextRectHeightStyle::IncludeLineSpaceTop => {
                OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_INCLUDELINESPACETOP
            }
            TextRectHeightStyle::IncludeLineSpaceBottom => {
                OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_INCLUDELINESPACEBOTTOM
            }
            TextRectHeightStyle::Struct => OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_STRUCT,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PositionAndAffinity {
    raw: NonNull<OH_Drawing_PositionAndAffinity>,
}

impl PositionAndAffinity {
    /// Wraps a non-owned native position-and-affinity pointer.
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

    pub fn add_text(&mut self, text: &str) {
        let text = CString::new(text).expect("text contains interior NUL");
        unsafe { OH_Drawing_TypographyHandlerAddText(self.raw.as_ptr(), text.as_ptr()) };
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
}

impl Drop for Typography {
    fn drop(&mut self) {
        unsafe { OH_Drawing_DestroyTypography(self.raw.as_ptr()) };
    }
}
