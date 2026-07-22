use crate::error::{CryptoError, Result};
use ohos_crypto_sys::*;
use std::ffi::{c_char, CStr, CString};
use std::marker::PhantomData;
use std::ptr;

/// Wrap a byte slice as an input `Crypto_DataBlob`. The blob borrows `data`, so
/// it must not outlive the call it is passed to.
pub(crate) fn blob_in(data: &[u8]) -> Crypto_DataBlob {
    Crypto_DataBlob {
        data: data.as_ptr() as *mut u8,
        len: data.len(),
    }
}

/// An optional input blob, kept in place so a pointer to it stays valid for the
/// duration of the call. `None` is passed to the C API as a null pointer.
pub(crate) struct OptionalBlob<'a>(Option<Crypto_DataBlob>, PhantomData<&'a [u8]>);

impl<'a> OptionalBlob<'a> {
    pub(crate) fn new(data: Option<&'a [u8]>) -> Self {
        OptionalBlob(data.map(blob_in), PhantomData)
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut Crypto_DataBlob {
        match &mut self.0 {
            Some(blob) => blob,
            None => ptr::null_mut(),
        }
    }
}

/// An output blob whose buffer is allocated by the framework and must be
/// released with `OH_Crypto_FreeDataBlob`.
pub(crate) struct OutBlob(Crypto_DataBlob);

impl OutBlob {
    pub(crate) fn new() -> Self {
        OutBlob(Crypto_DataBlob {
            data: ptr::null_mut(),
            len: 0,
        })
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut Crypto_DataBlob {
        &mut self.0
    }

    pub(crate) fn to_vec(&self) -> Vec<u8> {
        if self.0.data.is_null() {
            return Vec::new();
        }
        // SAFETY: on success the framework wrote `len` bytes at `data`.
        unsafe { std::slice::from_raw_parts(self.0.data, self.0.len) }.to_vec()
    }
}

impl Drop for OutBlob {
    fn drop(&mut self) {
        if !self.0.data.is_null() {
            // SAFETY: `data` was allocated by the framework and is released once.
            unsafe { OH_Crypto_FreeDataBlob(&mut self.0) };
        }
    }
}

/// Convert an algorithm / format name into a C string, rejecting interior NULs.
pub(crate) fn c_string(value: &str) -> Result<CString> {
    CString::new(value).map_err(|_| CryptoError::new(OH_Crypto_ErrCode_CRYPTO_INVALID_PARAMS))
}

/// Borrow a framework-owned C string.
///
/// # Safety
///
/// `ptr` must be null or point at a NUL-terminated string that outlives `'a`.
pub(crate) unsafe fn borrow_cstr<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }
    unsafe { CStr::from_ptr(ptr) }.to_str().ok()
}
