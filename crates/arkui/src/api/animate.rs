//! Module api::animate wrappers and related types.

use std::{cell::LazyCell, ffi::CString, ptr::NonNull};

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::{
    ArkUI_AnimateCompleteCallback, ArkUI_AnimateOption, ArkUI_ContextCallback, ArkUI_ContextHandle,
    ArkUI_KeyframeAnimateOption, ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_ANIMATE,
    ArkUI_NativeAnimateAPI_1, OH_ArkUI_QueryModuleInterfaceByName,
};

use crate::{
    check_arkui_status,
    common::{ArkUIError, ArkUIResult},
};

thread_local! {
    /// ArkUI_NativeNodeAPI_1 struct.
    /// Only can be used in main thread
    pub(crate) static ARK_UI_NATIVE_ANIMATE_API_1: LazyCell<ArkUINativeAnimateAPI1> =
    LazyCell::new(ArkUINativeAnimateAPI1::new);
}

pub(crate) struct ArkUINativeAnimateAPI1(pub(crate) NonNull<ArkUI_NativeAnimateAPI_1>);

impl ArkUINativeAnimateAPI1 {
    /// allow us to get the pointer of ArkUI_NativeAnimateAPI_1 and use it directly
    pub(crate) fn raw(&self) -> *mut ArkUI_NativeAnimateAPI_1 {
        self.0.as_ptr()
    }

    pub(crate) fn new() -> Self {
        let struct_name = CString::new("ArkUI_NativeAnimateAPI_1").unwrap();
        let raw_ptr = unsafe {
            OH_ArkUI_QueryModuleInterfaceByName(
                ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_ANIMATE,
                struct_name.as_ptr().cast(),
            )
        };
        let api = NonNull::new(raw_ptr.cast())
            .unwrap_or_else(|| panic!("ArkUI_NativeAnimateAPI_1 is NULL"));
        Self(api)
    }

    pub(crate) fn animate_to(
        &self,
        ctx: ArkUI_ContextHandle,
        option: *mut ArkUI_AnimateOption,
        update: *mut ArkUI_ContextCallback,
        finish: *mut ArkUI_AnimateCompleteCallback,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(animate_to_func) = (*self.raw()).animateTo {
                check_arkui_status!(animate_to_func(ctx, option, update, finish))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeAnimateAPI_1::animateTo is None",
                ))
            }
        }
    }

    pub(crate) fn keyframe_animate_to(
        &self,
        ctx: ArkUI_ContextHandle,
        option: *mut ArkUI_KeyframeAnimateOption,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(keyframe_animate_to_func) = (*self.raw()).keyframeAnimateTo {
                check_arkui_status!(keyframe_animate_to_func(ctx, option))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeAnimateAPI_1::keyframeAnimateTo is None",
                ))
            }
        }
    }
}

impl Default for ArkUINativeAnimateAPI1 {
    fn default() -> Self {
        Self::new()
    }
}
