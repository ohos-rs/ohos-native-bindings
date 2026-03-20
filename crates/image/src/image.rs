use std::{mem::MaybeUninit, ptr::NonNull};

#[cfg(feature = "napi")]
use napi_ohos::{
    bindgen_prelude::{Object, Unknown},
    Env,
};
use napi_sys_ohos::{napi_env, napi_value};

use crate::{
    common::{non_null, AsNapiValue, NapiValue},
    error::{check_status, ImageResult},
    sys,
    types::{OhosImageComponent, OhosImageRect, OhosImageSize},
};

/// JS-side `Image` object wrapper.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Image {
    raw: NapiValue,
}

impl Image {
    /// Creates a wrapper from a raw N-API image object.
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

    /// Returns the wrapped N-API image object.
    pub fn as_raw(&self) -> napi_value {
        self.raw.as_raw()
    }

    /// Returns the wrapped image as a `napi-ohos` object view.
    #[cfg(feature = "napi")]
    pub fn as_object<'env>(&self, env: &'env Env) -> Object<'env> {
        self.raw.as_object(env)
    }

    /// Returns the wrapped image as a `napi-ohos` unknown view.
    #[cfg(feature = "napi")]
    pub fn as_unknown<'env>(&self, env: &'env Env) -> Unknown<'env> {
        self.raw.as_unknown(env)
    }

    /// Converts the JS-side image into a native image wrapper.
    pub fn native(&self, env: napi_env) -> ImageResult<NativeImage> {
        NativeImage::from_napi(env, self)
    }
}

impl AsNapiValue for Image {
    fn raw_napi_value(&self) -> napi_value {
        self.as_raw()
    }
}

/// Owned native image parsed from a JS `Image` object.
pub struct NativeImage {
    raw: NonNull<sys::ImageNative>,
}

impl NativeImage {
    /// Parses a native image from any JS-side image wrapper.
    pub fn from_napi<T: AsNapiValue>(env: napi_env, image: &T) -> ImageResult<Self> {
        let raw = unsafe { sys::OH_Image_InitImageNative(env, image.raw_napi_value()) };
        non_null(raw, "OH_Image_InitImageNative").map(|raw| Self { raw })
    }

    /// Creates a wrapper from a raw native image pointer.
    pub fn from_raw(raw: *mut sys::ImageNative) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    /// Returns the wrapped raw native image pointer.
    pub fn as_raw(&self) -> *mut sys::ImageNative {
        self.raw.as_ptr()
    }

    /// Returns clipping rectangle information.
    pub fn clip_rect(&self) -> ImageResult<OhosImageRect> {
        let mut rect = MaybeUninit::<OhosImageRect>::uninit();
        check_status(unsafe { sys::OH_Image_ClipRect(self.as_raw(), rect.as_mut_ptr()) })?;
        Ok(unsafe { rect.assume_init() })
    }

    /// Returns image size.
    pub fn size(&self) -> ImageResult<OhosImageSize> {
        let mut size = MaybeUninit::<OhosImageSize>::uninit();
        check_status(unsafe { sys::OH_Image_Size(self.as_raw(), size.as_mut_ptr()) })?;
        Ok(unsafe { size.assume_init() })
    }

    /// Returns image format value.
    pub fn format(&self) -> ImageResult<i32> {
        let mut format = 0;
        check_status(unsafe { sys::OH_Image_Format(self.as_raw(), &mut format) })?;
        Ok(format)
    }

    /// Returns a single image component for the given component type.
    pub fn component(&self, component_type: i32) -> ImageResult<OhosImageComponent> {
        let mut component = MaybeUninit::<OhosImageComponent>::uninit();
        check_status(unsafe {
            sys::OH_Image_GetComponent(self.as_raw(), component_type, component.as_mut_ptr())
        })?;
        Ok(unsafe { component.assume_init() })
    }
}

impl Drop for NativeImage {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_Image_Release(self.as_raw()) });
    }
}
