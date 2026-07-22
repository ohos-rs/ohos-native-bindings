use std::ffi::CString;
use std::fmt;

/// Result alias for HiTrace operations.
pub type Result<T> = std::result::Result<T, HiTraceError>;

/// An error returned by a HiTrace operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HiTraceError {
    /// A `&str` argument contained an interior NUL byte and cannot be passed to
    /// the native API.
    NulByte,
    /// A byte slice was not the width of a `HiTraceId`.
    InvalidByteLength,
    /// All trace listener slots are taken.
    ListenerLimit,
    /// The trace listener or listener index was rejected by the runtime.
    InvalidListener,
}

impl fmt::Display for HiTraceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HiTraceError::NulByte => write!(f, "argument contains an interior NUL byte"),
            HiTraceError::InvalidByteLength => write!(f, "byte slice is not one HiTraceId wide"),
            HiTraceError::ListenerLimit => write!(f, "trace listener limit reached"),
            HiTraceError::InvalidListener => write!(f, "invalid trace listener"),
        }
    }
}

impl std::error::Error for HiTraceError {}

pub(crate) fn cstring(value: &str) -> Result<CString> {
    CString::new(value).map_err(|_| HiTraceError::NulByte)
}
