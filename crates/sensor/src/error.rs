#[derive(Debug)]
pub enum SensorError {
    /// Call system api failed
    InternalError(i32),

    /// Common error
    CommonError(String),
}

impl std::fmt::Display for SensorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SensorError::InternalError(err) => write!(f, "Call system api failed: {}", err),
            SensorError::CommonError(err) => write!(f, "Common error: {}", err),
        }
    }
}

impl std::error::Error for SensorError {}
