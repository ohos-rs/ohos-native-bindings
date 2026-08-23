use crate::blob::{
    borrow_cstr, c_string, CryptoDataBlob, OptionalCryptoDataBlob, OwnedCryptoDataBlob,
};
use crate::error::{check, CryptoError, Result};
use crate::r#type::{CryptoCipherMode, CryptoSymCipherParamsType};
use crate::value::IntoCryptoValue;
use ohos_crypto_sys::*;
use std::ptr::{self, NonNull};

/// Generator for symmetric keys, created from an algorithm name such as
/// `"AES256"` or `"SM4_128"`.
pub struct SymKeyGenerator {
    raw: NonNull<OH_CryptoSymKeyGenerator>,
}

impl SymKeyGenerator {
    /// Create a generator for `algo_name`.
    pub fn new(algo_name: &str) -> Result<Self> {
        let name = c_string(algo_name)?;
        let mut raw = ptr::null_mut();
        // SAFETY: `name` is NUL-terminated and `raw` receives the new context.
        check(unsafe { OH_CryptoSymKeyGenerator_Create(name.as_ptr(), &mut raw) })?;
        let raw =
            NonNull::new(raw).ok_or(CryptoError::new(OH_Crypto_ErrCode_CRYPTO_OPERTION_ERROR))?;
        Ok(SymKeyGenerator { raw })
    }

    /// Generate a random key.
    pub fn generate(&self) -> Result<SymKey> {
        let mut raw = ptr::null_mut();
        // SAFETY: the generator is live and `raw` receives the new key.
        check(unsafe { OH_CryptoSymKeyGenerator_Generate(self.raw.as_ptr(), &mut raw) })?;
        let raw =
            NonNull::new(raw).ok_or(CryptoError::new(OH_Crypto_ErrCode_CRYPTO_OPERTION_ERROR))?;
        Ok(SymKey { raw })
    }

    /// Build a key from existing key material.
    pub fn convert<'k>(&self, key_data: impl Into<CryptoDataBlob<'k>>) -> Result<SymKey> {
        let data = key_data.into().to_raw();
        let mut raw = ptr::null_mut();
        // SAFETY: `data` borrows `key_data` for the duration of the call.
        check(unsafe { OH_CryptoSymKeyGenerator_Convert(self.raw.as_ptr(), &data, &mut raw) })?;
        let raw =
            NonNull::new(raw).ok_or(CryptoError::new(OH_Crypto_ErrCode_CRYPTO_OPERTION_ERROR))?;
        Ok(SymKey { raw })
    }

    /// The algorithm name this generator was created with.
    pub fn algo_name(&self) -> Option<&str> {
        // SAFETY: the returned string is owned by the generator and lives as long as it.
        unsafe { borrow_cstr(OH_CryptoSymKeyGenerator_GetAlgoName(self.raw.as_ptr())) }
    }
}

impl Drop for SymKeyGenerator {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `_Create` and is destroyed once.
        unsafe { OH_CryptoSymKeyGenerator_Destroy(self.raw.as_ptr()) };
    }
}

/// A symmetric key. Unlike HUKS keys, the material lives in the process and can
/// be exported with [`SymKey::key_data`].
pub struct SymKey {
    pub(crate) raw: NonNull<OH_CryptoSymKey>,
}

impl SymKey {
    /// Export the raw key material.
    pub fn key_data(&self) -> Result<Vec<u8>> {
        let mut out = OwnedCryptoDataBlob::new();
        // SAFETY: the key is live and `out` is a zeroed blob the framework fills in.
        check(unsafe { OH_CryptoSymKey_GetKeyData(self.raw.as_ptr(), out.as_mut_ptr()) })?;
        Ok(out.to_vec())
    }

    /// The algorithm this key belongs to.
    pub fn algo_name(&self) -> Option<&str> {
        // SAFETY: the returned string is owned by the key and lives as long as it.
        unsafe { borrow_cstr(OH_CryptoSymKey_GetAlgoName(self.raw.as_ptr())) }
    }
}

impl Drop for SymKey {
    fn drop(&mut self) {
        // SAFETY: `raw` came from the generator and is destroyed once.
        unsafe { OH_CryptoSymKey_Destroy(self.raw.as_ptr()) };
    }
}

/// Per-operation cipher parameters (IV, AAD, authentication tag).
///
/// [`SymCipher::init`] captures the values set so far; setting a parameter
/// afterwards (e.g. the GCM tag for decryption) has no effect on that run.
pub struct SymCipherParams {
    pub(crate) raw: NonNull<OH_CryptoSymCipherParams>,
    // Backing storage for the values handed to the native params object, kept
    // alive for as long as it (the native side may reference this memory).
    values: Vec<Vec<u8>>,
}

