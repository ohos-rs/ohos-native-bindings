use std::fmt::{Display, Formatter};

#[cfg(feature = "api-18")]
use ohos_native_drawing_sys::OH_Drawing_ErrorCodeReset;
use ohos_native_drawing_sys::{
    OH_Drawing_ErrorCode, OH_Drawing_ErrorCodeGet, OH_Drawing_ErrorCode_OH_DRAWING_SUCCESS,
};

use crate::DrawingErrorCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawingError {
    pub raw_code: OH_Drawing_ErrorCode,
    pub code: Option<DrawingErrorCode>,
}

impl DrawingError {
    pub fn from_last_error() -> Self {
        let raw_code = unsafe { OH_Drawing_ErrorCodeGet() };
        Self {
            raw_code,
            code: DrawingErrorCode::try_from_raw(raw_code),
        }
    }

    #[cfg(feature = "api-18")]
    pub fn reset_last_error() {
        unsafe { OH_Drawing_ErrorCodeReset() }
    }
}

impl Display for DrawingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.code {
            Some(code) => write!(f, "ohos drawing error: {:?} (raw={})", code, self.raw_code),
            None => write!(f, "ohos drawing error: raw={}", self.raw_code),
        }
    }
}

impl std::error::Error for DrawingError {}

pub type Result<T> = std::result::Result<T, DrawingError>;

pub fn check_error(raw_code: OH_Drawing_ErrorCode) -> Result<()> {
    if raw_code == OH_Drawing_ErrorCode_OH_DRAWING_SUCCESS {
        Ok(())
    } else {
        Err(DrawingError {
            raw_code,
            code: DrawingErrorCode::try_from_raw(raw_code),
        })
    }
}
