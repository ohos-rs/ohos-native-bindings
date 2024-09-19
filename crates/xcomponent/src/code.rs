use ohos_xcomponent_sys::{
    OH_NATIVEXCOMPONENT_RESULT_BAD_PARAMETER, OH_NATIVEXCOMPONENT_RESULT_FAILED,
    OH_NATIVEXCOMPONENT_RESULT_SUCCESS,
};

#[derive(Debug, PartialEq)]
pub enum XComponentResultCode {
    Success,
    Failed,
    BadParams,
}

impl From<ohos_xcomponent_sys::_bindgen_ty_6> for XComponentResultCode {
    fn from(value: ohos_xcomponent_sys::_bindgen_ty_6) -> Self {
        match value {
            OH_NATIVEXCOMPONENT_RESULT_SUCCESS => XComponentResultCode::Success,
            OH_NATIVEXCOMPONENT_RESULT_FAILED => XComponentResultCode::Failed,
            OH_NATIVEXCOMPONENT_RESULT_BAD_PARAMETER => XComponentResultCode::BadParams,
            _ => unreachable!(),
        }
    }
}
