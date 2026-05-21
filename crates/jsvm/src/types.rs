use std::fmt;

use ohos_enum_derive::EnumFrom;
use ohos_jsvm_sys as sys;
use ohos_jsvm_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(JSVM_ValueType, "JSVM_ValueType_JSVM_")]
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
    #[suffix("BIGINT")]
    BigInt,
}

impl ValueType {
    pub fn from_raw(raw: sys::JSVM_ValueType) -> Option<Self> {
        Self::try_from_raw(raw)
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
