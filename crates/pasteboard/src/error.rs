#[derive(Debug)]
pub enum PasteboardError {
    /// Call system api failed
    IntervalError(i32),

    /// Common error
    CommonError(String),
}

impl std::fmt::Display for PasteboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PasteboardError::IntervalError(err) => write!(f, "Call system api failed: {}", err),
            PasteboardError::CommonError(err) => write!(f, "Common error: {}", err),
        }
    }
}

impl std::error::Error for PasteboardError {}
