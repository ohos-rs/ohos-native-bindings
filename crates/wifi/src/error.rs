use ohos_wifi_sys as sys;
use std::fmt;

/// Result alias for wifi operations.
pub type Result<T> = std::result::Result<T, WifiError>;

/// An error returned by a wifi operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WifiError {
    /// The application lacks a permission the call requires, such as
    /// `ohos.permission.GET_WIFI_INFO`.
    PermissionDenied,
    /// A parameter was rejected: a null pointer was passed on, or a value lies
    /// outside the range the native API defines.
    InvalidParam,
    /// The device does not provide the wifi capability the call needs.
    NotSupported,
    /// The request reached the wifi service but its internal execution failed.
    OperationFailed,
    /// The wifi station mode is disabled.
    #[cfg(feature = "api-21")]
    StaDisabled,
    /// The call succeeded but the MAC address it reported is not in the
    /// `AA:BB:CC:DD:EE:FF` form the native API documents.
    #[cfg(feature = "api-21")]
    MalformedMacAddress,
    /// A `Wifi_ResultCode` this crate does not know about.
    Unknown(u32),
}

impl WifiError {
    /// The raw `Wifi_ResultCode` behind this error, for errors that carry one.
    pub fn code(&self) -> Option<u32> {
        match self {
            WifiError::PermissionDenied => Some(sys::Wifi_ResultCode_WIFI_PERMISSION_DENIED),
            WifiError::InvalidParam => Some(sys::Wifi_ResultCode_WIFI_INVALID_PARAM),
            WifiError::NotSupported => Some(sys::Wifi_ResultCode_WIFI_NOT_SUPPORTED),
            WifiError::OperationFailed => Some(sys::Wifi_ResultCode_WIFI_OPERATION_FAILED),
            #[cfg(feature = "api-21")]
            WifiError::StaDisabled => Some(sys::Wifi_ResultCode_WIFI_STA_DISABLED),
            #[cfg(feature = "api-21")]
            WifiError::MalformedMacAddress => None,
            WifiError::Unknown(code) => Some(*code),
        }
    }
}

impl fmt::Display for WifiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.code() {
            Some(code) => write!(f, "wifi error {code} ({})", describe(code)),
            None => write!(f, "wifi reported a malformed MAC address"),
        }
    }
}

impl std::error::Error for WifiError {}

/// Map a raw `Wifi_ResultCode` to a short, stable description.
///
/// The constants are matched through qualified `sys::` paths so that an arm
/// whose constant is absent under the current feature set is a compile error
/// instead of a catch-all binding pattern.
pub fn describe(code: u32) -> &'static str {
    match code {
        sys::Wifi_ResultCode_WIFI_SUCCESS => "success",
        sys::Wifi_ResultCode_WIFI_PERMISSION_DENIED => "permission denied",
        sys::Wifi_ResultCode_WIFI_INVALID_PARAM => "invalid parameter",
        sys::Wifi_ResultCode_WIFI_NOT_SUPPORTED => "capability not supported",
        sys::Wifi_ResultCode_WIFI_OPERATION_FAILED => "operation failed",
        #[cfg(feature = "api-21")]
        sys::Wifi_ResultCode_WIFI_STA_DISABLED => "wifi station mode disabled",
        _ => "unknown error",
    }
}

/// Turn a raw `Wifi_ResultCode` into `Result<()>`.
pub(crate) fn check(code: sys::Wifi_ResultCode) -> Result<()> {
    match code {
        sys::Wifi_ResultCode_WIFI_SUCCESS => Ok(()),
        sys::Wifi_ResultCode_WIFI_PERMISSION_DENIED => Err(WifiError::PermissionDenied),
        sys::Wifi_ResultCode_WIFI_INVALID_PARAM => Err(WifiError::InvalidParam),
        sys::Wifi_ResultCode_WIFI_NOT_SUPPORTED => Err(WifiError::NotSupported),
        sys::Wifi_ResultCode_WIFI_OPERATION_FAILED => Err(WifiError::OperationFailed),
        #[cfg(feature = "api-21")]
        sys::Wifi_ResultCode_WIFI_STA_DISABLED => Err(WifiError::StaDisabled),
        other => Err(WifiError::Unknown(other)),
    }
}
