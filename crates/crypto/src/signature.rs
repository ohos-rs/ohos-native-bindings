use crate::asym::PubKey;
use crate::blob::{
    borrow_cstr, c_string, CryptoDataBlob, OptionalCryptoDataBlob, OwnedCryptoDataBlob,
};
use crate::error::{check, CryptoError, Result};
use crate::r#type::CryptoSignatureParamType;
use crate::value::IntoCryptoValue;
use ohos_crypto_sys::*;
use std::ptr::{self, NonNull};

#[cfg(feature = "api-20")]
use crate::asym::PrivKey;

/// A signature verification context: `init` → `update`* → `verify`.
///
/// The algorithm name combines key, digest and padding, e.g.
/// `"RSA2048|PKCS1|SHA256"` or `"ECC256|SHA256"`.
pub struct Verify {
    raw: NonNull<OH_CryptoVerify>,
    // Backing storage for parameter values handed to the native context.
    values: Vec<Vec<u8>>,
}

impl Verify {
    /// Create a verification context for `algo_name`.
    pub fn new(algo_name: &str) -> Result<Self> {
        let name = c_string(algo_name)?;
        let mut raw = ptr::null_mut();
        // SAFETY: `name` is NUL-terminated and `raw` receives the new context.
        check(unsafe { OH_CryptoVerify_Create(name.as_ptr(), &mut raw) })?;
        let raw =
            NonNull::new(raw).ok_or(CryptoError::new(OH_Crypto_ErrCode_CRYPTO_OPERTION_ERROR))?;
        Ok(Verify {
            raw,
            values: Vec::new(),
        })
    }

    /// Set a parameter, e.g. the PSS salt length. The bytes are copied and kept
    /// alive by `self`.
    pub fn set_param(
        &mut self,
        param_type: CryptoSignatureParamType,
        value: impl IntoCryptoValue,
    ) -> Result<()> {
        let owned = value.into_crypto_value();
        let mut blob = CryptoDataBlob::new(&owned).to_raw();
        // SAFETY: `blob` points at `owned`, whose heap buffer is kept alive below.
        check(unsafe {
            OH_CryptoVerify_SetParam(self.raw.as_ptr(), param_type.into(), &mut blob)
        })?;
        self.values.push(owned);
        Ok(())
    }

    /// Read a parameter back.
    pub fn param(&self, param_type: CryptoSignatureParamType) -> Result<Vec<u8>> {
        let mut out = OwnedCryptoDataBlob::new();
        // SAFETY: the context is live; `out` is filled in by the framework.
        check(unsafe {
            OH_CryptoVerify_GetParam(self.raw.as_ptr(), param_type.into(), out.as_mut_ptr())
        })?;
        Ok(out.to_vec())
    }

    /// Bind the context to a public key.
    pub fn init(&mut self, pub_key: &PubKey<'_>) -> Result<()> {
        // SAFETY: the key is valid for the duration of the call.
        unsafe {
            check(OH_CryptoVerify_Init(
                self.raw.as_ptr(),
                pub_key.raw.as_ptr(),
            ))
        }
    }

    /// Feed a chunk of the signed message.
    pub fn update<'i>(&mut self, input: impl Into<CryptoDataBlob<'i>>) -> Result<()> {
        let mut input = input.into().to_raw();
        // SAFETY: `input` borrows the caller's slice for the duration of the call.
        unsafe { check(OH_CryptoVerify_Update(self.raw.as_ptr(), &mut input)) }
    }

    /// Verify `signature`, optionally over a last chunk of message.
    ///
    /// The C API reports the outcome as a plain `bool`, so a rejected signature
    /// and a failed call are indistinguishable.
    pub fn verify<'s>(
        &mut self,
        input: Option<CryptoDataBlob<'_>>,
        signature: impl Into<CryptoDataBlob<'s>>,
    ) -> bool {
        let mut input = OptionalCryptoDataBlob::new(input);
        let mut signature = signature.into().to_raw();
        // SAFETY: both blobs borrow the caller's slices for the duration of the call.
        unsafe { OH_CryptoVerify_Final(self.raw.as_ptr(), input.as_mut_ptr(), &mut signature) }
    }

    /// Recover the message embedded in a signature, for schemes that support it.
    pub fn recover<'s>(&mut self, signature: impl Into<CryptoDataBlob<'s>>) -> Result<Vec<u8>> {
        let mut signature = signature.into().to_raw();
        let mut out = OwnedCryptoDataBlob::new();
        // SAFETY: `signature` borrows the caller's slice; `out` is filled in by the
        // framework.
        check(unsafe {
            OH_CryptoVerify_Recover(self.raw.as_ptr(), &mut signature, out.as_mut_ptr())
        })?;
        Ok(out.to_vec())
    }

    /// The algorithm name this context was created with.
    pub fn algo_name(&self) -> Option<&str> {
        // SAFETY: the returned string is owned by the context and lives as long as it.
        unsafe { borrow_cstr(OH_CryptoVerify_GetAlgoName(self.raw.as_ptr())) }
    }
}

