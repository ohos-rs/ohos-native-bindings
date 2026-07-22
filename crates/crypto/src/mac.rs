use crate::blob::{blob_in, c_string, OutBlob};
use crate::error::{check, CryptoError, Result};
use crate::r#type::CryptoMacParamType;
use crate::sym::SymKey;
use crate::value::IntoCryptoValue;
use ohos_crypto_sys::*;
use std::ptr::{self, NonNull};

/// A message authentication code context: `init` → `update`* → `finish`.
///
/// Created from `"HMAC"` or `"CMAC"`; the digest or cipher is then selected
/// with [`set_param`](Self::set_param).
pub struct Mac {
    raw: NonNull<OH_CryptoMac>,
    // Backing storage for parameter values handed to the native context.
    values: Vec<Vec<u8>>,
}

impl Mac {
    /// Create a MAC context for `algo_name`.
    pub fn new(algo_name: &str) -> Result<Self> {
        let name = c_string(algo_name)?;
        let mut raw = ptr::null_mut();
        // SAFETY: `name` is NUL-terminated and `raw` receives the new context.
        check(unsafe { OH_CryptoMac_Create(name.as_ptr(), &mut raw) })?;
        let raw =
            NonNull::new(raw).ok_or(CryptoError::new(OH_Crypto_ErrCode_CRYPTO_OPERTION_ERROR))?;
        Ok(Mac {
            raw,
            values: Vec::new(),
        })
    }

    /// Select the underlying digest (HMAC) or cipher (CMAC), e.g. `"SHA256"`.
    /// The bytes are copied and kept alive by `self`.
    pub fn set_param(
        &mut self,
        param_type: CryptoMacParamType,
        value: impl IntoCryptoValue,
    ) -> Result<()> {
        let mut owned = value.into_crypto_value();
        let blob = Crypto_DataBlob {
            data: owned.as_mut_ptr(),
            len: owned.len(),
        };
        // SAFETY: `blob` points at `owned`, whose heap buffer is kept alive below.
        check(unsafe { OH_CryptoMac_SetParam(self.raw.as_ptr(), param_type.into(), &blob) })?;
        self.values.push(owned);
        Ok(())
    }

    /// Bind the context to a key.
    pub fn init(&mut self, key: &SymKey) -> Result<()> {
        // SAFETY: the key is valid for the duration of the call.
        unsafe { check(OH_CryptoMac_Init(self.raw.as_ptr(), key.raw.as_ptr())) }
    }

    /// Feed a chunk of input.
    pub fn update(&mut self, input: &[u8]) -> Result<()> {
        let input = blob_in(input);
        // SAFETY: `input` borrows the caller's slice for the duration of the call.
        unsafe { check(OH_CryptoMac_Update(self.raw.as_ptr(), &input)) }
    }

    /// Produce the MAC.
    pub fn finish(&mut self) -> Result<Vec<u8>> {
        let mut out = OutBlob::new();
        // SAFETY: `out` is a zeroed blob the framework fills in.
        check(unsafe { OH_CryptoMac_Final(self.raw.as_ptr(), out.as_mut_ptr()) })?;
        Ok(out.to_vec())
    }

    /// MAC length in bytes.
    pub fn length(&self) -> Result<u32> {
        let mut length = 0u32;
        // SAFETY: the context is live and `length` is a valid out pointer.
        check(unsafe { OH_CryptoMac_GetLength(self.raw.as_ptr(), &mut length) })?;
        Ok(length)
    }
}

impl Drop for Mac {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `_Create` and is destroyed once.
        unsafe { OH_CryptoMac_Destroy(self.raw.as_ptr()) };
    }
}
