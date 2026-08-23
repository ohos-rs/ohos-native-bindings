use ohos_time_service_sys as sys;
use std::fmt;

/// Result alias for time service operations.
pub type Result<T> = std::result::Result<T, TimeServiceError>;

/// An error returned by a time service operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeServiceError {
    /// A native call reported this `TimeService_ErrCode`.
    Native(u32),
    /// The time zone ID does not fit into a buffer of this many bytes,
    /// terminating character included.
    BufferTooSmall(usize),
    /// The time zone ID returned by the service is not valid UTF-8.
    InvalidUtf8,
}

impl TimeServiceError {
    /// The raw time service error code (`TimeService_ErrCode`), for errors that
    /// carry one.
    pub fn code(&self) -> Option<u32> {
        match self {
            TimeServiceError::Native(code) => Some(*code),
            TimeServiceError::BufferTooSmall(_) | TimeServiceError::InvalidUtf8 => None,
        }
    }
}

impl fmt::Display for TimeServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeServiceError::Native(code) => {
                write!(f, "time service error {code} ({})", describe(*code))
            }
            TimeServiceError::BufferTooSmall(capacity) => {
                write!(f, "time zone id does not fit into {capacity} bytes")
            }
            TimeServiceError::InvalidUtf8 => write!(f, "time zone id is not valid utf-8"),
        }
    }
}

impl std::error::Error for TimeServiceError {}

/// Map a raw `TimeService_ErrCode` to a short, stable description.
///
/// The constants are matched through qualified `sys::` paths so that an arm
/// whose constant is absent under the current feature set is a compile error
/// instead of a catch-all binding pattern.
pub fn describe(code: u32) -> &'static str {
    match code {
        sys::TimeService_ErrCode_TIMESERVICE_ERR_OK => "success",
        sys::TimeService_ErrCode_TIMESERVICE_ERR_INTERNAL_ERROR => {
            "failed to obtain the system parameters"
        }
        sys::TimeService_ErrCode_TIMESERVICE_ERR_INVALID_PARAMETER => "invalid parameter",
        _ => "unknown error",
    }
}

/// Turn a raw `TimeService_ErrCode` into `Result<()>`.
pub(crate) fn check(code: sys::TimeService_ErrCode) -> Result<()> {
    if code == sys::TimeService_ErrCode_TIMESERVICE_ERR_OK {
        Ok(())
    } else {
        Err(TimeServiceError::Native(code))
    }
}
