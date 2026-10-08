use ohos_input_method_sys::{
    InputMethod_Direction, InputMethod_EnterKeyType, InputMethod_ExtendAction,
    InputMethod_KeyboardStatus, InputMethod_PrivateCommand, InputMethod_TextConfig,
    InputMethod_TextEditorProxy,
};

use crate::{
    private_command::PrivateCommand, Action, Direction, EnterKey, KeyboardStatus, Selection,
    TextConfig,
};

mod callbacks;

pub(crate) use callbacks::{
    callbacks_for, register_callbacks, unregister_callbacks, IMECallbacks, SharedCallbacks,
};

macro_rules! editor_callback {
    ($editor:expr, $field:ident) => {
        callbacks_for($editor).and_then(|callbacks| {
            let guard = callbacks.read().ok()?;
            guard.$field.clone()
        })
    };
}

fn char16_ptr_to_string(ptr: *const u16, length: usize) -> String {
    if ptr.is_null() || length == 0 {
        return String::new();
    }
    String::from_utf16_lossy(unsafe { std::slice::from_raw_parts(ptr, length) })
}

pub unsafe extern "C" fn delete_backward(editor: *mut InputMethod_TextEditorProxy, len: i32) {
    if let Some(callback) = editor_callback!(editor, delete_backward) {
        callback(len);
    }
}

pub unsafe extern "C" fn insert_text(
    editor: *mut InputMethod_TextEditorProxy,
    text: *const u16,
    len: usize,
) {
    if let Some(callback) = editor_callback!(editor, insert_text) {
        callback(char16_ptr_to_string(text, len));
    }
}

pub unsafe extern "C" fn delete_forward(editor: *mut InputMethod_TextEditorProxy, len: i32) {
    if let Some(callback) = editor_callback!(editor, delete_forward) {
        callback(len);
    }
}

pub unsafe extern "C" fn finish_text_preview(editor: *mut InputMethod_TextEditorProxy) {
    if let Some(callback) = editor_callback!(editor, finish_text_preview) {
        callback();
    }
}

pub unsafe extern "C" fn get_left_text_of_cursor(
    editor: *mut InputMethod_TextEditorProxy,
    number: i32,
    text: *mut u16,
    len: *mut usize,
) {
    let Some(callback) = editor_callback!(editor, get_left_text_of_cursor) else {
        if !len.is_null() {
            *len = 0;
        }
        return;
    };
    let utf16: Vec<u16> = callback(number).encode_utf16().collect();
    if !text.is_null() && !len.is_null() && *len >= utf16.len() {
        std::ptr::copy_nonoverlapping(utf16.as_ptr(), text, utf16.len());
        *len = utf16.len();
    } else if !len.is_null() {
        *len = 0;
    }
}

pub unsafe extern "C" fn get_right_text_of_cursor(
    editor: *mut InputMethod_TextEditorProxy,
    number: i32,
    text: *mut u16,
    len: *mut usize,
) {
    let Some(callback) = editor_callback!(editor, get_right_text_of_cursor) else {
        if !len.is_null() {
            *len = 0;
        }
        return;
    };
    let utf16: Vec<u16> = callback(number).encode_utf16().collect();
    if !text.is_null() && !len.is_null() && *len >= utf16.len() {
        std::ptr::copy_nonoverlapping(utf16.as_ptr(), text, utf16.len());
        *len = utf16.len();
    } else if !len.is_null() {
        *len = 0;
    }
}

pub unsafe extern "C" fn get_text_config(
    editor: *mut InputMethod_TextEditorProxy,
    config: *mut InputMethod_TextConfig,
) {
    if let Some(callback) = editor_callback!(editor, get_text_config) {
        // The service owns this config and its cursor. The callback only fills
        // it; dropping these wrappers must not destroy either native object.
        callback(TextConfig {
            raw: config,
            owned: false,
        });
    }
}

pub unsafe extern "C" fn get_text_index_at_cursor(editor: *mut InputMethod_TextEditorProxy) -> i32 {
    editor_callback!(editor, get_text_index_at_cursor).map_or(0, |callback| callback())
}

pub unsafe extern "C" fn handle_extend_action(
    editor: *mut InputMethod_TextEditorProxy,
    action: InputMethod_ExtendAction,
) {
    if let Some(callback) = editor_callback!(editor, handle_extend_action) {
        callback(Action::from(action));
    }
}

pub unsafe extern "C" fn handle_set_selection(
    editor: *mut InputMethod_TextEditorProxy,
    start: i32,
    end: i32,
) {
    if let Some(callback) = editor_callback!(editor, handle_set_selection) {
        callback(Selection { start, end });
    }
}

pub unsafe extern "C" fn move_cursor(
    editor: *mut InputMethod_TextEditorProxy,
    direction: InputMethod_Direction,
) {
    if let Some(callback) = editor_callback!(editor, move_cursor) {
        callback(Direction::from(direction));
    }
}

pub unsafe extern "C" fn receive_private_command(
    editor: *mut InputMethod_TextEditorProxy,
    command: *mut *mut InputMethod_PrivateCommand,
    len: usize,
) -> i32 {
    let Some(callback) = editor_callback!(editor, receive_private_command) else {
        return 0;
    };
    let commands = std::slice::from_raw_parts(command, len)
        .iter()
        .copied()
        .map(|raw| PrivateCommand { raw })
        .collect();
    callback(commands);
    0
}

pub unsafe extern "C" fn send_enter_key(
    editor: *mut InputMethod_TextEditorProxy,
    enter_key_type: InputMethod_EnterKeyType,
) {
    if let Some(callback) = editor_callback!(editor, send_enter_key) {
        callback(EnterKey::from(enter_key_type));
    }
}

pub unsafe extern "C" fn send_keyboard_status(
    editor: *mut InputMethod_TextEditorProxy,
    keyboard_status: InputMethod_KeyboardStatus,
) {
    if let Some(callback) = editor_callback!(editor, send_keyboard_status) {
        callback(KeyboardStatus::from(keyboard_status));
    }
}

pub unsafe extern "C" fn set_preview_text(
    editor: *mut InputMethod_TextEditorProxy,
    text: *const u16,
    length: usize,
    start: i32,
    end: i32,
) -> i32 {
    if let Some(callback) = editor_callback!(editor, set_preview_text) {
        callback(char16_ptr_to_string(text, length), start, end);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ndk_text_decoder_keeps_non_bmp_characters_and_replaces_invalid_units() {
        let text: Vec<u16> = "a😀中b".encode_utf16().collect();
        assert_eq!(char16_ptr_to_string(text.as_ptr(), text.len()), "a😀中b");
        assert_eq!(char16_ptr_to_string([0xd800, 0x61].as_ptr(), 2), "�a");
        assert_eq!(char16_ptr_to_string(std::ptr::null(), 0), "");
    }

    #[test]
    fn absent_surrounding_text_callback_reports_zero_output_length() {
        let mut text = [0u16; 4];
        let mut length = text.len();
        unsafe {
            get_left_text_of_cursor(std::ptr::null_mut(), 4, text.as_mut_ptr(), &mut length);
        }
        assert_eq!(length, 0);
        length = text.len();
        unsafe {
            get_right_text_of_cursor(std::ptr::null_mut(), 4, text.as_mut_ptr(), &mut length);
        }
        assert_eq!(length, 0);
    }
}
