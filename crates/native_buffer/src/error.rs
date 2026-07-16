#[derive(Debug)]
pub enum NativeBufferError {
    /// Call system api failed.
    InternalError(i32),
}

impl NativeBufferError {
    pub const fn code(&self) -> i32 {
        match self {
            Self::InternalError(code) => *code,
        }
    }
}

impl std::fmt::Display for NativeBufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NativeBufferError::InternalError(err) => write!(f, "Call system api failed: {}", err),
        }
    }
}
