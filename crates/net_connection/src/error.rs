use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, NetConnectionError>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetConnectionError {
    Code(i32),
    NullByte,
    StringTooLong,
    TooManyItems,
    Conversion,
}

impl NetConnectionError {
    pub fn code(self) -> i32 {
        match self {
            Self::Code(code) => code,
            Self::NullByte => -1,
            Self::StringTooLong => -2,
            Self::TooManyItems => -3,
            Self::Conversion => -4,
        }
    }
}

impl Display for NetConnectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "net connection error code {}", self.code())
    }
}

impl std::error::Error for NetConnectionError {}
