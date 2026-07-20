use ohos_huks_sys::*;
use std::ffi::CStr;
use std::fmt;

/// Result alias for HUKS operations.
pub type Result<T> = std::result::Result<T, HuksError>;

/// An error returned by a HUKS operation.
///
/// Carries the raw HUKS error code (`OH_Huks_ErrCode`) and, when the runtime
/// provides one, the native error message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HuksError {
    code: i32,
    message: Option<String>,
}

impl HuksError {
    pub(crate) fn from_result(result: OH_Huks_Result) -> Self {
        let message = if result.errorMsg.is_null() {
            None
        } else {
            // SAFETY: when non-null, errorMsg is a NUL-terminated string owned by HUKS.
            Some(
                unsafe { CStr::from_ptr(result.errorMsg) }
                    .to_string_lossy()
                    .into_owned(),
            )
        };
        HuksError {
            code: result.errorCode,
            message,
        }
    }

    /// The raw HUKS error code (`OH_Huks_ErrCode`).
    pub fn code(&self) -> i32 {
        self.code
    }

    /// The native error message, if HUKS supplied one.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

impl fmt::Display for HuksError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(msg) => write!(
                f,
                "HUKS error {} ({}): {msg}",
                self.code,
                describe(self.code)
            ),
            None => write!(f, "HUKS error {} ({})", self.code, describe(self.code)),
        }
    }
}

impl std::error::Error for HuksError {}

/// Map a raw HUKS error code to a short, stable description.
#[allow(non_upper_case_globals)] // matching bindgen's mixed-case `OH_Huks_ErrCode_*` consts
pub fn describe(code: i32) -> &'static str {
    match code as u32 {
        OH_Huks_ErrCode_OH_HUKS_SUCCESS => "success",
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_PERMISSION_FAIL => "permission denied",
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_ILLEGAL_ARGUMENT => "illegal argument",
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_NOT_SUPPORTED_API => "unsupported api",
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_FEATURE_NOT_SUPPORTED => "feature not supported",
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_MISSING_CRYPTO_ALG_ARGUMENT => "missing crypto argument",
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_INVALID_CRYPTO_ALG_ARGUMENT => "invalid crypto argument",
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_FILE_OPERATION_FAIL => "file operation failed",
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_COMMUNICATION_FAIL => "communication failed",
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_CRYPTO_FAIL => "crypto operation failed",
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_KEY_AUTH_PERMANENTLY_INVALIDATED => {
            "key auth permanently invalidated"
        }
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_KEY_AUTH_VERIFY_FAILED => "key auth verify failed",
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_KEY_AUTH_TIME_OUT => "key auth timed out",
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_SESSION_LIMIT => "session limit reached",
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_ITEM_NOT_EXIST => "item does not exist",
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_INTERNAL_ERROR => "internal error",
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_CREDENTIAL_NOT_EXIST => "credential does not exist",
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_INSUFFICIENT_MEMORY => "insufficient memory",
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_CALL_SERVICE_FAILED => "call service failed",
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_DEVICE_PASSWORD_UNSET => "device password unset",
        #[cfg(feature = "api-20")]
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_KEY_ALREADY_EXIST => "key already exists",
        #[cfg(feature = "api-20")]
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_INVALID_ARGUMENT => "invalid argument",
        #[cfg(feature = "api-22")]
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_ITEM_EXISTS => "item exists",
        #[cfg(feature = "api-22")]
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_EXTERNAL_MODULE => "external module error",
        #[cfg(feature = "api-22")]
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_PIN_LOCKED => "pin locked",
        #[cfg(feature = "api-22")]
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_PIN_INCORRECT => "pin incorrect",
        #[cfg(feature = "api-22")]
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_PIN_NO_AUTH => "pin not authenticated",
        #[cfg(feature = "api-22")]
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_BUSY => "busy",
        #[cfg(feature = "api-22")]
        OH_Huks_ErrCode_OH_HUKS_ERR_CODE_EXCEED_LIMIT => "limit exceeded",
        _ => "unknown error",
    }
}

/// Turn an `OH_Huks_Result` into `Result<()>`, mapping any non-success code
/// to a [`HuksError`].
pub(crate) fn check(result: OH_Huks_Result) -> Result<()> {
    if result.errorCode == OH_Huks_ErrCode_OH_HUKS_SUCCESS as i32 {
        Ok(())
    } else {
        Err(HuksError::from_result(result))
    }
}
