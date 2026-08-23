use crate::error::{ColorSpaceError, Result};
use crate::name::ColorSpaceName;
use ohos_native_color_space_manager_sys as sys;
use std::ptr::NonNull;

/// A chromaticity coordinate pair `(x, y)` in the CIE 1931 xy space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Chromaticity {
    /// Coordinate value x.
    pub x: f32,
    /// Coordinate value y.
    pub y: f32,
}

impl Chromaticity {
    /// Build a coordinate pair.
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// The red, green and blue primaries plus the white point of a custom color
/// space, in terms of real world chromaticities.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorSpacePrimaries {
    /// Chromaticity of the red primary.
    pub red: Chromaticity,
    /// Chromaticity of the green primary.
    pub green: Chromaticity,
    /// Chromaticity of the blue primary.
    pub blue: Chromaticity,
    /// Chromaticity of the white point.
    pub white: Chromaticity,
}

impl ColorSpacePrimaries {
    fn to_raw(self) -> sys::ColorSpacePrimaries {
        sys::ColorSpacePrimaries {
            rX: self.red.x,
            rY: self.red.y,
            gX: self.green.x,
            gY: self.green.y,
            bX: self.blue.x,
            bY: self.blue.y,
            wX: self.white.x,
            wY: self.white.y,
        }
    }
}

/// An owned color space manager instance.
///
/// The instance is released on drop, so it is never leaked and never used after
/// it has been destroyed.
///
/// The handle is not thread safe: it is neither `Send` nor `Sync`, and the
/// native library gives no guarantee about sharing one instance across threads.
///
/// # Example
///
/// ```no_run
/// use ohos_native_color_space_manager_binding as color_space;
/// use color_space::{Chromaticity, ColorSpaceManager, ColorSpaceName, ColorSpacePrimaries};
///
/// // A standard color space, queried back by name.
/// let srgb = ColorSpaceManager::from_name(ColorSpaceName::Srgb)?;
/// assert_eq!(srgb.name()?, ColorSpaceName::Srgb);
///
/// // A custom color space, described by its primaries and gamma.
/// let custom = ColorSpaceManager::from_primaries_and_gamma(
///     ColorSpacePrimaries {
///         red: Chromaticity::new(0.64, 0.33),
///         green: Chromaticity::new(0.30, 0.60),
///         blue: Chromaticity::new(0.15, 0.06),
///         white: Chromaticity::new(0.3127, 0.3290),
///     },
///     2.2,
/// )?;
/// println!("white point: {:?}", custom.white_point()?);
/// println!("gamma: {}", custom.gamma()?);
/// # Ok::<(), color_space::ColorSpaceError>(())
/// ```
pub struct ColorSpaceManager {
    handle: NonNull<sys::OH_NativeColorSpaceManager>,
}

impl ColorSpaceManager {
    /// Create an instance for a standard color space.
    pub fn from_name(name: ColorSpaceName) -> Result<Self> {
        // SAFETY: the name is a value of the native enum; the call allocates a
        // new instance and returns null on failure.
        let raw = unsafe { sys::OH_NativeColorSpaceManager_CreateFromName(name.to_raw()) };
        Self::from_raw(raw)
    }

    /// Create an instance for a custom color space described by its primaries
    /// and gamma.
    pub fn from_primaries_and_gamma(primaries: ColorSpacePrimaries, gamma: f32) -> Result<Self> {
        // SAFETY: the primaries struct is passed by value and the call
        // allocates a new instance, returning null on failure.
        let raw = unsafe {
            sys::OH_NativeColorSpaceManager_CreateFromPrimariesAndGamma(primaries.to_raw(), gamma)
        };
        Self::from_raw(raw)
    }

    fn from_raw(raw: *mut sys::OH_NativeColorSpaceManager) -> Result<Self> {
        NonNull::new(raw)
            .map(|handle| Self { handle })
            .ok_or(ColorSpaceError::CreationFailed)
    }

    /// The color space this instance describes.
    ///
    /// Custom color spaces report [`ColorSpaceName::Custom`].
    pub fn name(&self) -> Result<ColorSpaceName> {
        // SAFETY: the handle is non-null and owned by this instance.
        let value = unsafe { sys::OH_NativeColorSpaceManager_GetColorSpaceName(self.as_ptr()) };
        if value <= 0 {
            return Err(ColorSpaceError::NameUnavailable);
        }
        ColorSpaceName::from_raw(value as u32)
    }

    /// The white point of this color space.
    pub fn white_point(&self) -> Result<Chromaticity> {
        // SAFETY: the handle is non-null and owned by this instance; the result
        // is returned by value.
        let array = unsafe { sys::OH_NativeColorSpaceManager_GetWhitePoint(self.as_ptr()) };
        let point = Chromaticity::new(array.arr[0], array.arr[1]);
        if point.x == 0.0 && point.y == 0.0 {
            return Err(ColorSpaceError::WhitePointUnavailable);
        }
        Ok(point)
    }

    /// The gamma of this color space.
    pub fn gamma(&self) -> Result<f32> {
        // SAFETY: the handle is non-null and owned by this instance.
        let gamma = unsafe { sys::OH_NativeColorSpaceManager_GetGamma(self.as_ptr()) };
        if gamma == 0.0 {
            return Err(ColorSpaceError::GammaUnavailable);
        }
        Ok(gamma)
    }

    fn as_ptr(&self) -> *mut sys::OH_NativeColorSpaceManager {
        self.handle.as_ptr()
    }
}

impl Drop for ColorSpaceManager {
    fn drop(&mut self) {
        // SAFETY: the handle was created by one of the native create functions,
        // is destroyed exactly once here, and is not used afterwards.
        unsafe { sys::OH_NativeColorSpaceManager_Destroy(self.as_ptr()) };
    }
}
