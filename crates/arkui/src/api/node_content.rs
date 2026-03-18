use std::{os::raw::c_void, ptr::NonNull};

use ohos_arkui_sys::{
    ArkUI_NodeContentCallback, ArkUI_NodeContentEvent, ArkUI_NodeContentHandle,
    OH_ArkUI_NodeContentEvent_GetEventType, OH_ArkUI_NodeContentEvent_GetNodeContentHandle,
    OH_ArkUI_NodeContent_AddNode, OH_ArkUI_NodeContent_GetUserData,
    OH_ArkUI_NodeContent_InsertNode, OH_ArkUI_NodeContent_RegisterCallback,
    OH_ArkUI_NodeContent_RemoveNode, OH_ArkUI_NodeContent_SetUserData,
};

use crate::{check_arkui_status, ArkUINode, ArkUIResult};

fn non_null_or_panic<T>(ptr: *mut T, name: &'static str) -> NonNull<T> {
    NonNull::new(ptr).unwrap_or_else(|| panic!("{name} pointer is null"))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NodeContentHandle {
    raw: NonNull<c_void>,
}

impl NodeContentHandle {
    pub(crate) fn from_raw(raw: ArkUI_NodeContentHandle) -> Option<Self> {
        NonNull::new(raw.cast()).map(|raw| Self { raw })
    }

    pub(crate) fn raw(&self) -> ArkUI_NodeContentHandle {
        self.raw.as_ptr().cast()
    }
}

pub struct NodeContentEvent {
    raw: NonNull<ArkUI_NodeContentEvent>,
}

impl NodeContentEvent {
    fn from_raw(raw: *mut ArkUI_NodeContentEvent) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    fn raw(&self) -> *mut ArkUI_NodeContentEvent {
        self.raw.as_ptr()
    }

    pub fn event_type(&self) -> crate::NodeContentEventType {
        unsafe { OH_ArkUI_NodeContentEvent_GetEventType(self.raw()).into() }
    }

    pub fn node_content_handle(&self) -> Option<NodeContentHandle> {
        let handle = unsafe { OH_ArkUI_NodeContentEvent_GetNodeContentHandle(self.raw()) };
        NodeContentHandle::from_raw(handle)
    }
}

struct NodeContentCallbackContext {
    callback: Box<dyn Fn(&mut NodeContentEvent)>,
}

pub struct NodeContent {
    handle: NodeContentHandle,
    callback_context: Option<NonNull<NodeContentCallbackContext>>,
}

impl NodeContent {
    pub fn from_handle(handle: NodeContentHandle) -> Self {
        Self {
            handle,
            callback_context: None,
        }
    }

    pub(crate) fn from_raw(raw: ArkUI_NodeContentHandle) -> Option<Self> {
        NodeContentHandle::from_raw(raw).map(Self::from_handle)
    }

    #[cfg(feature = "napi")]
    pub fn from_napi_handle(handle: &crate::ArkUIHandle) -> Option<Self> {
        Self::from_raw(handle.raw())
    }

    pub fn handle(&self) -> NodeContentHandle {
        self.handle
    }

    pub fn add_node(&self, node: &ArkUINode) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_NodeContent_AddNode(self.handle.raw(), node.raw())) }
    }

    pub fn remove_node(&self, node: &ArkUINode) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeContent_RemoveNode(
                self.handle.raw(),
                node.raw()
            ))
        }
    }

    pub fn insert_node(&self, node: &ArkUINode, position: i32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeContent_InsertNode(
                self.handle.raw(),
                node.raw(),
                position
            ))
        }
    }

    pub fn register_callback<T: Fn(&mut NodeContentEvent) + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<()> {
        self.clear_callback()?;
        let context = Box::into_raw(Box::new(NodeContentCallbackContext {
            callback: Box::new(callback),
        }));
        let context = non_null_or_panic(context, "NodeContentCallbackContext");
        if let Err(err) = node_content_set_user_data_raw(self.handle.raw(), context.as_ptr().cast())
        {
            unsafe {
                drop(Box::from_raw(context.as_ptr()));
            }
            return Err(err);
        }
        let register_result = node_content_register_callback_raw(
            self.handle.raw(),
            Some(node_content_callback_trampoline),
        );
        if let Err(err) = register_result {
            let _ = node_content_set_user_data_raw(self.handle.raw(), std::ptr::null_mut());
            unsafe {
                drop(Box::from_raw(context.as_ptr()));
            }
            return Err(err);
        }
        self.callback_context = Some(context);
        Ok(())
    }

    pub fn clear_callback(&mut self) -> ArkUIResult<()> {
        let unregister_result = node_content_register_callback_raw(self.handle.raw(), None);
        let clear_data_result =
            node_content_set_user_data_raw(self.handle.raw(), std::ptr::null_mut());

        if unregister_result.is_ok() || clear_data_result.is_ok() {
            if let Some(context) = self.callback_context.take() {
                unsafe {
                    drop(Box::from_raw(context.as_ptr()));
                }
            }
        }

        unregister_result?;
        clear_data_result?;
        Ok(())
    }
}

unsafe extern "C" fn node_content_callback_trampoline(event: *mut ArkUI_NodeContentEvent) {
    let Some(mut event) = NodeContentEvent::from_raw(event) else {
        return;
    };
    let Some(content_handle) = event.node_content_handle() else {
        return;
    };
    let user_data = unsafe { OH_ArkUI_NodeContent_GetUserData(content_handle.raw()) };
    if user_data.is_null() {
        return;
    }
    let callback = unsafe { &*(user_data as *mut NodeContentCallbackContext) };
    (callback.callback)(&mut event);
}

fn node_content_register_callback_raw(
    content: ArkUI_NodeContentHandle,
    callback: ArkUI_NodeContentCallback,
) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(OH_ArkUI_NodeContent_RegisterCallback(content, callback)) }
}

fn node_content_set_user_data_raw(
    content: ArkUI_NodeContentHandle,
    user_data: *mut c_void,
) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(OH_ArkUI_NodeContent_SetUserData(content, user_data)) }
}
