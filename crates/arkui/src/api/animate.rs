use std::{cell::LazyCell, ffi::CString, ptr::NonNull};

use ohos_arkui_sys::{
    ArkUI_AnimateCompleteCallback, ArkUI_AnimateOption, ArkUI_ContextCallback, ArkUI_ContextHandle,
    ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_ANIMATE, ArkUI_NativeAnimateAPI_1,
    OH_ArkUI_QueryModuleInterfaceByName,
};

use crate::ArkUIError;

/// ArkUI_NativeNodeAPI_1 struct
/// Only can be used in main thread
pub const ARK_UI_NATIVE_ANIMATE_API_1: LazyCell<ArkUINativeAnimateAPI1> = LazyCell::new(|| {
    let api = ArkUINativeAnimateAPI1::new();
    api
});

pub struct ArkUINativeAnimateAPI1 {
    pub(crate) raw: NonNull<ArkUI_NativeAnimateAPI_1>,
}

impl ArkUINativeAnimateAPI1 {
    /// allow us to get the pointer of ArkUI_NativeAnimateAPI_1 and use it directly
    pub fn raw(&self) -> NonNull<ArkUI_NativeAnimateAPI_1> {
        self.raw
    }

    pub fn new() -> Self {
        #[allow(unused_assignments)]
        let mut api: *mut ArkUI_NativeAnimateAPI_1 = std::ptr::null_mut();
        let struct_name = c"ArkUI_NativeAnimateAPI_1";
        let raw_ptr = unsafe {
            OH_ArkUI_QueryModuleInterfaceByName(
                ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_ANIMATE,
                struct_name.as_ptr().cast(),
            )
        };
        #[cfg(debug_assertions)]
        assert!(!raw_ptr.is_null(), "ArkUI_NativeAnimateAPI_1 is NULL");
        api = raw_ptr.cast();
        Self {
            raw: unsafe { NonNull::new_unchecked(api) },
        }
    }

    pub fn animate_to(
        &self,
        ctx: ArkUI_ContextHandle,
        option: *mut ArkUI_AnimateOption,
        update: *mut ArkUI_ContextCallback,
        finish: *mut ArkUI_AnimateCompleteCallback,
    ) -> Result<(), ArkUIError> {
        unsafe {
            if let Some(animate_to_func) = (*self.raw.as_ptr()).animateTo {
                let ret = animate_to_func(ctx, option, update, finish);
                if ret != 0 {
                    Err(ArkUIError::InternalError(format!(
                        "api is: ArkUI_NativeAnimateAPI_1::animateTo, error_code is {:?}",
                        ret
                    )))
                } else {
                    Ok(())
                }
            } else {
                Err(ArkUIError::InternalError(format!(
                    "ArkUI_NativeAnimateAPI_1::animateTo is None"
                )))
            }
        }
    }
}
