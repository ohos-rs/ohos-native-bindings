use ohos_transient_task_sys as sys;
use std::fmt;

/// Result alias for transient task operations.
pub type Result<T> = std::result::Result<T, TransientTaskError>;

/// An error returned by a transient task operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransientTaskError {
    /// A native call reported this `TransientTask_ErrorCode`.
    Native(i32),
    /// The reason string contains an interior NUL byte and cannot be passed to
    /// the C API.
    InvalidReason,
}

impl TransientTaskError {
    /// The raw transient task error code, for errors that carry one.
    pub fn code(&self) -> Option<i32> {
        match self {
            TransientTaskError::Native(code) => Some(*code),
            TransientTaskError::InvalidReason => None,
        }
    }
}

impl fmt::Display for TransientTaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransientTaskError::Native(code) => {
                write!(f, "transient task error {code} ({})", describe(*code))
            }
            TransientTaskError::InvalidReason => {
                write!(f, "reason contains an interior NUL byte")
            }
        }
    }
}

impl std::error::Error for TransientTaskError {}

/// Map a raw `TransientTask_ErrorCode` to a short, stable description.
///
/// The constants are matched through qualified `sys::` paths so that an arm
/// whose constant is absent under the current feature set is a compile error
/// instead of a catch-all binding pattern.
// Every code below is `@since 13`, and this module only exists under `api-13`,
// so no arm needs its own gate.
pub fn describe(code: i32) -> &'static str {
    let Ok(code) = u32::try_from(code) else {
        return "unknown error";
    };
    match code {
        sys::TransientTask_ErrorCode_ERR_TRANSIENT_TASK_OK => "success",
        sys::TransientTask_ErrorCode_ERR_TRANSIENT_TASK_INVALID_PARAM => "invalid parameter",
        sys::TransientTask_ErrorCode_ERR_TRANSIENT_TASK_PARCEL_FAILED => "parcel operation failed",
        sys::TransientTask_ErrorCode_ERR_TRANSIENT_TASK_TRANSACTION_FAILED => {
            "internal transaction failed"
        }
        sys::TransientTask_ErrorCode_ERR_TRANSIENT_TASK_SYS_NOT_READY => "system service not ready",
        sys::TransientTask_ErrorCode_ERR_TRANSIENT_TASK_CLIENT_INFO_VERIFICATION_FAILED => {
            "caller uid or pid verification failed"
        }
        sys::TransientTask_ErrorCode_ERR_TRANSIENT_TASK_SERVICE_VERIFICATION_FAILED => {
            "transient task verification failed"
        }
        sys::TransientTask_ErrorCode_ERR_TRANSIENT_TASK_PARCELABLE_FAILED => {
            "failed to write data into parcel"
        }
        sys::TransientTask_ErrorCode_ERR_TRANSIENT_TASK_SERVICE_NOT_READY => {
            "system service operation failed"
        }
        _ => "unknown error",
    }
}

/// Turn a raw return code into `Result<()>`.
pub(crate) fn check(code: i32) -> Result<()> {
    if code == sys::TransientTask_ErrorCode_ERR_TRANSIENT_TASK_OK as i32 {
        Ok(())
    } else {
        Err(TransientTaskError::Native(code))
    }
}
