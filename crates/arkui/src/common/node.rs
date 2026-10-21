//! Module common::node wrappers and related types.

#[cfg(feature = "napi")]
use napi_ohos::bindgen_prelude::{check_status, FromNapiValue, TypeName, ValidateNapiValue};
#[cfg(feature = "napi")]
use napi_sys_ohos as sys;
#[cfg(all(feature = "accessibility", feature = "api-23"))]
use ohos_accessibility_binding::Provider;
use ohos_arkui_input_binding::{sys::ArkUI_NodeHandle, ArkUIErrorCode};
use ohos_arkui_sys::{
    ArkUI_IntOffset, ArkUI_IntSize, OH_ArkUI_GetContextByNode,
    OH_ArkUI_NodeUtils_GetLayoutPositionInWindow, OH_ArkUI_NodeUtils_GetLayoutSize,
    OH_ArkUI_NodeUtils_GetPositionWithTranslateInWindow,
};
#[cfg(feature = "xcomponent")]
use ohos_xcomponent_binding::{NativeXComponent, XComponentRaw};
#[cfg(feature = "xcomponent")]
use ohos_xcomponent_sys::OH_NativeXComponent_GetNativeXComponent;

#[cfg(feature = "napi")]
use ohos_arkui_sys::OH_ArkUI_GetNodeHandleFromNapiValue;
#[cfg(feature = "napi")]
use std::ptr;

use std::{cell::RefCell, rc::Rc};

use crate::{
    animate::options::Animation,
    api::node_custom_event::{IntOffset, IntSize},
    api::ARK_UI_NATIVE_ANIMATE_API_1,
    check_arkui_status, ArkUIAttributeBasic, ArkUICommonAttribute, ArkUIError, ArkUINodeType,
    EventHandle, NodeDirtyFlag, ARK_UI_NATIVE_NODE_API_1,
};

use super::ArkUIResult;

/// High-level ArkUI node wrapper used by component APIs.
pub struct ArkUINode {
    /// Underlying native ArkUI node handle.
    pub(crate) raw: ArkUI_NodeHandle,
    /// Node type tag.
    pub(crate) tag: ArkUINodeType,
    /// Child nodes owned by this node in wrapper layer.
    pub(crate) children: Vec<Rc<RefCell<ArkUINode>>>,
    pub(crate) pending_children: Vec<Rc<RefCell<ArkUINode>>>,
    /// Event callbacks bound to this node.
    pub(crate) event_handle: EventHandle,
    /// Whether this wrapper owns the reference acquired by `createNode`.
    /// Nodes obtained from query APIs are borrowed and must never decrement
    /// the native reference count.
    pub(crate) owns_raw: bool,
    pub(crate) retained: super::node_retention::NativeRetentions,
}

