use crate::error::{CertManagerError, Result};
use ohos_cert_manager_sys as sys;
use std::fmt;

/// The purpose a certificate is stored for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CertificatePurpose {
    /// The default purpose.
    Default,
    /// Every purpose; only meaningful when querying certificates.
    All,
    /// Signature.
    Sign,
    /// Encryption.
    Encrypt,
}

impl CertificatePurpose {
    /// Map a raw `OH_CM_CertificatePurpose` value, or `None` when the runtime
    /// reports a purpose this binding does not know.
    pub(crate) fn from_raw(raw: sys::OH_CM_CertificatePurpose) -> Option<Self> {
        // Patterns use fully qualified `sys::` paths on purpose: a bare constant
        // name that is not in scope silently degrades into a catch-all binding.
        match raw {
            sys::OH_CM_CertificatePurpose_OH_CM_CERT_PURPOSE_DEFAULT => Some(Self::Default),
            sys::OH_CM_CertificatePurpose_OH_CM_CERT_PURPOSE_ALL => Some(Self::All),
            sys::OH_CM_CertificatePurpose_OH_CM_CERT_PURPOSE_SIGN => Some(Self::Sign),
            sys::OH_CM_CertificatePurpose_OH_CM_CERT_PURPOSE_ENCRYPT => Some(Self::Encrypt),
            _ => None,
        }
    }

    pub(crate) fn to_raw(self) -> sys::OH_CM_CertificatePurpose {
        match self {
            Self::Default => sys::OH_CM_CertificatePurpose_OH_CM_CERT_PURPOSE_DEFAULT,
            Self::All => sys::OH_CM_CertificatePurpose_OH_CM_CERT_PURPOSE_ALL,
            Self::Sign => sys::OH_CM_CertificatePurpose_OH_CM_CERT_PURPOSE_SIGN,
            Self::Encrypt => sys::OH_CM_CertificatePurpose_OH_CM_CERT_PURPOSE_ENCRYPT,
        }
    }
}

/// The uri a certificate or credential is addressed by.
///
/// Checked on construction so an empty, over-long or NUL-containing uri fails
/// here rather than as an opaque error from the certificate manager. The native
/// API takes the uri as a sized blob whose length includes the terminator, so
/// the limit is `OH_CM_MAX_LEN_URI - 1` characters.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CertUri(String);

impl CertUri {
    /// Number of characters a uri may hold, excluding the terminator.
    pub const MAX_LEN: usize = sys::OH_CM_MAX_LEN_URI as usize - 1;

    /// Wrap `uri`, rejecting an empty uri, one longer than [`CertUri::MAX_LEN`],
    /// and one containing an interior NUL.
    pub fn new(uri: impl Into<String>) -> Result<Self> {
        let uri = uri.into();
        if uri.is_empty() {
            return Err(CertManagerError::invalid_argument(
                "certificate uri is empty",
            ));
        }
        if uri.len() > Self::MAX_LEN {
            return Err(CertManagerError::invalid_argument(format!(
                "certificate uri is {} bytes, limit is {}",
                uri.len(),
                Self::MAX_LEN
            )));
        }
        if uri.as_bytes().contains(&0) {
            return Err(CertManagerError::invalid_argument(
                "certificate uri contains an interior NUL",
            ));
        }
        Ok(CertUri(uri))
    }

    /// The uri as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The uri as the NUL-terminated byte buffer the native API expects. The
    /// returned buffer must stay alive for the duration of the call it backs.
    pub(crate) fn to_bytes_with_nul(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.0.len() + 1);
        bytes.extend_from_slice(self.0.as_bytes());
        bytes.push(0);
        bytes
    }
}

impl fmt::Display for CertUri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<&str> for CertUri {
    type Error = CertManagerError;

    fn try_from(uri: &str) -> Result<Self> {
        CertUri::new(uri)
    }
}

impl TryFrom<String> for CertUri {
    type Error = CertManagerError;

    fn try_from(uri: String) -> Result<Self> {
        CertUri::new(uri)
    }
}
