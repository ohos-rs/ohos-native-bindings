use ohos_dlp_permission_sys as sys;
use std::fmt;

/// Result alias for DLP permission operations.
pub type Result<T> = std::result::Result<T, DlpError>;

/// An error returned by a DLP permission operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DlpError {
    /// A native call reported this `DLP_ErrCode`.
    Native(u32),
    /// The supplied Rust string contains an interior NUL byte and cannot be
    /// passed to the C API.
    InteriorNul,
    /// The string returned by the native call is not valid UTF-8.
    InvalidUtf8,
    /// The native call reported success but left the output pointer null.
    MissingOutput,
    /// The native call reported a `DLP_FileAccess` value that is not part of
    /// the documented enumeration.
    UnknownFileAccess(u32),
}

impl DlpError {
    /// The raw DLP error code (`DLP_ErrCode`), for errors that carry one.
    pub fn code(&self) -> Option<u32> {
        match self {
            DlpError::Native(code) => Some(*code),
            _ => None,
        }
    }
}

impl fmt::Display for DlpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DlpError::Native(code) => write!(f, "dlp error {code} ({})", describe(*code)),
            DlpError::InteriorNul => write!(f, "string contains an interior NUL byte"),
            DlpError::InvalidUtf8 => write!(f, "native string is not valid UTF-8"),
            DlpError::MissingOutput => {
                write!(f, "native call succeeded but produced no output value")
            }
            DlpError::UnknownFileAccess(raw) => {
                write!(f, "unknown DLP file access value {raw}")
            }
        }
    }
}

impl std::error::Error for DlpError {}

/// Map a raw `DLP_ErrCode` to a short, stable description.
///
/// The constants are matched through qualified `sys::` paths so that an arm
/// whose constant is absent under the current feature set is a compile error
/// instead of a catch-all binding pattern.
pub fn describe(code: u32) -> &'static str {
    match code {
        sys::DLP_ErrCode_ERR_OH_SUCCESS => "success",
        sys::DLP_ErrCode_ERR_OH_INVALID_PARAMETER => "invalid parameter",
        sys::DLP_ErrCode_ERR_OH_API_ONLY_FOR_SANDBOX => {
            "api available only to DLP sandbox applications"
        }
        sys::DLP_ErrCode_ERR_OH_API_NOT_FOR_SANDBOX => {
            "api available only to non-sandbox applications"
        }
        sys::DLP_ErrCode_ERR_OH_SYSTEM_SERVICE_EXCEPTION => "system ability works abnormally",
        sys::DLP_ErrCode_ERR_OH_OUT_OF_MEMORY => "out of memory",
        sys::DLP_ErrCode_ERR_OH_APPLICATION_NOT_AUTHORIZED => "application is not authorized",
        _ => "unknown error",
    }
}

/// Turn a raw `DLP_ErrCode` into `Result<()>`.
pub(crate) fn check(code: sys::DLP_ErrCode) -> Result<()> {
    if code == sys::DLP_ErrCode_ERR_OH_SUCCESS {
        Ok(())
    } else {
        Err(DlpError::Native(code))
    }
}
