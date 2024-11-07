#![cfg(feature = "napi")]

use napi_ohos::bindgen_prelude::{check_status, FromNapiValue, TypeName, ValidateNapiValue};
use napi_sys_ohos as sys;
use ohos_arkui_sys::{ArkUI_NodeContentHandle, OH_ArkUI_GetNodeContentFromNapiValue};
use std::ptr;

#[derive(Clone, Copy)]
pub struct ArkUIHandle {
    pub(crate) env: sys::napi_env,
    pub(crate) value: sys::napi_value,
    raw: ArkUI_NodeContentHandle,
}

impl ArkUIHandle {
    pub fn raw(&self) -> ArkUI_NodeContentHandle {
        self.raw
    }
}

impl TypeName for ArkUIHandle {
    fn type_name() -> &'static str {
        "ArkUIHandle"
    }
    fn value_type() -> napi_ohos::ValueType {
        napi_ohos::ValueType::Object
    }
}

impl ValidateNapiValue for ArkUIHandle {}

impl FromNapiValue for ArkUIHandle {
    unsafe fn from_napi_value(
        env: sys::napi_env,
        napi_val: sys::napi_value,
    ) -> napi_ohos::Result<Self> {
        let mut slot = ptr::null_mut();
        unsafe {
            check_status!(
                OH_ArkUI_GetNodeContentFromNapiValue(env, napi_val, &mut slot),
                "Get Node Content Slot failed."
            )?
        };
        Ok(ArkUIHandle {
            env,
            value: napi_val,
            raw: slot,
        })
    }
}
