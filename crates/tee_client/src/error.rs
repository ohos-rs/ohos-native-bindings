use ohos_tee_client_sys as sys;
use std::fmt;

/// Result alias for TEE client operations.
pub type Result<T> = std::result::Result<T, TeeError>;

/// The component that produced an error code.
///
/// The TEE Client API reports, alongside the error code, which layer detected
/// the failure. It is only meaningful for the calls that can reach the secure
/// world: opening a session and invoking a command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReturnOrigin {
    /// The client API itself rejected the call.
    Api,
    /// The communication channel between the REE and the TEE failed.
    Communication,
    /// The TEE detected the failure.
    Tee,
    /// The trusted application returned the error.
    TrustedApp,
}

impl ReturnOrigin {
    /// Map a raw `TEEC_ReturnCodeOrigin` value, or `None` if it is not one of
    /// the four defined origins.
    #[allow(non_upper_case_globals)] // the native constants are mixed case
    fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            sys::TEEC_ReturnCodeOrigin_TEEC_ORIGIN_API => Some(ReturnOrigin::Api),
            sys::TEEC_ReturnCodeOrigin_TEEC_ORIGIN_COMMS => Some(ReturnOrigin::Communication),
            sys::TEEC_ReturnCodeOrigin_TEEC_ORIGIN_TEE => Some(ReturnOrigin::Tee),
            sys::TEEC_ReturnCodeOrigin_TEEC_ORIGIN_TRUSTED_APP => Some(ReturnOrigin::TrustedApp),
            _ => None,
        }
    }

    /// A short, stable description of the origin.
    pub fn as_str(self) -> &'static str {
        match self {
            ReturnOrigin::Api => "client api",
            ReturnOrigin::Communication => "ree/tee communication",
            ReturnOrigin::Tee => "tee",
            ReturnOrigin::TrustedApp => "trusted application",
        }
    }
}

impl fmt::Display for ReturnOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A recognised TEE client error code.
///
/// Every variant corresponds to one `TEEC_ReturnCode` constant. An error whose
/// code is not one of them keeps the raw value and reports no kind — see
/// [`TeeError::kind`] and [`TeeError::code`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    /// The command is not supported by the trusted application.
    InvalidCommand,
    /// The trusted application does not exist.
    ServiceNotExist,
    /// The session between the client and the trusted application does not exist.
    SessionNotExist,
    /// The number of connections to the trusted application reached its limit.
    SessionMaximum,
    /// The trusted application to register already exists.
    RegisterExistService,
    /// Secure OS framework error.
    TargetDeadFatal,
    /// Failed to read the file.
    ReadData,
    /// Failed to write the file.
    WriteData,
    /// Failed to truncate the file.
    TruncateObject,
    /// Failed to seek data.
    SeekData,
    /// File synchronisation error.
    FsyncData,
    /// Failed to rename the file.
    RenameObject,
    /// The trusted application failed to load while opening a session.
    TrustedAppLoadError,
    /// Non-specific error, also reported when the trusted application failed to
    /// initialise.
    Generic,
    /// Permission verification failed.
    AccessDenied,
    /// The operation was cancelled.
    Cancel,
    /// Concurrent access caused a permission conflict.
    AccessConflict,
    /// More data was passed than the trusted application can parse.
    ExcessData,
    /// The trusted application failed to parse the parameters.
    BadFormat,
    /// An input parameter was null or invalid.
    BadParameters,
    /// The operation is not valid in the current state.
    BadState,
    /// The requested data was not found.
    ItemNotFound,
    /// The requested operation is not implemented.
    NotImplemented,
    /// The requested operation is valid but unsupported by this implementation.
    NotSupported,
    /// Expected data for the requested operation was not found.
    NoData,
    /// The available system resources are insufficient.
    OutOfMemory,
    /// The system is busy; some resources are used exclusively.
    Busy,
    /// Communication between the client application and the trusted application failed.
    Communication,
    /// A security fault was detected in the TEE.
    Security,
    /// The supplied buffer is too short for the generated output.
    ShortBuffer,
    /// MAC value check error.
    MacInvalid,
    /// The trusted application crashed.
    TargetDead,
    /// Common, unclassified failure.
    Fail,
}

