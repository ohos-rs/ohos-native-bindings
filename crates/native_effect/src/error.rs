use ohos_native_effect_sys as sys;
use std::fmt;

/// Result alias for native effect operations.
pub type Result<T> = std::result::Result<T, EffectError>;

/// An error returned by a native effect operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectError {
    /// A native call reported this `EffectErrorCode`.
    Native(u32),
    /// The filter was reported as created, but the runtime handed out a null
    /// filter handle.
    NullFilter,
    /// The effect was reported as applied, but the runtime handed out a null
    /// pixel map handle.
    NullPixelMap,
}

impl EffectError {
    /// The raw effect error code (`EffectErrorCode`), for errors that carry one.
    pub fn code(&self) -> Option<u32> {
        match self {
            EffectError::Native(code) => Some(*code),
            EffectError::NullFilter | EffectError::NullPixelMap => None,
        }
    }
}

impl fmt::Display for EffectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EffectError::Native(code) => write!(f, "effect error {code} ({})", describe(*code)),
            EffectError::NullFilter => write!(f, "the runtime returned a null filter handle"),
            EffectError::NullPixelMap => write!(f, "the runtime returned a null pixel map handle"),
        }
    }
}

impl std::error::Error for EffectError {}

/// Map a raw `EffectErrorCode` to a short, stable description.
///
/// The constants are matched through qualified `sys::` paths so that an arm
/// whose constant is absent under the current feature set is a compile error
/// instead of a catch-all binding pattern.
pub fn describe(code: u32) -> &'static str {
    match code {
        sys::EffectErrorCode_EFFECT_SUCCESS => "success",
        sys::EffectErrorCode_EFFECT_BAD_PARAMETER => "invalid parameter",
        sys::EffectErrorCode_EFFECT_UNSUPPORTED_OPERATION => "unsupported operation",
        sys::EffectErrorCode_EFFECT_UNKNOWN_ERROR => "unknown error",
        _ => "unknown error",
    }
}

/// Turn a raw `EffectErrorCode` into `Result<()>`.
pub(crate) fn check(code: sys::EffectErrorCode) -> Result<()> {
    if code == sys::EffectErrorCode_EFFECT_SUCCESS {
        Ok(())
    } else {
        Err(EffectError::Native(code))
    }
}
