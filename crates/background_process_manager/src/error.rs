use ohos_background_process_manager_sys as sys;
use std::fmt;
use std::os::raw::c_int;

/// Result alias for background process manager operations.
pub type Result<T> = std::result::Result<T, BackgroundProcessManagerError>;

/// An error returned by a background process manager operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundProcessManagerError {
    /// A parameter was rejected, typically a priority outside the supported
    /// range.
    InvalidParam,
    /// The request could not be delivered to the resource schedule service.
    RemoteError,
    /// A `BackgroundProcessManager_ErrorCode` this crate does not know about.
    Unknown(u32),
}

impl BackgroundProcessManagerError {
    /// The raw `BackgroundProcessManager_ErrorCode` behind this error.
    pub fn code(&self) -> u32 {
        match self {
            BackgroundProcessManagerError::InvalidParam => {
                sys::BackgroundProcessManager_ErrorCode_ERR_BACKGROUND_PROCESS_MANAGER_INVALID_PARAM
            }
            BackgroundProcessManagerError::RemoteError => {
                sys::BackgroundProcessManager_ErrorCode_ERR_BACKGROUND_PROCESS_MANAGER_REMOTE_ERROR
            }
            BackgroundProcessManagerError::Unknown(code) => *code,
        }
    }
}

impl fmt::Display for BackgroundProcessManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = self.code();
        write!(
            f,
            "background process manager error {code} ({})",
            describe(code)
        )
    }
}

impl std::error::Error for BackgroundProcessManagerError {}

/// Map a raw `BackgroundProcessManager_ErrorCode` to a short, stable
/// description.
///
/// The constants are matched through qualified `sys::` paths so that an arm
/// whose constant is absent under the current feature set is a compile error
/// instead of a catch-all binding pattern.
pub fn describe(code: u32) -> &'static str {
    match code {
        sys::BackgroundProcessManager_ErrorCode_ERR_BACKGROUND_PROCESS_MANAGER_SUCCESS => "success",
        sys::BackgroundProcessManager_ErrorCode_ERR_BACKGROUND_PROCESS_MANAGER_INVALID_PARAM => {
            "invalid parameter"
        }
        sys::BackgroundProcessManager_ErrorCode_ERR_BACKGROUND_PROCESS_MANAGER_REMOTE_ERROR => {
            "remote call failed"
        }
        _ => "unknown error",
    }
}

/// Turn the `int` returned by a native call into `Result<()>`.
pub(crate) fn check(code: c_int) -> Result<()> {
    match code as u32 {
        sys::BackgroundProcessManager_ErrorCode_ERR_BACKGROUND_PROCESS_MANAGER_SUCCESS => Ok(()),
        sys::BackgroundProcessManager_ErrorCode_ERR_BACKGROUND_PROCESS_MANAGER_INVALID_PARAM => {
            Err(BackgroundProcessManagerError::InvalidParam)
        }
        sys::BackgroundProcessManager_ErrorCode_ERR_BACKGROUND_PROCESS_MANAGER_REMOTE_ERROR => {
            Err(BackgroundProcessManagerError::RemoteError)
        }
        other => Err(BackgroundProcessManagerError::Unknown(other)),
    }
}
