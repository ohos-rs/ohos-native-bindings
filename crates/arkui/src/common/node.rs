use napi_ohos::bindgen_prelude::{check_status, FromNapiValue, TypeName, ValidateNapiValue};
use napi_sys_ohos as sys;
use ohos_arkui_sys::{ArkUI_NodeHandle, OH_ArkUI_GetNodeHandleFromNapiValue};
use ohos_hilog_binding::hilog_info;
use std::ptr;

use crate::ArkUINodeType;

use super::ARK_UI_NATIVE_NODE_API_1;

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

    /// Clear dom
    /// We can't use drop impl, because it will be called when the object is dropped.
    pub fn dispose(&mut self) {
        ARK_UI_NATIVE_NODE_API_1.dispose(self);
        self.children.iter_mut().for_each(|child| {
            child.dispose();
        });
        self.children.clear();
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
