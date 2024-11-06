use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArkUIError {
    #[error("ArkUI call `{0}` failed with params invalid")]
    ParamsInvalid(String),
}

/// This type is used for ArkUI result.
pub type ArkUIResult = Result<(), ArkUIError>;

macro_rules! check_arkui_status {
    () => {};
}
