//! Module common::handle wrappers and related types.

use std::{os::raw::c_void, ptr::NonNull};

#[cfg(feature = "napi")]
use ohos_arkui_sys::OH_ArkUI_GetNodeContentFromNapiValue;
use ohos_arkui_sys::{
    ArkUI_NodeContentHandle, OH_ArkUI_NodeContent_AddNode, OH_ArkUI_NodeContent_InsertNode,
    OH_ArkUI_NodeContent_RemoveNode,
};

#[cfg(feature = "napi")]
use napi_ohos::bindgen_prelude::{check_status, FromNapiValue, TypeName, ValidateNapiValue};
#[cfg(feature = "napi")]
use napi_sys_ohos as sys;

use crate::{check_arkui_status, ArkUINode, ArkUIResult};

#[derive(Clone, Copy)]
/// Opaque handle for ArkUI node content.
///
/// This wrapper owns no memory by itself. It provides safe method wrappers
/// around `ArkUI_NodeContentHandle` operations.
pub struct ArkUIHandle {
    raw: NonNull<c_void>,
}

impl ArkUIHandle {
    /// Construct from a raw ArkUI handle.
    pub(crate) fn from_raw(raw: ArkUI_NodeContentHandle) -> Option<Self> {
        NonNull::new(raw.cast()).map(|raw| Self { raw })
    }

    /// Return the underlying raw handle.
    pub(crate) fn raw(&self) -> ArkUI_NodeContentHandle {
        self.raw.as_ptr().cast()
    }

    pub(crate) fn add_node(&self, node: &ArkUINode) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_NodeContent_AddNode(self.raw(), node.raw())) }
    }

    pub(crate) fn remove_node(&self, node: &ArkUINode) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_NodeContent_RemoveNode(self.raw(), node.raw())) }
    }

    pub(crate) fn insert_node(&self, node: &ArkUINode, position: i32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeContent_InsertNode(
                self.raw(),
                node.raw(),
                position
            ))
        }
    }
}

#[cfg(feature = "napi")]
impl TypeName for ArkUIHandle {
    fn type_name() -> &'static str {
        "ArkUIHandle"
    }
    fn value_type() -> napi_ohos::ValueType {
        napi_ohos::ValueType::Object
    }
}

#[cfg(feature = "napi")]
impl ValidateNapiValue for ArkUIHandle {}

#[cfg(feature = "napi")]
impl FromNapiValue for ArkUIHandle {
    unsafe fn from_napi_value(
        env: sys::napi_env,
        napi_val: sys::napi_value,
    ) -> napi_ohos::Result<Self> {
        let mut slot = std::ptr::null_mut();
        unsafe {
            check_status!(
                OH_ArkUI_GetNodeContentFromNapiValue(env, napi_val, &mut slot),
                "Get Node Content Slot failed."
            )?
        };
        ArkUIHandle::from_raw(slot).ok_or_else(|| {
            napi_ohos::Error::from_reason("OH_ArkUI_GetNodeContentFromNapiValue returned null")
        })
    }
}
