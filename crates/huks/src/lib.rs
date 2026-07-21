//! Safe Rust bindings for OpenHarmony **HUKS** (Universal Keystore).
//!
//! HUKS is the HarmonyOS hardware-backed key store: keys are generated and used
//! inside the keystore and never leave it. This crate wraps the native
//! `native_huks_api.h` C API with a safe layer — an RAII [`ParamSet`] builder,
//! `Result`-based error handling, key management, and a three-stage crypto
//! [`Session`].
//!
//! The raw bindings are re-exported as [`sys`] for anything not yet covered.
//!
//! # Example
//!
//! ```no_run
//! use ohos_huks_binding as huks;
//! use huks::{HuksAlias, HuksKeyAlg, HuksKeyDigest, HuksKeyPadding, HuksKeyPurpose, ParamSet};
//!
//! // Generate an RSA-2048 sign/verify key.
//! let params = ParamSet::builder()
//!     .algorithm(HuksKeyAlg::Rsa)
//!     .purposes(&[HuksKeyPurpose::Sign, HuksKeyPurpose::Verify])
//!     .key_size(2048)
//!     .digest(HuksKeyDigest::Sha256)
//!     .padding(HuksKeyPadding::Pss)
//!     .build()?;
//!
//! let alias = HuksAlias::new(b"my_key")?;
//! huks::generate_key(alias, &params)?;
//! assert!(huks::key_exists(alias)?);
//! huks::delete_key(alias)?;
//! # Ok::<(), huks::HuksError>(())
//! ```

pub use ohos_huks_sys as sys;

mod error;
mod key;
mod param;
mod session;
mod r#type;

pub use error::{describe, HuksError, Result};
pub use key::{delete_key, export_public_key, generate_key, import_key, key_exists, HuksAlias};
pub use param::{HuksValue, IntoHuksValue, ParamSet, ParamSetBuilder};
pub use r#type::*;
pub use session::{init_session, Session};
