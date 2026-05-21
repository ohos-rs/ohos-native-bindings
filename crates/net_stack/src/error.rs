use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, NetStackError>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetStackError {
    Code(i32),
    NullByte,
    NullPointer,
    StringTooLong,
    Conversion,
}

impl NetStackError {
    pub fn code(self) -> i32 {
        match self {
            Self::Code(code) => code,
            Self::NullByte => -1,
            Self::NullPointer => -2,
            Self::StringTooLong => -3,
            Self::Conversion => -4,
        }
    }
}

impl Display for NetStackError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "net stack error code {}", self.code())
    }
}

impl std::error::Error for NetStackError {}