impl ErrorKind {
    /// Map a raw `TEEC_Result` value, or `None` if it is not a defined error
    /// code (including success).
    #[allow(non_upper_case_globals)] // the native constants are mixed case
    fn from_raw(raw: u32) -> Option<Self> {
        match raw {
            sys::TEEC_ReturnCode_TEEC_ERROR_INVALID_CMD => Some(ErrorKind::InvalidCommand),
            sys::TEEC_ReturnCode_TEEC_ERROR_SERVICE_NOT_EXIST => Some(ErrorKind::ServiceNotExist),
            sys::TEEC_ReturnCode_TEEC_ERROR_SESSION_NOT_EXIST => Some(ErrorKind::SessionNotExist),
            sys::TEEC_ReturnCode_TEEC_ERROR_SESSION_MAXIMUM => Some(ErrorKind::SessionMaximum),
            sys::TEEC_ReturnCode_TEEC_ERROR_REGISTER_EXIST_SERVICE => {
                Some(ErrorKind::RegisterExistService)
            }
            sys::TEEC_ReturnCode_TEEC_ERROR_TAGET_DEAD_FATAL => Some(ErrorKind::TargetDeadFatal),
            sys::TEEC_ReturnCode_TEEC_ERROR_READ_DATA => Some(ErrorKind::ReadData),
            sys::TEEC_ReturnCode_TEEC_ERROR_WRITE_DATA => Some(ErrorKind::WriteData),
            sys::TEEC_ReturnCode_TEEC_ERROR_TRUNCATE_OBJECT => Some(ErrorKind::TruncateObject),
            sys::TEEC_ReturnCode_TEEC_ERROR_SEEK_DATA => Some(ErrorKind::SeekData),
            sys::TEEC_ReturnCode_TEEC_ERROR_FSYNC_DATA => Some(ErrorKind::FsyncData),
            sys::TEEC_ReturnCode_TEEC_ERROR_RENAME_OBJECT => Some(ErrorKind::RenameObject),
            sys::TEEC_ReturnCode_TEEC_ERROR_TRUSTED_APP_LOAD_ERROR => {
                Some(ErrorKind::TrustedAppLoadError)
            }
            sys::TEEC_ReturnCode_TEEC_ERROR_GENERIC => Some(ErrorKind::Generic),
            sys::TEEC_ReturnCode_TEEC_ERROR_ACCESS_DENIED => Some(ErrorKind::AccessDenied),
            sys::TEEC_ReturnCode_TEEC_ERROR_CANCEL => Some(ErrorKind::Cancel),
            sys::TEEC_ReturnCode_TEEC_ERROR_ACCESS_CONFLICT => Some(ErrorKind::AccessConflict),
            sys::TEEC_ReturnCode_TEEC_ERROR_EXCESS_DATA => Some(ErrorKind::ExcessData),
            sys::TEEC_ReturnCode_TEEC_ERROR_BAD_FORMAT => Some(ErrorKind::BadFormat),
            sys::TEEC_ReturnCode_TEEC_ERROR_BAD_PARAMETERS => Some(ErrorKind::BadParameters),
            sys::TEEC_ReturnCode_TEEC_ERROR_BAD_STATE => Some(ErrorKind::BadState),
            sys::TEEC_ReturnCode_TEEC_ERROR_ITEM_NOT_FOUND => Some(ErrorKind::ItemNotFound),
            sys::TEEC_ReturnCode_TEEC_ERROR_NOT_IMPLEMENTED => Some(ErrorKind::NotImplemented),
            sys::TEEC_ReturnCode_TEEC_ERROR_NOT_SUPPORTED => Some(ErrorKind::NotSupported),
            sys::TEEC_ReturnCode_TEEC_ERROR_NO_DATA => Some(ErrorKind::NoData),
            sys::TEEC_ReturnCode_TEEC_ERROR_OUT_OF_MEMORY => Some(ErrorKind::OutOfMemory),
            sys::TEEC_ReturnCode_TEEC_ERROR_BUSY => Some(ErrorKind::Busy),
            sys::TEEC_ReturnCode_TEEC_ERROR_COMMUNICATION => Some(ErrorKind::Communication),
            sys::TEEC_ReturnCode_TEEC_ERROR_SECURITY => Some(ErrorKind::Security),
            sys::TEEC_ReturnCode_TEEC_ERROR_SHORT_BUFFER => Some(ErrorKind::ShortBuffer),
            sys::TEEC_ReturnCode_TEEC_ERROR_MAC_INVALID => Some(ErrorKind::MacInvalid),
            sys::TEEC_ReturnCode_TEEC_ERROR_TARGET_DEAD => Some(ErrorKind::TargetDead),
            sys::TEEC_ReturnCode_TEEC_FAIL => Some(ErrorKind::Fail),
            _ => None,
        }
    }

