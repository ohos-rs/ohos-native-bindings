#[derive(Debug, Clone)]
pub enum ArkWebError {
    WebviewCreateFailed(String),
    ArkWebApiMemberMissing(String),
    EvaluateScriptCallbackAlreadyExists
}

impl std::fmt::Display for ArkWebError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArkWebError::WebviewCreateFailed(web_tag) => {
                write!(f, "Webview create failed: {}", web_tag)
            }
            ArkWebError::ArkWebApiMemberMissing(member) => {
                write!(f, "ArkWeb API member missing: {}", member)
            }
            ArkWebError::EvaluateScriptCallbackAlreadyExists => {
                write!(f, "Evaluate script callback already exists")
            }
        }
    }
}

impl std::error::Error for ArkWebError {}
