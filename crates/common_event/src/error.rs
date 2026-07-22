use ohos_common_event_sys as sys;
use std::ffi::NulError;
use std::fmt;

/// Result alias for common event operations.
pub type Result<T> = std::result::Result<T, CommonEventError>;

/// `COMMONEVENT_ERR_SENDING_LIMIT_EXCEEDED` is declared behind `api-20`, but the
/// runtime can report it whatever the crate is compiled against, so the mapping
/// below spells it as a literal to stay complete under every feature set. This
/// assertion ties that literal back to the generated constant whenever it is in
/// scope, turning a future renumbering into a build failure instead of silent
/// drift.
#[cfg(feature = "api-20")]
const _: () = assert!(
    sys::CommonEvent_ErrCode_COMMONEVENT_ERR_SENDING_LIMIT_EXCEEDED == SENDING_LIMIT_EXCEEDED
);

/// Raw code of [`CommonEventError::SendingLimitExceeded`].
const SENDING_LIMIT_EXCEEDED: sys::CommonEvent_ErrCode = 1500003;

/// An error returned by a common event operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommonEventError {
    /// Permission verification failed.
    PermissionDenied,
    /// A parameter handed to the native layer was rejected.
    InvalidParameter,
    /// Events are being published too fast.
    SendingLimitExceeded,
    /// The application may not send system common events.
    NotSystemService,
    /// The IPC request could not be sent.
    SendingRequestFailed,
    /// The common event service is not initialized yet.
    InitUndone,
    /// The system parameters could not be read.
    ObtainSystemParams,
    /// The number of subscribers exceeds the system limit.
    SubscriberNumExceeded,
    /// The native layer could not allocate memory.
    AllocMemoryFailed,
    /// A native call reported an error code this crate does not know.
    Unknown(u32),
    /// A native call that reports only success or failure failed.
    Failed,
    /// A native handle could not be allocated.
    Alloc,
    /// A string argument contained an interior NUL byte.
    Nul,
    /// A collection was longer than the native length type can express.
    TooLong,
    /// A value did not fit the native type it is stored in.
    OutOfRange,
}

impl CommonEventError {
    /// The raw error code reported by the native layer, for errors that carry one.
    pub fn code(&self) -> Option<u32> {
        // The `CommonEvent_ErrCode_*` constants are matched through a qualified
        // path so that a name which is absent under the selected features is a
        // compile error rather than a catch-all binding.
        let code = match self {
            CommonEventError::PermissionDenied => {
                sys::CommonEvent_ErrCode_COMMONEVENT_ERR_PERMISSION_ERROR
            }
            CommonEventError::InvalidParameter => {
                sys::CommonEvent_ErrCode_COMMONEVENT_ERR_INVALID_PARAMETER
            }
            CommonEventError::SendingLimitExceeded => SENDING_LIMIT_EXCEEDED,
            CommonEventError::NotSystemService => {
                sys::CommonEvent_ErrCode_COMMONEVENT_ERR_NOT_SYSTEM_SERVICE
            }
            CommonEventError::SendingRequestFailed => {
                sys::CommonEvent_ErrCode_COMMONEVENT_ERR_SENDING_REQUEST_FAILED
            }
            CommonEventError::InitUndone => sys::CommonEvent_ErrCode_COMMONEVENT_ERR_INIT_UNDONE,
            CommonEventError::ObtainSystemParams => {
                sys::CommonEvent_ErrCode_COMMONEVENT_ERR_OBTAIN_SYSTEM_PARAMS
            }
            CommonEventError::SubscriberNumExceeded => {
                sys::CommonEvent_ErrCode_COMMONEVENT_ERR_SUBSCRIBER_NUM_EXCEEDED
            }
            CommonEventError::AllocMemoryFailed => {
                sys::CommonEvent_ErrCode_COMMONEVENT_ERR_ALLOC_MEMORY_FAILED
            }
            CommonEventError::Unknown(code) => *code,
            _ => return None,
        };
        Some(code)
    }

