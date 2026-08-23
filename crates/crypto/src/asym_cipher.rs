use crate::asym::KeyPair;
use crate::blob::{c_string, CryptoDataBlob, OwnedCryptoDataBlob};
use crate::error::{check, CryptoError, Result};
use crate::r#type::CryptoCipherMode;
use ohos_crypto_sys::*;
use std::ptr::{self, NonNull};

/// An asymmetric cipher context: `init` → `finish`.
///
/// The algorithm name combines key, padding and digests, e.g.
/// `"RSA2048|PKCS1"` or `"SM2_256|SM3"`.
pub struct AsymCipher {
    raw: NonNull<OH_CryptoAsymCipher>,
}

impl AsymCipher {
    /// Create an asymmetric cipher context for `algo_name`.
    pub fn new(algo_name: &str) -> Result<Self> {
        let name = c_string(algo_name)?;
        let mut raw = ptr::null_mut();
        // SAFETY: `name` is NUL-terminated and `raw` receives the new context.
        check(unsafe { OH_CryptoAsymCipher_Create(name.as_ptr(), &mut raw) })?;
        let raw =
            NonNull::new(raw).ok_or(CryptoError::new(OH_Crypto_ErrCode_CRYPTO_OPERTION_ERROR))?;
        Ok(AsymCipher { raw })
    }

    /// Start an encryption or decryption run. Encryption uses the public half
    /// of `key`, decryption the private half.
    pub fn init(&mut self, mode: CryptoCipherMode, key: &KeyPair) -> Result<()> {
        // SAFETY: the key pair is valid for the duration of the call.
        unsafe {
            check(OH_CryptoAsymCipher_Init(
                self.raw.as_ptr(),
                mode.into(),
                key.raw.as_ptr(),
            ))
        }
    }

    /// Transform `input` in one shot.
    pub fn finish<'i>(&mut self, input: impl Into<CryptoDataBlob<'i>>) -> Result<Vec<u8>> {
        let input = input.into().to_raw();
        let mut out = OwnedCryptoDataBlob::new();
        // SAFETY: `input` borrows the caller's slice; `out` is filled in by the
        // framework.
        check(unsafe { OH_CryptoAsymCipher_Final(self.raw.as_ptr(), &input, out.as_mut_ptr()) })?;
        Ok(out.to_vec())
    }
}

impl Drop for AsymCipher {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `_Create` and is destroyed once.
        unsafe { OH_CryptoAsymCipher_Destroy(self.raw.as_ptr()) };
    }
}
