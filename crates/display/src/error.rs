use std::fmt;

#[cfg(feature = "api-14")]
use ohos_display_sys::{
    NativeDisplayManager_ErrorCode, NativeDisplayManager_ErrorCode_DISPLAY_MANAGER_OK,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisplayError {
    /// Native display manager API returned a non-zero error code.
    InternalError(u32),
    /// Native display manager returned a null pointer.
    UnexpectedNull,
}

impl DisplayError {
    pub const fn code(self) -> Option<u32> {
        match self {
            Self::InternalError(code) => Some(code),
            Self::UnexpectedNull => None,
        }
    }
}

impl fmt::Display for DisplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InternalError(code) => write!(f, "display manager error code {code}"),
            Self::UnexpectedNull => f.write_str("display manager returned a null pointer"),
        }
    }
}

impl std::error::Error for DisplayError {}

pub type Result<T> = std::result::Result<T, DisplayError>;

#[cfg(feature = "api-14")]
#[allow(non_upper_case_globals)]
pub(crate) fn check_status(code: NativeDisplayManager_ErrorCode) -> Result<()> {
    if code == NativeDisplayManager_ErrorCode_DISPLAY_MANAGER_OK {
        Ok(())
    } else {
        Err(DisplayError::InternalError(code))
    }
}
