use ohos_location_sys as sys;
use std::fmt;

/// Result alias for location operations.
pub type Result<T> = std::result::Result<T, LocationError>;

/// An error returned by a location operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationError {
    /// A native call reported this `Location_ResultCode`.
    Native(u32),
    /// The location service refused to allocate a request parameter instance,
    /// typically because the address space of the application is exhausted.
    AllocationFailed,
    /// The requested reporting interval is outside the range the native API
    /// accepts (it must be at least one second and fit into a C `int`).
    InvalidInterval(u32),
}

impl LocationError {
    /// The raw location result code (`Location_ResultCode`), for errors that
    /// carry one.
    pub fn code(&self) -> Option<u32> {
        match self {
            LocationError::Native(code) => Some(*code),
            LocationError::AllocationFailed | LocationError::InvalidInterval(_) => None,
        }
    }

    /// Whether the error is the native permission-denied code, meaning the
    /// application is missing `ohos.permission.APPROXIMATELY_LOCATION`.
    pub fn is_permission_denied(&self) -> bool {
        self.code() == Some(sys::Location_ResultCode_LOCATION_PERMISSION_DENIED)
    }

    /// Whether the error is the native switch-off code, meaning the user has
    /// turned the system location switch off.
    pub fn is_switch_off(&self) -> bool {
        self.code() == Some(sys::Location_ResultCode_LOCATION_SWITCH_OFF)
    }
}

impl fmt::Display for LocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocationError::Native(code) => {
                write!(f, "location error {code} ({})", describe(*code))
            }
            LocationError::AllocationFailed => {
                write!(
                    f,
                    "failed to allocate a location request parameter instance"
                )
            }
            LocationError::InvalidInterval(interval) => {
                write!(
                    f,
                    "reporting interval {interval} is out of range (expected 1..={})",
                    i32::MAX
                )
            }
        }
    }
}

impl std::error::Error for LocationError {}

/// Map a raw `Location_ResultCode` to a short, stable description.
///
/// The constants are matched through qualified `sys::` paths so that an arm
/// whose constant is absent under the current feature set is a compile error
/// instead of a catch-all binding pattern.
pub fn describe(code: u32) -> &'static str {
    match code {
        sys::Location_ResultCode_LOCATION_SUCCESS => "success",
        sys::Location_ResultCode_LOCATION_PERMISSION_DENIED => "permission denied",
        sys::Location_ResultCode_LOCATION_INVALID_PARAM => "invalid parameter",
        sys::Location_ResultCode_LOCATION_NOT_SUPPORTED => "capability not supported",
        sys::Location_ResultCode_LOCATION_SERVICE_UNAVAILABLE => "location service unavailable",
        sys::Location_ResultCode_LOCATION_SWITCH_OFF => "location switch is off",
        _ => "unknown error",
    }
}

/// Turn a raw `Location_ResultCode` into `Result<()>`.
pub(crate) fn check(code: sys::Location_ResultCode) -> Result<()> {
    if code == sys::Location_ResultCode_LOCATION_SUCCESS {
        Ok(())
    } else {
        Err(LocationError::Native(code))
    }
}
