use std::{ffi::CString, ptr::NonNull};

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
    OH_Drawing_FontCollection, OH_Drawing_RegisterFont, OH_Drawing_RegisterFontBuffer,
    OH_Drawing_SetTextStyleColor, OH_Drawing_SetTextStyleFontFamilies,
    OH_Drawing_SetTextStyleFontSize, OH_Drawing_SetTextStyleFontStyle,
    OH_Drawing_SetTextStyleFontWeight, OH_Drawing_SetTypographyTextAlign,
    OH_Drawing_SetTypographyTextDirection, OH_Drawing_SetTypographyTextMaxLines,
    OH_Drawing_TextStyle, OH_Drawing_Typography, OH_Drawing_TypographyCreate,
    OH_Drawing_TypographyGetHeight, OH_Drawing_TypographyGetLongestLine,
    OH_Drawing_TypographyGetMaxWidth, OH_Drawing_TypographyHandlerAddText,
    OH_Drawing_TypographyHandlerPopTextStyle, OH_Drawing_TypographyHandlerPushTextStyle,
    OH_Drawing_TypographyLayout, OH_Drawing_TypographyPaint, OH_Drawing_TypographyStyle,
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
        let mut ptrs: Vec<*const std::os::raw::c_char> =
            c_families.iter().map(|s| s.as_ptr()).collect();
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
