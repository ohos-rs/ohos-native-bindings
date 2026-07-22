use crate::error::{CryptoError, Result};
use ohos_crypto_sys::*;
use std::ffi::{c_char, CStr, CString};
use std::marker::PhantomData;
use std::ptr;

/// Borrowed bytes passed to the crypto framework as a `Crypto_DataBlob`.
///
/// Every operation that takes byte input accepts `impl Into<CryptoDataBlob>`,
/// so a slice, an array or a `Vec` can be passed directly. The blob borrows the
/// bytes, so it must not outlive the call it is passed to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CryptoDataBlob<'a>(&'a [u8]);

impl<'a> CryptoDataBlob<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        CryptoDataBlob(data)
    }

    pub fn as_bytes(&self) -> &'a [u8] {
        self.0
    }

    /// The raw blob borrows `self`, so it must not outlive the call it is
    /// passed to. `Crypto_DataBlob::len` is a `size_t`, so no length conversion
    /// can fail here.
    pub(crate) fn to_raw(self) -> Crypto_DataBlob {
        Crypto_DataBlob {
            data: self.0.as_ptr().cast_mut(),
            len: self.0.len(),
        }
    }
}

impl<'a> From<&'a [u8]> for CryptoDataBlob<'a> {
    fn from(data: &'a [u8]) -> Self {
        CryptoDataBlob::new(data)
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for CryptoDataBlob<'a> {
    fn from(data: &'a [u8; N]) -> Self {
        CryptoDataBlob::new(data)
    }
}

impl<'a> From<&'a Vec<u8>> for CryptoDataBlob<'a> {
    fn from(data: &'a Vec<u8>) -> Self {
        CryptoDataBlob::new(data)
    }
}

/// An optional input blob, kept in place so a pointer to it stays valid for the
/// duration of the call. `None` is passed to the C API as a null pointer.
pub(crate) struct OptionalCryptoDataBlob<'a>(Option<Crypto_DataBlob>, PhantomData<&'a [u8]>);

impl<'a> OptionalCryptoDataBlob<'a> {
    pub(crate) fn new(data: Option<CryptoDataBlob<'a>>) -> Self {
        OptionalCryptoDataBlob(data.map(CryptoDataBlob::to_raw), PhantomData)
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut Crypto_DataBlob {
        match &mut self.0 {
            Some(blob) => blob,
            None => ptr::null_mut(),
        }
    }
}

/// The owned counterpart of [`CryptoDataBlob`]: an output blob whose buffer is
/// allocated by the framework and released with `OH_Crypto_FreeDataBlob` when
/// the value is dropped.
pub(crate) struct OwnedCryptoDataBlob(Crypto_DataBlob);

impl OwnedCryptoDataBlob {
    pub(crate) fn new() -> Self {
        OwnedCryptoDataBlob(Crypto_DataBlob {
            data: ptr::null_mut(),
            len: 0,
        })
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut Crypto_DataBlob {
        &mut self.0
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        if self.0.data.is_null() {
            return &[];
        }
        // SAFETY: on success the framework wrote `len` bytes at `data`, which it
        // owns for as long as this value is alive.
        unsafe { std::slice::from_raw_parts(self.0.data, self.0.len) }
    }

    pub(crate) fn to_vec(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl Drop for OwnedCryptoDataBlob {
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
