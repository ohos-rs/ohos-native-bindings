use enum_macro::EnumFrom;
use ohos_xcomponent_sys::{
    _bindgen_ty_6, OH_NATIVEXCOMPONENT_RESULT_BAD_PARAMETER, OH_NATIVEXCOMPONENT_RESULT_FAILED,
    OH_NATIVEXCOMPONENT_RESULT_SUCCESS,
};

#[derive(Debug, PartialEq, EnumFrom)]
#[enum_from_config(_bindgen_ty_6, "OH_NATIVEXCOMPONENT_RESULT_")]
pub enum XComponentResultCode {
    Success,
    Failed,
    BadParameter,
}
