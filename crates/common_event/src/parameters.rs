use crate::error::{CommonEventError, Result};
use ohos_common_event_sys as sys;
use std::ffi::{c_char, c_long, CString};
use std::marker::PhantomData;
use std::ptr::NonNull;

#[cfg(feature = "api-18")]
use crate::error::check;

/// The key/value payload carried by a common event, borrowed from whatever owns
/// it.
///
/// A view is handed out by [`RcvData::parameters`](crate::RcvData::parameters),
/// where it borrows data the common event service allocated for the duration of
/// the callback, and by [`Parameters::view`], where it borrows an owned payload.
/// Its lifetime keeps it from escaping either.
///
/// Every getter takes the default to report when the key is missing or holds a
/// different type, mirroring the native API. The array getters copy the values
/// out, because the native buffer belongs to the payload and is released with
/// it.
#[derive(Clone, Copy)]
pub struct ParametersRef<'a> {
    raw: NonNull<sys::CommonEvent_Parameters>,
    _marker: PhantomData<&'a sys::CommonEvent_Parameters>,
}

impl<'a> ParametersRef<'a> {
    /// Wrap a payload pointer owned by something that outlives `'a`.
    pub(crate) fn from_raw(raw: NonNull<sys::CommonEvent_Parameters>) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// Whether the payload holds a value under `key`.
    pub fn has_key(&self, key: &str) -> Result<bool> {
        let key = CString::new(key)?;
        // SAFETY: the payload is borrowed for `'a` and the key outlives the call.
        Ok(unsafe { sys::OH_CommonEvent_HasKeyInParameters(self.raw.as_ptr(), key.as_ptr()) })
    }

    /// The `int` value under `key`, or `default`.
    pub fn int(&self, key: &str, default: i32) -> Result<i32> {
        let key = CString::new(key)?;
        // SAFETY: the payload is borrowed for `'a` and the key outlives the call.
        let value = unsafe {
            sys::OH_CommonEvent_GetIntFromParameters(self.raw.as_ptr(), key.as_ptr(), default)
        };
        Ok(value)
    }

    /// The `long` value under `key`, or `default`.
    ///
    /// The native type is a C `long`, which is 32 bits wide on 32-bit targets;
    /// a `default` that does not fit it is rejected with
    /// [`CommonEventError::OutOfRange`].
    pub fn long(&self, key: &str, default: i64) -> Result<i64> {
        let key = CString::new(key)?;
        let default = c_long::try_from(default).map_err(|_| CommonEventError::OutOfRange)?;
        // SAFETY: the payload is borrowed for `'a` and the key outlives the call.
        let value = unsafe {
            sys::OH_CommonEvent_GetLongFromParameters(self.raw.as_ptr(), key.as_ptr(), default)
        };
        // The conversion widens on targets whose C `long` is 32 bits and is the
        // identity on the others.
        #[allow(clippy::useless_conversion)]
        Ok(i64::from(value))
    }

    /// The `bool` value under `key`, or `default`.
    pub fn bool(&self, key: &str, default: bool) -> Result<bool> {
        let key = CString::new(key)?;
        // SAFETY: the payload is borrowed for `'a` and the key outlives the call.
        let value = unsafe {
            sys::OH_CommonEvent_GetBoolFromParameters(self.raw.as_ptr(), key.as_ptr(), default)
        };
        Ok(value)
    }

    /// The `char` value under `key`, or `default`, as a raw byte.
    pub fn char(&self, key: &str, default: u8) -> Result<u8> {
        let key = CString::new(key)?;
        // SAFETY: the payload is borrowed for `'a` and the key outlives the call.
        let value = unsafe {
            sys::OH_CommonEvent_GetCharFromParameters(
                self.raw.as_ptr(),
                key.as_ptr(),
                default as c_char,
            )
        };
        Ok(value as u8)
    }

    /// The `double` value under `key`, or `default`.
    pub fn double(&self, key: &str, default: f64) -> Result<f64> {
        let key = CString::new(key)?;
        // SAFETY: the payload is borrowed for `'a` and the key outlives the call.
        let value = unsafe {
            sys::OH_CommonEvent_GetDoubleFromParameters(self.raw.as_ptr(), key.as_ptr(), default)
        };
        Ok(value)
    }