impl SymCipherParams {
    /// Create an empty parameter set.
    pub fn new() -> Result<Self> {
        let mut raw = ptr::null_mut();
        // SAFETY: `raw` receives the new params context.
        check(unsafe { OH_CryptoSymCipherParams_Create(&mut raw) })?;
        let raw =
            NonNull::new(raw).ok_or(CryptoError::new(OH_Crypto_ErrCode_CRYPTO_OPERTION_ERROR))?;
        Ok(SymCipherParams {
            raw,
            values: Vec::new(),
        })
    }

    /// Set one parameter; the bytes are copied and kept alive by `self`.
    pub fn set(
        &mut self,
        params_type: CryptoSymCipherParamsType,
        value: impl IntoCryptoValue,
    ) -> Result<()> {
        let owned = value.into_crypto_value();
        let mut blob = CryptoDataBlob::new(&owned).to_raw();
        // SAFETY: `blob` points at `owned`, whose heap buffer is kept alive below.
        check(unsafe {
            OH_CryptoSymCipherParams_SetParam(self.raw.as_ptr(), params_type.into(), &mut blob)
        })?;
        self.values.push(owned);
        Ok(())
    }
}

impl Drop for SymCipherParams {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `_Create` and is destroyed once.
        unsafe { OH_CryptoSymCipherParams_Destroy(self.raw.as_ptr()) };
    }
}

/// A symmetric cipher context: `init` → `update`* → `finish`.
///
/// The algorithm name combines cipher, mode and padding, e.g.
/// `"AES256|GCM|PKCS7"` or `"SM4_128|CBC|PKCS7"`.
pub struct SymCipher {
    raw: NonNull<OH_CryptoSymCipher>,
}

impl SymCipher {
    /// Create a cipher context for `algo_name`.
    pub fn new(algo_name: &str) -> Result<Self> {
        let name = c_string(algo_name)?;
        let mut raw = ptr::null_mut();
        // SAFETY: `name` is NUL-terminated and `raw` receives the new context.
        check(unsafe { OH_CryptoSymCipher_Create(name.as_ptr(), &mut raw) })?;
        let raw =
            NonNull::new(raw).ok_or(CryptoError::new(OH_Crypto_ErrCode_CRYPTO_OPERTION_ERROR))?;
        Ok(SymCipher { raw })
    }

    /// Start an encryption or decryption run. `params` is required for modes
    /// that take an IV and may be `None` for ECB.
    pub fn init(
        &mut self,
        mode: CryptoCipherMode,
        key: &SymKey,
        params: Option<&SymCipherParams>,
    ) -> Result<()> {
        let params = params.map_or(ptr::null_mut(), |p| p.raw.as_ptr());
        // SAFETY: key and params outlive the call.
        unsafe {
            check(OH_CryptoSymCipher_Init(
                self.raw.as_ptr(),
                mode.into(),
                key.raw.as_ptr(),
                params,
            ))
        }
    }

    /// Feed a chunk of input; returns whatever output is ready.
    pub fn update<'i>(&mut self, input: impl Into<CryptoDataBlob<'i>>) -> Result<Vec<u8>> {
        let mut input = input.into().to_raw();
        let mut out = OwnedCryptoDataBlob::new();
        // SAFETY: `input` borrows the caller's slice for the call; `out` is filled in
        // by the framework.
        check(unsafe {
            OH_CryptoSymCipher_Update(self.raw.as_ptr(), &mut input, out.as_mut_ptr())
        })?;
        Ok(out.to_vec())
    }

    /// Finish the run with an optional last chunk. For GCM encryption the
    /// authentication tag is appended to the returned output; for GCM
    /// decryption the tag is verified against the one set on the
    /// [`SymCipherParams`] before [`init`](Self::init).
    pub fn finish(&mut self, input: Option<CryptoDataBlob<'_>>) -> Result<Vec<u8>> {
        let mut input = OptionalCryptoDataBlob::new(input);
        let mut out = OwnedCryptoDataBlob::new();
        // SAFETY: `input` borrows the caller's slice for the call; `out` is filled in
        // by the framework.
        check(unsafe {
            OH_CryptoSymCipher_Final(self.raw.as_ptr(), input.as_mut_ptr(), out.as_mut_ptr())
        })?;
        Ok(out.to_vec())
    }

    /// The algorithm name this context was created with.
    pub fn algo_name(&self) -> Option<&str> {
        // SAFETY: the returned string is owned by the context and lives as long as it.
        unsafe { borrow_cstr(OH_CryptoSymCipher_GetAlgoName(self.raw.as_ptr())) }
    }
}

impl Drop for SymCipher {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `_Create` and is destroyed once.
        unsafe { OH_CryptoSymCipher_Destroy(self.raw.as_ptr()) };
    }
}
