use std::ptr::NonNull;

use ohos_web_sys::{
    ArkWeb_ComponentAPI, ArkWeb_NativeAPIVariantKind_ARKWEB_NATIVE_COMPONENT,
    OH_ArkWeb_GetNativeAPI,
};

pub struct Component {
    raw: NonNull<ArkWeb_ComponentAPI>,
}

impl Component {
    pub fn new() -> Self {
        let ret = unsafe {
            OH_ArkWeb_GetNativeAPI(ArkWeb_NativeAPIVariantKind_ARKWEB_NATIVE_COMPONENT)
                as *mut ArkWeb_ComponentAPI
        };
        #[cfg(debug_assertions)]
        assert!(
            !ret.is_null(),
            "Failed to get Component API by OH_ArkWeb_GetNativeAPI"
        );

        Self {
            raw: unsafe { NonNull::new_unchecked(ret) },
        }
    }
}
