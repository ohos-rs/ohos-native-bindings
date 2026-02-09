use ohos_arkui_input_sys::*;
use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone)]
pub enum ArkUIInputError {
    InternalError(i32),
    DeviceTypeNotSupported(String, u32),
}

impl std::fmt::Display for ArkUIInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArkUIInputError::InternalError(err) => write!(f, "Internal error: {}", err),
            ArkUIInputError::DeviceTypeNotSupported(api, device_type) => {
                write!(
                    f,
                    "You'r trying to use {} api, current device type not supported: {}",
                    api, device_type
                )
            }
        }
    }
}

impl std::error::Error for ArkUIInputError {}

#[derive(Debug, EnumFrom)]
#[enum_from_config(ArkUI_ErrorCode, "ArkUI_ErrorCode_ARKUI_ERROR_CODE_")]
pub enum ArkUIErrorCode {
    ParamInvalid,
    AttributeOrEventNotSupported,
    ArkTSNodeNotSupported,
    AdapterNotBound,
    AdapterExist,
    ChildNodeExist,
    NodeEventParamIndexOutOfRange,
    NodeEventParamInvalid,
    NodeIndexInvalid,
    BufferSizeError,
    NonScrollableContainer,
    BufferSizeNotEnough,
}

impl AsRef<str> for ArkUIErrorCode {
    fn as_ref(&self) -> &str {
        match self {
            ArkUIErrorCode::AdapterExist => "AdapterExist",
            ArkUIErrorCode::AdapterNotBound => "AdapterNotBound",
            ArkUIErrorCode::ArkTSNodeNotSupported => "ArkTSNodeNotSupported",
            ArkUIErrorCode::AttributeOrEventNotSupported => "AttributeOrEventNotSupported",
            ArkUIErrorCode::BufferSizeError => "BufferSizeError",
            ArkUIErrorCode::BufferSizeNotEnough => "BufferSizeNotEnough",
            ArkUIErrorCode::ChildNodeExist => "ChildNodeExist",
            ArkUIErrorCode::NonScrollableContainer => "NonScrollableContainer",
            ArkUIErrorCode::NodeEventParamIndexOutOfRange => "NodeEventParamIndexOutOfRange",
            ArkUIErrorCode::NodeEventParamInvalid => "NodeEventParamInvalid",
            ArkUIErrorCode::NodeIndexInvalid => "NodeIndexInvalid",
            ArkUIErrorCode::ParamInvalid => "ParamInvalid",
        }
    }
}
