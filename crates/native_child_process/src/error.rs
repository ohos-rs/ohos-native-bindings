use std::{ffi::NulError, fmt};

pub type Result<T> = std::result::Result<T, NativeChildProcessError>;

#[derive(Debug)]
#[non_exhaustive]
pub enum NativeChildProcessError {
    /// A native API returned this unmodified error code.
    InternalError(u32),
    /// A Rust string cannot be passed as a NUL-terminated native string.
    InvalidString(NulError),
    /// The official FD list limit is 16 entries.
    TooManyFileDescriptors,
    /// Native configs allocation returned a null pointer.
    NullPointer,
}

impl NativeChildProcessError {
    pub(crate) fn check(code: u32) -> Result<()> {
        if code == 0 {
            Ok(())
        } else {
            Err(Self::InternalError(code))
        }
    }
}

impl From<NulError> for NativeChildProcessError {
    fn from(error: NulError) -> Self {
        Self::InvalidString(error)
    }
}

impl fmt::Display for NativeChildProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InternalError(code) => write!(f, "native child-process API failed: {code}"),
            Self::InvalidString(error) => error.fmt(f),
            Self::TooManyFileDescriptors => f.write_str("child-process FD list exceeds 16 entries"),
            Self::NullPointer => f.write_str("failed to create child-process configs"),
        }
    }
}

impl std::error::Error for NativeChildProcessError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidString(error) => Some(error),
            _ => None,
        }
    }
}
