use ohos_hidebug_sys as sys;
use std::fmt;

/// Result alias for hidebug operations.
pub type Result<T> = std::result::Result<T, HiDebugError>;

/// An error returned by a hidebug operation, carrying the raw
/// `HiDebug_ErrorCode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HiDebugError {
    code: u32,
}

impl HiDebugError {
    pub(crate) fn from_code(code: sys::HiDebug_ErrorCode) -> Self {
        HiDebugError { code }
    }

    /// The raw hidebug error code (`HiDebug_ErrorCode`).
    pub fn code(&self) -> u32 {
        self.code
    }
}

impl fmt::Display for HiDebugError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hidebug error {} ({})", self.code, describe(self.code))
    }
}

impl std::error::Error for HiDebugError {}

/// Map a raw `HiDebug_ErrorCode` to a short, stable description.
///
/// The constants are matched through qualified `sys::` paths so that an arm
/// whose constant is absent under the current feature set is a compile error
/// instead of a catch-all binding pattern.
pub fn describe(code: u32) -> &'static str {
    match code {
        sys::HiDebug_ErrorCode_HIDEBUG_SUCCESS => "success",
        sys::HiDebug_ErrorCode_HIDEBUG_INVALID_ARGUMENT => "invalid argument",
        sys::HiDebug_ErrorCode_HIDEBUG_TRACE_CAPTURED_ALREADY => "trace already being captured",
        sys::HiDebug_ErrorCode_HIDEBUG_NO_PERMISSION => "permission denied",
        sys::HiDebug_ErrorCode_HIDEBUG_TRACE_ABNORMAL => "trace state abnormal",
        sys::HiDebug_ErrorCode_HIDEBUG_NO_TRACE_RUNNING => "no trace running",
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_OH_HIDEBUG_TRACE_STORAGE_LIMIT => "trace storage limit reached",
        #[cfg(feature = "api-20")]
        sys::HiDebug_ErrorCode_HIDEBUG_INVALID_SYMBOLIC_PC_ADDRESS => "invalid symbolic pc address",
        #[cfg(feature = "api-22")]
        sys::HiDebug_ErrorCode_HIDEBUG_NOT_SUPPORTED => "not supported on this device",
        #[cfg(feature = "api-22")]
        sys::HiDebug_ErrorCode_HIDEBUG_UNDER_SAMPLING => "sampling in progress",
        #[cfg(feature = "api-22")]
        sys::HiDebug_ErrorCode_HIDEBUG_RESOURCE_UNAVAILABLE => "resource unavailable",
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_SUCCESS => "resource profiler success",
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_INVALID_ARG => "invalid resource profiler argument",
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_INVALID_MAX_DURATION => "invalid maximum duration",
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_INVALID_FILTER_SIZE => "invalid filter size",
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_INVALID_MAX_STACK_DEPTH => {
            "invalid maximum stack depth"
        }
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_INVALID_STATISTICS_INTERVAL => {
            "invalid statistics interval"
        }
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_INVALID_SAMPLE_INTERVAL => {
            "invalid sample interval"
        }
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_INVALID_RESOURCE_TYPE => "invalid resource type",
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_PERMISSION_DENIED => {
            "resource profiler permission denied"
        }
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_ALREADY_STARTED => {
            "resource profiler already started"
        }
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_NOT_STARTED => "resource profiler not started",
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_PROCESS_OVERLIMIT => {
            "resource profiler process count exceeded"
        }
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_CONFLICT => {
            "resource profiler conflicts with another profiling task"
        }
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_AUTO_STOPPED_BY_DURATION => {
            "resource profiler stopped by duration limit"
        }
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_DAILY_QUOTA_EXCEEDED => "daily quota exceeded",
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_CPU_OVERLOADED => "cpu overloaded",
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_MEM_PRESSURE_CRITICAL => "insufficient memory",
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_STORAGE_PRESSURE_CRITICAL => "insufficient storage",
        #[cfg(feature = "api-24")]
        sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_FAILURE => "resource profiler failure",
        _ => "unknown error",
    }
}

/// Turn a raw `HiDebug_ErrorCode` into `Result<()>`.
pub(crate) fn check(code: sys::HiDebug_ErrorCode) -> Result<()> {
    if code == sys::HiDebug_ErrorCode_HIDEBUG_SUCCESS {
        Ok(())
    } else {
        Err(HiDebugError::from_code(code))
    }
}

/// The resource profiler reports success as `HIDEBUG_RES_PROF_SUCCESS`, not 0.
#[cfg(feature = "api-24")]
pub(crate) fn check_profiler(code: sys::HiDebug_ErrorCode) -> Result<()> {
    if code == sys::HiDebug_ErrorCode_HIDEBUG_RES_PROF_SUCCESS {
        Ok(())
    } else {
        Err(HiDebugError::from_code(code))
    }
}
