use napi_ohos::bindgen_prelude::{check_status, FromNapiValue, TypeName, ValidateNapiValue};
use napi_sys_ohos as sys;
use ohos_arkui_sys::{ArkUI_NodeHandle, OH_ArkUI_GetNodeHandleFromNapiValue};
use std::ptr;

use crate::ArkUINodeType;

#[derive(Clone)]
pub struct ArkUINode {
    pub(crate) raw: ArkUI_NodeHandle,
    pub(crate) tag: ArkUINodeType,
    pub(crate) children: Vec<Box<ArkUINode>>,
}

impl ArkUINode {
    pub fn children(&self) -> &[Box<ArkUINode>] {
        self.children.as_slice()
    }

    pub fn children_mut(&mut self) -> &mut Vec<Box<ArkUINode>> {
        self.children.as_mut()
    }

    pub fn raw(&self) -> ArkUI_NodeHandle {
        self.raw
    }

    pub fn remove_child(&mut self, index: usize) -> Option<Box<ArkUINode>> {
        if index < self.children().len() {
            Some(self.children_mut().remove(index))
        } else {
            None
        }
    }

    pub fn add_child(&mut self, child: Box<ArkUINode>) {
        self.children_mut().push(child);
    }

    pub fn insert_child(&mut self, child: Box<ArkUINode>, index: usize) {
        self.children_mut().insert(index, child);
    }
}

/// Convert ArkUI node to native node
pub struct ArkUINodeRaw {
    pub(crate) env: sys::napi_env,
    pub(crate) value: sys::napi_value,
    pub raw: ArkUI_NodeHandle,
}

impl TypeName for ArkUINodeRaw {
    fn type_name() -> &'static str {
        "ArkUINode"
    }
    fn value_type() -> napi_ohos::ValueType {
        napi_ohos::ValueType::Object
    }
}

impl ValidateNapiValue for ArkUINodeRaw {}

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
