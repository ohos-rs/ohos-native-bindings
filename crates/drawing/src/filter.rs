use std::ptr::NonNull;

use ohos_native_drawing_sys::{
    OH_Drawing_BlurType_NORMAL, OH_Drawing_ColorFilter, OH_Drawing_ColorFilterCreateMatrix,
    OH_Drawing_ColorFilterDestroy, OH_Drawing_Filter, OH_Drawing_FilterCreate,
    OH_Drawing_FilterDestroy, OH_Drawing_FilterSetColorFilter, OH_Drawing_FilterSetMaskFilter,
    OH_Drawing_MaskFilter, OH_Drawing_MaskFilterCreateBlur, OH_Drawing_MaskFilterDestroy,
};

#[derive(Debug)]
pub struct ColorFilter {
    raw: NonNull<OH_Drawing_ColorFilter>,
}

impl ColorFilter {
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

impl Filter {
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