impl ArkUINode {
    pub(crate) fn retain_native(&self) -> ArkUIResult<super::node_retention::NodeRetention> {
        if !self.owns_raw || self.raw.is_null() {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "native retention requires an owned node",
            ));
        }
        self.retained.retain().ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "native node retention count exhausted",
            )
        })
    }
    /// Synchronously requests focus for this live node (API 15+).
    #[cfg(feature = "api-15")]
    pub fn request_focus(&self) -> ArkUIResult<()> {
        check_arkui_status!(unsafe { ohos_arkui_sys::OH_ArkUI_FocusRequest(self.raw) })
    }

    /// Returns the native ArkUI node handle.
    pub fn raw_handle(&self) -> ArkUI_NodeHandle {
        self.raw
    }

    /// Returns the native XComponent represented by this ArkUI node.
    ///
    /// Non-XComponent nodes and nodes whose native surface has already been
    /// released return `None`.
    #[cfg(feature = "xcomponent")]
    pub fn native_xcomponent(&self) -> Option<NativeXComponent> {
        let raw = unsafe { OH_NativeXComponent_GetNativeXComponent(self.raw) };
        std::ptr::NonNull::new(raw).map(|raw| NativeXComponent::new(XComponentRaw(raw.as_ptr())))
    }

    /// Obtain the accessibility provider associated with this custom node.
    ///
    /// ArkUI only supports this operation for `ARKUI_NODE_CUSTOM` nodes.
    #[cfg(all(feature = "accessibility", feature = "api-23"))]
    pub fn accessibility_provider(&self) -> ohos_accessibility_binding::Result<Provider<'_>> {
        let mut raw = self.raw;
        unsafe { Provider::from_node_handle(&mut raw) }
    }

    /// Immutable children view.
    pub fn children(&self) -> &[Rc<RefCell<ArkUINode>>] {
        self.children.as_slice()
    }

    /// Mutable children view.
    pub fn children_mut(&mut self) -> &mut Vec<Rc<RefCell<ArkUINode>>> {
        self.children.as_mut()
    }

    /// Whether this wrapper owns the reference returned by `createNode`.
    pub fn is_owned(&self) -> bool {
        self.owns_raw
    }

    pub(crate) fn raw(&self) -> ArkUI_NodeHandle {
        self.raw
    }

    /// Create a non-owning view of the same native handle.
    ///
    /// Compatibility wrappers such as `XComponent` use this for read/mutate
    /// access from multiple application handles. The returned view can never
    /// release the `createNode` reference; ownership stays with `self`.
    pub(crate) fn borrowed_clone(&self) -> Self {
        Self {
            raw: self.raw,
            tag: self.tag,
            children: self.children.clone(),
            pending_children: self.pending_children.clone(),
            event_handle: self.event_handle.clone(),
            owns_raw: false,
            retained: self.retained.clone(),
        }
    }

    /// Borrows a native node without acquiring its create reference.
    ///
    /// # Safety
    /// A non-null handle must remain live on its owning UI thread until the
    /// returned wrapper and every attachment using it have been discarded.
    ///
    /// ```compile_fail
    /// use ohos_arkui_binding::common::node::ArkUINode;
    /// let node = ArkUINode::from_raw_handle(std::ptr::null_mut());
    /// ```
    pub unsafe fn from_raw_handle(raw: ArkUI_NodeHandle) -> Option<Self> {
        if raw.is_null() {
            return None;
        }

        Some(Self {
            raw,
            tag: ArkUINodeType::Custom,
            children: vec![],
            pending_children: vec![],
            event_handle: Default::default(),
            owns_raw: false,
            retained: Default::default(),
        })
    }

    #[cfg(all(feature = "api-22", feature = "drawing"))]
    pub fn text_layout_manager(&self) -> ArkUIResult<Option<crate::TextLayoutManager>> {
        match self.get_attribute(crate::ArkUINodeAttributeType::TextLayoutManager)? {
            crate::ArkUINodeAttributeItem::Object(ptr) => {
                Ok(Some(crate::TextLayoutManager::from_raw(ptr.cast())))
            }
            _ => Ok(None),
        }
    }

    fn invalidate_owned_handle(&mut self) {
        self.owns_raw = false;
        self.raw = std::ptr::null_mut();
        self.event_handle = Default::default();
    }

    /// Release this wrapper-owned native subtree using ArkUI's documented
    /// child-before-parent lifecycle.
    ///
    /// The root must already be detached from its external parent. Each direct
    /// child edge is removed before that child is recursively disposed; only
    /// then is this node's own `createNode` reference released. Borrowed
    /// children are detached but never disposed by this wrapper.
    pub fn dispose(&mut self) -> ArkUIResult<()> {
        if self.retained.is_retained() {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "cannot dispose a node with active native owners",
            ));
        }
        if !self.owns_raw {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "cannot dispose a borrowed or already disposed ArkUI node",
            ));
        }
        if ARK_UI_NATIVE_NODE_API_1
            .with(|api| api.get_parent(self))?
            .is_some()
        {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "cannot dispose an ArkUI node before removing it from its parent",
            ));
        }

        let parent = self.borrowed_clone();
        super::child_retirement::retire(
            &mut self.children,
            &mut self.pending_children,
            |child| ARK_UI_NATIVE_NODE_API_1.with(|api| api.remove_child(&parent, &child.borrow())),
            |child| {
                if child.borrow().is_owned() {
                    child.borrow_mut().dispose()?;
                }
                Ok(())
            },
        )?;

        let registered_events = self
            .event_handle
            .callbacks
            .iter()
            .map(|(event_type, _)| *event_type)
            .collect::<Vec<_>>();
        for event_type in registered_events {
            let _ =
                ARK_UI_NATIVE_NODE_API_1.with(|api| api.unregister_node_event(self, event_type));
        }
        let _ = ARK_UI_NATIVE_NODE_API_1.with(|api| api.remove_event_receiver(self));
        crate::common::node_registry::unregister(self);
        ARK_UI_NATIVE_NODE_API_1.with(|api| api.dispose(self))?;
        self.invalidate_owned_handle();
        Ok(())
    }

    /// Runs an explicit ArkUI animation update against this node.
    pub fn animate_to(&self, animation: &Animation) -> ArkUIResult<()> {
        let context = unsafe { OH_ArkUI_GetContextByNode(self.raw()) };
        if context.is_null() {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_GetContextByNode returned null",
            ));
        }

        let update_ctx_raw = animation.update_ctx.borrow().raw();
        let finish_ctx_raw = animation.finish_ctx.borrow().raw();
        ARK_UI_NATIVE_ANIMATE_API_1
            .with(|api| api.animate_to(context, animation.raw(), update_ctx_raw, finish_ctx_raw))
    }

    /// Returns the layout size measured for this node.
    pub fn layout_size(&self) -> ArkUIResult<IntSize> {
        let mut size: ArkUI_IntSize = unsafe { std::mem::zeroed() };
        unsafe { check_arkui_status!(OH_ArkUI_NodeUtils_GetLayoutSize(self.raw(), &mut size)) }?;
        Ok(size.into())
    }

    /// Sets the unit used by numeric node attributes (for example physical pixels).
    pub fn set_length_metric_unit(
        &self,
        unit: crate::types::advanced::LengthMetricUnit,
    ) -> ArkUIResult<()> {
        ARK_UI_NATIVE_NODE_API_1.with(|api| api.set_length_metric_unit(self, unit))
    }

    /// Returns the layout position in physical screen coordinates.
    pub fn layout_position_in_screen(&self) -> ArkUIResult<IntOffset> {
        let mut offset = ArkUI_IntOffset { x: 0, y: 0 };
        unsafe {
            check_arkui_status!(
                ohos_arkui_sys::OH_ArkUI_NodeUtils_GetLayoutPositionInScreen(
                    self.raw(),
                    &mut offset
                )
            )
        }?;
        Ok(offset.into())
    }

    /// Returns the untransformed layout offset from the native parent, in pixels (API 12).
    pub fn layout_position(&self) -> ArkUIResult<IntOffset> {
        let mut offset = ArkUI_IntOffset { x: 0, y: 0 };
        // SAFETY: the live node and writable offset remain valid throughout this synchronous call.
        unsafe {
            check_arkui_status!(ohos_arkui_sys::OH_ArkUI_NodeUtils_GetLayoutPosition(
                self.raw(),
                &mut offset
            ))
        }?;
        Ok(offset.into())
    }

    /// Returns this node's layout position in the current window.
    pub fn layout_position_in_window(&self) -> ArkUIResult<IntOffset> {
        let mut offset: ArkUI_IntOffset = unsafe { std::mem::zeroed() };
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_GetLayoutPositionInWindow(
                self.raw(),
                &mut offset
            ))
        }?;
        Ok(offset.into())
    }

    /// Returns this node's translated position in the current window.
    pub fn position_with_translate_in_window(&self) -> ArkUIResult<IntOffset> {
        let mut offset: ArkUI_IntOffset = unsafe { std::mem::zeroed() };
        unsafe {
            check_arkui_status!(OH_ArkUI_NodeUtils_GetPositionWithTranslateInWindow(
                self.raw(),
                &mut offset
            ))
        }?;
        Ok(offset.into())
    }

    /// Mark this node dirty so ArkUI recomputes the requested render pipeline stage.
    pub fn mark_dirty(&self, dirty_flag: NodeDirtyFlag) -> ArkUIResult<()> {
        ARK_UI_NATIVE_NODE_API_1.with(|api| api.mark_dirty(self, dirty_flag))
    }
}

