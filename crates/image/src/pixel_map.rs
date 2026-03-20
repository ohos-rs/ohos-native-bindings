use std::{ffi::c_void, mem::MaybeUninit, ptr::NonNull};

#[cfg(feature = "napi")]
use napi_ohos::{
    bindgen_prelude::{Object, Unknown},
    Env,
};
use napi_sys_ohos::{napi_env, napi_value};

use crate::{
    common::{non_null, AsNapiValue, NapiValue},
    error::{check_status, ImageError, ImageResult},
    sys,
    types::{OhosPixelMapCreateOps, OhosPixelMapInfos, PixelMapAntiAliasingLevel},
};

/// JS-side `PixelMap` object wrapper.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PixelMap {
    raw: NapiValue,
}

impl PixelMap {
    /// Creates a wrapper from a raw N-API `PixelMap`.
    pub fn from_raw(raw: napi_value) -> Option<Self> {
        NapiValue::from_raw(raw).map(|raw| Self { raw })
    }

    /// Creates a wrapper from a `napi-ohos` object.
    #[cfg(feature = "napi")]
    pub fn from_object(value: &Object<'_>) -> Self {
        Self {
            raw: NapiValue::from_object(value),
        }
    }

    /// Creates a wrapper from a `napi-ohos` unknown value.
    #[cfg(feature = "napi")]
    pub fn from_unknown(value: &Unknown<'_>) -> Self {
        Self {
            raw: NapiValue::from_unknown(value),
        }
    }

    /// Returns the wrapped N-API `PixelMap`.
    pub fn as_raw(&self) -> napi_value {
        self.raw.as_raw()
    }

    /// Returns the wrapped pixel map as a `napi-ohos` object view.
    #[cfg(feature = "napi")]
    pub fn as_object<'env>(&self, env: &'env Env) -> Object<'env> {
        self.raw.as_object(env)
    }

    /// Returns the wrapped pixel map as a `napi-ohos` unknown view.
    #[cfg(feature = "napi")]
    pub fn as_unknown<'env>(&self, env: &'env Env) -> Unknown<'env> {
        self.raw.as_unknown(env)
    }

    /// Creates a JS-side `PixelMap` from a raw buffer.
    pub fn create(
        env: napi_env,
        info: OhosPixelMapCreateOps,
        buffer: &mut [u8],
    ) -> ImageResult<Self> {
        let mut pixel_map = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PixelMap_CreatePixelMap(
                env,
                info,
                buffer.as_mut_ptr().cast(),
                buffer.len(),
                &mut pixel_map,
            )
        })?;
        Self::from_raw(pixel_map).ok_or(ImageError { code: -1 })
    }

    /// Creates a JS-side `PixelMap` from a raw buffer with explicit stride.
    pub fn create_with_stride(
        env: napi_env,
        info: OhosPixelMapCreateOps,
        buffer: &mut [u8],
        row_stride: i32,
    ) -> ImageResult<Self> {
        let mut pixel_map = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PixelMap_CreatePixelMapWithStride(
                env,
                info,
                buffer.as_mut_ptr().cast(),
                buffer.len(),
                row_stride,
                &mut pixel_map,
            )
        })?;
        Self::from_raw(pixel_map).ok_or(ImageError { code: -1 })
    }

    /// Creates an alpha-only `PixelMap` from a source `PixelMap`.
    pub fn create_alpha(env: napi_env, source: &impl AsNapiValue) -> ImageResult<Self> {
        let mut pixel_map = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_PixelMap_CreateAlphaPixelMap(env, source.raw_napi_value(), &mut pixel_map)
        })?;
        Self::from_raw(pixel_map).ok_or(ImageError { code: -1 })
    }

    /// Converts the JS-side `PixelMap` into a native pixel-map wrapper.
    pub fn native(&self, env: napi_env) -> ImageResult<NativePixelMapHandle> {
        NativePixelMapHandle::from_napi(env, self.as_raw())
    }
}

impl AsNapiValue for PixelMap {
    fn raw_napi_value(&self) -> napi_value {
        self.as_raw()
    }
}

/// Borrowed native pixel-map wrapper.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativePixelMapHandle {
    raw: NonNull<sys::NativePixelMap>,
}

impl NativePixelMapHandle {
    /// Parses a native pixel-map from a JS-side `PixelMap`.
    pub fn from_napi(env: napi_env, pixel_map: napi_value) -> ImageResult<Self> {
        let raw = unsafe { sys::OH_PixelMap_InitNativePixelMap(env, pixel_map) };
        non_null(raw, "OH_PixelMap_InitNativePixelMap").map(|raw| Self { raw })
    }

