use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraErrorKind {
    NoCamera,
    Unsupported,
    InvalidState,
    Native,
    Image,
    Surface,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CameraError {
    kind: CameraErrorKind,
    operation: &'static str,
    native_code: Option<u32>,
    message: String,
}

impl CameraError {
    pub fn kind(&self) -> CameraErrorKind {
        self.kind
    }

    pub fn operation(&self) -> &'static str {
        self.operation
    }

    pub fn native_code(&self) -> Option<u32> {
        self.native_code
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub(crate) fn native(operation: &'static str, code: u32) -> Self {
        Self {
            kind: CameraErrorKind::Native,
            operation,
            native_code: Some(code),
            message: format!("CameraKit `{operation}` failed with code {code}"),
        }
    }

    pub(crate) fn image(operation: &'static str, code: u32) -> Self {
        Self {
            kind: CameraErrorKind::Image,
            operation,
            native_code: Some(code),
            message: format!("ImageKit `{operation}` failed with code {code}"),
        }
    }

    pub(crate) fn no_camera(position: crate::CameraPosition) -> Self {
        Self {
            kind: CameraErrorKind::NoCamera,
            operation: "OH_CameraManager_GetSupportedCameras",
            native_code: None,
            message: format!("no {position:?} camera is available"),
        }
    }

    pub(crate) fn unsupported(operation: &'static str, message: impl Into<String>) -> Self {
        Self {
            kind: CameraErrorKind::Unsupported,
            operation,
            native_code: None,
            message: message.into(),
        }
    }

    pub(crate) fn invalid_state(operation: &'static str, message: impl Into<String>) -> Self {
        Self {
            kind: CameraErrorKind::InvalidState,
            operation,
            native_code: None,
            message: message.into(),
        }
    }

    pub(crate) fn surface(operation: &'static str, message: impl Into<String>) -> Self {
        Self {
            kind: CameraErrorKind::Surface,
            operation,
            native_code: None,
            message: message.into(),
        }
    }
}

impl fmt::Display for CameraError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for CameraError {}

pub type CameraResult<T> = Result<T, CameraError>;
