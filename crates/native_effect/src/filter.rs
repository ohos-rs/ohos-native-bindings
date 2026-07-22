use ohos_native_effect_sys as sys;

use std::ptr::{self, NonNull};

use crate::error::{check, EffectError, Result};
#[cfg(feature = "api-14")]
use crate::types::TileMode;
use crate::types::{ColorMatrix, PixelMapHandle};

/// An effect filter built on top of a source pixel map.
///
/// A filter accumulates effects (blur, brighten, gray scale, invert, color
/// matrix) in the order they are added; [`Filter::effect_pixel_map`] then
/// renders them into a new pixel map. The native filter object is released
/// when the value is dropped.
#[derive(Debug)]
pub struct Filter {
    raw: NonNull<sys::OH_Filter>,
}

impl Filter {
    /// Create a filter for the given source pixel map.
    ///
    /// The source pixel map is only borrowed: it is not released with the
    /// filter and must stay alive while the filter is in use.
    pub fn create(pixel_map: PixelMapHandle) -> Result<Self> {
        let mut raw: *mut sys::OH_Filter = ptr::null_mut();
        let code = unsafe { sys::OH_Filter_CreateEffect(pixel_map.as_raw().cast(), &mut raw) };
        check(code)?;
        NonNull::new(raw)
            .map(|raw| Self { raw })
            .ok_or(EffectError::NullFilter)
    }

    /// Add a blur effect with the given radius.
    pub fn blur(&mut self, radius: f32) -> Result<()> {
        check(unsafe { sys::OH_Filter_Blur(self.raw.as_ptr(), radius) })
    }

    /// Add a blur effect with the given radius and tile mode.
    #[cfg(feature = "api-14")]
    pub fn blur_with_tile_mode(&mut self, radius: f32, tile_mode: TileMode) -> Result<()> {
        check(unsafe {
            sys::OH_Filter_BlurWithTileMode(self.raw.as_ptr(), radius, tile_mode.to_sys())
        })
    }

    /// Add a brighten effect with the given brightness.
    pub fn brighten(&mut self, brightness: f32) -> Result<()> {
        check(unsafe { sys::OH_Filter_Brighten(self.raw.as_ptr(), brightness) })
    }

    /// Add a gray scale effect.
    pub fn gray_scale(&mut self) -> Result<()> {
        check(unsafe { sys::OH_Filter_GrayScale(self.raw.as_ptr()) })
    }

    /// Add an invert effect.
    pub fn invert(&mut self) -> Result<()> {
        check(unsafe { sys::OH_Filter_Invert(self.raw.as_ptr()) })
    }

    /// Add a custom effect described by a 5x4 color matrix.
    pub fn set_color_matrix(&mut self, matrix: &ColorMatrix) -> Result<()> {
        // The native call takes a mutable pointer but only reads the matrix, so
        // a local copy is handed over and dropped afterwards.
        let mut raw_matrix = sys::OH_Filter_ColorMatrix {
            val: *matrix.values(),
        };
        check(unsafe { sys::OH_Filter_SetColorMatrix(self.raw.as_ptr(), &mut raw_matrix) })
    }

    /// Render the accumulated effects into a new pixel map.
    ///
    /// The returned handle points at a freshly created native pixel map that
    /// this crate does not own: the caller takes the ownership and must release
    /// it through the image bindings once done with it.
    pub fn effect_pixel_map(&self) -> Result<PixelMapHandle> {
        let mut pixel_map = ptr::null_mut();
        let code = unsafe { sys::OH_Filter_GetEffectPixelMap(self.raw.as_ptr(), &mut pixel_map) };
        check(code)?;
        // SAFETY: the runtime just handed out a pixel map it created.
        unsafe { PixelMapHandle::from_raw(pixel_map.cast()) }.ok_or(EffectError::NullPixelMap)
    }
}

impl Drop for Filter {
    fn drop(&mut self) {
        unsafe { sys::OH_Filter_Release(self.raw.as_ptr()) };
    }
}
