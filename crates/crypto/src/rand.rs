use crate::blob::{borrow_cstr, CryptoDataBlob, OwnedCryptoDataBlob};
use crate::error::{check, checked_len, CryptoError, Result};
use ohos_crypto_sys::*;
use std::ptr::{self, NonNull};

/// A cryptographically secure random number generator.
pub struct Rand {
    raw: NonNull<OH_CryptoRand>,
}

impl Rand {
    /// Create a random number generator.
    pub fn new() -> Result<Self> {
        let mut raw = ptr::null_mut();
        // SAFETY: `raw` receives the new context.
        check(unsafe { OH_CryptoRand_Create(&mut raw) })?;
        let raw =
            NonNull::new(raw).ok_or(CryptoError::new(OH_Crypto_ErrCode_CRYPTO_OPERTION_ERROR))?;
        Ok(Rand { raw })
    }

    /// Generate `len` random bytes.
    pub fn generate(&mut self, len: usize) -> Result<Vec<u8>> {
        let len = checked_len(len)?;
        let mut out = OwnedCryptoDataBlob::new();
        // SAFETY: `out` is a zeroed blob the framework fills in.
        check(unsafe { OH_CryptoRand_GenerateRandom(self.raw.as_ptr(), len, out.as_mut_ptr()) })?;
        Ok(out.to_vec())
    }

    /// Mix additional entropy into the generator.
    pub fn set_seed<'s>(&mut self, seed: impl Into<CryptoDataBlob<'s>>) -> Result<()> {
        let mut seed = seed.into().to_raw();
        // SAFETY: `seed` borrows the caller's slice for the duration of the call.
        unsafe { check(OH_CryptoRand_SetSeed(self.raw.as_ptr(), &mut seed)) }
    }

    /// Draw entropy from the hardware source instead of the software one.
    #[cfg(feature = "api-21")]
    pub fn enable_hardware_entropy(&mut self) -> Result<()> {
        // SAFETY: the context is live.
        unsafe { check(OH_CryptoRand_EnableHardwareEntropy(self.raw.as_ptr())) }
    }

    /// The name of the underlying algorithm.
    pub fn algo_name(&self) -> Option<&str> {
        // SAFETY: the returned string is owned by the context and lives as long as it.
        unsafe { borrow_cstr(OH_CryptoRand_GetAlgoName(self.raw.as_ptr())) }
    }
}

impl Drop for Rand {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `_Create` and is destroyed once.
        unsafe { OH_CryptoRand_Destroy(self.raw.as_ptr()) };
    }
}
