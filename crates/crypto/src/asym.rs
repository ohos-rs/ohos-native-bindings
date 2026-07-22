use crate::blob::{borrow_cstr, c_string, OptionalBlob, OutBlob};
use crate::error::{check, CryptoError, Result};
use crate::r#type::{CryptoAsymKeyParamType, CryptoEncodingType};
use ohos_crypto_sys::*;
use std::marker::PhantomData;
use std::ptr::{self, NonNull};

/// Generator for asymmetric key pairs, created from an algorithm name such as
/// `"RSA2048"`, `"ECC256"`, `"Ed25519"` or `"SM2_256"`.
pub struct AsymKeyGenerator {
    raw: NonNull<OH_CryptoAsymKeyGenerator>,
}

impl AsymKeyGenerator {
    /// Create a generator for `algo_name`.
    pub fn new(algo_name: &str) -> Result<Self> {
        let name = c_string(algo_name)?;
        let mut raw = ptr::null_mut();
        // SAFETY: `name` is NUL-terminated and `raw` receives the new context.
        check(unsafe { OH_CryptoAsymKeyGenerator_Create(name.as_ptr(), &mut raw) })?;
        let raw =
            NonNull::new(raw).ok_or(CryptoError::new(OH_Crypto_ErrCode_CRYPTO_OPERTION_ERROR))?;
        Ok(AsymKeyGenerator { raw })
    }

    /// Generate a fresh key pair.
    pub fn generate(&self) -> Result<KeyPair> {
        let mut raw = ptr::null_mut();
        // SAFETY: the generator is live and `raw` receives the new key pair.
        check(unsafe { OH_CryptoAsymKeyGenerator_Generate(self.raw.as_ptr(), &mut raw) })?;
        let raw =
            NonNull::new(raw).ok_or(CryptoError::new(OH_Crypto_ErrCode_CRYPTO_OPERTION_ERROR))?;
        Ok(KeyPair { raw })
    }

    /// Build a key pair from encoded key material. Either half may be omitted.
    pub fn convert(
        &self,
        encoding: CryptoEncodingType,
        pub_key: Option<&[u8]>,
        priv_key: Option<&[u8]>,
    ) -> Result<KeyPair> {
        let mut pub_key = OptionalBlob::new(pub_key);
        let mut priv_key = OptionalBlob::new(priv_key);
        let mut raw = ptr::null_mut();
        // SAFETY: both blobs borrow the caller's slices for the duration of the call.
        check(unsafe {
            OH_CryptoAsymKeyGenerator_Convert(
                self.raw.as_ptr(),
                encoding.into(),
                pub_key.as_mut_ptr(),
                priv_key.as_mut_ptr(),
                &mut raw,
            )
        })?;
        let raw =
            NonNull::new(raw).ok_or(CryptoError::new(OH_Crypto_ErrCode_CRYPTO_OPERTION_ERROR))?;
        Ok(KeyPair { raw })
    }

    /// Set the password used to decrypt an encrypted private key passed to
    /// [`convert`](Self::convert).
    #[cfg(feature = "api-20")]
    pub fn set_password(&mut self, password: &[u8]) -> Result<()> {
        let len = crate::error::checked_u32(password.len())?;
        // SAFETY: the pointer and length describe `password` for the duration of the call.
        unsafe {
            check(OH_CryptoAsymKeyGenerator_SetPassword(
                self.raw.as_ptr(),
                password.as_ptr(),
                len,
            ))
        }
    }

    /// The algorithm name this generator was created with.
    pub fn algo_name(&self) -> Option<&str> {
        // SAFETY: the returned string is owned by the generator and lives as long as it.
        unsafe { borrow_cstr(OH_CryptoAsymKeyGenerator_GetAlgoName(self.raw.as_ptr())) }
    }
}

impl Drop for AsymKeyGenerator {
    fn drop(&mut self) {
        // SAFETY: `raw` came from `_Create` and is destroyed once.
        unsafe { OH_CryptoAsymKeyGenerator_Destroy(self.raw.as_ptr()) };
    }
}

/// An asymmetric key pair, owning both halves.
pub struct KeyPair {
    pub(crate) raw: NonNull<OH_CryptoKeyPair>,
}

