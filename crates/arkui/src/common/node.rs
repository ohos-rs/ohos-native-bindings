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
    /// Event callbacks bound to this node.
    pub(crate) event_handle: EventHandle,
    /// Whether this wrapper owns the reference acquired by `createNode`.
    /// Nodes obtained from query APIs are borrowed and must never decrement
    /// the native reference count.
    pub(crate) owns_raw: bool,
}

impl ArkUINode {
    /// Returns the native ArkUI node handle.
    pub fn raw_handle(&self) -> ArkUI_NodeHandle {
        self.raw
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
            event_handle: self.event_handle.clone(),
            owns_raw: false,
        }
    }

    pub fn from_raw_handle(raw: ArkUI_NodeHandle) -> Option<Self> {
        if raw.is_null() {
            return None;
        }

        Some(Self {
            raw,
            tag: ArkUINodeType::Custom,
            children: vec![],
            event_handle: Default::default(),
            owns_raw: false,
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

        while let Some(child) = self.children.last().cloned() {
            ARK_UI_NATIVE_NODE_API_1.with(|api| api.remove_child(self, &child.borrow()))?;
            self.children.pop();
            if child.borrow().is_owned() {
                child.borrow_mut().dispose()?;
            }
        }

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

/// This implementation just for event and animation to use it.
/// When you need to create a new node, you should add raw and tag at the same time.
impl Default for ArkUINode {
    fn default() -> Self {
        Self {
            raw: std::ptr::null_mut(),
            tag: ArkUINodeType::Custom,
            children: vec![],
            event_handle: Default::default(),
            owns_raw: false,
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
            event_handle: Default::default(),
            owns_raw,
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
