//! Safe Rust bindings for the OpenHarmony **Certificate Manager**.
//!
//! The certificate manager stores the certificates and credentials an
//! application is allowed to use: private (application) certificates, public
//! (user) certificates, and certificates held on a USB key. This crate wraps the
//! native `cm_native_api.h` C API with a safe layer — a validated [`CertUri`],
//! `Result`-based error handling, and RAII [`Credential`] / [`CredentialList`]
//! types that release the buffers the service allocated.
//!
//! The whole API is available from API 22 on, behind the `api-22` feature. The
//! raw bindings are re-exported as [`sys`] for anything not yet covered.
//!
//! Every call needs the `ohos.permission.ACCESS_CERT_MANAGER` permission.
//!
//! # Example
//!
//! ```no_run
//! use ohos_cert_manager_binding as cert_manager;
//! use cert_manager::CertUri;
//!
//! let uri = CertUri::new("oh:t=ak;o=alias;u=0;a=0")?;
//! let credential = cert_manager::get_public_certificate(&uri)?;
//!
//! println!("alias: {}", credential.alias());
//! println!("chain: {} bytes", credential.data().len());
//! # Ok::<(), cert_manager::CertManagerError>(())
//! ```

pub use ohos_cert_manager_sys as sys;

#[cfg(feature = "api-22")]
mod credential;
#[cfg(feature = "api-22")]
mod error;
#[cfg(feature = "api-22")]
mod r#type;

#[cfg(feature = "api-22")]
pub use credential::{
    get_private_certificate, get_public_certificate, get_ukey_certificate, Credential,
    CredentialList, CredentialView, MAX_LEN_CERTIFICATE_CHAIN,
};
#[cfg(feature = "api-22")]
pub use error::{describe, CertManagerError, Result};
#[cfg(feature = "api-22")]
pub use r#type::{CertUri, CertificatePurpose};
