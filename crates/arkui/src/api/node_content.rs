//! Module api::node_content wrappers and related types.

use std::{os::raw::c_void, ptr::NonNull, rc::Rc};

use ohos_arkui_sys::{
    ArkUI_NodeContentEvent, ArkUI_NodeContentHandle, OH_ArkUI_NodeContentEvent_GetEventType,
    OH_ArkUI_NodeContentEvent_GetNodeContentHandle, OH_ArkUI_NodeContent_GetUserData,
    OH_ArkUI_NodeContent_RegisterCallback, OH_ArkUI_NodeContent_SetUserData,
};

use crate::{check_arkui_status, ArkUINode, ArkUIResult};

fn non_null_or_panic<T>(ptr: *mut T, name: &'static str) -> NonNull<T> {
    NonNull::new(ptr).unwrap_or_else(|| panic!("{name} pointer is null"))
}

/// Event payload passed to node-content callbacks.
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

    /// Returns the concrete event kind.
    pub fn event_type(&self) -> crate::NodeContentEventType {
        unsafe { OH_ArkUI_NodeContentEvent_GetEventType(self.raw()).into() }
    }

    /// Returns the `NodeContent` handle associated with this callback.
    pub fn node_content_handle(&self) -> Option<crate::ArkUIHandle> {
        let handle = unsafe { OH_ArkUI_NodeContentEvent_GetNodeContentHandle(self.raw()) };
        crate::ArkUIHandle::from_raw(handle)
    }
}

struct NodeContentCallbackContext {
    callback: Rc<dyn Fn(&mut NodeContentEvent)>,
}

/// Safe wrapper for `ArkUI_NodeContentHandle`.
pub struct NodeContent {
    handle: crate::ArkUIHandle,
    callback_context: Option<NonNull<NodeContentCallbackContext>>,
}

impl NodeContent {
    /// Wrap an existing node-content handle.
    pub fn from_handle(handle: crate::ArkUIHandle) -> Self {
        Self {
            handle,
            callback_context: None,
        }
    }

    pub(crate) fn from_raw(raw: ArkUI_NodeContentHandle) -> Option<Self> {
        crate::ArkUIHandle::from_raw(raw).map(Self::from_handle)
    }

    #[cfg(feature = "napi")]
    pub fn from_napi_handle(handle: &crate::ArkUIHandle) -> Option<Self> {
        Some(Self::from_handle(*handle))
    }

    /// Returns the underlying node-content handle wrapper.
    pub fn handle(&self) -> crate::ArkUIHandle {
        self.handle
    }

    /// Appends a child node to this content.
    pub fn add_node(&self, node: &ArkUINode) -> ArkUIResult<()> {
        self.handle.add_node(node)
    }

    /// Removes a child node from this content.
    pub fn remove_node(&self, node: &ArkUINode) -> ArkUIResult<()> {
        self.handle.remove_node(node)
    }

    /// Inserts a child node at the given position.
    pub fn insert_node(&self, node: &ArkUINode, position: i32) -> ArkUIResult<()> {
        self.handle.insert_node(node, position)
    }

    /// Registers a callback invoked when node-content events are emitted.
    /// Panics are logged and contained at the native callback boundary.
    pub fn register_callback<T: Fn(&mut NodeContentEvent) + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<()> {
        self.clear_callback()?;
        let context = Box::into_raw(Box::new(NodeContentCallbackContext {
            callback: Rc::new(callback),
        }));
        let context = non_null_or_panic(context, "NodeContentCallbackContext");
        if let Err(err) = self
            .handle
            .set_node_content_user_data(context.as_ptr().cast())
        {
            // SAFETY: native registration has not acquired the allocated box.
            unsafe {
                drop(Box::from_raw(context.as_ptr()));
            }
            return Err(err);
        }
        self.callback_context = Some(context);
        let register_result = self
            .handle
            .register_node_content_callback(Some(node_content_callback_trampoline));
        if let Err(err) = register_result {
            // If reset fails, retain the context for a later cleanup attempt;
            // a previously installed trampoline could still read this data.
            if let Err(error) = self.clear_callback() {
                log::error!(
                    "NodeContent callback cleanup failed; retaining native context: {error:?}"
                );
            }
            return Err(err);
        }
        Ok(())
    }

    /// Unregisters the callback and releases callback resources.
    pub fn clear_callback(&mut self) -> ArkUIResult<()> {
        if self.callback_context.is_none() {
            return Ok(());
        }
        // RegisterCallback rejects a null callback. Keep the static trampoline
        // installed and remove its user data before releasing the closure.
        self.handle
            .set_node_content_user_data(std::ptr::null_mut())?;
        if let Some(context) = self.callback_context.take() {
            // SAFETY: registration allocated this box; native code can no
            // longer obtain it after the successful user-data reset.
            unsafe {
                drop(Box::from_raw(context.as_ptr()));
            }
        }

        Ok(())
    }
}

impl Drop for NodeContent {
    fn drop(&mut self) {
        // If native code refuses retirement, leave its context allocated so a
        // later callback cannot dereference freed Rust state.
        if let Err(error) = self.clear_callback() {
            log::error!("NodeContent Drop cleanup failed; retaining native context: {error:?}");
        }
    }
}

impl crate::ArkUIHandle {
    pub(crate) fn register_node_content_callback(
        &self,
        callback: Option<unsafe extern "C" fn(event: *mut ArkUI_NodeContentEvent)>,
    ) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_NodeContent_RegisterCallback(self.raw(), callback)) }
    }

    pub(crate) fn set_node_content_user_data(&self, user_data: *mut c_void) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_NodeContent_SetUserData(self.raw(), user_data)) }
    }

    pub(crate) fn node_content_user_data(&self) -> *mut c_void {
        unsafe { OH_ArkUI_NodeContent_GetUserData(self.raw()) }
    }
}

unsafe extern "C" fn node_content_callback_trampoline(event: *mut ArkUI_NodeContentEvent) {
    let Some(mut event) = NodeContentEvent::from_raw(event) else {
        return;
    };
    let Some(content_handle) = event.node_content_handle() else {
        return;
    };
    let user_data = content_handle.node_content_user_data();
    if user_data.is_null() {
        return;
    }
    // SAFETY: user data is installed by register_callback and is live on this
    // UI thread. Clone before calling so clearing from the callback is safe.
    let callback = unsafe { &*(user_data as *mut NodeContentCallbackContext) }
        .callback
        .clone();
    // A user panic must not unwind across ArkUI's native callback boundary.
    if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| callback(&mut event))).is_err() {
        log::error!("NodeContent callback panicked");
    }
}