impl ArkUIAttributeBasic for ArkUINode {
    fn raw(&self) -> &ArkUINode {
        self
    }

    fn borrow_mut(&mut self) -> &mut ArkUINode {
        self
    }
}

impl ArkUICommonAttribute for ArkUINode {}

// Preserve event registration after a component has become an identity-stable
// shared node through `into_node`, just as common attribute mutation does.
impl crate::ArkUIEvent for ArkUINode {}

// Identity-stable nodes keep gesture attachment available after `into_node`.
impl crate::ArkUIGesture for ArkUINode {}

/// This implementation just for event and animation to use it.
/// When you need to create a new node, you should add raw and tag at the same time.
impl Default for ArkUINode {
    fn default() -> Self {
        Self {
            raw: std::ptr::null_mut(),
            tag: ArkUINodeType::Custom,
            children: vec![],
            pending_children: vec![],
            event_handle: Default::default(),
            owns_raw: false,
            retained: Default::default(),
        }
    }
}

#[cfg(feature = "napi")]
/// Convert ArkUI node to native node
pub struct ArkUINodeRaw {
    /// N-API environment.
    pub(crate) env: sys::napi_env,
    /// N-API value.
    pub(crate) value: sys::napi_value,
    /// Native ArkUI handle.
    pub raw: ArkUI_NodeHandle,
}

