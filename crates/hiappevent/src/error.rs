use std::ffi::NulError;
use std::fmt;

/// Result alias for HiAppEvent operations.
pub type Result<T> = std::result::Result<T, HiAppEventError>;

/// An error returned by a HiAppEvent operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HiAppEventError {
    /// A native call reported this HiAppEvent error code.
    Native(i32),
    /// A native call that reports only success or failure failed.
    Failed,
    /// A native handle could not be allocated.
    Alloc,
    /// A string argument contained an interior NUL byte.
    Nul,
}

impl HiAppEventError {
    /// The raw HiAppEvent error code, for errors that carry one.
    pub fn code(&self) -> Option<i32> {
        match self {
            HiAppEventError::Native(code) => Some(*code),
            _ => None,
        }
    }
}

impl From<NulError> for HiAppEventError {
    fn from(_: NulError) -> Self {
        HiAppEventError::Nul
    }
}

impl fmt::Display for HiAppEventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HiAppEventError::Native(code) => {
                write!(f, "HiAppEvent error {code} ({})", describe(*code))
            }
            HiAppEventError::Failed => write!(f, "HiAppEvent operation failed"),
            HiAppEventError::Alloc => write!(f, "HiAppEvent handle allocation failed"),
            HiAppEventError::Nul => write!(f, "string argument contains an interior NUL byte"),
        }
    }
}

impl std::error::Error for HiAppEventError {}

/// Map a raw HiAppEvent error code to a short, stable description.
// The `HiAppEvent_ErrorCode_*` constants are feature-gated in the sys crate, so
// the codes are matched as literals to keep the table complete under any feature
// set. It also covers the write-path codes, which have no named constants.
pub fn describe(code: i32) -> &'static str {
    match code {
        0 => "success",
        1 => "invalid event parameter name",
        4 => "invalid event parameter value length",
        5 => "invalid number of event parameters",
        6 => "invalid event parameter array length",
        8 => "duplicate event parameter",
        -1 => "invalid event name",
        -4 => "invalid event domain",
        -5 => "the watcher is null",
        -6 => "the watcher has not been added",
        -7 => "the processor is null",
        -8 => "processor not found",
        -9 => "invalid parameter value",
        -10 => "the event config is null",
        -99 => "event logging is disabled",
        -100 => "operation failed",
        -200 => "invalid uid",
        _ => "unknown error",
    }
}

/// Turn a raw HiAppEvent status into `Result<()>`.
pub(crate) fn check(code: i32) -> Result<()> {
    if code == 0 {
        Ok(())
    } else {
        Err(HiAppEventError::Native(code))
    }
}
