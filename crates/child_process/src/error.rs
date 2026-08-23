use ohos_child_process_sys as sys;
use std::fmt;

/// Result alias for native child process operations.
pub type Result<T> = std::result::Result<T, ChildProcessError>;

/// An error returned by a native child process operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ChildProcessError {
    /// A native call reported this `Ability_NativeChildProcess_ErrCode`.
    Native(u32),
    /// The supplied Rust string contains an interior NUL byte and cannot be
    /// passed to the C API.
    InteriorNul,
    /// The native call to create a child process configs object returned a null
    /// pointer, which the C API uses to report an internal or allocation error.
    ConfigsCreationFailed,
}

impl ChildProcessError {
    /// The raw error code (`Ability_NativeChildProcess_ErrCode`), for errors
    /// that carry one.
    pub fn code(&self) -> Option<u32> {
        match self {
            ChildProcessError::Native(code) => Some(*code),
            _ => None,
        }
    }
}

impl fmt::Display for ChildProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChildProcessError::Native(code) => {
                write!(f, "child process error {code} ({})", describe(*code))
            }
            ChildProcessError::InteriorNul => {
                write!(f, "string contains an interior NUL byte")
            }
            ChildProcessError::ConfigsCreationFailed => {
                write!(f, "failed to create a child process configs object")
            }
        }
    }
}

impl std::error::Error for ChildProcessError {}

/// Map a raw `Ability_NativeChildProcess_ErrCode` to a short, stable
/// description.
///
/// The constants are matched through qualified `sys::` paths so that an arm
/// whose constant is absent under the current feature set is a compile error
/// instead of a catch-all binding pattern.
pub fn describe(code: u32) -> &'static str {
    match code {
        sys::Ability_NativeChildProcess_ErrCode_NCP_NO_ERROR => "success",
        sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_INVALID_PARAM => "invalid parameter",
        sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_NOT_SUPPORTED => {
            "creating a native child process is not supported"
        }
        sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_INTERNAL => "internal error",
        sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_BUSY => {
            "another native child process is still starting up"
        }
        sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_TIMEOUT => {
            "starting the native child process timed out"
        }
        sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_SERVICE_ERROR => "server error",
        sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_MULTI_PROCESS_DISABLED => {
            "the multi-process mode is disabled"
        }
        sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_ALREADY_IN_CHILD => {
            "a process cannot be created in a child process"
        }
        sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_MAX_CHILD_PROCESSES_REACHED => {
            "the maximum number of native child processes is reached"
        }
        sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_LIB_LOADING_FAILED => {
            "the child process failed to load the dynamic library"
        }
        sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_CONNECTION_FAILED => {
            "the child process failed to return a valid IPC object"
        }
        #[cfg(feature = "api-20")]
        sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_CALLBACK_NOT_EXIST => {
            "the callback is not registered"
        }
        #[cfg(feature = "api-22")]
        sys::Ability_NativeChildProcess_ErrCode_NCP_ERR_INVALID_PID => {
            "the pid is not a terminable child process of this process"
        }
        _ => "unknown error",
    }
}

/// Turn a raw `Ability_NativeChildProcess_ErrCode` into `Result<()>`.
#[cfg(feature = "api-13")]
pub(crate) fn check(code: sys::Ability_NativeChildProcess_ErrCode) -> Result<()> {
    if code == sys::Ability_NativeChildProcess_ErrCode_NCP_NO_ERROR {
        Ok(())
    } else {
        Err(ChildProcessError::Native(code))
    }
}
