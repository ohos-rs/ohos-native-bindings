//! Safe Rust bindings for the OpenHarmony **Crypto Architecture Kit**.
//!
//! The kit is the framework's general-purpose software crypto library: keys live
//! in the application process and can be imported and exported, which is what
//! sets it apart from HUKS. This crate wraps the native `libohcrypto` C API with
//! RAII contexts and `Result`-based error handling — symmetric ciphers, message
//! digests, asymmetric keys and signature verification, plus MAC, KDF, key
//! agreement, asymmetric ciphers and random number generation behind the
//! `api-20` feature.
//!
//! Byte input is taken as [`CryptoDataBlob`], a borrowing wrapper every
//! operation accepts through `impl Into<CryptoDataBlob>`, so slices, arrays and
//! `Vec`s can be passed directly. Output buffers are allocated by the framework;
//! the safe layer copies them out and releases them on drop, so every operation
//! returns a plain `Vec<u8>`.
//!
//! The raw bindings are re-exported as [`sys`] for anything not yet covered.
//!
//! # Example
//!
//! ```no_run
//! use ohos_crypto_binding as crypto;
//! use crypto::{CryptoCipherMode, CryptoSymCipherParamsType, SymCipher, SymCipherParams, SymKeyGenerator};
//!
//! // AES-256-GCM encryption under a freshly generated key.
//! let key = SymKeyGenerator::new("AES256")?.generate()?;
//!
//! let mut params = SymCipherParams::new()?;
//! params.set(CryptoSymCipherParamsType::Iv, &[0u8; 12][..])?;
//!
//! let mut cipher = SymCipher::new("AES256|GCM|PKCS7")?;
//! cipher.init(CryptoCipherMode::Encrypt, &key, Some(&params))?;
//! let mut ciphertext = cipher.update(b"hello")?;
//! ciphertext.extend(cipher.finish(None)?);
//! # Ok::<(), crypto::CryptoError>(())
//! ```

pub use ohos_crypto_sys as sys;

mod asym;
mod blob;
mod digest;
mod error;
mod signature;
mod sym;
mod r#type;
mod value;

#[cfg(feature = "api-20")]
mod asym_cipher;
#[cfg(feature = "api-20")]
mod kdf;
#[cfg(feature = "api-20")]
mod key_agreement;
#[cfg(feature = "api-20")]
mod mac;
#[cfg(feature = "api-20")]
mod rand;

pub use asym::{AsymKeyGenerator, KeyPair, PubKey};
pub use blob::CryptoDataBlob;
pub use digest::Digest;
pub use error::{describe, CryptoError, Result};
pub use r#type::*;
pub use signature::Verify;
pub use sym::{SymCipher, SymCipherParams, SymKey, SymKeyGenerator};
pub use value::IntoCryptoValue;

#[cfg(feature = "api-20")]
pub use asym::PrivKey;
#[cfg(feature = "api-20")]
pub use asym_cipher::AsymCipher;
#[cfg(feature = "api-20")]
pub use kdf::{Kdf, KdfParams};
#[cfg(feature = "api-20")]
pub use key_agreement::KeyAgreement;
#[cfg(feature = "api-20")]
pub use mac::Mac;
#[cfg(feature = "api-20")]
pub use rand::Rand;
#[cfg(feature = "api-20")]
pub use signature::Sign;