    /// The `int` array under `key`, empty when there is none.
    pub fn int_array(&self, key: &str) -> Result<Vec<i32>> {
        self.array(key, sys::OH_CommonEvent_GetIntArrayFromParameters)
    }

    /// The `long` array under `key`, empty when there is none.
    ///
    /// The elements are widened from the C `long` of the target.
    pub fn long_array(&self, key: &str) -> Result<Vec<i64>> {
        let values = self.array(key, sys::OH_CommonEvent_GetLongArrayFromParameters)?;
        // Widening, as in `long`.
        #[allow(clippy::useless_conversion)]
        Ok(values.into_iter().map(i64::from).collect())
    }

    /// The `bool` array under `key`, empty when there is none.
    pub fn bool_array(&self, key: &str) -> Result<Vec<bool>> {
        let bytes = self.array_bytes(key, sys::OH_CommonEvent_GetBoolArrayFromParameters)?;
        Ok(bytes.into_iter().map(|byte| byte != 0).collect())
    }

    /// The `char` array under `key` as raw bytes, empty when there is none.
    ///
    /// The native layer keeps a `char` array as a string rather than as an
    /// array: `OH_CommonEvent_GetCharArrayFromParameters` copies the stored
    /// `std::string` into a buffer of `length() + 1` bytes and reports that
    /// length, so the last byte is the terminator it appended and not part of
    /// the value. It is dropped here, which makes a value written with
    /// [`Parameters::set_char_array`] read back byte for byte. The reported
    /// length is exact, so only the appended byte is removed and a stored zero
    /// byte survives.
    pub fn char_array(&self, key: &str) -> Result<Vec<u8>> {
        let mut bytes = self.array_bytes(key, sys::OH_CommonEvent_GetCharArrayFromParameters)?;
        if bytes.last() == Some(&0) {
            bytes.pop();
        }
        Ok(bytes)
    }

    /// The `double` array under `key`, empty when there is none.
    pub fn double_array(&self, key: &str) -> Result<Vec<f64>> {
        self.array(key, sys::OH_CommonEvent_GetDoubleArrayFromParameters)
    }

    /// Run one of the native array getters and copy the result out.
    ///
    /// The native side allocates the buffer, hands out a pointer to it and
    /// keeps ownership: the payload frees every buffer it handed out when it is
    /// destroyed. Copying here is therefore both required — the buffer dies
    /// with the payload — and sufficient; freeing it would be a double free.
    fn array<T: Copy>(
        &self,
        key: &str,
        get: unsafe extern "C" fn(
            *const sys::CommonEvent_Parameters,
            *const c_char,
            *mut *mut T,
        ) -> i32,
    ) -> Result<Vec<T>> {
        let key = CString::new(key)?;
        let mut buffer: *mut T = std::ptr::null_mut();
        // SAFETY: the payload is borrowed for `'a`, the key outlives the call and
        // `buffer` is a live out-parameter.
        let len = unsafe { get(self.raw.as_ptr(), key.as_ptr(), &mut buffer) };
        let Ok(len) = usize::try_from(len) else {
            return Ok(Vec::new());
        };
        if buffer.is_null() || len == 0 {
            return Ok(Vec::new());
        }
        // SAFETY: the getter reported `len` elements written into `buffer`, which
        // stays alive at least as long as the borrowed payload.
        let values = unsafe { std::slice::from_raw_parts(buffer, len) };
        Ok(values.to_vec())
    }

