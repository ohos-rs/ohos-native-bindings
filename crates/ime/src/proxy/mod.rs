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
    let mut result = String::new();

    unsafe {
        let slice = std::slice::from_raw_parts(ptr, length);

        for &unit in slice {
            if let Some(Ok(c)) = char::decode_utf16(std::iter::once(unit)).next() {
                result.push(c);
            }
        }
    }

    result
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
        return;
    };
    let utf16: Vec<u16> = callback(number).encode_utf16().collect();
    if !text.is_null() && !len.is_null() && *len >= utf16.len() {
        std::ptr::copy_nonoverlapping(utf16.as_ptr(), text, utf16.len());
        *len = utf16.len();
    }
}

pub unsafe extern "C" fn get_right_text_of_cursor(
    editor: *mut InputMethod_TextEditorProxy,
    number: i32,
    text: *mut u16,
    len: *mut usize,
) {
    let Some(callback) = editor_callback!(editor, get_right_text_of_cursor) else {
        return;
    };
    let utf16: Vec<u16> = callback(number).encode_utf16().collect();
    if !text.is_null() && !len.is_null() && *len >= utf16.len() {
        std::ptr::copy_nonoverlapping(utf16.as_ptr(), text, utf16.len());
        *len = utf16.len();
    }
}

pub unsafe extern "C" fn get_text_config(
    editor: *mut InputMethod_TextEditorProxy,
    config: *mut InputMethod_TextConfig,
) {
    if let Some(callback) = editor_callback!(editor, get_text_config) {
        callback(TextConfig { raw: config });
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