    /// Creates a wrapper from a raw native pixel-map pointer.
    pub fn from_raw(raw: *mut sys::NativePixelMap) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    /// Returns the wrapped raw native pixel-map pointer.
    pub fn as_raw(&self) -> *mut sys::NativePixelMap {
        self.raw.as_ptr()
    }

    /// Returns bytes per row.
    pub fn bytes_number_per_row(&self) -> ImageResult<i32> {
        let mut bytes = 0;
        check_status(unsafe { sys::OH_PixelMap_GetBytesNumberPerRow(self.as_raw(), &mut bytes) })?;
        Ok(bytes)
    }

    /// Returns whether the pixel-map is editable.
    pub fn is_editable(&self) -> ImageResult<bool> {
        let mut editable = 0;
        check_status(unsafe { sys::OH_PixelMap_GetIsEditable(self.as_raw(), &mut editable) })?;
        Ok(editable != 0)
    }

    /// Returns whether the pixel-map supports alpha.
    pub fn supports_alpha(&self) -> ImageResult<bool> {
        let mut alpha = 0;
        check_status(unsafe { sys::OH_PixelMap_IsSupportAlpha(self.as_raw(), &mut alpha) })?;
        Ok(alpha != 0)
    }

    /// Enables or disables alpha.
    pub fn set_alpha_able(&self, alpha: bool) -> ImageResult<()> {
        check_status(unsafe { sys::OH_PixelMap_SetAlphaAble(self.as_raw(), i32::from(alpha)) })
    }

    /// Returns density.
    pub fn density(&self) -> ImageResult<i32> {
        let mut density = 0;
        check_status(unsafe { sys::OH_PixelMap_GetDensity(self.as_raw(), &mut density) })?;
        Ok(density)
    }

    /// Sets density.
    pub fn set_density(&self, density: i32) -> ImageResult<()> {
        check_status(unsafe { sys::OH_PixelMap_SetDensity(self.as_raw(), density) })
    }

    /// Sets opacity.
    pub fn set_opacity(&self, opacity: f32) -> ImageResult<()> {
        check_status(unsafe { sys::OH_PixelMap_SetOpacity(self.as_raw(), opacity) })
    }

    /// Scales the pixel-map.
    pub fn scale(&self, x: f32, y: f32) -> ImageResult<()> {
        check_status(unsafe { sys::OH_PixelMap_Scale(self.as_raw(), x, y) })
    }

    /// Scales the pixel-map with anti-aliasing.
    pub fn scale_with_anti_aliasing(
        &self,
        x: f32,
        y: f32,
        level: PixelMapAntiAliasingLevel,
    ) -> ImageResult<()> {
        check_status(unsafe {
            sys::OH_PixelMap_ScaleWithAntiAliasing(self.as_raw(), x, y, level.into())
        })
    }

    /// Translates the pixel-map.
    pub fn translate(&self, x: f32, y: f32) -> ImageResult<()> {
        check_status(unsafe { sys::OH_PixelMap_Translate(self.as_raw(), x, y) })
    }

    /// Rotates the pixel-map.
    pub fn rotate(&self, angle: f32) -> ImageResult<()> {
        check_status(unsafe { sys::OH_PixelMap_Rotate(self.as_raw(), angle) })
    }

    /// Flips the pixel-map.
    pub fn flip(&self, x: bool, y: bool) -> ImageResult<()> {
        check_status(unsafe { sys::OH_PixelMap_Flip(self.as_raw(), i32::from(x), i32::from(y)) })
    }

    /// Crops the pixel-map.
    pub fn crop(&self, x: i32, y: i32, width: i32, height: i32) -> ImageResult<()> {
        check_status(unsafe { sys::OH_PixelMap_Crop(self.as_raw(), x, y, width, height) })
    }

    /// Returns pixel-map info.
    pub fn image_info(&self) -> ImageResult<OhosPixelMapInfos> {
        let mut info = MaybeUninit::<OhosPixelMapInfos>::uninit();
        check_status(unsafe { sys::OH_PixelMap_GetImageInfo(self.as_raw(), info.as_mut_ptr()) })?;
        Ok(unsafe { info.assume_init() })
    }

    /// Locks the pixel buffer and returns its address.
    pub fn access_pixels(&self) -> ImageResult<NonNull<c_void>> {
        let mut addr = std::ptr::null_mut();
        check_status(unsafe { sys::OH_PixelMap_AccessPixels(self.as_raw(), &mut addr) })?;
        NonNull::new(addr).ok_or(ImageError { code: -1 })
    }

    /// Unlocks the pixel buffer.
    pub fn unaccess_pixels(&self) -> ImageResult<()> {
        check_status(unsafe { sys::OH_PixelMap_UnAccessPixels(self.as_raw()) })
    }
}

/// Backward-compatible alias.
pub type PixelMapNativeHandle = NativePixelMapHandle;
