#[derive(Debug)]
pub enum RawFileError {
    IsNotRawDir(String),
    FfiInnerError(String),
}

impl std::fmt::Display for RawFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawFileError::IsNotRawDir(path) => write!(f, "Is not raw dir: {}", path),
            RawFileError::FfiInnerError(msg) => write!(f, "Ffi inner error: {}", msg),
        }
    }
}

impl std::error::Error for RawFileError {}
