use std::{ffi::CString, path::Path, ptr::NonNull, rc::Rc};

use ohos_native_drawing_sys::{
    OH_Drawing_Font, OH_Drawing_FontCountText, OH_Drawing_FontCreate, OH_Drawing_FontDestroy,
    OH_Drawing_FontGetMetrics, OH_Drawing_FontGetTextSize, OH_Drawing_FontMeasureText,
    OH_Drawing_FontMgr, OH_Drawing_FontMgrCreate, OH_Drawing_FontMgrDestroy,
    OH_Drawing_FontMgrMatchFamilyStyle, OH_Drawing_FontSetBaselineSnap, OH_Drawing_FontSetEdging,
    OH_Drawing_FontSetFakeBoldText, OH_Drawing_FontSetHinting, OH_Drawing_FontSetLinearText,
    OH_Drawing_FontSetScaleX, OH_Drawing_FontSetSubpixel, OH_Drawing_FontSetTextSize,
    OH_Drawing_FontSetTextSkewX, OH_Drawing_FontSetTypeface, OH_Drawing_Font_Metrics,
    OH_Drawing_MemoryStreamCreate, OH_Drawing_Typeface, OH_Drawing_TypefaceCreateFromFile,
    OH_Drawing_TypefaceCreateFromStream, OH_Drawing_TypefaceDestroy,
};
#[cfg(feature = "api-20")]
use ohos_native_drawing_sys::{
    OH_Drawing_FontFeatures, OH_Drawing_FontFeaturesAddFeature, OH_Drawing_FontFeaturesCreate,
    OH_Drawing_FontFeaturesDestroy, OH_Drawing_FontMeasureSingleCharacterWithFeatures,
};

use crate::{check_error, FontEdging, FontHinting, FontStyle, Rect, Result, TextEncoding};

#[derive(Debug)]
pub struct Typeface {
    raw: NonNull<OH_Drawing_Typeface>,
}

impl Typeface {
    /// Loads a typeface from a font file.
    pub fn from_file(path: &Path, index: i32) -> Result<Self> {
        let path = path
            .to_str()
            .ok_or_else(crate::DrawingError::invalid_parameter)?;
        let path = CString::new(path).map_err(|_| crate::DrawingError::invalid_parameter())?;
        let raw = unsafe { OH_Drawing_TypefaceCreateFromFile(path.as_ptr(), index) };
        NonNull::new(raw)
            .map(|raw| Self { raw })
            .ok_or_else(crate::DrawingError::from_last_error)
    }

    /// Loads a typeface from copied in-memory font data.
    pub fn from_data(data: &[u8], index: i32) -> Result<Self> {
        if data.is_empty() {
            return Err(crate::DrawingError::invalid_parameter());
        }
        let stream =
            unsafe { OH_Drawing_MemoryStreamCreate(data.as_ptr().cast(), data.len(), true) };
        let stream = NonNull::new(stream).ok_or_else(crate::DrawingError::from_last_error)?;
        // The native typeface constructor takes ownership of the stream on
        // both success and failure, so it must not be destroyed here.
        let raw = unsafe { OH_Drawing_TypefaceCreateFromStream(stream.as_ptr(), index) };
        NonNull::new(raw)
            .map(|raw| Self { raw })
            .ok_or_else(crate::DrawingError::from_last_error)
    }

    pub(crate) fn as_ptr(&self) -> *mut OH_Drawing_Typeface {
        self.raw.as_ptr()
    }
}

impl Drop for Typeface {
    fn drop(&mut self) {
        unsafe { OH_Drawing_TypefaceDestroy(self.raw.as_ptr()) };
    }
}

#[cfg(feature = "api-20")]
#[derive(Debug)]
pub struct FontFeatures {
    raw: NonNull<OH_Drawing_FontFeatures>,
}

