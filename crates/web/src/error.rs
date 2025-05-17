#[derive(Debug, Clone)]
pub enum ArkWebError {
    WebviewCreateFailed(String),
}

impl std::fmt::Display for ArkWebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for ArkWebError {}
