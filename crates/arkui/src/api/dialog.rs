use std::{cell::LazyCell, ffi::CString};

use ohos_arkui_sys::{
    ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_DIALOG, ArkUI_NativeDialogAPI_1,
    ArkUI_NativeDialogHandle, OH_ArkUI_QueryModuleInterfaceByName,
};

use crate::{ArkUIError, ArkUIErrorCode, ArkUIResult};

/// ArkUI_NativeNodeAPI_1 struct
/// Only can be used in main thread
pub const ARK_UI_NATIVE_DIALOG_API_1: LazyCell<ArkUINativeDialogAPI1> = LazyCell::new(|| {
    let api = ArkUINativeDialogAPI1::new();
    api
});

pub struct ArkUINativeDialogAPI1(pub(crate) *mut ArkUI_NativeDialogAPI_1);

impl ArkUINativeDialogAPI1 {
    /// allow us to get the pointer of ArkUI_NativeDialogAPI_1 and use it directly
    pub fn raw(&self) -> *mut ArkUI_NativeDialogAPI_1 {
        self.0
    }

    pub fn new() -> Self {
        #[allow(unused_assignments)]
        let mut api: *mut ArkUI_NativeDialogAPI_1 = std::ptr::null_mut();
        let struct_name = CString::new("ArkUI_NativeDialogAPI_1").unwrap();
        let raw_ptr = unsafe {
            OH_ArkUI_QueryModuleInterfaceByName(
                ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_DIALOG,
                struct_name.as_ptr().cast(),
            )
        };
        #[cfg(debug_assertions)]
        assert!(!raw_ptr.is_null(), "ArkUI_NativeDialogAPI_1 is NULL");
        api = raw_ptr.cast();
        Self(api)
    }

    pub fn create(&self) -> ArkUIResult<ArkUI_NativeDialogHandle> {
        unsafe {
            if let Some(create_dialog_controller) = (*self.0).create {
                let ret = create_dialog_controller();
                Ok(ret)
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeDialogAPI_1::create is None",
                ))
            }
        }
    }
}
