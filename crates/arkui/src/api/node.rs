use std::cell::RefCell;
use std::os::raw::c_void;
use std::rc::Rc;
use std::{cell::LazyCell, ffi::CString};

use ohos_arkui_sys::{
    ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_NODE, ArkUI_NativeNodeAPI_1, ArkUI_NodeEvent,
    ArkUI_NodeEventType, ArkUI_NodeHandle, OH_ArkUI_NodeEvent_GetEventType,
    OH_ArkUI_NodeEvent_GetNodeHandle, OH_ArkUI_QueryModuleInterfaceByName,
};

use crate::{check_arkui_status, ArkUINodeAttributeType, ArkUINodeType, NodeEventType};

use crate::common::{ArkUIError, ArkUIErrorCode, ArkUINode, ArkUINodeAttributeItem, ArkUIResult};

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
                        ArkUIErrorCode::ArkTSNodeNotSupported,
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
                    "ArkUI_NativeNodeAPI_1::disposeNode is None",
                ))
            }
        }
    }

    pub fn register_node_event(
        &self,
        node: &ArkUINode,
        event_type: NodeEventType,
    ) -> ArkUIResult<()> {
        unsafe {
            if let Some(register_node_event) = (*self.0).registerNodeEvent {
                let t: ArkUI_NodeEventType = event_type.into();
                check_arkui_status!(register_node_event(node.raw(), t, 0, std::ptr::null_mut()))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::registerNodeEvent is None",
                ))
            }
        }
    }

    pub fn add_event_receiver(&self, node: &ArkUINode) -> ArkUIResult<()> {
        unsafe {
            if let Some(add_node_event_receiver) = (*self.0).addNodeEventReceiver {
                check_arkui_status!(add_node_event_receiver(
                    node.raw(),
                    Some(node_event_receiver)
                ))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::addNodeEventReceiver is None",
                ))
            }
        }
    }

    pub fn remove_event_receiver(&self, node: &ArkUINode) -> ArkUIResult<()> {
        unsafe {
            if let Some(remove_node_event_receiver) = (*self.0).removeNodeEventReceiver {
                check_arkui_status!(remove_node_event_receiver(
                    node.raw(),
                    Some(node_event_receiver)
                ))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::removeNodeEventReceiver is None",
                ))
            }
        }
    }

    pub fn set_user_data(&self, node: &ArkUINode, user_data: *mut c_void) -> ArkUIResult<()> {
        unsafe {
            if let Some(set_user_data) = (*self.0).setUserData {
                check_arkui_status!(set_user_data(node.raw(), user_data))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::setUserData is None",
                ))
            }
        }
    }

    pub fn get_user_data(&self, node_handle: ArkUI_NodeHandle) -> ArkUIResult<*mut c_void> {
        unsafe {
            if let Some(get_user_data) = (*self.0).getUserData {
                Ok(get_user_data(node_handle))
            } else {
                Err(ArkUIError::new(
                    ArkUIErrorCode::AttributeOrEventNotSupported,
                    "ArkUI_NativeNodeAPI_1::setUserData is None",
                ))
            }
        }
    }
}

unsafe extern "C" fn node_event_receiver(event: *mut ArkUI_NodeEvent) {
    let handle = OH_ArkUI_NodeEvent_GetNodeHandle(event);
    let user_data = ARK_UI_NATIVE_NODE_API_1.get_user_data(handle).unwrap();

    #[cfg(debug_assertions)]
    assert!(!user_data.is_null(), "user_data is null");

    let user_data_rc: &Rc<RefCell<ArkUINode>> = &*(user_data as *const Rc<RefCell<ArkUINode>>);

    let node = user_data_rc.borrow();

    let event_type = OH_ArkUI_NodeEvent_GetEventType(event);
    let event_type = NodeEventType::from(event_type);
    match event_type {
        NodeEventType::OnClick => {
            if let Some(cb) = node.event_handle.click.as_ref() {
                cb.borrow()();
            }
        }
        _ => {}
    }
}