#[cfg(feature = "api-20")]
impl FontFeatures {
    pub fn new() -> Self {
        let raw = unsafe { OH_Drawing_FontFeaturesCreate() };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_FontFeaturesCreate returned null"),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut OH_Drawing_FontFeatures {
        self.raw.as_ptr()
    }

    pub fn add(&mut self, tag: &str, value: f32) -> Result<()> {
        let tag = CString::new(tag).map_err(|_| crate::DrawingError::invalid_parameter())?;
        let code =
            unsafe { OH_Drawing_FontFeaturesAddFeature(self.raw.as_ptr(), tag.as_ptr(), value) };
        check_error(code)
    }
}

#[cfg(feature = "api-20")]
impl Default for FontFeatures {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "api-20")]
impl Drop for FontFeatures {
    fn drop(&mut self) {
        let _ = unsafe { OH_Drawing_FontFeaturesDestroy(self.raw.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct FontManager {
    raw: NonNull<OH_Drawing_FontMgr>,
}

impl FontManager {
    pub fn new() -> Self {
        let raw = unsafe { OH_Drawing_FontMgrCreate() };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_FontMgrCreate returned null"),
        }
    }

    pub fn match_family_style(&self, family: &str, style: FontStyle) -> Option<Typeface> {
        let family = CString::new(family).ok()?;
        let raw = unsafe {
            OH_Drawing_FontMgrMatchFamilyStyle(self.raw.as_ptr(), family.as_ptr(), style.into_raw())
        };
        NonNull::new(raw).map(|raw| Typeface { raw })
    }
}

impl Default for FontManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for FontManager {
    fn drop(&mut self) {
        unsafe { OH_Drawing_FontMgrDestroy(self.raw.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct Font {
    raw: NonNull<OH_Drawing_Font>,
    // The native font only borrows this pointer, so keep the owner alive with the font.
    _owned_typeface: Option<Typeface>,
    _shared_typeface: Option<Rc<Typeface>>,
}

#[derive(Debug)]
pub struct TextMeasure {
    pub width: f32,
    pub bounds: Rect,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FontMetrics {
    pub top: f32,
    pub ascent: f32,
    pub descent: f32,
    pub bottom: f32,
    pub leading: f32,
    pub average_character_width: f32,
    pub maximum_character_width: f32,
    pub x_min: f32,
    pub x_max: f32,
    pub x_height: f32,
    pub cap_height: f32,
}

impl Font {
    pub fn new() -> Self {
        let raw = unsafe { OH_Drawing_FontCreate() };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_FontCreate returned null"),
            _owned_typeface: None,
            _shared_typeface: None,
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut OH_Drawing_Font {
        self.raw.as_ptr()
    }

    pub fn set_text_size(&mut self, text_size: f32) {
        unsafe { OH_Drawing_FontSetTextSize(self.raw.as_ptr(), text_size) };
    }

    pub fn text_size(&self) -> f32 {
        unsafe { OH_Drawing_FontGetTextSize(self.raw.as_ptr()) }
    }

    pub fn set_fake_bold(&mut self, fake_bold: bool) {
        unsafe { OH_Drawing_FontSetFakeBoldText(self.raw.as_ptr(), fake_bold) };
    }

    pub fn set_typeface(&mut self, typeface: Typeface) {
        unsafe { OH_Drawing_FontSetTypeface(self.raw.as_ptr(), typeface.as_ptr()) };
        self._shared_typeface = None;
        self._owned_typeface = Some(typeface);
    }

    /// Uses a shared typeface while retaining it for the lifetime of this font.
    pub fn set_shared_typeface(&mut self, typeface: Rc<Typeface>) {
        unsafe { OH_Drawing_FontSetTypeface(self.raw.as_ptr(), typeface.as_ptr()) };
        self._owned_typeface = None;
        self._shared_typeface = Some(typeface);
    }

    pub fn set_scale_x(&mut self, scale_x: f32) {
        unsafe { OH_Drawing_FontSetScaleX(self.raw.as_ptr(), scale_x) };
    }

    pub fn set_linear_text(&mut self, linear: bool) {
        unsafe { OH_Drawing_FontSetLinearText(self.raw.as_ptr(), linear) };
    }

    pub fn set_subpixel(&mut self, subpixel: bool) {
        unsafe { OH_Drawing_FontSetSubpixel(self.raw.as_ptr(), subpixel) };
    }

    pub fn set_baseline_snap(&mut self, baseline_snap: bool) {
        unsafe { OH_Drawing_FontSetBaselineSnap(self.raw.as_ptr(), baseline_snap) };
    }

    pub fn set_hinting(&mut self, hinting: FontHinting) {
        unsafe { OH_Drawing_FontSetHinting(self.raw.as_ptr(), hinting.into()) };
    }

    pub fn set_edging(&mut self, edging: FontEdging) {
        unsafe { OH_Drawing_FontSetEdging(self.raw.as_ptr(), edging.into()) };
    }

    pub fn set_skew_x(&mut self, skew_x: f32) {
        unsafe { OH_Drawing_FontSetTextSkewX(self.raw.as_ptr(), skew_x) };
    }

    pub fn count_text(&self, text: &str, encoding: TextEncoding) -> i32 {
        unsafe {
            OH_Drawing_FontCountText(
                self.raw.as_ptr(),
                text.as_ptr().cast(),
                text.len(),
                encoding.into(),
            )
        }
    }

    pub fn measure_text(&self, text: &str, encoding: TextEncoding) -> Result<TextMeasure> {
        let bounds = Rect::new(0.0, 0.0, 0.0, 0.0);
        let mut width = 0.0;
        let code = unsafe {
            OH_Drawing_FontMeasureText(
                self.raw.as_ptr(),
                text.as_ptr().cast(),
                text.len(),
                encoding.into(),
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
            ohos_native_drawing_sys::OH_Drawing_FontMeasureSingleCharacter(
                self.raw.as_ptr(),
                c.as_ptr(),
                &mut width,
            )
        };
        check_error(code)?;
        Ok(width)
    }

    #[cfg(feature = "api-20")]
    pub fn measure_single_character_with_features(
        &self,
        ch: &str,
        features: &FontFeatures,
    ) -> Result<f32> {
        let ch = CString::new(ch).map_err(|_| crate::DrawingError::invalid_parameter())?;
        let mut width = 0.0;
        let code = unsafe {
            OH_Drawing_FontMeasureSingleCharacterWithFeatures(
                self.raw.as_ptr(),
                ch.as_ptr(),
                features.as_ptr(),
                &mut width,
            )
        };
        check_error(code)?;
        Ok(width)
    }

    pub fn metrics(&mut self) -> FontMetrics {
        let mut metrics = OH_Drawing_Font_Metrics {
            flags: 0,
            top: 0.0,
            ascent: 0.0,
            descent: 0.0,
            bottom: 0.0,
            leading: 0.0,
            avgCharWidth: 0.0,
            maxCharWidth: 0.0,
            xMin: 0.0,
            xMax: 0.0,
            xHeight: 0.0,
            capHeight: 0.0,
            underlineThickness: 0.0,
            underlinePosition: 0.0,
            strikeoutThickness: 0.0,
            strikeoutPosition: 0.0,
        };
        unsafe { OH_Drawing_FontGetMetrics(self.raw.as_ptr(), &mut metrics) };
        FontMetrics {
            top: metrics.top,
            ascent: metrics.ascent,
            descent: metrics.descent,
            bottom: metrics.bottom,
            leading: metrics.leading,
            average_character_width: metrics.avgCharWidth,
            maximum_character_width: metrics.maxCharWidth,
            x_min: metrics.xMin,
            x_max: metrics.xMax,
            x_height: metrics.xHeight,
            cap_height: metrics.capHeight,
        }
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
