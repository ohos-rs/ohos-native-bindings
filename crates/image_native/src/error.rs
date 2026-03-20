use std::fmt::{Display, Formatter};

use crate::sys;

/// Error wrapper for image-native API status codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageNativeError {
    /// Raw image-native status code.
    pub code: sys::Image_ErrorCode,
}

impl Display for ImageNativeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "image-native error code {}", self.code)
    }
}

impl std::error::Error for ImageNativeError {}

/// Result alias for image-native wrapper APIs.
pub type ImageNativeResult<T> = Result<T, ImageNativeError>;

pub(crate) fn check_status(status: sys::Image_ErrorCode) -> ImageNativeResult<()> {
    if status == sys::Image_ErrorCode_IMAGE_SUCCESS {
        Ok(())
    } else {
        Err(ImageNativeError { code: status })
    }
}
