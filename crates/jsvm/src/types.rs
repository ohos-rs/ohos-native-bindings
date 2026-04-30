use std::fmt;

use ohos_jsvm_sys as sys;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Undefined,
    Null,
    Boolean,
    Number,
    String,
    Symbol,
    Object,
    Function,
    External,
    BigInt,
}

impl ValueType {
    pub fn from_raw(raw: sys::JSVM_ValueType) -> Option<Self> {
        match raw {
            sys::JSVM_ValueType_JSVM_UNDEFINED => Some(Self::Undefined),
            sys::JSVM_ValueType_JSVM_NULL => Some(Self::Null),
            sys::JSVM_ValueType_JSVM_BOOLEAN => Some(Self::Boolean),
            sys::JSVM_ValueType_JSVM_NUMBER => Some(Self::Number),
            sys::JSVM_ValueType_JSVM_STRING => Some(Self::String),
            sys::JSVM_ValueType_JSVM_SYMBOL => Some(Self::Symbol),
            sys::JSVM_ValueType_JSVM_OBJECT => Some(Self::Object),
            sys::JSVM_ValueType_JSVM_FUNCTION => Some(Self::Function),
            sys::JSVM_ValueType_JSVM_EXTERNAL => Some(Self::External),
            sys::JSVM_ValueType_JSVM_BIGINT => Some(Self::BigInt),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Undefined => "undefined",
            Self::Null => "null",
            Self::Boolean => "boolean",
            Self::Number => "number",
            Self::String => "string",
            Self::Symbol => "symbol",
            Self::Object => "object",
            Self::Function => "function",
            Self::External => "external",
            Self::BigInt => "bigint",
        }
    }
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
