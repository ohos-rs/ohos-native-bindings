use ohos_enum_derive::EnumFrom;
use ohos_native_drawing_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(OH_Drawing_ErrorCode, "OH_Drawing_ErrorCode_OH_DRAWING_ERROR_")]
pub enum DrawingErrorCode {
    #[prefix("OH_Drawing_ErrorCode_OH_DRAWING_")]
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
