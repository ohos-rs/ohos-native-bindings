#![cfg(feature = "napi")]

use napi_ohos::bindgen_prelude::{check_status, FromNapiValue, TypeName, ValidateNapiValue};
use napi_sys_ohos as sys;
use ohos_arkui_sys::{ArkUI_ContextHandle, OH_ArkUI_GetContextFromNapiValue};
use std::ptr;

#[derive(Clone, Copy)]
pub struct ArkUIContext {
    pub(crate) env: sys::napi_env,
    pub(crate) value: sys::napi_value,
    raw: ArkUI_ContextHandle,
}

impl ArkUIContext {
    pub fn raw(&self) -> ArkUI_ContextHandle {
        self.raw
    }
}

impl TypeName for ArkUIContext {
    fn type_name() -> &'static str {
        "ArkUIContext"
    }
    fn value_type() -> napi_ohos::ValueType {
        napi_ohos::ValueType::Object
    }
}

impl ValidateNapiValue for ArkUIContext {}

impl FromNapiValue for ArkUIContext {
    unsafe fn from_napi_value(
        env: sys::napi_env,
        napi_val: sys::napi_value,
    ) -> napi_ohos::Result<Self> {
        let mut slot = ptr::null_mut();
        unsafe {
            check_status!(
                OH_ArkUI_GetContextFromNapiValue(env, napi_val, &mut slot),
                "Get UIContext failed."
            )?
        };
        Ok(ArkUIContext {
            env,
            value: napi_val,
            raw: slot,
        })
    }
}
