use crate::blob::{blob_in, borrow_cstr, c_string, OutBlob};
use crate::error::{check, CryptoError, Result};
use ohos_crypto_sys::*;
use std::ptr::{self, NonNull};

/// A message digest context: `update`* → `finish`.
///
/// Created from a digest name such as `"SHA256"`, `"SHA512"` or `"SM3"`.
pub struct Digest {
    raw: NonNull<OH_CryptoDigest>,
}

impl Digest {
    /// Create a digest context for `algo_name`.
    pub fn new(algo_name: &str) -> Result<Self> {
        let name = c_string(algo_name)?;
        let mut raw = ptr::null_mut();
        // SAFETY: `name` is NUL-terminated and `raw` receives the new context.
        check(unsafe { OH_CryptoDigest_Create(name.as_ptr(), &mut raw) })?;
        let raw =
            NonNull::new(raw).ok_or(CryptoError::new(OH_Crypto_ErrCode_CRYPTO_OPERTION_ERROR))?;
        Ok(Digest { raw })
    }

    /// Feed a chunk of input.
    pub fn update(&mut self, input: &[u8]) -> Result<()> {
        let mut input = blob_in(input);
        // SAFETY: `input` borrows the caller's slice for the duration of the call.
        unsafe { check(OH_CryptoDigest_Update(self.raw.as_ptr(), &mut input)) }
    }

    /// Produce the digest.
    pub fn finish(&mut self) -> Result<Vec<u8>> {
        let mut out = OutBlob::new();
        // SAFETY: `out` is a zeroed blob the framework fills in.
        check(unsafe { OH_CryptoDigest_Final(self.raw.as_ptr(), out.as_mut_ptr()) })?;
        Ok(out.to_vec())
    }

    /// Digest length in bytes, or 0 if the framework cannot report it.
    pub fn length(&self) -> u32 {
        // SAFETY: the context is live.
        unsafe { OH_CryptoDigest_GetLength(self.raw.as_ptr()) }
    }

    /// The algorithm name this context was created with.
    pub fn algo_name(&self) -> Option<&str> {
        // SAFETY: the returned string is owned by the context and lives as long as it.
        unsafe { borrow_cstr(OH_CryptoDigest_GetAlgoName(self.raw.as_ptr())) }
    }
}

impl Drop for Digest {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `_Create` and is destroyed once. Note the
        // framework spells this destructor `OH_DigestCrypto_Destroy`.
        unsafe { OH_DigestCrypto_Destroy(self.raw.as_ptr()) };
    }
}