    /// Same as [`array`](Self::array) for one-byte elements, read as bytes.
    ///
    /// The native side allocates the buffer up front and only writes back the
    /// elements it could decode, so a trailing element may be left as the
    /// allocator returned it. Reading the buffer as `u8` rather than as `bool`
    /// or `char` keeps that out of the domain of a Rust type with invalid bit
    /// patterns.
    fn array_bytes<T>(
        &self,
        key: &str,
        get: unsafe extern "C" fn(
            *const sys::CommonEvent_Parameters,
            *const c_char,
            *mut *mut T,
        ) -> i32,
    ) -> Result<Vec<u8>> {
        debug_assert_eq!(std::mem::size_of::<T>(), 1, "element is not one byte wide");
        let key = CString::new(key)?;
        let mut buffer: *mut T = std::ptr::null_mut();
        // SAFETY: the payload is borrowed for `'a`, the key outlives the call and
        // `buffer` is a live out-parameter.
        let len = unsafe { get(self.raw.as_ptr(), key.as_ptr(), &mut buffer) };
        let Ok(len) = usize::try_from(len) else {
            return Ok(Vec::new());
        };
        if buffer.is_null() || len == 0 {
            return Ok(Vec::new());
        }
        // SAFETY: the getter reported `len` one-byte elements in `buffer`, which
        // stays alive at least as long as the borrowed payload.
        let bytes = unsafe { std::slice::from_raw_parts(buffer.cast::<u8>(), len) };
        Ok(bytes.to_vec())
    }
}

/// An owned key/value payload to publish with a common event.
///
/// Created by `OH_CommonEvent_CreateParameters` and released on drop, which
/// also frees every buffer the getters handed out. Read it back through
/// [`view`](Self::view).
#[cfg(feature = "api-18")]
pub struct Parameters {
    raw: NonNull<sys::CommonEvent_Parameters>,
}

#[cfg(feature = "api-18")]
impl Parameters {
    /// Create an empty payload.
    pub fn new() -> Result<Self> {
        // SAFETY: the call takes no arguments and yields an owned payload or null.
        let raw = unsafe { sys::OH_CommonEvent_CreateParameters() };
        NonNull::new(raw)
            .map(|raw| Self { raw })
            .ok_or(CommonEventError::Alloc)
    }

