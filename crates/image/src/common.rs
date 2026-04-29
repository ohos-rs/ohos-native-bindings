use std::{ffi::CString, ptr::NonNull};

#[cfg(feature = "napi")]
use napi_ohos::{
    bindgen_prelude::{Object, Unknown},
    Env, JsValue,
};
use napi_sys_ohos::napi_value;

use crate::{
    error::{check_status, ImageError, ImageResult},
    sys,
};

/// Trait for wrappers backed by an N-API value.
pub trait AsNapiValue {
    /// Returns the wrapped raw N-API value.
    fn raw_napi_value(&self) -> napi_value;
}

/// Borrowed N-API value wrapper used by JS-side image objects.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NapiValue {
    raw: NonNull<std::ffi::c_void>,
}

impl NapiValue {
    /// Creates a wrapper from a raw N-API value.
    pub fn from_raw(raw: napi_value) -> Option<Self> {
        NonNull::new(raw.cast()).map(|raw| Self { raw })
    }

    /// Creates a wrapper from any `napi-ohos` JS value.
    #[cfg(feature = "napi")]
    pub fn from_js_value<'env, V>(value: &V) -> Self
    where
        V: JsValue<'env>,
    {
        Self::from_raw(value.raw()).expect("napi-ohos value should not be null")
    }

    /// Creates a wrapper from a `napi-ohos` unknown value.
    #[cfg(feature = "napi")]
    pub fn from_unknown(value: &Unknown<'_>) -> Self {
        Self::from_js_value(value)
    }

    /// Creates a wrapper from a `napi-ohos` object value.
    #[cfg(feature = "napi")]
    pub fn from_object(value: &Object<'_>) -> Self {
        Self::from_js_value(value)
    }

    /// Returns the wrapped raw N-API value.
    pub fn as_raw(&self) -> napi_value {
        self.raw.as_ptr().cast()
    }

    /// Creates a `napi-ohos` unknown view from the wrapped raw value.
    #[cfg(feature = "napi")]
    pub fn as_unknown<'env>(&self, env: &'env Env) -> Unknown<'env> {
        unsafe { Unknown::from_raw_unchecked(env.raw(), self.as_raw()) }
    }

    /// Creates a `napi-ohos` object view from the wrapped raw value.
    #[cfg(feature = "napi")]
    pub fn as_object<'env>(&self, env: &'env Env) -> Object<'env> {
        Object::from_raw(env.raw(), self.as_raw())
    }
}

impl AsNapiValue for NapiValue {
    fn raw_napi_value(&self) -> napi_value {
        self.as_raw()
    }
}

#[cfg(feature = "napi")]
impl AsNapiValue for Unknown<'_> {
    fn raw_napi_value(&self) -> napi_value {
        self.raw()
    }
}

#[cfg(feature = "napi")]
impl AsNapiValue for Object<'_> {
    fn raw_napi_value(&self) -> napi_value {
        self.raw()
    }
}

pub(crate) fn non_null<T>(raw: *mut T, context: &'static str) -> ImageResult<NonNull<T>> {
    NonNull::new(raw)
        .ok_or(ImageError { code: -1 })
        .map_err(|_| {
            let _ = context;
            ImageError { code: -1 }
        })
}

pub(crate) fn cstring(value: &str) -> ImageResult<CString> {
    CString::new(value).map_err(|_| ImageError { code: -1 })
}

pub(crate) fn fill_char_buffer<F>(initial_len: usize, mut f: F) -> ImageResult<String>
where
    F: FnMut(*mut std::os::raw::c_char, usize) -> i32,
{
    let mut len = initial_len.max(1);
    loop {
        let mut buf = vec![0_u8; len];
        match check_status(f(buf.as_mut_ptr().cast(), buf.len())) {
            Ok(()) => {
                let end = buf.iter().position(|byte| *byte == 0).unwrap_or(buf.len());
                return Ok(String::from_utf8_lossy(&buf[..end]).into_owned());
            }
            Err(err) if len < 4096 => {
                let _ = err;
                len *= 2;
            }
            Err(err) => return Err(err),
        }
    }
}

pub(crate) fn image_source_from_uri(
    env: napi_sys_ohos::napi_env,
    uri: &str,
    ops: &mut sys::OhosImageSourceOps,
    out: *mut napi_value,
) -> ImageResult<()> {
    let uri = cstring(uri)?;
    check_status(unsafe {
        sys::OH_ImageSource_CreateFromUri(
            env,
            uri.as_ptr().cast_mut(),
            uri.as_bytes().len(),
            ops,
            out,
        )
    })
}