impl KeyPair {
    /// Borrow the public half.
    pub fn pub_key(&self) -> Option<PubKey<'_>> {
        // SAFETY: the key pair is live for the returned borrow.
        let raw = unsafe { OH_CryptoKeyPair_GetPubKey(self.raw.as_ptr()) };
        NonNull::new(raw).map(|raw| PubKey {
            raw,
            _owner: PhantomData,
        })
    }

    /// Borrow the private half.
    #[cfg(feature = "api-20")]
    pub fn priv_key(&self) -> Option<PrivKey<'_>> {
        // SAFETY: the key pair is live for the returned borrow.
        let raw = unsafe { OH_CryptoKeyPair_GetPrivKey(self.raw.as_ptr()) };
        NonNull::new(raw).map(|raw| PrivKey {
            raw,
            _owner: PhantomData,
        })
    }
}

impl Drop for KeyPair {
    fn drop(&mut self) {
        // SAFETY: `raw` came from the generator and is destroyed once.
        unsafe { OH_CryptoKeyPair_Destroy(self.raw.as_ptr()) };
    }
}

/// The public half of a [`KeyPair`].
///
/// The framework exposes no destructor for it: it is a view into the key pair,
/// which therefore has to outlive it.
pub struct PubKey<'a> {
    pub(crate) raw: NonNull<OH_CryptoPubKey>,
    _owner: PhantomData<&'a KeyPair>,
}

impl PubKey<'_> {
    /// Encode the key. `standard` selects the encoding standard where the
    /// algorithm offers a choice, e.g. `"X509"` or `"PKCS1"` for RSA.
    pub fn encode(&self, encoding: CryptoEncodingType, standard: Option<&str>) -> Result<Vec<u8>> {
        let standard = standard.map(c_string).transpose()?;
        let standard = standard.as_ref().map_or(ptr::null(), |s| s.as_ptr());
        let mut out = OutBlob::new();
        // SAFETY: the owning key pair is alive; `out` is filled in by the framework.
        check(unsafe {
            OH_CryptoPubKey_Encode(
                self.raw.as_ptr(),
                encoding.into(),
                standard,
                out.as_mut_ptr(),
            )
        })?;
        Ok(out.to_vec())
    }

    /// Read a single key component, e.g. the RSA modulus or an EC coordinate.
    pub fn param(&self, item: CryptoAsymKeyParamType) -> Result<Vec<u8>> {
        let mut out = OutBlob::new();
        // SAFETY: the owning key pair is alive; `out` is filled in by the framework.
        check(unsafe {
            OH_CryptoPubKey_GetParam(self.raw.as_ptr(), item.into(), out.as_mut_ptr())
        })?;
        Ok(out.to_vec())
    }
}

/// The private half of a [`KeyPair`].
///
/// Like [`PubKey`] this is a view into the key pair and has no destructor.
#[cfg(feature = "api-20")]
pub struct PrivKey<'a> {
    pub(crate) raw: NonNull<OH_CryptoPrivKey>,
    _owner: PhantomData<&'a KeyPair>,
}

#[cfg(feature = "api-20")]
impl PrivKey<'_> {
    /// Encode the key unencrypted. `standard` selects the encoding standard
    /// where the algorithm offers a choice, e.g. `"PKCS8"` or `"PKCS1"`.
    ///
    /// Password-protected encoding needs `OH_CryptoPrivKeyEncodingParams`,
    /// which this crate does not wrap yet.
    pub fn encode(&self, encoding: CryptoEncodingType, standard: Option<&str>) -> Result<Vec<u8>> {
        let standard = standard.map(c_string).transpose()?;
        let standard = standard.as_ref().map_or(ptr::null(), |s| s.as_ptr());
        let mut out = OutBlob::new();
        // SAFETY: the owning key pair is alive; a null params pointer selects
        // unencrypted output.
        check(unsafe {
            OH_CryptoPrivKey_Encode(
                self.raw.as_ptr(),
                encoding.into(),
                standard,
                ptr::null_mut(),
                out.as_mut_ptr(),
            )
        })?;
        Ok(out.to_vec())
    }

    /// Read a single key component, e.g. the RSA private exponent.
    pub fn param(&self, item: CryptoAsymKeyParamType) -> Result<Vec<u8>> {
        let mut out = OutBlob::new();
        // SAFETY: the owning key pair is alive; `out` is filled in by the framework.
        check(unsafe {
            OH_CryptoPrivKey_GetParam(self.raw.as_ptr(), item.into(), out.as_mut_ptr())
        })?;
        Ok(out.to_vec())
    }
}
