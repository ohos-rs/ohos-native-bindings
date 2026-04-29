use std::{
    ffi::{c_void, CString},
    ptr::NonNull,
};

use napi_sys_ohos::napi_value;

use crate::{
    error::{check_status, ImageNativeError, ImageNativeResult},
    sys,
};

/// Owned wrapper for `Image_String`.
pub struct ImageString {
    raw: sys::Image_String,
    _owned: Option<CString>,
    free_on_drop: bool,
}

impl Default for ImageString {
    fn default() -> Self {
        Self::empty()
    }
}

impl ImageString {
    /// Creates an empty string for output parameters.
    pub fn empty() -> Self {
        Self {
            raw: sys::Image_String {
                data: std::ptr::null_mut(),
                size: 0,
            },
            _owned: None,
            free_on_drop: false,
        }
    }

    /// Creates a string from a Rust `&str`.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(value: &str) -> ImageNativeResult<Self> {
        let owned = CString::new(value).map_err(|_| ImageNativeError {
            code: sys::Image_ErrorCode_IMAGE_BAD_PARAMETER,
        })?;
        Ok(Self {
            raw: sys::Image_String {
                data: owned.as_ptr().cast_mut(),
                size: owned.as_bytes().len(),
            },
            _owned: Some(owned),
            free_on_drop: false,
        })
    }

    /// Returns the wrapped raw string.
    pub fn as_raw(&self) -> sys::Image_String {
        self.raw
    }

    /// Returns the mutable raw pointer used by sys output parameters.
    pub fn as_mut_ptr(&mut self) -> *mut sys::Image_String {
        &mut self.raw
    }

    /// Returns the string contents as UTF-8-lossy text.
    pub fn to_string_lossy(&self) -> Option<String> {
        if self.raw.data.is_null() || self.raw.size == 0 {
            None
        } else {
            let bytes =
                unsafe { std::slice::from_raw_parts(self.raw.data.cast::<u8>(), self.raw.size) };
            Some(String::from_utf8_lossy(bytes).into_owned())
        }
    }

    pub(crate) fn mark_free_on_drop(&mut self) {
        self.free_on_drop = !self.raw.data.is_null();
    }
}

impl std::str::FromStr for ImageString {
    type Err = ImageNativeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str(s)
    }
}

impl Drop for ImageString {
    fn drop(&mut self) {
        if self.free_on_drop && !self.raw.data.is_null() {
            unsafe {
                libc::free(self.raw.data.cast());
            }
            self.raw.data = std::ptr::null_mut();
            self.raw.size = 0;
        }
    }
}

pub(crate) fn output_string<F>(f: F) -> ImageNativeResult<ImageString>
where
    F: FnOnce(*mut sys::Image_String) -> sys::Image_ErrorCode,
{
    let mut value = ImageString::empty();
    check_status(f(value.as_mut_ptr()))?;
    value.mark_free_on_drop();
    Ok(value)
}

pub(crate) fn non_null<T>(
    raw: *mut T,
    code: sys::Image_ErrorCode,
) -> ImageNativeResult<NonNull<T>> {
    NonNull::new(raw).ok_or(ImageNativeError { code })
}

pub(crate) fn maybe_non_null<T>(raw: *mut T) -> Option<NonNull<T>> {
    NonNull::new(raw)
}

#[cfg(feature = "api-20")]
pub(crate) fn collect_mime_types(ptr: *mut sys::Image_MimeType, length: usize) -> Vec<String> {
    if ptr.is_null() || length == 0 {
        return Vec::new();
    }
    let values = unsafe { std::slice::from_raw_parts(ptr, length) };
    let mut result = Vec::with_capacity(length);
    for value in values {
        if !value.data.is_null() && value.size > 0 {
            let bytes = unsafe { std::slice::from_raw_parts(value.data.cast::<u8>(), value.size) };
            result.push(String::from_utf8_lossy(bytes).into_owned());
            unsafe {
                libc::free(value.data.cast());
            }
        } else {
            result.push(String::new());
        }
    }
    unsafe {
        libc::free(ptr.cast());
    }
    result
}

/// Borrowed handle for `OH_NativeBuffer`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativeBufferHandle {
    raw: NonNull<ohos_native_buffer_sys::OH_NativeBuffer>,
}

impl NativeBufferHandle {
    /// Creates a handle from a raw native-buffer pointer.
    pub fn from_raw(raw: *mut ohos_native_buffer_sys::OH_NativeBuffer) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    /// Returns the wrapped raw pointer.
    pub fn as_raw(&self) -> *mut ohos_native_buffer_sys::OH_NativeBuffer {
        self.raw.as_ptr()
    }
}

/// Borrowed handle for `OH_NativeColorSpaceManager`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NativeColorSpaceManagerHandle {
    raw: NonNull<sys::OH_NativeColorSpaceManager>,
}

impl NativeColorSpaceManagerHandle {
    /// Creates a handle from a raw color-space pointer.
    pub fn from_raw(raw: *mut sys::OH_NativeColorSpaceManager) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    /// Returns the wrapped raw pointer.
    pub fn as_raw(&self) -> *mut sys::OH_NativeColorSpaceManager {
        self.raw.as_ptr()
    }
}

/// Borrowed non-null handle for native pixel-map objects.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PixelMapNativeHandle {
    raw: NonNull<c_void>,
}

impl PixelMapNativeHandle {
    /// Creates a handle from a raw native pixel-map pointer.
    pub fn from_raw(raw: *mut sys::OH_PixelmapNative) -> Option<Self> {
        NonNull::new(raw.cast()).map(|raw| Self { raw })
    }

    /// Creates a handle from an erased raw pointer.
    pub fn from_erased_raw(raw: *mut c_void) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    /// Returns the wrapped raw pointer as `c_void`.
    pub fn as_raw(&self) -> *mut c_void {
        self.raw.as_ptr()
    }
}

pub(crate) fn napi_value_or_error(raw: napi_value) -> ImageNativeResult<napi_value> {
    if raw.is_null() {
        Err(ImageNativeError {
            code: sys::Image_ErrorCode_IMAGE_BAD_PARAMETER,
        })
    } else {
        Ok(raw)
    }
}
