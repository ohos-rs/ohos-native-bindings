use ohos_hicollie_sys as sys;
use std::fmt;

/// Result alias for hicollie operations.
pub type Result<T> = std::result::Result<T, HiCollieError>;

/// An error returned by a hicollie operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HiCollieError {
    /// A native call reported this `HiCollie_ErrorCode`.
    Native(u32),
    /// Jank detection is already set up on the calling thread; the runtime
    /// reports success without handing out the stubs again.
    JankAlreadyInitialized,
}

impl HiCollieError {
    /// The raw hicollie error code (`HiCollie_ErrorCode`), for errors that
    /// carry one.
    pub fn code(&self) -> Option<u32> {
        match self {
            HiCollieError::Native(code) => Some(*code),
            HiCollieError::JankAlreadyInitialized => None,
        }
    }
}

impl fmt::Display for HiCollieError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HiCollieError::Native(code) => {
                write!(f, "hicollie error {code} ({})", describe(*code))
            }
            HiCollieError::JankAlreadyInitialized => {
                write!(f, "jank detection is already initialized on this thread")
            }
        }
    }
}

impl std::error::Error for HiCollieError {}

/// Map a raw `HiCollie_ErrorCode` to a short, stable description.
///
/// The constants are matched through qualified `sys::` paths so that an arm
/// whose constant is absent under the current feature set is a compile error
/// instead of a catch-all binding pattern.
pub fn describe(code: u32) -> &'static str {
    match code {
        sys::HiCollie_ErrorCode_HICOLLIE_SUCCESS => "success",
        sys::HiCollie_ErrorCode_HICOLLIE_INVALID_ARGUMENT => "invalid argument",
        sys::HiCollie_ErrorCode_HICOLLIE_WRONG_THREAD_CONTEXT => "wrong thread context",
        sys::HiCollie_ErrorCode_HICOLLIE_REMOTE_FAILED => "remote call failed",
        #[cfg(feature = "api-18")]
        sys::HiCollie_ErrorCode_HICOLLIE_INVALID_TIMER_NAME => "invalid timer name",
        #[cfg(feature = "api-18")]
        sys::HiCollie_ErrorCode_HICOLLIE_INVALID_TIMEOUT_VALUE => "invalid timeout value",
        #[cfg(feature = "api-18")]
        sys::HiCollie_ErrorCode_HICOLLIE_WRONG_PROCESS_CONTEXT => "wrong process context",
        #[cfg(feature = "api-18")]
        sys::HiCollie_ErrorCode_HICOLLIE_WRONG_TIMER_ID_OUTPUT_PARAM => {
            "null timer id output pointer"
        }
        #[cfg(feature = "api-24")]
        sys::HiCollie_ErrorCode_OH_HICOLLIE_REACH_REPORT_LIMIT => "report rate limit reached",
        _ => "unknown error",
    }
}

/// Turn a raw `HiCollie_ErrorCode` into `Result<()>`.
pub(crate) fn check(code: sys::HiCollie_ErrorCode) -> Result<()> {
    if code == sys::HiCollie_ErrorCode_HICOLLIE_SUCCESS {
        Ok(())
    } else {
        Err(HiCollieError::Native(code))
    }
}
