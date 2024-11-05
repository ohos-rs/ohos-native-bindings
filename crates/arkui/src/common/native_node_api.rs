use std::{cell::LazyCell, ffi::CString};

use napi_ohos::{Error, Result};
use ohos_arkui_sys::{
    ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_NODE, ArkUI_NativeNodeAPI_1, ArkUI_NodeHandle,
    OH_ArkUI_QueryModuleInterfaceByName,
};

use crate::{ArkUINodeAttributeType, ArkUINodeType};

use super::{ArkUINode, ArkUINodeAttributeItem};

/// ArkUI_NativeNodeAPI_1 struct
/// Only can be used in main thread
pub const ARK_UI_NATIVE_NODE_API_1: LazyCell<ArkUINativeNodeAPI1> = LazyCell::new(|| {
    let api = ArkUINativeNodeAPI1::new();
    api
});

pub struct ArkUINativeNodeAPI1(pub(crate) *mut ArkUI_NativeNodeAPI_1);

impl ArkUINativeNodeAPI1 {
    pub fn new() -> Self {
        let mut api: *mut ArkUI_NativeNodeAPI_1 = std::ptr::null_mut();
        let struct_name = CString::new("ArkUI_NativeNodeAPI_1").unwrap();
        let raw_ptr = unsafe {
            OH_ArkUI_QueryModuleInterfaceByName(
                ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_NODE,
                struct_name.as_ptr().cast(),
            )
        };
        #[cfg(debug_assertions)]
        assert!(!raw_ptr.is_null(), "ArkUI_NativeNodeAPI_1 is NULL");
        api = raw_ptr.cast();
        Self(api)
    }

    pub fn create_node(&self, node_type: ArkUINodeType) -> Result<ArkUI_NodeHandle> {
        unsafe {
            if let Some(api) = (*self.0).createNode {
                let handle = api(node_type.into());
                Ok(handle)
            } else {
                Err(Error::from_reason(
                    "ArkUI_NativeNodeAPI_1::createNode is None",
                ))
            }
        }
    }

    pub fn set_attribute(
        &self,
        node: &ArkUINode,
        attr: ArkUINodeAttributeType,
        value: ArkUINodeAttributeItem,
    ) -> Result<()> {
        unsafe {
            if let Some(api) = (*self.0).setAttribute {
                api(
                    node.raw(),
                    attr.into(),
                    Box::into_raw(Box::new(value.into())),
                );
                Ok(())
            } else {
                Err(Error::from_reason(
                    "ArkUI_NativeNodeAPI_1::setAttribute is None",
                ))
            }
        }
    }
}
