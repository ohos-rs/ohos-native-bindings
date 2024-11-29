use std::{cell::LazyCell, ffi::CString};

use ohos_arkui_sys::{
    ArkUI_AnimateCompleteCallback, ArkUI_AnimateOption, ArkUI_ContextCallback, ArkUI_ContextHandle,
    ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_ANIMATE, ArkUI_NativeAnimateAPI_1,
    OH_ArkUI_QueryModuleInterfaceByName,
};

use crate::{
    check_arkui_status,
    common::{ArkUIError, ArkUIErrorCode, ArkUIResult},
};

/// ArkUI_NativeNodeAPI_1 struct
/// Only can be used in main thread
pub const ARK_UI_NATIVE_ANIMATE_API_1: LazyCell<ArkUINativeAnimateAPI1> = LazyCell::new(|| {
    let api = ArkUINativeAnimateAPI1::new();
    api
});

pub struct ArkUINativeAnimateAPI1(pub(crate) *mut ArkUI_NativeAnimateAPI_1);

impl ArkUINativeAnimateAPI1 {
    /// allow us to get the pointer of ArkUI_NativeAnimateAPI_1 and use it directly
    pub fn raw(&self) -> *mut ArkUI_NativeAnimateAPI_1 {
        self.0
    }

    pub fn new() -> Self {
        #[allow(unused_assignments)]
        let mut api: *mut ArkUI_NativeAnimateAPI_1 = std::ptr::null_mut();
        let struct_name = CString::new("ArkUI_NativeAnimateAPI_1").unwrap();
        let raw_ptr = unsafe {
            OH_ArkUI_QueryModuleInterfaceByName(
                ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_ANIMATE,
                struct_name.as_ptr().cast(),
            )
        };
        #[cfg(debug_assertions)]
        assert!(!raw_ptr.is_null(), "ArkUI_NativeAnimateAPI_1 is NULL");
        api = raw_ptr.cast();
        Self(api)
    }

    pub fn animate_to(
        &self,
        ctx: ArkUI_ContextHandle,
        option: *mut ArkUI_AnimateOption,
        update: *mut ArkUI_ContextCallback,
        finish: *mut ArkUI_AnimateCompleteCallback,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(animate_to_func) = (*self.0).animateTo {
                check_arkui_status!(animate_to_func(ctx, option, update, finish))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeAnimateAPI_1::animateTo is None",
                ))
            }
        }
    }
}