impl Drop for Verify {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `_Create` and is destroyed once.
        unsafe { OH_CryptoVerify_Destroy(self.raw.as_ptr()) };
    }
}

/// A signing context: `init` → `update`* → `finish`.
#[cfg(feature = "api-20")]
pub struct Sign {
    raw: NonNull<OH_CryptoSign>,
    // Backing storage for parameter values handed to the native context.
    values: Vec<Vec<u8>>,
}

#[cfg(feature = "api-20")]
impl Sign {
    /// Create a signing context for `algo_name`.
    pub fn new(algo_name: &str) -> Result<Self> {
        let name = c_string(algo_name)?;
        let mut raw = ptr::null_mut();
        // SAFETY: `name` is NUL-terminated and `raw` receives the new context.
        check(unsafe { OH_CryptoSign_Create(name.as_ptr(), &mut raw) })?;
        let raw =
            NonNull::new(raw).ok_or(CryptoError::new(OH_Crypto_ErrCode_CRYPTO_OPERTION_ERROR))?;
        Ok(Sign {
            raw,
            values: Vec::new(),
        })
    }

    /// Set a parameter, e.g. the PSS salt length. The bytes are copied and kept
    /// alive by `self`.
    pub fn set_param(
        &mut self,
        param_type: CryptoSignatureParamType,
        value: impl IntoCryptoValue,
    ) -> Result<()> {
        let owned = value.into_crypto_value();
        let blob = CryptoDataBlob::new(&owned).to_raw();
        // SAFETY: `blob` points at `owned`, whose heap buffer is kept alive below.
        check(unsafe { OH_CryptoSign_SetParam(self.raw.as_ptr(), param_type.into(), &blob) })?;
        self.values.push(owned);
        Ok(())
    }

    /// Read a parameter back.
    pub fn param(&self, param_type: CryptoSignatureParamType) -> Result<Vec<u8>> {
        let mut out = OwnedCryptoDataBlob::new();
        // SAFETY: the context is live; `out` is filled in by the framework.
        check(unsafe {
            OH_CryptoSign_GetParam(self.raw.as_ptr(), param_type.into(), out.as_mut_ptr())
        })?;
        Ok(out.to_vec())
    }

    /// Bind the context to a private key.
    pub fn init(&mut self, priv_key: &PrivKey<'_>) -> Result<()> {
        // SAFETY: the key is valid for the duration of the call.
        unsafe { check(OH_CryptoSign_Init(self.raw.as_ptr(), priv_key.raw.as_ptr())) }
    }

    /// Feed a chunk of the message to sign.
    pub fn update<'i>(&mut self, input: impl Into<CryptoDataBlob<'i>>) -> Result<()> {
        let input = input.into().to_raw();
        // SAFETY: `input` borrows the caller's slice for the duration of the call.
        unsafe { check(OH_CryptoSign_Update(self.raw.as_ptr(), &input)) }
    }

    /// Produce the signature, optionally over a last chunk of message.
    pub fn finish(&mut self, input: Option<CryptoDataBlob<'_>>) -> Result<Vec<u8>> {
        let mut input = OptionalCryptoDataBlob::new(input);
        let mut out = OwnedCryptoDataBlob::new();
        // SAFETY: `input` borrows the caller's slice; `out` is filled in by the
        // framework.
        check(unsafe {
            OH_CryptoSign_Final(self.raw.as_ptr(), input.as_mut_ptr(), out.as_mut_ptr())
        })?;
        Ok(out.to_vec())
    }

    /// The algorithm name this context was created with.
    pub fn algo_name(&self) -> Option<&str> {
        // SAFETY: the returned string is owned by the context and lives as long as it.
        unsafe { borrow_cstr(OH_CryptoSign_GetAlgoName(self.raw.as_ptr())) }
    }
}

#[cfg(feature = "api-20")]
impl Drop for Sign {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `_Create` and is destroyed once.
        unsafe { OH_CryptoSign_Destroy(self.raw.as_ptr()) };
    }
}
