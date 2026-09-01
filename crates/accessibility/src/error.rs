use std::{error::Error as StdError, ffi::NulError, fmt};

use ohos_accessibility_sys::{
    ArkUI_AcessbilityErrorCode_ARKUI_ACCESSIBILITY_NATIVE_RESULT_BAD_PARAMETER,
    ArkUI_AcessbilityErrorCode_ARKUI_ACCESSIBILITY_NATIVE_RESULT_FAILED,
    ArkUI_AcessbilityErrorCode_ARKUI_ACCESSIBILITY_NATIVE_RESULT_OUT_OF_MEMORY,
    ArkUI_AcessbilityErrorCode_ARKUI_ACCESSIBILITY_NATIVE_RESULT_SUCCESSFUL,
};

pub type Result<T> = std::result::Result<T, AccessibilityError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessibilityError {
    Failed,
    BadParameter,
    OutOfMemory,
    UnknownCode(i32),
    NullHandle(&'static str),
    InteriorNul,
    AlreadyRegistered,
    CallbackPanicked,
    LockPoisoned,
    Unsupported,
}

impl AccessibilityError {
    #[allow(non_upper_case_globals)]
    pub(crate) fn from_code(code: i32) -> Self {
        match code {
            ArkUI_AcessbilityErrorCode_ARKUI_ACCESSIBILITY_NATIVE_RESULT_FAILED => Self::Failed,
            ArkUI_AcessbilityErrorCode_ARKUI_ACCESSIBILITY_NATIVE_RESULT_BAD_PARAMETER => {
                Self::BadParameter
            }
            ArkUI_AcessbilityErrorCode_ARKUI_ACCESSIBILITY_NATIVE_RESULT_OUT_OF_MEMORY => {
                Self::OutOfMemory
            }
            other => Self::UnknownCode(other),
        }
    }

    pub(crate) fn into_code(self) -> i32 {
        match self {
            Self::BadParameter | Self::InteriorNul | Self::NullHandle(_) => {
                ArkUI_AcessbilityErrorCode_ARKUI_ACCESSIBILITY_NATIVE_RESULT_BAD_PARAMETER
            }
            Self::OutOfMemory => {
                ArkUI_AcessbilityErrorCode_ARKUI_ACCESSIBILITY_NATIVE_RESULT_OUT_OF_MEMORY
            }
            Self::UnknownCode(code) => code,
            Self::Failed
            | Self::AlreadyRegistered
            | Self::CallbackPanicked
            | Self::LockPoisoned
            | Self::Unsupported => {
                ArkUI_AcessbilityErrorCode_ARKUI_ACCESSIBILITY_NATIVE_RESULT_FAILED
            }
        }
    }
}

impl fmt::Display for AccessibilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Failed => f.write_str("the ArkUI accessibility operation failed"),
            Self::BadParameter => f.write_str("ArkUI rejected an accessibility parameter"),
            Self::OutOfMemory => f.write_str("ArkUI could not allocate accessibility data"),
            Self::UnknownCode(code) => write!(f, "unknown ArkUI accessibility error code {code}"),
            Self::NullHandle(name) => write!(f, "ArkUI returned a null {name} handle"),
            Self::InteriorNul => f.write_str("a string contains an interior NUL byte"),
            Self::AlreadyRegistered => {
                f.write_str("an accessibility callback is already registered for this key")
            }
            Self::CallbackPanicked => f.write_str("an accessibility callback panicked"),
            Self::LockPoisoned => f.write_str("the accessibility callback registry is poisoned"),
            Self::Unsupported => f.write_str("the accessibility operation is not supported"),
        }
    }
}

impl StdError for AccessibilityError {}

impl From<NulError> for AccessibilityError {
    fn from(_: NulError) -> Self {
        Self::InteriorNul
    }
}

pub(crate) fn check(code: i32) -> Result<()> {
    if code == ArkUI_AcessbilityErrorCode_ARKUI_ACCESSIBILITY_NATIVE_RESULT_SUCCESSFUL {
        Ok(())
    } else {
        Err(AccessibilityError::from_code(code))
    }
}
