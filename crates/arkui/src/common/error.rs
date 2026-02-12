#[cfg(feature = "napi")]
use napi_ohos::{Error, Result};
use ohos_arkui_input_binding::ArkUIErrorCode;

#[cfg(not(feature = "napi"))]
pub struct ArkUIError {
    pub code: ArkUIErrorCode,
    pub message: Option<String>,
}

#[cfg(not(feature = "napi"))]
impl ArkUIError {
    pub fn new<T: AsRef<str>>(code: ArkUIErrorCode, message: T) -> Self {
        Self {
            code,
            message: Some(message.as_ref().to_string()),
        }
    }

    pub fn from_status(code: ArkUIErrorCode) -> Self {
        Self {
            code,
            message: None,
        }
    }

    pub fn from_reason<T: AsRef<str>>(message: T) -> Self {
        Self {
            code: ArkUIErrorCode::ParamInvalid,
            message: Some(message.as_ref().to_string()),
        }
    }
}

#[cfg(not(feature = "napi"))]
/// This type is used for ArkUI result.
pub type ArkUIResult<T> = Result<T, ArkUIError>;

#[cfg(feature = "napi")]
pub type ArkUIResult<T> = Result<T, ArkUIErrorCode>;

#[cfg(feature = "napi")]
pub type ArkUIError = Error<ArkUIErrorCode>;

#[doc(hidden)]
#[macro_export]
macro_rules! check_arkui_status {
  ($code:expr) => {{
    let c = $code as u32;
    match c {
      ohos_arkui_input_binding::sys::ArkUI_ErrorCode_ARKUI_ERROR_CODE_NO_ERROR => Ok(()),
      _ => Err($crate::ArkUIError::new(c.into(), "".to_owned())),
    }
  }};

  ($code:expr, $($msg:tt)*) => {{
    let c = $code as u32;
    match c {
        ohos_arkui_input_binding::sys::ArkUI_ErrorCode_ARKUI_ERROR_CODE_NO_ERROR => Ok(()),
      _ => Err($crate::ArkUIError::new(c.into(), format!($($msg)*))),
    }
  }};
}
