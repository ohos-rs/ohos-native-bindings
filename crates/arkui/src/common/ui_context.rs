//! Module common::ui_context wrappers and related types.

use napi_ohos::bindgen_prelude::{check_status, FromNapiValue, TypeName, ValidateNapiValue};
use napi_sys_ohos as sys;
use ohos_arkui_sys::{ArkUI_ContextHandle, OH_ArkUI_GetContextFromNapiValue};
use std::ptr;

#[cfg(feature = "api-26")]
/// Raw input event stream category controlled by [`ArkUIContext::enable_event_passthrough`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawInputEventType {
    /// Touch input events (`ARKUI_RAW_INPUT_EVENT_TYPE_TOUCH`).
    Touch,
    /// Mouse pointer events (`ARKUI_RAW_INPUT_EVENT_TYPE_MOUSE`).
    Mouse,
}

#[cfg(feature = "api-26")]
impl From<RawInputEventType> for ohos_arkui_sys::ArkUI_RawInputEventType {
    fn from(value: RawInputEventType) -> Self {
        match value {
            RawInputEventType::Touch => {
                ohos_arkui_sys::ArkUI_RawInputEventType_ARKUI_RAW_INPUT_EVENT_TYPE_TOUCH
            }
            RawInputEventType::Mouse => {
                ohos_arkui_sys::ArkUI_RawInputEventType_ARKUI_RAW_INPUT_EVENT_TYPE_MOUSE
            }
        }
    }
}

#[derive(Clone, Copy)]
/// Wrapper around `ArkUI_ContextHandle` obtained from N-API values.
pub struct ArkUIContext {
    /// N-API environment.
    pub(crate) env: sys::napi_env,
    /// N-API UIContext value.
    pub(crate) value: sys::napi_value,
    raw: ArkUI_ContextHandle,
}

impl ArkUIContext {
    /// Returns the underlying native `ArkUI_ContextHandle`.
    pub fn raw_handle(&self) -> ArkUI_ContextHandle {
        self.raw
    }

    pub(crate) fn raw(&self) -> ArkUI_ContextHandle {
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
