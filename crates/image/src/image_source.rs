use std::{mem::MaybeUninit, ptr::NonNull};

#[cfg(feature = "napi")]
use napi_ohos::{
    bindgen_prelude::{Object, Unknown},
    Env,
};
use napi_sys_ohos::{napi_env, napi_value};

use crate::{
    common::{image_source_from_uri, non_null, AsNapiValue, NapiValue},
    error::{check_status, ImageError, ImageResult},
    pixel_map::PixelMap,
    sys,
    types::{
        OhosImageDecodingOps, OhosImageSource, OhosImageSourceDelayTimeList, OhosImageSourceInfo,
        OhosImageSourceOps, OhosImageSourceProperty, OhosImageSourceSupportedFormatList,
        OhosImageSourceUpdateData, RawFileDescriptor,
    },
};

/// JS-side `ImageSource` object wrapper.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageSource {
    raw: NapiValue,
}

impl ImageSource {
    /// Creates a wrapper from a raw N-API `ImageSource`.
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

    /// Returns the wrapped raw N-API `ImageSource`.
    pub fn as_raw(&self) -> napi_value {
        self.raw.as_raw()
    }

    /// Returns the wrapped source as a `napi-ohos` object view.
    #[cfg(feature = "napi")]
    pub fn as_object<'env>(&self, env: &'env Env) -> Object<'env> {
        self.raw.as_object(env)
    }

    /// Returns the wrapped source as a `napi-ohos` unknown view.
    #[cfg(feature = "napi")]
    pub fn as_unknown<'env>(&self, env: &'env Env) -> Unknown<'env> {
        self.raw.as_unknown(env)
    }

    /// Creates an `ImageSource` from a generic source descriptor.
    pub fn create(
        env: napi_env,
        src: &mut OhosImageSource,
        ops: &mut OhosImageSourceOps,
    ) -> ImageResult<Self> {
        let mut value = std::ptr::null_mut();
        check_status(unsafe { sys::OH_ImageSource_Create(env, src, ops, &mut value) })?;
        Self::from_raw(value).ok_or(ImageError { code: -1 })
    }

    /// Creates an `ImageSource` from a URI.
    pub fn create_from_uri(
        env: napi_env,
        uri: &str,
        ops: &mut OhosImageSourceOps,
    ) -> ImageResult<Self> {
        let mut value = std::ptr::null_mut();
        image_source_from_uri(env, uri, ops, &mut value)?;
        Self::from_raw(value).ok_or(ImageError { code: -1 })
    }

    /// Creates an `ImageSource` from a file descriptor.
    pub fn create_from_fd(
        env: napi_env,
        fd: i32,
        ops: &mut OhosImageSourceOps,
    ) -> ImageResult<Self> {
        let mut value = std::ptr::null_mut();
        check_status(unsafe { sys::OH_ImageSource_CreateFromFd(env, fd, ops, &mut value) })?;
        Self::from_raw(value).ok_or(ImageError { code: -1 })
    }

    /// Creates an `ImageSource` from in-memory data.
    pub fn create_from_data(
        env: napi_env,
        data: &mut [u8],
        ops: &mut OhosImageSourceOps,
    ) -> ImageResult<Self> {
        let mut value = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_ImageSource_CreateFromData(env, data.as_mut_ptr(), data.len(), ops, &mut value)
        })?;
        Self::from_raw(value).ok_or(ImageError { code: -1 })
    }

    /// Creates an `ImageSource` from a raw-file descriptor.
    pub fn create_from_raw_file(
        env: napi_env,
        raw_file: RawFileDescriptor,
        ops: &mut OhosImageSourceOps,
    ) -> ImageResult<Self> {
        let mut value = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_ImageSource_CreateFromRawFile(env, raw_file, ops, &mut value)
        })?;
        Self::from_raw(value).ok_or(ImageError { code: -1 })
    }

    /// Creates an incremental `ImageSource`.
    pub fn create_incremental(
        env: napi_env,
        src: &mut OhosImageSource,
        ops: &mut OhosImageSourceOps,
    ) -> ImageResult<Self> {
        let mut value = std::ptr::null_mut();
        check_status(unsafe { sys::OH_ImageSource_CreateIncremental(env, src, ops, &mut value) })?;
        Self::from_raw(value).ok_or(ImageError { code: -1 })
    }

    /// Creates an incremental `ImageSource` from memory data.
    pub fn create_incremental_from_data(
        env: napi_env,
        data: &mut [u8],
        ops: &mut OhosImageSourceOps,
    ) -> ImageResult<Self> {
        let mut value = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_ImageSource_CreateIncrementalFromData(
                env,
                data.as_mut_ptr(),
                data.len(),
                ops,
                &mut value,
            )
        })?;
        Self::from_raw(value).ok_or(ImageError { code: -1 })
    }

    /// Converts the JS-side object into a native image source wrapper.
    pub fn native(&self, env: napi_env) -> ImageResult<NativeImageSource> {
        NativeImageSource::from_napi(env, self)
    }

    /// Obtains supported decoding formats.
    pub fn supported_formats(
        supported_formats: &mut OhosImageSourceSupportedFormatList,
    ) -> ImageResult<()> {
        check_status(unsafe { sys::OH_ImageSource_GetSupportedFormats(supported_formats) })
    }
}

