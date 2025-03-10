#[derive(Debug)]
pub enum NativeWindowError {
    /// Call system api failed
    InternalError(i32),
}

impl std::fmt::Display for NativeWindowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NativeWindowError::InternalError(err) => write!(f, "Call system api failed: {}", err),
        }
    }
}

impl std::error::Error for NativeWindowError {}
