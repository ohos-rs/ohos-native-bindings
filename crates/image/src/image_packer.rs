use std::ptr::NonNull;

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
    types::ImagePacker_Opts,
};

/// JS-side `ImagePacker` object wrapper.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImagePacker {
    raw: NapiValue,
}

impl ImagePacker {
    /// Creates a JS-side `ImagePacker`.
    pub fn create(env: napi_env) -> ImageResult<Self> {
        let mut value = std::ptr::null_mut();
        check_status(unsafe { sys::OH_ImagePacker_Create(env, &mut value) })?;
        Self::from_raw(value).ok_or(ImageError { code: -1 })
    }

    /// Creates a wrapper from a raw N-API value.
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

    /// Returns the wrapped raw N-API value.
    pub fn as_raw(&self) -> napi_value {
        self.raw.as_raw()
    }

    /// Returns the wrapped packer as a `napi-ohos` object view.
    #[cfg(feature = "napi")]
    pub fn as_object<'env>(&self, env: &'env Env) -> Object<'env> {
        self.raw.as_object(env)
    }

    /// Returns the wrapped packer as a `napi-ohos` unknown view.
    #[cfg(feature = "napi")]
    pub fn as_unknown<'env>(&self, env: &'env Env) -> Unknown<'env> {
        self.raw.as_unknown(env)
    }

    /// Converts the JS-side `ImagePacker` into a native packer.
    pub fn native(&self, env: napi_env) -> ImageResult<NativeImagePacker> {
        NativeImagePacker::from_napi(env, self)
    }
}

impl AsNapiValue for ImagePacker {
    fn raw_napi_value(&self) -> napi_value {
        self.as_raw()
    }
}

/// Owned native image packer wrapper.
pub struct NativeImagePacker {
    raw: NonNull<sys::ImagePacker_Native>,
}

impl NativeImagePacker {
    /// Parses a native image packer from a JS-side packer.
    pub fn from_napi<T: AsNapiValue>(env: napi_env, packer: &T) -> ImageResult<Self> {
        let raw = unsafe { sys::OH_ImagePacker_InitNative(env, packer.raw_napi_value()) };
        non_null(raw, "OH_ImagePacker_InitNative").map(|raw| Self { raw })
    }

    /// Creates a wrapper from a raw native image packer.
    pub fn from_raw(raw: *mut sys::ImagePacker_Native) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    /// Returns the wrapped raw native image packer pointer.
    pub fn as_raw(&self) -> *mut sys::ImagePacker_Native {
        self.raw.as_ptr()
    }

    /// Packs a source object into a memory buffer.
    pub fn pack_to_data<T: AsNapiValue>(
        &mut self,
        source: &T,
        opts: &mut ImagePacker_Opts,
        out_data: &mut [u8],
    ) -> ImageResult<usize> {
        let mut size = out_data.len();
        check_status(unsafe {
            sys::OH_ImagePacker_PackToData(
                self.as_raw(),
                source.raw_napi_value(),
                opts,
                out_data.as_mut_ptr(),
                &mut size,
            )
        })?;
        Ok(size)
    }

    /// Packs a source object into a file descriptor.
    pub fn pack_to_file<T: AsNapiValue>(
        &mut self,
        source: &T,
        opts: &mut ImagePacker_Opts,
        fd: i32,
    ) -> ImageResult<()> {
        check_status(unsafe {
            sys::OH_ImagePacker_PackToFile(self.as_raw(), source.raw_napi_value(), opts, fd)
        })
    }
}

impl Drop for NativeImagePacker {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_ImagePacker_Release(self.as_raw()) });
    }
}
