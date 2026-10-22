use std::ptr::NonNull;

use ohos_native_drawing_sys::{
    OH_Drawing_BlurType_NORMAL, OH_Drawing_ColorFilter, OH_Drawing_ColorFilterCreateBlendMode,
    OH_Drawing_ColorFilterCreateMatrix, OH_Drawing_ColorFilterDestroy, OH_Drawing_Filter,
    OH_Drawing_FilterCreate, OH_Drawing_FilterDestroy, OH_Drawing_FilterSetColorFilter,
    OH_Drawing_FilterSetImageFilter, OH_Drawing_FilterSetMaskFilter, OH_Drawing_ImageFilter,
    OH_Drawing_ImageFilterCreateBlur, OH_Drawing_ImageFilterDestroy, OH_Drawing_MaskFilter,
    OH_Drawing_MaskFilterCreateBlur, OH_Drawing_MaskFilterDestroy,
};

#[derive(Debug)]
pub struct ColorFilter {
    raw: NonNull<OH_Drawing_ColorFilter>,
}

impl ColorFilter {
    /// Blends an ARGB color with the filtered pixels (API 11).
    pub fn blend(color: u32, mode: crate::BlendMode) -> Option<Self> {
        // SAFETY: the color and mode are values; the returned filter is uniquely
        // owned by this wrapper and released by Drop.
        let raw = unsafe { OH_Drawing_ColorFilterCreateBlendMode(color, mode.into()) };
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub fn matrix(matrix: [f32; 20]) -> Option<Self> {
        let raw = unsafe { OH_Drawing_ColorFilterCreateMatrix(matrix.as_ptr()) };
        NonNull::new(raw).map(|raw| Self { raw })
    }

    fn as_ptr(&self) -> *mut OH_Drawing_ColorFilter {
        self.raw.as_ptr()
    }
}

impl Drop for ColorFilter {
    fn drop(&mut self) {
        unsafe { OH_Drawing_ColorFilterDestroy(self.raw.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct MaskFilter {
    raw: NonNull<OH_Drawing_MaskFilter>,
}

impl MaskFilter {
    pub fn blur(sigma: f32, respect_transform: bool) -> Option<Self> {
        if !sigma.is_finite() || sigma <= 0.0 {
            return None;
        }
        let raw = unsafe {
            OH_Drawing_MaskFilterCreateBlur(OH_Drawing_BlurType_NORMAL, sigma, respect_transform)
        };
        NonNull::new(raw).map(|raw| Self { raw })
    }

    fn as_ptr(&self) -> *mut OH_Drawing_MaskFilter {
        self.raw.as_ptr()
    }
}

impl Drop for MaskFilter {
    fn drop(&mut self) {
        unsafe { OH_Drawing_MaskFilterDestroy(self.raw.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct Filter {
    raw: NonNull<OH_Drawing_Filter>,
}

/// Owned API-12 image filter, applied to bitmap content rather than vector coverage.
#[derive(Debug)]
pub struct ImageFilter {
    raw: NonNull<OH_Drawing_ImageFilter>,
}
impl ImageFilter {
    /// Gaussian blur of the original image. Sigma is in canvas pixels.
    /// API 12 requires both sigma values to be finite and greater than zero.
    pub fn blur(sigma_x: f32, sigma_y: f32, tile: crate::TileMode) -> Option<Self> {
        if !sigma_x.is_finite() || !sigma_y.is_finite() || sigma_x <= 0.0 || sigma_y <= 0.0 {
            return None;
        }
        // SAFETY: finite arguments and no borrowed input; the returned native
        // image filter is uniquely owned and destroyed below.
        NonNull::new(unsafe {
            OH_Drawing_ImageFilterCreateBlur(sigma_x, sigma_y, tile.into(), std::ptr::null_mut())
        })
        .map(|raw| Self { raw })
    }
}
impl Drop for ImageFilter {
    fn drop(&mut self) {
        // SAFETY: this is the unique owner returned by the native constructor.
        unsafe { OH_Drawing_ImageFilterDestroy(self.raw.as_ptr()) };
    }
}

impl Filter {
    /// Set a retained native image filter, or clear the current one.
    pub fn set_image_filter(&mut self, filter: Option<&ImageFilter>) {
        // SAFETY: both handles are live for this call. The drawing Filter
        // retains its own image-filter reference, just as for color filters.
        unsafe {
            OH_Drawing_FilterSetImageFilter(
                self.raw.as_ptr(),
                filter.map_or(std::ptr::null_mut(), |filter| filter.raw.as_ptr()),
            )
        };
    }
    pub fn new() -> Self {
        let raw = unsafe { OH_Drawing_FilterCreate() };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_FilterCreate returned null"),
        }
    }

    pub fn set_color_filter(&mut self, filter: Option<&ColorFilter>) {
        unsafe {
            OH_Drawing_FilterSetColorFilter(
                self.raw.as_ptr(),
                filter.map_or(std::ptr::null_mut(), ColorFilter::as_ptr),
            )
        };
    }

    pub fn set_mask_filter(&mut self, filter: Option<&MaskFilter>) {
        unsafe {
            OH_Drawing_FilterSetMaskFilter(
                self.raw.as_ptr(),
                filter.map_or(std::ptr::null_mut(), MaskFilter::as_ptr),
            )
        };
    }

    pub(crate) fn as_ptr(&self) -> *mut OH_Drawing_Filter {
        self.raw.as_ptr()
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Filter {
    fn drop(&mut self) {
        unsafe { OH_Drawing_FilterDestroy(self.raw.as_ptr()) };
    }
}
