use std::fmt::{Display, Formatter};

/// Error wrapper for image API return codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImageError {
    /// Raw image error/status code.
    pub code: i32,
}

impl Display for ImageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "image error code {}", self.code)
    }
}

impl std::error::Error for ImageError {}

/// Result alias for image wrapper APIs.
pub type ImageResult<T> = Result<T, ImageError>;

pub(crate) fn check_status(code: i32) -> ImageResult<()> {
    if code == 0 {
        Ok(())
    } else {
        Err(ImageError { code })
    }
}
