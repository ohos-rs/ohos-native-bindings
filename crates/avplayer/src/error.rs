use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvPlayerErrorKind {
    InvalidConfiguration,
    InvalidState,
    Native,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AvPlayerError {
    kind: AvPlayerErrorKind,
    operation: &'static str,
    native_code: Option<u32>,
    message: String,
}

impl AvPlayerError {
    pub fn kind(&self) -> AvPlayerErrorKind {
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
            kind: AvPlayerErrorKind::Native,
            operation,
            native_code: Some(code),
            message: format!("AVPlayer `{operation}` failed with code {code}"),
        }
    }

    pub(crate) fn callback(code: i32, message: String) -> Self {
        Self {
            kind: AvPlayerErrorKind::Native,
            operation: "OH_AVPlayerOnErrorCallback",
            native_code: u32::try_from(code).ok(),
            message: if message.is_empty() {
                format!("AVPlayer callback reported error {code}")
            } else {
                format!("AVPlayer callback reported error {code}: {message}")
            },
        }
    }

    pub(crate) fn invalid_configuration(
        operation: &'static str,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind: AvPlayerErrorKind::InvalidConfiguration,
            operation,
            native_code: None,
            message: message.into(),
        }
    }

    pub(crate) fn invalid_state(operation: &'static str, message: impl Into<String>) -> Self {
        Self {
            kind: AvPlayerErrorKind::InvalidState,
            operation,
            native_code: None,
            message: message.into(),
        }
    }

    pub(crate) fn unavailable(operation: &'static str, message: impl Into<String>) -> Self {
        Self {
            kind: AvPlayerErrorKind::Unavailable,
            operation,
            native_code: None,
            message: message.into(),
        }
    }
}

impl fmt::Display for AvPlayerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for AvPlayerError {}

pub type AvPlayerResult<T> = Result<T, AvPlayerError>;
