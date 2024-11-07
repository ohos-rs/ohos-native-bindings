use enum_macro::EnumFrom;
#[cfg(feature = "napi")]
use napi_ohos::{Error, Result};
use ohos_arkui_sys::*;

#[derive(Debug, EnumFrom)]
#[enum_from_config(ArkUI_ErrorCode, "ArkUI_ErrorCode_ARKUI_ERROR_CODE_")]
pub enum ArkUIErrorCode {
    ParamInvalid,
    AttributeOrEventNotSupported,
    ArkTSNodeNotSupported,
    AdapterNotBound,
    AdapterExist,
    ChildNodeExist,
    NodeEventParamIndexOutOfRange,
    NodeEventParamInvalid,
    NodeIndexInvalid,
    BufferSizeError,
    NonScrollableContainer,
    BufferSizeNotEnough,
}

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

impl AsRef<str> for ArkUIErrorCode {
    fn as_ref(&self) -> &str {
        match self {
            ArkUIErrorCode::AdapterExist => "AdapterExist",
            ArkUIErrorCode::AdapterNotBound => "AdapterNotBound",
            ArkUIErrorCode::ArkTSNodeNotSupported => "ArkTSNodeNotSupported",
            ArkUIErrorCode::AttributeOrEventNotSupported => "AttributeOrEventNotSupported",
            ArkUIErrorCode::BufferSizeError => "BufferSizeError",
            ArkUIErrorCode::BufferSizeNotEnough => "BufferSizeNotEnough",
            ArkUIErrorCode::ChildNodeExist => "ChildNodeExist",
            ArkUIErrorCode::NonScrollableContainer => "NonScrollableContainer",
            ArkUIErrorCode::NodeEventParamIndexOutOfRange => "NodeEventParamIndexOutOfRange",
            ArkUIErrorCode::NodeEventParamInvalid => "NodeEventParamInvalid",
            ArkUIErrorCode::NodeIndexInvalid => "NodeIndexInvalid",
            ArkUIErrorCode::ParamInvalid => "ParamInvalid",
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
      ohos_arkui_sys::ArkUI_ErrorCode_ARKUI_ERROR_CODE_NO_ERROR => Ok(()),
      _ => Err($crate::ArkUIError::new(c.into(), "".to_owned())),
    }
  }};

  ($code:expr, $($msg:tt)*) => {{
    let c = $code as u32;
    match c {
      ohos_arkui_sys::ArkUI_ErrorCode_ARKUI_ERROR_CODE_NO_ERROR => Ok(()),
      _ => Err($crate::ArkUIError::new(c.into(), format!($($msg)*))),
    }
  }};
}
