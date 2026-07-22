use crate::blob::{c_string, OutBlob};
use crate::error::{check, checked_len, CryptoError, Result};
use crate::r#type::CryptoKdfParamType;
use crate::value::IntoCryptoValue;
use ohos_crypto_sys::*;
use std::ptr::{self, NonNull};

/// Parameters for a key derivation, created from the KDF name (`"HKDF"`,
/// `"PBKDF2"` or `"SCRYPT"`).
///
/// Numeric parameters follow the `_INT` / `_UINT64` suffixes in
/// [`CryptoKdfParamType`]: pass an `i32` for an `_INT` type and a `u64` for a
/// `_UINT64` type and [`set`](KdfParams::set) encodes the bytes.
pub struct KdfParams {
    pub(crate) raw: NonNull<OH_CryptoKdfParams>,
    // Backing storage for the values handed to the native params object.
    values: Vec<Vec<u8>>,
}

impl KdfParams {
    /// Create an empty parameter set for `algo_name`.
    pub fn new(algo_name: &str) -> Result<Self> {
        let name = c_string(algo_name)?;
        let mut raw = ptr::null_mut();
        // SAFETY: `name` is NUL-terminated and `raw` receives the new params context.
        check(unsafe { OH_CryptoKdfParams_Create(name.as_ptr(), &mut raw) })?;
        let raw =
            NonNull::new(raw).ok_or(CryptoError::new(OH_Crypto_ErrCode_CRYPTO_OPERTION_ERROR))?;
        Ok(KdfParams {
            raw,
            values: Vec::new(),
        })
    }

    /// Set one parameter; the bytes are copied and kept alive by `self`.
    pub fn set(
        &mut self,
        param_type: CryptoKdfParamType,
        value: impl IntoCryptoValue,
    ) -> Result<()> {
        let mut owned = value.into_crypto_value();
        let mut blob = Crypto_DataBlob {
            data: owned.as_mut_ptr(),
            len: owned.len(),
        };
        // SAFETY: `blob` points at `owned`, whose heap buffer is kept alive below.
        check(unsafe {
            OH_CryptoKdfParams_SetParam(self.raw.as_ptr(), param_type.into(), &mut blob)
        })?;
        self.values.push(owned);
        Ok(())
    }
}

impl Drop for KdfParams {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `_Create` and is destroyed once.
        unsafe { OH_CryptoKdfParams_Destroy(self.raw.as_ptr()) };
    }
}

/// A key derivation function context.
pub struct Kdf {
    raw: NonNull<OH_CryptoKdf>,
}

impl Kdf {
    /// Create a KDF context for `algo_name`, e.g. `"HKDF|SHA256|EXTRACT_AND_EXPAND"`.
    pub fn new(algo_name: &str) -> Result<Self> {
        let name = c_string(algo_name)?;
        let mut raw = ptr::null_mut();
        // SAFETY: `name` is NUL-terminated and `raw` receives the new context.
        check(unsafe { OH_CryptoKdf_Create(name.as_ptr(), &mut raw) })?;
        let raw =
            NonNull::new(raw).ok_or(CryptoError::new(OH_Crypto_ErrCode_CRYPTO_OPERTION_ERROR))?;
        Ok(Kdf { raw })
    }

    /// Derive `key_len` bytes of key material.
    pub fn derive(&mut self, params: &KdfParams, key_len: usize) -> Result<Vec<u8>> {
        let key_len = checked_len(key_len)?;
        let mut out = OutBlob::new();
        // SAFETY: `params` outlives the call; `out` is filled in by the framework.
        check(unsafe {
            OH_CryptoKdf_Derive(
                self.raw.as_ptr(),
                params.raw.as_ptr(),
                key_len,
                out.as_mut_ptr(),
            )
        })?;
        Ok(out.to_vec())
    }
}

impl Drop for Kdf {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `_Create` and is destroyed once.
        unsafe { OH_CryptoKdf_Destroy(self.raw.as_ptr()) };
    }
}
