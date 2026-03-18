use std::ptr::NonNull;

use ohos_arkui_input_binding::ArkUIErrorCode;

use crate::{ArkUIError, ArkUIResult};

pub(crate) fn non_null_or_error<T>(ptr: *mut T, func: &'static str) -> ArkUIResult<NonNull<T>> {
    NonNull::new(ptr).ok_or_else(|| {
        ArkUIError::new(
            ArkUIErrorCode::ParamInvalid,
            format!("{func} returned null"),
        )
    })
}
