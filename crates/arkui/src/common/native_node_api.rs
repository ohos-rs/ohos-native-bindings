use std::{cell::LazyCell, ffi::CString};

use ohos_arkui_sys::{
    ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_NODE, ArkUI_NativeNodeAPI_1, ArkUI_NodeHandle,
    OH_ArkUI_QueryModuleInterfaceByName,
};

use crate::{check_arkui_status, ArkUINodeAttributeType, ArkUINodeType};

use super::{ArkUIError, ArkUIErrorCode, ArkUINode, ArkUINodeAttributeItem, ArkUIResult};

/// ArkUI_NativeNodeAPI_1 struct
/// Only can be used in main thread
pub const ARK_UI_NATIVE_NODE_API_1: LazyCell<ArkUINativeNodeAPI1> = LazyCell::new(|| {
    let api = ArkUINativeNodeAPI1::new();
    api
});

pub struct ArkUINativeNodeAPI1(pub(crate) *mut ArkUI_NativeNodeAPI_1);

impl ArkUINativeNodeAPI1 {
    /// allow us to get the pointer of ArkUI_NativeNodeAPI_1 and use it directly
    pub fn raw(&self) -> *mut ArkUI_NativeNodeAPI_1 {
        self.0
    }

    pub fn new() -> Self {
        #[allow(unused_assignments)]
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

    pub fn create_node(&self, node_type: ArkUINodeType) -> ArkUIResult<ArkUI_NodeHandle> {
        unsafe {
            if let Some(create_node) = (*self.0).createNode {
                let handle = create_node(node_type.into());
                if handle.is_null() {
                    return Err(ArkUIError::new(
                        super::ArkUIErrorCode::ArkTSNodeNotSupported,
                        "Create node failed",
                    ));
                }
                Ok(handle)
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
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
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(set_attribute) = (*self.0).setAttribute {
                check_arkui_status!(set_attribute(node.raw(), attr.into(), &value.into()))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::setAttribute is None",
                ))
            }
        }
    }

    pub fn add_child(&self, parent: &ArkUINode, child: &ArkUINode) -> ArkUIResult<()> {
        unsafe {
            if let Some(add_child) = (*self.0).addChild {
                check_arkui_status!(add_child(parent.raw(), child.raw()))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::addChild is None",
                ))
            }
        }
    }

    pub fn remove_child(&self, parent: &ArkUINode, child: &ArkUINode) -> ArkUIResult<()> {
        unsafe {
            if let Some(remove_child) = (*self.0).removeChild {
                check_arkui_status!(remove_child(parent.raw(), child.raw()))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::removeChild is None",
                ))
            }
        }
    }

    pub fn insert_child(
        &self,
        parent: &ArkUINode,
        child: &ArkUINode,
        index: i32,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(insert_child_at) = (*self.0).insertChildAt {
                check_arkui_status!(insert_child_at(parent.raw(), child.raw(), index))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::insertChild is None",
                ))
            }
        }
    }

    pub fn dispose(&self, node: &ArkUINode) -> ArkUIResult<()> {
        unsafe {
            if let Some(dispose_node) = (*self.0).disposeNode {
                dispose_node(node.raw());
                Ok(())
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::dispose is None",
                ))
            }
        }
    }
}
