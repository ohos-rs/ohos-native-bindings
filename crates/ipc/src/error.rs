use ohos_ipc_sys as sys;
use std::fmt;
use std::os::raw::c_int;

/// Result alias for IPC operations.
pub type Result<T> = std::result::Result<T, IpcError>;

/// An error returned by an IPC operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcError {
    /// A native call reported this `OH_IPC_ErrorCode`. Codes in the range
    /// \[1909001, 1909999\] are defined by the stub that handled the request.
    Native(i32),
    /// A native call reported failure without an error code, by returning a
    /// null pointer or `-1`.
    Failed(&'static str),
    /// A Rust value could not be passed to the native API.
    InvalidArgument(&'static str),
}

impl IpcError {
    /// Build the error a stub request handler returns to its caller, from a
    /// custom code in the user range \[1909001, 1909999\].
    ///
    /// A code outside that range is replaced by the native
    /// invalid-user-error-code error, which is what the runtime would report
    /// for it anyway.
    pub fn user(code: u32) -> Self {
        let min = sys::OH_IPC_ErrorCode_OH_IPC_USER_ERROR_CODE_MIN + 1;
        let max = sys::OH_IPC_ErrorCode_OH_IPC_USER_ERROR_CODE_MAX;
        if (min..=max).contains(&code) {
            IpcError::Native(code as i32)
        } else {
            IpcError::Native(sys::OH_IPC_ErrorCode_OH_IPC_INVALID_USER_ERROR_CODE as i32)
        }
    }

    /// The raw IPC error code (`OH_IPC_ErrorCode`), for errors that carry one.
    pub fn code(&self) -> Option<i32> {
        match self {
            IpcError::Native(code) => Some(*code),
            IpcError::Failed(_) | IpcError::InvalidArgument(_) => None,
        }
    }

    /// Whether the error says the remote stub object is dead.
    pub fn is_dead_remote_object(&self) -> bool {
        self.code() == Some(sys::OH_IPC_ErrorCode_OH_IPC_DEAD_REMOTE_OBJECT as i32)
    }
}

impl fmt::Display for IpcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IpcError::Native(code) => write!(f, "IPC error {code} ({})", describe(*code)),
            IpcError::Failed(what) => write!(f, "IPC operation failed: {what}"),
            IpcError::InvalidArgument(what) => write!(f, "invalid argument: {what}"),
        }
    }
}

impl std::error::Error for IpcError {}

/// Map a raw `OH_IPC_ErrorCode` to a short, stable description.
///
/// The constants are matched through qualified `sys::` paths so that an arm
/// whose constant is absent under the current feature set is a compile error
/// instead of a catch-all binding pattern.
pub fn describe(code: i32) -> &'static str {
    match code as u32 {
        sys::OH_IPC_ErrorCode_OH_IPC_SUCCESS => "success",
        // `OH_IPC_ERROR_CODE_BASE` shares this value and is deliberately not
        // matched: a duplicate arm would be unreachable.
        sys::OH_IPC_ErrorCode_OH_IPC_CHECK_PARAM_ERROR => "invalid parameters",
        sys::OH_IPC_ErrorCode_OH_IPC_PARCEL_WRITE_ERROR => "failed to write to the parcel",
        sys::OH_IPC_ErrorCode_OH_IPC_PARCEL_READ_ERROR => "failed to read from the parcel",
        sys::OH_IPC_ErrorCode_OH_IPC_MEM_ALLOCATOR_ERROR => "failed to allocate memory",
        sys::OH_IPC_ErrorCode_OH_IPC_CODE_OUT_OF_RANGE => "command word out of range",
        sys::OH_IPC_ErrorCode_OH_IPC_DEAD_REMOTE_OBJECT => "the remote object is dead",
        sys::OH_IPC_ErrorCode_OH_IPC_INVALID_USER_ERROR_CODE => "custom error code out of range",
        sys::OH_IPC_ErrorCode_OH_IPC_INNER_ERROR => "internal error",
        sys::OH_IPC_ErrorCode_OH_IPC_USER_ERROR_CODE_MIN
            ..=sys::OH_IPC_ErrorCode_OH_IPC_USER_ERROR_CODE_MAX => "custom error",
        _ => "unknown error",
    }
}

/// Turn a raw return code into `Result<()>`.
pub(crate) fn check(code: c_int) -> Result<()> {
    if code == sys::OH_IPC_ErrorCode_OH_IPC_SUCCESS as c_int {
        Ok(())
    } else {
        Err(IpcError::Native(code))
    }
}
