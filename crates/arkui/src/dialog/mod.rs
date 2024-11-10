use crate::{ArkUIResult, ARK_UI_NATIVE_DIALOG_API_1};

use ohos_arkui_sys::ArkUI_NativeDialogHandle;

pub struct Dialog(pub(crate) ArkUI_NativeDialogHandle);

impl Dialog {
    pub fn new() -> ArkUIResult<Self> {
        let dialog_controller = ARK_UI_NATIVE_DIALOG_API_1.create()?;
        Ok(Dialog(dialog_controller))
    }
}
