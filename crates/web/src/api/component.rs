use std::{ptr::NonNull, sync::LazyLock};

use ohos_web_sys::{
    ArkWeb_ComponentAPI, ArkWeb_NativeAPIVariantKind_ARKWEB_NATIVE_COMPONENT,
    OH_ArkWeb_GetNativeAPI,
};

pub struct Component {
    pub(crate) raw: NonNull<ArkWeb_ComponentAPI>,
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

unsafe impl Send for Component {}
unsafe impl Sync for Component {}

pub static ARK_WEB_COMPONENT_API: LazyLock<Component> = LazyLock::new(Component::new);