    /// Map a raw common event error code to its variant.
    fn from_code(code: u32) -> Self {
        // Qualified paths again, for the same reason as in `code`.
        match code {
            sys::CommonEvent_ErrCode_COMMONEVENT_ERR_PERMISSION_ERROR => {
                CommonEventError::PermissionDenied
            }
            sys::CommonEvent_ErrCode_COMMONEVENT_ERR_INVALID_PARAMETER => {
                CommonEventError::InvalidParameter
            }
            SENDING_LIMIT_EXCEEDED => CommonEventError::SendingLimitExceeded,
            sys::CommonEvent_ErrCode_COMMONEVENT_ERR_NOT_SYSTEM_SERVICE => {
                CommonEventError::NotSystemService
            }
            sys::CommonEvent_ErrCode_COMMONEVENT_ERR_SENDING_REQUEST_FAILED => {
                CommonEventError::SendingRequestFailed
            }
            sys::CommonEvent_ErrCode_COMMONEVENT_ERR_INIT_UNDONE => CommonEventError::InitUndone,
            sys::CommonEvent_ErrCode_COMMONEVENT_ERR_OBTAIN_SYSTEM_PARAMS => {
                CommonEventError::ObtainSystemParams
            }
            sys::CommonEvent_ErrCode_COMMONEVENT_ERR_SUBSCRIBER_NUM_EXCEEDED => {
                CommonEventError::SubscriberNumExceeded
            }
            sys::CommonEvent_ErrCode_COMMONEVENT_ERR_ALLOC_MEMORY_FAILED => {
                CommonEventError::AllocMemoryFailed
            }
            other => CommonEventError::Unknown(other),
        }
    }
}

impl From<NulError> for CommonEventError {
    fn from(_: NulError) -> Self {
        CommonEventError::Nul
    }
}

impl fmt::Display for CommonEventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommonEventError::Unknown(code) => write!(f, "common event error {code}"),
            CommonEventError::Failed => write!(f, "common event operation failed"),
            CommonEventError::Alloc => write!(f, "common event handle allocation failed"),
            CommonEventError::Nul => write!(f, "string argument contains an interior NUL byte"),
            CommonEventError::TooLong => write!(f, "collection too long for the native length"),
            CommonEventError::OutOfRange => write!(f, "value out of range for the native type"),
            other => {
                let code = other.code().unwrap_or_default();
                write!(f, "common event error {code} ({})", describe(code))
            }
        }
    }
}

impl std::error::Error for CommonEventError {}

/// Map a raw common event error code to a short, stable description.
pub fn describe(code: u32) -> &'static str {
    match CommonEventError::from_code(code) {
        CommonEventError::PermissionDenied => "permission verification failed",
        CommonEventError::InvalidParameter => "invalid input parameter",
        CommonEventError::SendingLimitExceeded => "common event sending frequency too high",
        CommonEventError::NotSystemService => "the application cannot send system common events",
        CommonEventError::SendingRequestFailed => "IPC request failed to send",
        CommonEventError::InitUndone => "common event service not initialized",
        CommonEventError::ObtainSystemParams => "failed to obtain system parameters",
        CommonEventError::SubscriberNumExceeded => "subscriber number exceeds the system limit",
        CommonEventError::AllocMemoryFailed => "a memory allocation error occurred",
        _ if code == 0 => "success",
        _ => "unknown error",
    }
}

/// Turn a raw common event status into `Result<()>`.
pub(crate) fn check(code: sys::CommonEvent_ErrCode) -> Result<()> {
    if code == sys::CommonEvent_ErrCode_COMMONEVENT_ERR_OK {
        Ok(())
    } else {
        Err(CommonEventError::from_code(code))
    }
}

/// Turn the success flag of a native call into `Result<()>`.
///
/// Only the ordered-event calls report success this way, and they arrived in
/// API 18.
#[cfg(feature = "api-18")]
pub(crate) fn check_bool(ok: bool) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(CommonEventError::Failed)
    }
}
