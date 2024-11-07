use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArkUIError {
    #[error("ArkUI call `{0}` failed with params invalid")]
    ParamsInvalid(String),
}

/// This type is used for ArkUI result.
pub type ArkUIResult = Result<(), ArkUIError>;

#[doc(hidden)]
#[macro_export]
macro_rules! check_arkui_status {
  ($code:expr) => {{
    let c = $code;
    match c {
      $crate::sys::Status::napi_ok => Ok(()),
      _ => Err($crate::Error::new($crate::Status::from(c), "".to_owned())),
    }
  }};

  ($code:expr, $($msg:tt)*) => {{
    let c = $code;
    match c {
      $crate::sys::Status::napi_ok => Ok(()),
      _ => Err($crate::Error::new($crate::Status::from(c), format!($($msg)*))),
    }
  }};

  ($code:expr, $msg:expr, $env:expr, $val:expr) => {{
    let c = $code;
    match c {
      $crate::sys::Status::napi_ok => Ok(()),
      _ => Err($crate::Error::new($crate::Status::from(c), format!($msg, $crate::type_of!($env, $val)?))),
    }
  }};
}