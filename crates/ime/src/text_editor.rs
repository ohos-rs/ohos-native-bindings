use ohos_input_method_sys::{
    InputMethod_TextEditorProxy, OH_TextEditorProxy_Create, OH_TextEditorProxy_Destroy,
    OH_TextEditorProxy_SetDeleteBackwardFunc, OH_TextEditorProxy_SetDeleteForwardFunc,
    OH_TextEditorProxy_SetFinishTextPreviewFunc, OH_TextEditorProxy_SetGetLeftTextOfCursorFunc,
    OH_TextEditorProxy_SetGetRightTextOfCursorFunc, OH_TextEditorProxy_SetGetTextConfigFunc,
    OH_TextEditorProxy_SetGetTextIndexAtCursorFunc, OH_TextEditorProxy_SetHandleExtendActionFunc,
    OH_TextEditorProxy_SetHandleSetSelectionFunc, OH_TextEditorProxy_SetInsertTextFunc,
    OH_TextEditorProxy_SetMoveCursorFunc, OH_TextEditorProxy_SetReceivePrivateCommandFunc,
    OH_TextEditorProxy_SetSendEnterKeyFunc, OH_TextEditorProxy_SetSendKeyboardStatusFunc,
    OH_TextEditorProxy_SetSetPreviewTextFunc,
};

use crate::proxy::{
    delete_backward, delete_forward, finish_text_preview, get_left_text_of_cursor,
    get_right_text_of_cursor, get_text_config, get_text_index_at_cursor, handle_extend_action,
    handle_set_selection, insert_text, move_cursor, receive_private_command, send_enter_key,
    send_keyboard_status, set_preview_text,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TextEditor {
    pub(crate) raw: *mut InputMethod_TextEditorProxy,
}

impl TextEditor {
    pub fn new() -> Self {
        unsafe {
            let raw = OH_TextEditorProxy_Create();
            let status = OH_TextEditorProxy_SetDeleteBackwardFunc(raw, Some(delete_backward));
            #[cfg(debug_assertions)]
            assert!(status == 0, "Failed to set delete backward function");

            let status = OH_TextEditorProxy_SetDeleteForwardFunc(raw, Some(delete_forward));
            #[cfg(debug_assertions)]
            assert!(status == 0, "Failed to set delete forward function");

            let status =
                OH_TextEditorProxy_SetFinishTextPreviewFunc(raw, Some(finish_text_preview));
            #[cfg(debug_assertions)]
            assert!(status == 0, "Failed to set finish text preview function");

            let status =
                OH_TextEditorProxy_SetGetLeftTextOfCursorFunc(raw, Some(get_left_text_of_cursor));
            #[cfg(debug_assertions)]
            assert!(
                status == 0,
                "Failed to set get left text of cursor function"
            );

            let status =
                OH_TextEditorProxy_SetGetRightTextOfCursorFunc(raw, Some(get_right_text_of_cursor));
            #[cfg(debug_assertions)]
            assert!(
                status == 0,
                "Failed to set get right text of cursor function"
            );

            let status = OH_TextEditorProxy_SetGetTextConfigFunc(raw, Some(get_text_config));
            #[cfg(debug_assertions)]
            assert!(status == 0, "Failed to set get text config function");

            let status =
                OH_TextEditorProxy_SetGetTextIndexAtCursorFunc(raw, Some(get_text_index_at_cursor));
            #[cfg(debug_assertions)]
            assert!(
                status == 0,
                "Failed to set get text index at cursor function"
            );

            let status =
                OH_TextEditorProxy_SetHandleExtendActionFunc(raw, Some(handle_extend_action));
            #[cfg(debug_assertions)]
            assert!(status == 0, "Failed to set handle extend action function");

            let status =
                OH_TextEditorProxy_SetHandleSetSelectionFunc(raw, Some(handle_set_selection));
            #[cfg(debug_assertions)]
            assert!(status == 0, "Failed to set handle set selection function");

            let status = OH_TextEditorProxy_SetInsertTextFunc(raw, Some(insert_text));
            #[cfg(debug_assertions)]
            assert!(status == 0, "Failed to set insert text function");

            let status = OH_TextEditorProxy_SetMoveCursorFunc(raw, Some(move_cursor));
            #[cfg(debug_assertions)]
            assert!(status == 0, "Failed to set move cursor function");

            let status =
                OH_TextEditorProxy_SetReceivePrivateCommandFunc(raw, Some(receive_private_command));
            #[cfg(debug_assertions)]
            assert!(
                status == 0,
                "Failed to set receive private command function"
            );

            let status = OH_TextEditorProxy_SetSendEnterKeyFunc(raw, Some(send_enter_key));
            #[cfg(debug_assertions)]
            assert!(status == 0, "Failed to set send enter key function");

            let status =
                OH_TextEditorProxy_SetSendKeyboardStatusFunc(raw, Some(send_keyboard_status));
            #[cfg(debug_assertions)]
            assert!(status == 0, "Failed to set send keyboard status function");

            let status = OH_TextEditorProxy_SetSetPreviewTextFunc(raw, Some(set_preview_text));
            #[cfg(debug_assertions)]
            assert!(status == 0, "Failed to set set preview text function");

            TextEditor { raw }
        }
    }
}

impl Drop for TextEditor {
    fn drop(&mut self) {
        unsafe {
            OH_TextEditorProxy_Destroy(self.raw);
        }
    }
}

impl Default for TextEditor {
    fn default() -> Self {
        Self::new()
    }
}
