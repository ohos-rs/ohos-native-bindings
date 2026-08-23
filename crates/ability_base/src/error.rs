use ohos_ability_base_sys as sys;
use std::ffi::NulError;
use std::fmt;

/// Result alias for ability base operations.
pub type Result<T> = std::result::Result<T, AbilityBaseError>;

/// An error returned by an ability base operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityBaseError {
    /// A native call reported this `AbilityBase_ErrorCode`.
    Native(u32),
    /// `OH_AbilityBase_CreateWant` returned no want.
    CreateWantFailed,
    /// A string returned by the want does not fit into a buffer of this many
    /// bytes, terminating character included.
    BufferTooSmall(usize),
    /// A string returned by the want is not valid UTF-8.
    InvalidUtf8,
    /// A key or value passed in contains a NUL byte and cannot be handed to a
    /// C string API.
    InteriorNul,
}

impl AbilityBaseError {
    /// The raw ability base error code (`AbilityBase_ErrorCode`), for errors
    /// that carry one.
    pub fn code(&self) -> Option<u32> {
        match self {
            AbilityBaseError::Native(code) => Some(*code),
            AbilityBaseError::CreateWantFailed
            | AbilityBaseError::BufferTooSmall(_)
            | AbilityBaseError::InvalidUtf8
            | AbilityBaseError::InteriorNul => None,
        }
    }
}

impl fmt::Display for AbilityBaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AbilityBaseError::Native(code) => {
                write!(f, "ability base error {code} ({})", describe(*code))
            }
            AbilityBaseError::CreateWantFailed => write!(f, "failed to create want"),
            AbilityBaseError::BufferTooSmall(capacity) => {
                write!(f, "value does not fit into {capacity} bytes")
            }
            AbilityBaseError::InvalidUtf8 => write!(f, "value is not valid utf-8"),
            AbilityBaseError::InteriorNul => write!(f, "argument contains an interior nul byte"),
        }
    }
}

impl std::error::Error for AbilityBaseError {}

impl From<NulError> for AbilityBaseError {
    fn from(_: NulError) -> Self {
        AbilityBaseError::InteriorNul
    }
}

/// Map a raw `AbilityBase_ErrorCode` to a short, stable description.
///
/// The constants are matched through qualified `sys::` paths so that an arm
/// whose constant is absent under the current feature set is a compile error
/// instead of a catch-all binding pattern.
pub fn describe(code: u32) -> &'static str {
    match code {
        sys::AbilityBase_ErrorCode_ABILITY_BASE_ERROR_CODE_NO_ERROR => "no error",
        sys::AbilityBase_ErrorCode_ABILITY_BASE_ERROR_CODE_PARAM_INVALID => "invalid parameters",
        _ => "unknown error",
    }
}

/// Turn a raw `AbilityBase_ErrorCode` into `Result<()>`.
pub(crate) fn check(code: sys::AbilityBase_ErrorCode) -> Result<()> {
    if code == sys::AbilityBase_ErrorCode_ABILITY_BASE_ERROR_CODE_NO_ERROR {
        Ok(())
    } else {
        Err(AbilityBaseError::Native(code))
    }
}

/// Whether a raw code is the single failure code the want API reports.
pub(crate) fn is_param_invalid(error: &AbilityBaseError) -> bool {
    matches!(
        error,
        AbilityBaseError::Native(sys::AbilityBase_ErrorCode_ABILITY_BASE_ERROR_CODE_PARAM_INVALID)
    )
}
