use ohos_cert_manager_sys as sys;
use std::fmt;

/// Result alias for certificate manager operations.
pub type Result<T> = std::result::Result<T, CertManagerError>;

/// An error returned by a certificate manager operation.
///
/// Carries the raw error code (`OH_CM_ErrorCode`) and, for failures detected by
/// this crate before reaching the service, a message describing what was wrong
/// with the input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertManagerError {
    code: i32,
    message: Option<String>,
}

impl CertManagerError {
    pub(crate) fn from_code(code: i32) -> Self {
        CertManagerError {
            code,
            message: None,
        }
    }

    /// A failure detected before reaching the certificate manager, reported with
    /// the native parameter-validation code.
    pub(crate) fn invalid_argument(message: impl Into<String>) -> Self {
        CertManagerError {
            code: sys::OH_CM_ErrorCode_OH_CM_PARAMETER_VALIDATION_FAILED as i32,
            message: Some(message.into()),
        }
    }

    /// The raw error code (`OH_CM_ErrorCode`).
    pub fn code(&self) -> i32 {
        self.code
    }

    /// The message attached to a locally detected input error, if any.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Whether the certificate manager reported that the certificate does not
    /// exist.
    pub fn is_not_found(&self) -> bool {
        self.code == sys::OH_CM_ErrorCode_OH_CM_NOT_FOUND as i32
    }
}

impl fmt::Display for CertManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(msg) => write!(
                f,
                "cert manager error {} ({}): {msg}",
                self.code,
                describe(self.code)
            ),
            None => write!(
                f,
                "cert manager error {} ({})",
                self.code,
                describe(self.code)
            ),
        }
    }
}

impl std::error::Error for CertManagerError {}

/// Map a raw certificate manager error code to a short, stable description.
///
/// The mapping follows the `OH_CM_ErrorCode` enum in `cm_native_type.h`.
pub fn describe(code: i32) -> &'static str {
    // Patterns use fully qualified `sys::` paths on purpose: a bare constant
    // name that is not in scope silently degrades into a catch-all binding.
    match code as u32 {
        sys::OH_CM_ErrorCode_OH_CM_SUCCESS => "success",
        sys::OH_CM_ErrorCode_OH_CM_HAS_NO_PERMISSION => "permission denied",
        sys::OH_CM_ErrorCode_OH_CM_CAPABILITY_NOT_SUPPORTED => "capability not supported",
        sys::OH_CM_ErrorCode_OH_CM_INNER_FAILURE => "internal error",
        sys::OH_CM_ErrorCode_OH_CM_NOT_FOUND => "certificate does not exist",
        sys::OH_CM_ErrorCode_OH_CM_INVALID_CERT_FORMAT => "invalid keystore format or password",
        sys::OH_CM_ErrorCode_OH_CM_MAX_CERT_COUNT_REACHED => "maximum certificate count reached",
        sys::OH_CM_ErrorCode_OH_CM_NO_AUTHORIZATION => "application not authorized by the user",
        sys::OH_CM_ErrorCode_OH_CM_DEVICE_ENTER_ADVSECMODE => "device in advanced security mode",
        sys::OH_CM_ErrorCode_OH_CM_STORE_PATH_NOT_SUPPORTED => {
            "certificate store path not supported"
        }
        sys::OH_CM_ErrorCode_OH_CM_ACCESS_UKEY_SERVICE_FAILED => "access to usb key service failed",
        sys::OH_CM_ErrorCode_OH_CM_PARAMETER_VALIDATION_FAILED => "parameter validation failed",
        _ => "unknown error",
    }
}

/// Turn a native return code into `Result<()>`.
pub(crate) fn check(code: i32) -> Result<()> {
    if code == sys::OH_CM_ErrorCode_OH_CM_SUCCESS as i32 {
        Ok(())
    } else {
        Err(CertManagerError::from_code(code))
    }
}
