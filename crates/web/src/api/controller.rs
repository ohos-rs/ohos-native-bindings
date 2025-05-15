use std::ptr::NonNull;

use ohos_web_sys::{
    ArkWeb_ControllerAPI, ArkWeb_NativeAPIVariantKind_ARKWEB_NATIVE_CONTROLLER,
    OH_ArkWeb_GetNativeAPI,
};

pub struct Controller {
    raw: NonNull<ArkWeb_ControllerAPI>,
}

impl Controller {
    pub fn new() -> Self {
        let ret = unsafe {
            OH_ArkWeb_GetNativeAPI(ArkWeb_NativeAPIVariantKind_ARKWEB_NATIVE_CONTROLLER)
                as *mut ArkWeb_ControllerAPI
        };
        #[cfg(debug_assertions)]
        assert!(
            !ret.is_null(),
            "Failed to get Controller API by OH_ArkWeb_GetNativeAPI"
        );

        Self {
            raw: unsafe { NonNull::new_unchecked(ret) },
        }
    }
}
