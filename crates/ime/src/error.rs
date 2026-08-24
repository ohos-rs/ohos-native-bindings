use std::fmt;

use ohos_input_method_sys::InputMethod_ErrorCode_IME_ERR_DETACHED;

/// HarmonyOS reports this when a previously attached editor lost ownership to
/// another input client or became invalid across an application lifecycle
/// transition. The NDK header does not currently expose a named constant.
const IME_ERR_CLIENT_NOT_EDITABLE: u32 = 12_800_016;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImeError {
    operation: &'static str,
    code: u32,
}

impl ImeError {
    pub(crate) const fn new(operation: &'static str, code: u32) -> Self {
        Self { operation, code }
    }

    pub const fn operation(self) -> &'static str {
        self.operation
    }

    pub const fn code(self) -> u32 {
        self.code
    }

    pub(crate) const fn is_stale_session(self) -> bool {
        self.code == InputMethod_ErrorCode_IME_ERR_DETACHED
            || self.code == IME_ERR_CLIENT_NOT_EDITABLE
    }
}

impl fmt::Display for ImeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "HarmonyOS IME operation {} failed with code {}",
            self.operation, self.code
        )
    }
}

impl std::error::Error for ImeError {}

pub type ImeResult<T> = Result<T, ImeError>;
