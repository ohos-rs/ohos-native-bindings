use ohos_drawing_sys::*;
use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(OH_Drawing_ErrorCode, "OH_Drawing_ErrorCode_OH_DRAWING_ERROR_")]
pub enum DrawingErrorCode {
    #[enum_prefix("OH_Drawing_ErrorCode_OH_DRAWING_")]
    Success,
    NoPermission,
    InvalidParameter,
    ParameterOutOfRange,
    #[cfg(feature = "api-13")]
    AllocationFailed,
    #[cfg(feature = "api-21")]
    AttributeIdMismatch,
    #[cfg(feature = "api-22")]
    IncorrectParameter,
}