impl AsNapiValue for ImageSource {
    fn raw_napi_value(&self) -> napi_value {
        self.as_raw()
    }
}

/// Owned native image source wrapper.
pub struct NativeImageSource {
    raw: NonNull<sys::ImageSourceNative>,
}

impl NativeImageSource {
    /// Parses a native image source from a JS-side source object.
    pub fn from_napi<T: AsNapiValue>(env: napi_env, source: &T) -> ImageResult<Self> {
        let raw = unsafe { sys::OH_ImageSource_InitNative(env, source.raw_napi_value()) };
        non_null(raw, "OH_ImageSource_InitNative").map(|raw| Self { raw })
    }

    /// Creates a wrapper from a raw native image source pointer.
    pub fn from_raw(raw: *mut sys::ImageSourceNative) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    /// Returns the wrapped raw native image source pointer.
    pub fn as_raw(&self) -> *mut sys::ImageSourceNative {
        self.raw.as_ptr()
    }

    /// Decodes a `PixelMap` from the source.
    pub fn create_pixel_map(&self, ops: &mut OhosImageDecodingOps) -> ImageResult<PixelMap> {
        let mut value = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_ImageSource_CreatePixelMap(self.as_raw(), ops, &mut value)
        })?;
        PixelMap::from_raw(value).ok_or(ImageError { code: -1 })
    }

    /// Decodes a JS array of `PixelMap` objects from the source.
    pub fn create_pixel_map_list(&self, ops: &mut OhosImageDecodingOps) -> ImageResult<NapiValue> {
        let mut value = std::ptr::null_mut();
        check_status(unsafe {
            sys::OH_ImageSource_CreatePixelMapList(self.as_raw(), ops, &mut value)
        })?;
        NapiValue::from_raw(value).ok_or(ImageError { code: -1 })
    }

    /// Decodes a JS array of `PixelMap` objects and returns it as a `napi-ohos` object.
    #[cfg(feature = "napi")]
    pub fn create_pixel_map_list_object<'env>(
        &self,
        env: &'env Env,
        ops: &mut OhosImageDecodingOps,
    ) -> ImageResult<Object<'env>> {
        self.create_pixel_map_list(ops)
            .map(|value| value.as_object(env))
    }

    /// Decodes a JS array of `PixelMap` objects and returns it as a `napi-ohos` unknown value.
    #[cfg(feature = "napi")]
    pub fn create_pixel_map_list_unknown<'env>(
        &self,
        env: &'env Env,
        ops: &mut OhosImageDecodingOps,
    ) -> ImageResult<Unknown<'env>> {
        self.create_pixel_map_list(ops)
            .map(|value| value.as_unknown(env))
    }

    /// Returns delay-time information.
    pub fn delay_time(&self) -> ImageResult<OhosImageSourceDelayTimeList> {
        let mut info = MaybeUninit::<OhosImageSourceDelayTimeList>::uninit();
        check_status(unsafe {
            sys::OH_ImageSource_GetDelayTime(self.as_raw(), info.as_mut_ptr())
        })?;
        Ok(unsafe { info.assume_init() })
    }

    /// Returns the frame count.
    pub fn frame_count(&self) -> ImageResult<u32> {
        let mut count = 0;
        check_status(unsafe { sys::OH_ImageSource_GetFrameCount(self.as_raw(), &mut count) })?;
        Ok(count)
    }

    /// Returns image info for an indexed frame.
    pub fn image_info(&self, index: i32) -> ImageResult<OhosImageSourceInfo> {
        let mut info = MaybeUninit::<OhosImageSourceInfo>::uninit();
        check_status(unsafe {
            sys::OH_ImageSource_GetImageInfo(self.as_raw(), index, info.as_mut_ptr())
        })?;
        Ok(unsafe { info.assume_init() })
    }

    /// Reads an image property into the provided value buffer.
    pub fn get_image_property(
        &self,
        key: &mut OhosImageSourceProperty,
        value: &mut OhosImageSourceProperty,
    ) -> ImageResult<()> {
        check_status(unsafe { sys::OH_ImageSource_GetImageProperty(self.as_raw(), key, value) })
    }

    /// Modifies an image property.
    pub fn modify_image_property(
        &self,
        key: &mut OhosImageSourceProperty,
        value: &mut OhosImageSourceProperty,
    ) -> ImageResult<()> {
        check_status(unsafe { sys::OH_ImageSource_ModifyImageProperty(self.as_raw(), key, value) })
    }

    /// Updates incremental source data.
    pub fn update_data(&self, data: &mut OhosImageSourceUpdateData) -> ImageResult<()> {
        check_status(unsafe { sys::OH_ImageSource_UpdateData(self.as_raw(), data) })
    }
}

impl Drop for NativeImageSource {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_ImageSource_Release(self.as_raw()) });
    }
}
