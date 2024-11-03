use napi_ohos::{bindgen_prelude::check_status, Env, JsUnknown, NapiRaw, Result};
use ohos_arkui_sys::{ArkUI_NodeContentHandle, OH_ArkUI_GetNodeContentFromNapiValue};
use std::ptr;

pub struct NativeEntry {
    handle: ArkUI_NodeContentHandle,
    env: Env,
}

impl NativeEntry {
    /// Accept Content Slot from JS thread   
    /// ```ts
    /// import { NodeContent } from '@kit.ArkUI';
    /// // content_slot should be rootSlot
    /// private rootSlot = new NodeContent();
    /// ```
    pub fn new(env: Env, content_slot: JsUnknown) -> Result<Self> {
        let mut slot = ptr::null_mut();
        unsafe {
            check_status!(
                OH_ArkUI_GetNodeContentFromNapiValue(env.raw(), content_slot.raw(), &mut slot),
                "Get Node Content Slot failed."
            )?
        };
        Ok(NativeEntry { handle: slot, env })
    }
}

impl Drop for NativeEntry {
    fn drop(&mut self) {}
}
