#[derive(Debug, Clone)]
pub enum ArkWebError {
    WebviewCreateFailed(String),
    ArkWebApiMemberMissing(String),
    EvaluateScriptCallbackAlreadyExists,
    JsApiRegisterFailed(String),
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
            ArkWebError::JsApiRegisterFailed(obj_name) => {
                write!(f, "JS API register failed: {}", obj_name)
            }
        }
    }
}

impl std::error::Error for ArkWebError {}

/// An error registering, initializing, or reading an HTTP request body stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpBodyStreamError {
    NullStream,
    SetUserDataFailed(i32),
    SetReadCallbackFailed(i32),
    NotInitialized,
    InitializationFailed(i32),
    InvalidReadSize(usize),
    ReadFailed(i32),
    InvalidReadCount { count: i32, capacity: usize },
    UnexpectedBuffer,
    NoProgress,
}

impl std::fmt::Display for HttpBodyStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NullStream => f.write_str("HTTP body stream is null"),
            Self::SetUserDataFailed(code) => {
                write!(f, "Failed to register body stream state: {code}")
            }
            Self::SetReadCallbackFailed(code) => {
                write!(f, "Failed to register body read callback: {code}")
            }
            Self::NotInitialized => f.write_str("HTTP body stream has not been initialized"),
            Self::InitializationFailed(code) => write!(f, "Body initialization failed: {code}"),
            Self::InvalidReadSize(size) => write!(f, "Body read size exceeds i32::MAX: {size}"),
            Self::ReadFailed(code) => write!(f, "Body read failed: {code}"),
            Self::InvalidReadCount { count, capacity } => write!(
                f,
                "Body read returned {count} bytes for a {capacity}-byte buffer"
            ),
            Self::UnexpectedBuffer => f.write_str("Body read returned an unexpected buffer"),
            Self::NoProgress => f.write_str("Body stream stopped before EOF"),
        }
    }
}

impl std::error::Error for HttpBodyStreamError {}
