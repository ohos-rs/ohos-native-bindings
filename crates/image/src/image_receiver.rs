use std::{mem::MaybeUninit, ptr::NonNull};

#[cfg(feature = "napi")]
use napi_ohos::{
    bindgen_prelude::{Object, Unknown},
    Env,
};
use napi_sys_ohos::{napi_env, napi_value};

use crate::{
    common::{fill_char_buffer, non_null, AsNapiValue, NapiValue},
    error::{check_status, ImageError, ImageResult},
    image::Image,
    sys,
    types::{OhosImageReceiverInfo, OhosImageSize},
};

/// JS-side `ImageReceiver` object wrapper.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageReceiver {
    raw: NapiValue,
}

impl ImageReceiver {
    /// Creates a JS-side `ImageReceiver`.
    pub fn create(env: napi_env, info: OhosImageReceiverInfo) -> ImageResult<Self> {
        let mut value = std::ptr::null_mut();
        check_status(unsafe { sys::OH_Image_Receiver_CreateImageReceiver(env, info, &mut value) })?;
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

    /// Returns the wrapped receiver as a `napi-ohos` object view.
    #[cfg(feature = "napi")]
    pub fn as_object<'env>(&self, env: &'env Env) -> Object<'env> {
        self.raw.as_object(env)
    }

    /// Returns the wrapped receiver as a `napi-ohos` unknown view.
    #[cfg(feature = "napi")]
    pub fn as_unknown<'env>(&self, env: &'env Env) -> Unknown<'env> {
        self.raw.as_unknown(env)
    }

    /// Converts the JS-side object into a native receiver wrapper.
    pub fn native(&self, env: napi_env) -> ImageResult<NativeImageReceiver> {
        NativeImageReceiver::from_napi(env, self)
    }
}

impl AsNapiValue for ImageReceiver {
    fn raw_napi_value(&self) -> napi_value {
        self.as_raw()
    }
}

/// Owned native image receiver wrapper.
pub struct NativeImageReceiver {
    raw: NonNull<sys::ImageReceiverNative>,
}

impl NativeImageReceiver {
    /// Parses a native receiver from a JS-side receiver.
    pub fn from_napi<T: AsNapiValue>(env: napi_env, receiver: &T) -> ImageResult<Self> {
        let raw = unsafe {
            sys::OH_Image_Receiver_InitImageReceiverNative(env, receiver.raw_napi_value())
        };
        non_null(raw, "OH_Image_Receiver_InitImageReceiverNative").map(|raw| Self { raw })
    }

    /// Creates a wrapper from a raw native receiver pointer.
    pub fn from_raw(raw: *mut sys::ImageReceiverNative) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    /// Returns the wrapped raw native receiver pointer.
    pub fn as_raw(&self) -> *mut sys::ImageReceiverNative {
        self.raw.as_ptr()
    }

    /// Returns the receiving surface ID string.
    pub fn receiving_surface_id(&self) -> ImageResult<String> {
        fill_char_buffer(128, |ptr, len| unsafe {
            sys::OH_Image_Receiver_GetReceivingSurfaceId(self.as_raw(), ptr, len)
        })
    }

    /// Reads the latest image.
    pub fn read_latest_image(&self) -> ImageResult<Image> {
        let mut image = std::ptr::null_mut();
        check_status(unsafe { sys::OH_Image_Receiver_ReadLatestImage(self.as_raw(), &mut image) })?;
        Image::from_raw(image).ok_or(ImageError { code: -1 })
    }

    /// Reads the next image.
    pub fn read_next_image(&self) -> ImageResult<Image> {
        let mut image = std::ptr::null_mut();
        check_status(unsafe { sys::OH_Image_Receiver_ReadNextImage(self.as_raw(), &mut image) })?;
        Image::from_raw(image).ok_or(ImageError { code: -1 })
    }

    /// Registers the raw receiver callback.
    pub fn on_raw(&self, callback: sys::OH_Image_Receiver_On_Callback) -> ImageResult<()> {
        check_status(unsafe { sys::OH_Image_Receiver_On(self.as_raw(), callback) })
    }

    /// Returns receiver size.
    pub fn size(&self) -> ImageResult<OhosImageSize> {
        let mut size = MaybeUninit::<OhosImageSize>::uninit();
        check_status(unsafe { sys::OH_Image_Receiver_GetSize(self.as_raw(), size.as_mut_ptr()) })?;
        Ok(unsafe { size.assume_init() })
    }

    /// Returns receiver capacity.
    pub fn capacity(&self) -> ImageResult<i32> {
        let mut capacity = 0;
        check_status(unsafe { sys::OH_Image_Receiver_GetCapacity(self.as_raw(), &mut capacity) })?;
        Ok(capacity)
    }

    /// Returns receiver format.
    pub fn format(&self) -> ImageResult<i32> {
        let mut format = 0;
        check_status(unsafe { sys::OH_Image_Receiver_GetFormat(self.as_raw(), &mut format) })?;
        Ok(format)
    }
}

impl Drop for NativeImageReceiver {
    fn drop(&mut self) {
        let _ = check_status(unsafe { sys::OH_Image_Receiver_Release(self.as_raw()) });
    }
}
