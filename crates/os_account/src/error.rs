use ohos_os_account_sys as sys;
use std::fmt;

/// Result alias for os account operations.
pub type Result<T> = std::result::Result<T, OsAccountError>;

/// An error returned by an os account operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OsAccountError {
    /// A native call reported this `OsAccount_ErrCode`.
    Native(u32),
    /// The account name returned by the platform was not valid UTF-8.
    NameNotUtf8,
    /// The platform filled the whole buffer without a terminating NUL byte.
    NameNotTerminated,
    /// The account name did not fit into a buffer of `limit` bytes, the largest
    /// buffer the growing read is willing to allocate.
    NameTooLong {
        /// The buffer size, in bytes, that was tried last.
        limit: usize,
    },
}

impl OsAccountError {
    /// The raw `OsAccount_ErrCode`, for errors that carry one.
    pub fn code(&self) -> Option<u32> {
        match self {
            OsAccountError::Native(code) => Some(*code),
            OsAccountError::NameNotUtf8
            | OsAccountError::NameNotTerminated
            | OsAccountError::NameTooLong { .. } => None,
        }
    }
}

impl fmt::Display for OsAccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OsAccountError::Native(code) => {
                write!(f, "os account error {code} ({})", describe(*code))
            }
            OsAccountError::NameNotUtf8 => write!(f, "account name is not valid UTF-8"),
            OsAccountError::NameNotTerminated => {
                write!(f, "account name is not NUL terminated")
            }
            OsAccountError::NameTooLong { limit } => {
                write!(f, "account name does not fit into {limit} bytes")
            }
        }
    }
}

impl std::error::Error for OsAccountError {}

/// Map a raw `OsAccount_ErrCode` to a short, stable description.
///
/// The constants are matched through qualified `sys::` paths so an arm whose
/// constant is absent under the current feature set is a compile error rather
/// than a catch-all binding pattern.
pub fn describe(code: u32) -> &'static str {
    match code {
        sys::OsAccount_ErrCode_OS_ACCOUNT_ERR_OK => "success",
        sys::OsAccount_ErrCode_OS_ACCOUNT_ERR_INTERNAL_ERROR => "internal error",
        sys::OsAccount_ErrCode_OS_ACCOUNT_ERR_INVALID_PARAMETER => "invalid parameter",
        _ => "unknown error",
    }
}

/// Turn a raw `OsAccount_ErrCode` into `Result<()>`.
pub(crate) fn check(code: sys::OsAccount_ErrCode) -> Result<()> {
    if code == sys::OsAccount_ErrCode_OS_ACCOUNT_ERR_OK {
        Ok(())
    } else {
        Err(OsAccountError::Native(code))
    }
}

/// Whether a raw code means "the output buffer was rejected", which for a
/// non-null buffer means it was too small for the name.
pub(crate) fn is_buffer_too_small(code: sys::OsAccount_ErrCode) -> bool {
    code == sys::OsAccount_ErrCode_OS_ACCOUNT_ERR_INVALID_PARAMETER
}
