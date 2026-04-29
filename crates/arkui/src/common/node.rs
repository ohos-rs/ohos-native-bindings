//! Module common::node wrappers and related types.

#[cfg(feature = "napi")]
use napi_ohos::bindgen_prelude::{check_status, FromNapiValue, TypeName, ValidateNapiValue};
#[cfg(feature = "napi")]
use napi_sys_ohos as sys;
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

#[derive(Clone)]
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
}

impl ArkUINode {
    /// Returns the native ArkUI node handle.
    pub fn raw_handle(&self) -> ArkUI_NodeHandle {
        self.raw
    }

    /// Immutable children view.
    pub fn children(&self) -> &[Rc<RefCell<ArkUINode>>] {
        self.children.as_slice()
    }

    /// Mutable children view.
    pub fn children_mut(&mut self) -> &mut Vec<Rc<RefCell<ArkUINode>>> {
        self.children.as_mut()
    }

    pub(crate) fn raw(&self) -> ArkUI_NodeHandle {
        self.raw
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

    /// Clear dom
    /// We can't use drop impl, because it will be called when the object is dropped.
    pub fn dispose(&mut self) -> ArkUIResult<()> {
        let handle = &self.event_handle;
        if handle.has_callback() {
            ARK_UI_NATIVE_NODE_API_1.with(|api| api.remove_event_receiver(self))?;
        }
        // `disposeNode` tears down the native subtree. Disposing wrapper children again will
        // double free the descendant handles during patch/remount flows.
        ARK_UI_NATIVE_NODE_API_1.with(|api| api.dispose(self))?;
        self.children.clear();
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