    /// A short, stable description of the error.
    pub fn as_str(self) -> &'static str {
        match self {
            ErrorKind::InvalidCommand => "invalid command",
            ErrorKind::ServiceNotExist => "trusted application does not exist",
            ErrorKind::SessionNotExist => "session does not exist",
            ErrorKind::SessionMaximum => "session limit reached",
            ErrorKind::RegisterExistService => "service already registered",
            ErrorKind::TargetDeadFatal => "secure os framework error",
            ErrorKind::ReadData => "failed to read file",
            ErrorKind::WriteData => "failed to write file",
            ErrorKind::TruncateObject => "failed to truncate file",
            ErrorKind::SeekData => "failed to seek data",
            ErrorKind::FsyncData => "file synchronisation error",
            ErrorKind::RenameObject => "failed to rename file",
            ErrorKind::TrustedAppLoadError => "failed to load trusted application",
            ErrorKind::Generic => "generic error",
            ErrorKind::AccessDenied => "access denied",
            ErrorKind::Cancel => "operation cancelled",
            ErrorKind::AccessConflict => "access conflict",
            ErrorKind::ExcessData => "excess data",
            ErrorKind::BadFormat => "bad format",
            ErrorKind::BadParameters => "bad parameters",
            ErrorKind::BadState => "bad state",
            ErrorKind::ItemNotFound => "item not found",
            ErrorKind::NotImplemented => "not implemented",
            ErrorKind::NotSupported => "not supported",
            ErrorKind::NoData => "no data",
            ErrorKind::OutOfMemory => "out of memory",
            ErrorKind::Busy => "busy",
            ErrorKind::Communication => "communication failed",
            ErrorKind::Security => "security fault",
            ErrorKind::ShortBuffer => "short buffer",
            ErrorKind::MacInvalid => "mac invalid",
            ErrorKind::TargetDead => "trusted application dead",
            ErrorKind::Fail => "failed",
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// An error returned by a TEE client operation.
///
/// Carries the classified [`ErrorKind`], the raw `TEEC_Result` value it came
/// from, and — for the calls that report one — the [`ReturnOrigin`] that tells
/// which layer detected the failure. Failures detected by this crate before
/// reaching the native API carry a message instead.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeeError {
    code: u32,
    kind: Option<ErrorKind>,
    origin: Option<ReturnOrigin>,
    message: Option<String>,
}

impl TeeError {
    pub(crate) fn from_code(code: u32, origin: Option<u32>) -> Self {
        TeeError {
            code,
            kind: ErrorKind::from_raw(code),
            origin: origin.and_then(ReturnOrigin::from_raw),
            message: None,
        }
    }

    /// A failure detected before reaching the native API, reported with the
    /// bad-parameters code.
    pub(crate) fn invalid_argument(message: impl Into<String>) -> Self {
        TeeError {
            code: sys::TEEC_ReturnCode_TEEC_ERROR_BAD_PARAMETERS,
            kind: Some(ErrorKind::BadParameters),
            origin: Some(ReturnOrigin::Api),
            message: Some(message.into()),
        }
    }

    /// The classified error, or `None` for a code this crate does not know.
    pub fn kind(&self) -> Option<ErrorKind> {
        self.kind
    }

    /// The layer that detected the failure, when the call reports one.
    pub fn origin(&self) -> Option<ReturnOrigin> {
        self.origin
    }

    /// The raw `TEEC_Result` value, for diagnostics and for codes outside
    /// [`ErrorKind`].
    pub fn code(&self) -> u32 {
        self.code
    }

    /// Extra context for a failure detected by this crate.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

impl fmt::Display for TeeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = self.kind.map_or("unknown error", ErrorKind::as_str);
        write!(f, "TEE client error 0x{:08X} ({description})", self.code)?;
        if let Some(origin) = self.origin {
            write!(f, " from {origin}")?;
        }
        if let Some(message) = &self.message {
            write!(f, ": {message}")?;
        }
        Ok(())
    }
}

impl std::error::Error for TeeError {}

/// Turn a `TEEC_Result` into `Result<()>`, attaching the return origin the call
/// reported.
pub(crate) fn check(code: u32, origin: Option<u32>) -> Result<()> {
    if code == sys::TEEC_ReturnCode_TEEC_SUCCESS {
        Ok(())
    } else {
        Err(TeeError::from_code(code, origin))
    }
}
