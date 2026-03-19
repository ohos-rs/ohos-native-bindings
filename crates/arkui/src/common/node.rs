//! Module common::node wrappers and related types.

#[cfg(feature = "napi")]
use napi_ohos::bindgen_prelude::{check_status, FromNapiValue, TypeName, ValidateNapiValue};
#[cfg(feature = "napi")]
use napi_sys_ohos as sys;
use ohos_arkui_input_binding::sys::ArkUI_NodeHandle;

#[cfg(feature = "napi")]
use ohos_arkui_sys::OH_ArkUI_GetNodeHandleFromNapiValue;
#[cfg(feature = "napi")]
use std::ptr;

use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use crate::{ArkUINodeType, EventHandle, ARK_UI_NATIVE_NODE_API_1};

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

    pub(crate) fn from_raw_handle(raw: ArkUI_NodeHandle) -> Option<Self> {
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

    /// Clear dom
    /// We can't use drop impl, because it will be called when the object is dropped.
    pub fn dispose(&mut self) -> ArkUIResult<()> {
        let handle = &self.event_handle;
        if handle.has_callback() {
            ARK_UI_NATIVE_NODE_API_1.with(|api| api.remove_event_receiver(self))?;
        }
        ARK_UI_NATIVE_NODE_API_1.with(|api| api.dispose(self))?;
        for mut child in self.children.iter() {
            let child_ref = child.borrow_mut();
            child_ref.take().dispose()?;
        }
        self.children.clear();
        Ok(())
    }
}

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