    /// Read the payload back through a borrowed view.
    pub fn view(&self) -> ParametersRef<'_> {
        ParametersRef::from_raw(self.raw)
    }

    /// Store an `int` under `key`.
    pub fn set_int(&mut self, key: &str, value: i32) -> Result<()> {
        let key = CString::new(key)?;
        // SAFETY: the payload is owned here and the key outlives the call.
        unsafe {
            check(sys::OH_CommonEvent_SetIntToParameters(
                self.raw.as_ptr(),
                key.as_ptr(),
                value,
            ))
        }
    }

    /// Store a `long` under `key`.
    ///
    /// A value that does not fit the C `long` of the target is rejected with
    /// [`CommonEventError::OutOfRange`].
    pub fn set_long(&mut self, key: &str, value: i64) -> Result<()> {
        let key = CString::new(key)?;
        let value = c_long::try_from(value).map_err(|_| CommonEventError::OutOfRange)?;
        // SAFETY: the payload is owned here and the key outlives the call.
        unsafe {
            check(sys::OH_CommonEvent_SetLongToParameters(
                self.raw.as_ptr(),
                key.as_ptr(),
                value,
            ))
        }
    }

    /// Store a `bool` under `key`.
    pub fn set_bool(&mut self, key: &str, value: bool) -> Result<()> {
        let key = CString::new(key)?;
        // SAFETY: the payload is owned here and the key outlives the call.
        unsafe {
            check(sys::OH_CommonEvent_SetBoolToParameters(
                self.raw.as_ptr(),
                key.as_ptr(),
                value,
            ))
        }
    }

    /// Store a `char` under `key`, as a raw byte.
    pub fn set_char(&mut self, key: &str, value: u8) -> Result<()> {
        let key = CString::new(key)?;
        // SAFETY: the payload is owned here and the key outlives the call.
        unsafe {
            check(sys::OH_CommonEvent_SetCharToParameters(
                self.raw.as_ptr(),
                key.as_ptr(),
                value as c_char,
            ))
        }
    }

    /// Store a `double` under `key`.
    pub fn set_double(&mut self, key: &str, value: f64) -> Result<()> {
        let key = CString::new(key)?;
        // SAFETY: the payload is owned here and the key outlives the call.
        unsafe {
            check(sys::OH_CommonEvent_SetDoubleToParameters(
                self.raw.as_ptr(),
                key.as_ptr(),
                value,
            ))
        }
    }

    /// Store an `int` array under `key`.
    pub fn set_int_array(&mut self, key: &str, values: &[i32]) -> Result<()> {
        let key = CString::new(key)?;
        // SAFETY: `values` is a live slice of `values.len()` elements and the key
        // outlives the call, which copies both.
        unsafe {
            check(sys::OH_CommonEvent_SetIntArrayToParameters(
                self.raw.as_ptr(),
                key.as_ptr(),
                values.as_ptr(),
                values.len(),
            ))
        }
    }

    /// Store a `long` array under `key`.
    ///
    /// A value that does not fit the C `long` of the target is rejected with
    /// [`CommonEventError::OutOfRange`].
    pub fn set_long_array(&mut self, key: &str, values: &[i64]) -> Result<()> {
        let key = CString::new(key)?;
        let values = values
            .iter()
            .map(|value| c_long::try_from(*value).map_err(|_| CommonEventError::OutOfRange))
            .collect::<Result<Vec<c_long>>>()?;
        // SAFETY: `values` is a live slice of `values.len()` elements and the key
        // outlives the call, which copies both.
        unsafe {
            check(sys::OH_CommonEvent_SetLongArrayToParameters(
                self.raw.as_ptr(),
                key.as_ptr(),
                values.as_ptr(),
                values.len(),
            ))
        }
    }

    /// Store a `bool` array under `key`.
    pub fn set_bool_array(&mut self, key: &str, values: &[bool]) -> Result<()> {
        let key = CString::new(key)?;
        // SAFETY: `values` is a live slice of `values.len()` elements and the key
        // outlives the call, which copies both.
        unsafe {
            check(sys::OH_CommonEvent_SetBoolArrayToParameters(
                self.raw.as_ptr(),
                key.as_ptr(),
                values.as_ptr(),
                values.len(),
            ))
        }
    }

    /// Store a `char` array under `key`, as raw bytes.
    ///
    /// The native layer keeps the bytes as a string, not as an array: it builds
    /// the value from `min(strlen(values), num)` bytes, so it reads the buffer
    /// with `strlen` and stops at the first zero byte. The bytes are therefore
    /// copied into a terminated buffer here, since handing the native side an
    /// unterminated one would let it read past the end, and an interior zero is
    /// rejected with [`CommonEventError::Nul`] rather than silently truncating
    /// the value. [`ParametersRef::char_array`] reads the value back byte for
    /// byte.
    pub fn set_char_array(&mut self, key: &str, values: &[u8]) -> Result<()> {
        let key = CString::new(key)?;
        let values = CString::new(values)?;
        let len = values.as_bytes().len();
        // SAFETY: `values` is a terminated buffer of `len` bytes, as the native
        // side requires, and both it and the key outlive the call, which copies
        // them.
        unsafe {
            check(sys::OH_CommonEvent_SetCharArrayToParameters(
                self.raw.as_ptr(),
                key.as_ptr(),
                values.as_ptr(),
                len,
            ))
        }
    }

    /// Store a `double` array under `key`.
    pub fn set_double_array(&mut self, key: &str, values: &[f64]) -> Result<()> {
        let key = CString::new(key)?;
        // SAFETY: `values` is a live slice of `values.len()` elements and the key
        // outlives the call, which copies both.
        unsafe {
            check(sys::OH_CommonEvent_SetDoubleArrayToParameters(
                self.raw.as_ptr(),
                key.as_ptr(),
                values.as_ptr(),
                values.len(),
            ))
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut sys::CommonEvent_Parameters {
        self.raw.as_ptr()
    }
}

#[cfg(feature = "api-18")]
impl Drop for Parameters {
    fn drop(&mut self) {
        // SAFETY: the payload came from OH_CommonEvent_CreateParameters, is owned
        // here and is released once. Every view of it borrows `self`, so none can
        // outlive this point.
        unsafe { sys::OH_CommonEvent_DestroyParameters(self.raw.as_ptr()) };
    }
}
