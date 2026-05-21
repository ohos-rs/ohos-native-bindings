use std::fmt::{Display, Formatter};

use hms_opengtx_sys::*;
use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(OpenGTX_ErrorCode, "OpenGTX_ErrorCode_OPENGTX_")]
pub enum OpenGtxErrorCode {
    Success,
    InvalidParameter,
    ContextNotConfig,
    ContextNotActive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenGtxError {
    InvalidString,
    CreateContextFailed,
    UnknownErrorCode(OpenGTX_ErrorCode),
    Api(OpenGtxErrorCode),
}

impl Display for OpenGtxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidString => write!(f, "string contains an interior null byte"),
            Self::CreateContextFailed => write!(f, "failed to create OpenGTX context"),
            Self::UnknownErrorCode(code) => write!(f, "unknown OpenGTX error code {}", code),
            Self::Api(code) => write!(f, "OpenGTX API error: {:?}", code),
        }
    }
}

impl std::error::Error for OpenGtxError {}

pub type OpenGtxResult<T> = Result<T, OpenGtxError>;

pub(crate) fn check_status(status: OpenGTX_ErrorCode) -> OpenGtxResult<()> {
    let code =
        OpenGtxErrorCode::try_from_raw(status).ok_or(OpenGtxError::UnknownErrorCode(status))?;
    if code == OpenGtxErrorCode::Success {
        Ok(())
    } else {
        Err(OpenGtxError::Api(code))
    }
}
