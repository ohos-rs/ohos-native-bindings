use std::fmt;

use ohos_multi_modal_input_sys as sys;

/// Errors returned by OpenHarmony multimodal InputKit native APIs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputError {
    /// Native InputKit API returned a non-zero error code.
    InternalError(u32),
    /// Common error (e.g. invalid argument or native allocation failure).
    CommonError(String),
}

impl InputError {
    /// Returns the raw native error code when this error originated from InputKit.
    pub const fn code(&self) -> Option<u32> {
        match self {
            Self::InternalError(code) => Some(*code),
            Self::CommonError(_) => None,
        }
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InternalError(code) => write!(f, "InputKit API failed with error code {code}"),
            Self::CommonError(msg) => write!(f, "Common error: {msg}"),
        }
    }
}

impl std::error::Error for InputError {}

pub type Result<T> = std::result::Result<T, InputError>;

#[allow(non_upper_case_globals)]
pub(crate) fn check_status(code: sys::Input_Result) -> Result<()> {
    if code == sys::Input_Result_INPUT_SUCCESS {
        Ok(())
    } else {
        Err(InputError::InternalError(code))
    }
}
