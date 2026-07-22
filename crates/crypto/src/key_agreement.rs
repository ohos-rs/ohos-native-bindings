use crate::asym::{PrivKey, PubKey};
use crate::blob::{c_string, OutBlob};
use crate::error::{check, CryptoError, Result};
use ohos_crypto_sys::*;
use std::ptr::{self, NonNull};

/// A key agreement context, created from an algorithm name such as `"ECC256"`,
/// `"X25519"` or `"DH_modp1536"`.
pub struct KeyAgreement {
    raw: NonNull<OH_CryptoKeyAgreement>,
}

impl KeyAgreement {
    /// Create a key agreement context for `algo_name`.
    pub fn new(algo_name: &str) -> Result<Self> {
        let name = c_string(algo_name)?;
        let mut raw = ptr::null_mut();
        // SAFETY: `name` is NUL-terminated and `raw` receives the new context.
        check(unsafe { OH_CryptoKeyAgreement_Create(name.as_ptr(), &mut raw) })?;
        let raw =
            NonNull::new(raw).ok_or(CryptoError::new(OH_Crypto_ErrCode_CRYPTO_OPERTION_ERROR))?;
        Ok(KeyAgreement { raw })
    }

    /// Derive the shared secret from the local private key and the peer's
    /// public key.
    pub fn generate_secret(
        &mut self,
        priv_key: &PrivKey<'_>,
        pub_key: &PubKey<'_>,
    ) -> Result<Vec<u8>> {
        let mut out = OutBlob::new();
        // SAFETY: both keys are valid for the call; `out` is filled in by the framework.
        check(unsafe {
            OH_CryptoKeyAgreement_GenerateSecret(
                self.raw.as_ptr(),
                priv_key.raw.as_ptr(),
                pub_key.raw.as_ptr(),
                out.as_mut_ptr(),
            )
        })?;
        Ok(out.to_vec())
    }
}

impl Drop for KeyAgreement {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `_Create` and is destroyed once.
        unsafe { OH_CryptoKeyAgreement_Destroy(self.raw.as_ptr()) };
    }
}
