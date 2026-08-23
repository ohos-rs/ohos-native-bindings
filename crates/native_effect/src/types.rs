#[cfg(feature = "api-14")]
use ohos_native_effect_sys as sys;

use std::ffi::c_void;
use std::ptr::NonNull;

/// Number of entries in a [`ColorMatrix`]: a 5x4 matrix stored row by row.
pub const COLOR_MATRIX_LEN: usize = 20;

/// Borrowed, non-null handle to a native pixel map (`OH_PixelmapNative`).
///
/// The effect filter never takes ownership of a pixel map passed to it, and
/// this type deliberately has no `Drop`: whoever created the pixel map keeps
/// the ownership and stays responsible for releasing it. The handle is a plain
/// pointer view, so it must not outlive the pixel map it points at.
///
/// A pixel map produced by the image bindings can be adopted with
/// [`PixelMapHandle::from_raw`] from its erased pointer, and the pointer of a
/// handle obtained from [`crate::Filter::effect_pixel_map`] can be handed back
/// to the image bindings with [`PixelMapHandle::as_raw`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PixelMapHandle {
    raw: NonNull<c_void>,
}

impl PixelMapHandle {
    /// Wrap a raw native pixel map pointer, returning `None` when it is null.
    ///
    /// # Safety
    ///
    /// The pointer must be a valid `OH_PixelmapNative` that stays alive for as
    /// long as the returned handle is used.
    pub unsafe fn from_raw(raw: *mut c_void) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    /// The wrapped pixel map pointer, erased to `c_void`.
    pub fn as_raw(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

/// A 5x4 color matrix applied by [`crate::Filter::set_color_matrix`].
///
/// The 20 values are stored row by row: each of the four rows holds the red,
/// green, blue and alpha weights followed by a constant offset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorMatrix {
    values: [f32; COLOR_MATRIX_LEN],
}

impl ColorMatrix {
    /// The identity matrix, which leaves the colors of a pixel map unchanged.
    pub const IDENTITY: Self = Self {
        values: [
            1.0, 0.0, 0.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, 0.0, 0.0, //
            0.0, 0.0, 1.0, 0.0, 0.0, //
            0.0, 0.0, 0.0, 1.0, 0.0,
        ],
    };

    /// Build a color matrix from its 20 values, given row by row.
    pub const fn new(values: [f32; COLOR_MATRIX_LEN]) -> Self {
        Self { values }
    }

    /// The 20 values of the matrix, row by row.
    pub const fn values(&self) -> &[f32; COLOR_MATRIX_LEN] {
        &self.values
    }

    /// The 20 values of the matrix, row by row, for in-place editing.
    pub fn values_mut(&mut self) -> &mut [f32; COLOR_MATRIX_LEN] {
        &mut self.values
    }
}

impl Default for ColorMatrix {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl From<[f32; COLOR_MATRIX_LEN]> for ColorMatrix {
    fn from(values: [f32; COLOR_MATRIX_LEN]) -> Self {
        Self::new(values)
    }
}

impl From<ColorMatrix> for [f32; COLOR_MATRIX_LEN] {
    fn from(matrix: ColorMatrix) -> Self {
        matrix.values
    }
}

/// How a blur samples outside of the bounds of the original image.
#[cfg(feature = "api-14")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TileMode {
    /// Replicate the edge color outside of the original bounds.
    Clamp,
    /// Repeat the image horizontally and vertically.
    Repeat,
    /// Repeat the image horizontally and vertically, mirroring every other
    /// copy so that adjacent copies always seam.
    Mirror,
    /// Draw only within the original domain and return transparent black
    /// everywhere else.
    Decal,
}

#[cfg(feature = "api-14")]
impl TileMode {
    pub(crate) fn to_sys(self) -> sys::EffectTileMode {
        match self {
            TileMode::Clamp => sys::EffectTileMode_CLAMP,
            TileMode::Repeat => sys::EffectTileMode_REPEAT,
            TileMode::Mirror => sys::EffectTileMode_MIRROR,
            TileMode::Decal => sys::EffectTileMode_DECAL,
        }
    }
}
