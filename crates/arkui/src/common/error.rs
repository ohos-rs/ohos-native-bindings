#[derive(Debug)]
pub enum ArkUIError {
    /// Call system api failed
    InternalError(String),

    /// Std Error
    StdError(String),

    /// Null Error
    NullError(String),
}

impl std::fmt::Display for ArkUIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArkUIError::InternalError(err) => write!(f, "Call system api failed: {}", err),
            ArkUIError::StdError(err) => write!(f, "Std Error: {}", err),
            ArkUIError::NullError(err) => {
                write!(f, "Call api failed and return value is null: {}", err)
            }
        }
    }
}

impl std::error::Error for ArkUIError {}
