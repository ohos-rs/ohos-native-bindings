#[derive(Debug)]
pub enum VibratorError {
    /// Call system api failed
    InternalError(i32),

    /// Common error
    CommonError(String),
}

impl std::fmt::Display for VibratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VibratorError::InternalError(err) => write!(f, "Call system api failed: {}", err),
            VibratorError::CommonError(err) => write!(f, "Common error: {}", err),
        }
    }
}

impl std::error::Error for VibratorError {}