#[cfg(feature = "napi")]
impl ArkUINodeRaw {
    /// Borrows the node represented by this ArkTS value. Keep its ArkTS owner
    /// alive until the returned node is detached; this does not acquire ownership.
    ///
    /// # Safety
    /// The ArkTS owner must keep the native handle alive until this borrowed
    /// wrapper has been detached and discarded, on the owning UI thread.
    pub unsafe fn borrowed_node(&self) -> ArkUIResult<ArkUINode> {
        // SAFETY: the caller keeps this ArkTS node owner alive through retirement.
        unsafe { ArkUINode::from_raw_handle(self.raw) }.ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "ArkTS value has no native node",
            )
        })
    }
}

#[cfg(feature = "napi")]
impl TypeName for ArkUINodeRaw {
    fn type_name() -> &'static str {
        "ArkUINode"
    }
    fn value_type() -> napi_ohos::ValueType {
        napi_ohos::ValueType::Object
    }
}

#[cfg(feature = "napi")]
impl ValidateNapiValue for ArkUINodeRaw {}

#[cfg(feature = "napi")]
impl FromNapiValue for ArkUINodeRaw {
    unsafe fn from_napi_value(
        env: sys::napi_env,
        napi_val: sys::napi_value,
    ) -> napi_ohos::Result<Self> {
        let mut slot = ptr::null_mut();
        unsafe {
            check_status!(
                OH_ArkUI_GetNodeHandleFromNapiValue(env, napi_val, &mut slot),
                "Get Node failed."
            )?
        };
        Ok(ArkUINodeRaw {
            env,
            value: napi_val,
            raw: slot,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_node(id: usize, owns_raw: bool) -> ArkUINode {
        ArkUINode {
            raw: id as ArkUI_NodeHandle,
            tag: ArkUINodeType::Custom,
            children: Vec::new(),
            pending_children: Vec::new(),
            event_handle: Default::default(),
            owns_raw,
            retained: Default::default(),
        }
    }

    #[test]
    fn invalidating_an_owned_handle_is_local() {
        let child = Rc::new(RefCell::new(fake_node(2, true)));
        let mut root = fake_node(1, true);
        root.children.push(child.clone());

        root.invalidate_owned_handle();

        assert!(root.raw_handle().is_null());
        assert!(!root.is_owned());
        assert_eq!(child.borrow().raw_handle() as usize, 2);
        assert!(child.borrow().is_owned());
    }

    #[test]
    fn borrowed_node_cannot_release_a_foreign_handle() {
        let mut node = fake_node(7, false);
        assert!(node.dispose().is_err());
        assert_eq!(node.raw_handle() as usize, 7);
    }
}
