use ohos_input_method_sys::{
    InputMethod_TextConfig, OH_TextConfig_Create, OH_TextConfig_Destroy,
    OH_TextConfig_GetCursorInfo, OH_TextConfig_GetEnterKeyType, OH_TextConfig_GetInputType,
    OH_TextConfig_GetSelection, OH_TextConfig_GetWindowId, OH_TextConfig_IsPreviewTextSupported,
    OH_TextConfig_SetEnterKeyType, OH_TextConfig_SetInputType, OH_TextConfig_SetPreviewTextSupport,
    OH_TextConfig_SetSelection, OH_TextConfig_SetWindowId,
};
use std::ptr;

use crate::{Cursor, EnterKey, InputType};

pub struct TextConfig {
    pub(crate) raw: *mut InputMethod_TextConfig,
}

#[derive(Debug, Clone, Copy)]
pub struct Selection {
    pub start: i32,
    pub end: i32,
}

impl TextConfig {
    pub fn new() -> Self {
        let raw = unsafe { OH_TextConfig_Create() };
        TextConfig { raw }
    }

    pub fn cursor(&self) -> Cursor {
        let mut cursor_raw = ptr::null_mut();
        unsafe { OH_TextConfig_GetCursorInfo(self.raw, &mut cursor_raw) };
        Cursor { raw: cursor_raw }
    }

    pub fn enter_key(&self) -> EnterKey {
        let mut raw_key = 0;
        unsafe { OH_TextConfig_GetEnterKeyType(self.raw, &mut raw_key) };
        EnterKey::from(raw_key)
    }

    pub fn set_enter_key(&self, key: EnterKey) {
        unsafe { OH_TextConfig_SetEnterKeyType(self.raw, key.into()) };
    }

    pub fn input_type(&self) -> InputType {
        let mut raw_type = 0;
        unsafe { OH_TextConfig_GetInputType(self.raw, &mut raw_type) };
        InputType::from(raw_type)
    }

    pub fn set_input_type(&self, input_type: InputType) {
        unsafe { OH_TextConfig_SetInputType(self.raw, input_type.into()) };
    }

    pub fn selection(&self) -> Selection {
        let mut start = 0;
        let mut end = 0;
        unsafe { OH_TextConfig_GetSelection(self.raw, &mut start, &mut end) };
        Selection { start, end }
    }

    pub fn set_selection(&self, selection: Selection) {
        unsafe { OH_TextConfig_SetSelection(self.raw, selection.start, selection.end) };
    }

    pub fn window_id(&self) -> i32 {
        let mut id = 0;
        unsafe { OH_TextConfig_GetWindowId(self.raw, &mut id) };
        id
    }

    pub fn set_window_id(&self, id: i32) {
        unsafe { OH_TextConfig_SetWindowId(self.raw, id) };
    }

    pub fn is_preview_text_supported(&self) -> bool {
        let mut is_supported = false;
        unsafe { OH_TextConfig_IsPreviewTextSupported(self.raw, &mut is_supported) };
        is_supported
    }

    pub fn set_preview_text_supported(&self, supported: bool) {
        unsafe { OH_TextConfig_SetPreviewTextSupport(self.raw, supported) };
    }
}

impl Drop for TextConfig {
    fn drop(&mut self) {
        unsafe {
            OH_TextConfig_Destroy(self.raw);
        }
    }
}

impl Default for TextConfig {
    fn default() -> Self {
        Self::new()
    }
}
