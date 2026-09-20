use std::fmt::{Display, Formatter};

use ohos_native_window_manager_sys::WindowManager_ErrorCode_OK;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Native(i64),
    UnexpectedNull,
    InteriorNul,
}

impl Error {
    pub const fn code(&self) -> Option<i64> {
        match self {
            Self::Native(code) => Some(*code),
            Self::UnexpectedNull | Self::InteriorNul => None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Native(code) => write!(f, "window manager error code {code}"),
            Self::UnexpectedNull => f.write_str("window manager returned a null pointer"),
            Self::InteriorNul => f.write_str("string contains an interior NUL byte"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(any(feature = "api-15", test))]
pub(crate) fn check(code: i32) -> Result<()> {
    if i64::from(code) == i64::from(WindowManager_ErrorCode_OK) {
        Ok(())
    } else {
        Err(Error::Native(i64::from(code)))
    }
}

pub(crate) fn check_status(code: u32) -> Result<()> {
    if code == WindowManager_ErrorCode_OK {
        Ok(())
    } else {
        Err(Error::Native(i64::from(code)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_signed_native_status() {
        assert_eq!(check(0), Ok(()));
        assert_eq!(check(-7), Err(Error::Native(-7)));
        assert_eq!(check(801), Err(Error::Native(801)));
        assert_eq!(
            check(-7).unwrap_err().to_string(),
            "window manager error code -7"
        );
    }

    #[test]
    fn preserves_unsigned_native_status() {
        assert_eq!(check_status(0), Ok(()));
        assert_eq!(check_status(1000), Err(Error::Native(1000)));
    }
}
