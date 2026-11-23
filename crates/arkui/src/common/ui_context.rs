//! Module common::ui_context wrappers and related types.

use napi_ohos::bindgen_prelude::{check_status, FromNapiValue, TypeName, ValidateNapiValue};
use napi_sys_ohos as sys;
#[cfg(feature = "api-15")]
use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::{ArkUI_ContextHandle, OH_ArkUI_GetContextFromNapiValue};
#[cfg(feature = "api-15")]
use ohos_arkui_sys::{
    ArkUI_KeyProcessingMode,
    ArkUI_KeyProcessingMode_ARKUI_KEY_PROCESSING_MODE_FOCUS_ANCESTOR_EVENT,
    ArkUI_KeyProcessingMode_ARKUI_KEY_PROCESSING_MODE_FOCUS_NAVIGATION,
};
#[cfg(feature = "api-15")]
use ohos_enum_derive::EnumFrom;
use std::ptr;

#[cfg(feature = "api-15")]
use crate::{ArkUIError, ArkUIResult};

/// Key event processing priority mode configured via [`ArkUIContext::set_key_processing_mode`] (API 15).
#[cfg(feature = "api-15")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_KeyProcessingMode,
    "ArkUI_KeyProcessingMode_ARKUI_KEY_PROCESSING_MODE_"
)]
pub enum KeyProcessingMode {
    /// Key events are used for focus navigation.
    FocusNavigation,
    /// Key events are propagated up to ancestor components first.
    FocusAncestorEvent,
}

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

    /// Clears focus back to the root container of this UI context (API 15).
    #[cfg(feature = "api-15")]
    pub fn clear_focus(&self) -> ArkUIResult<()> {
        if self.raw.is_null() {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "ArkUI_ContextHandle is null",
            ));
        }
        // SAFETY: `self.raw` is a live non-null context on the UI thread.
        unsafe { ohos_arkui_sys::OH_ArkUI_FocusClear(self.raw) };
        Ok(())
    }

    /// Enters or exits the focus activation state for the current page (API 15).
    #[cfg(feature = "api-15")]
    pub fn activate_focus(&self, is_active: bool, is_auto_inactive: bool) -> ArkUIResult<()> {
        if self.raw.is_null() {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "ArkUI_ContextHandle is null",
            ));
        }
        // SAFETY: `self.raw` is a live non-null context on the UI thread.
        unsafe { ohos_arkui_sys::OH_ArkUI_FocusActivate(self.raw, is_active, is_auto_inactive) };
        Ok(())
    }

    /// Configures whether focus automatically transfers on page switch (API 15).
    #[cfg(feature = "api-15")]
    pub fn set_focus_auto_transfer(&self, auto_transfer: bool) -> ArkUIResult<()> {
        if self.raw.is_null() {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "ArkUI_ContextHandle is null",
            ));
        }
        // SAFETY: `self.raw` is a live non-null context on the UI thread.
        unsafe { ohos_arkui_sys::OH_ArkUI_FocusSetAutoTransfer(self.raw, auto_transfer) };
        Ok(())
    }

    /// Configures the key event processing priority mode (API 15).
    #[cfg(feature = "api-15")]
    pub fn set_key_processing_mode(&self, mode: KeyProcessingMode) -> ArkUIResult<()> {
        if self.raw.is_null() {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "ArkUI_ContextHandle is null",
            ));
        }
        // SAFETY: `self.raw` is a live non-null context on the UI thread.
        unsafe { ohos_arkui_sys::OH_ArkUI_FocusSetKeyProcessingMode(self.raw, mode.into()) };
        Ok(())
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
