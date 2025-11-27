use ohos_input_method_sys::{
    char16_t, InputMethod_Direction, InputMethod_EnterKeyType, InputMethod_ExtendAction,
    InputMethod_KeyboardStatus, InputMethod_PrivateCommand, InputMethod_TextConfig,
    InputMethod_TextEditorProxy,
};

use crate::{
    private_command::PrivateCommand, Action, Direction, EnterKey, KeyboardStatus, Selection,
    TextConfig,
};

mod callbacks;

pub use callbacks::*;

fn char16_ptr_to_string(ptr: *const u16, length: usize) -> String {
    let mut result = String::new();

    unsafe {
        let slice = std::slice::from_raw_parts(ptr, length);

        for &unit in slice.iter() {
            if let Some(Ok(c)) = char::decode_utf16(std::iter::once(unit)).next() {
                result.push(c);
            }
        }
    }

    result
}

pub unsafe extern "C" fn delete_backward(_text_editor: *mut InputMethod_TextEditorProxy, len: i32) {
    let guard = OHOS_RS_IME_CALLBACKS
        .read()
        .expect("Failed to acquire read lock");
    if let Some(f) = &guard.delete_backward {
        f(len);
    }
}

pub unsafe extern "C" fn insert_text(
    _text_editor: *mut InputMethod_TextEditorProxy,
    text: *const char16_t,
    len: usize,
) {
    let guard = OHOS_RS_IME_CALLBACKS
        .read()
        .expect("Failed to acquire read lock");
    if let Some(f) = &guard.insert_text {
        let ret = char16_ptr_to_string(text, len);
        f(ret);
    }
}

pub unsafe extern "C" fn delete_forward(_text_editor: *mut InputMethod_TextEditorProxy, len: i32) {
    let guard = OHOS_RS_IME_CALLBACKS
        .read()
        .expect("Failed to acquire read lock");

    if let Some(f) = &guard.delete_forward {
        f(len);
    }
}

pub unsafe extern "C" fn finish_text_preview(_text_editor: *mut InputMethod_TextEditorProxy) {
    let guard = OHOS_RS_IME_CALLBACKS
        .read()
        .expect("Failed to acquire read lock");
    if let Some(f) = &guard.finish_text_preview {
        f();
    }
}

pub unsafe extern "C" fn get_left_text_of_cursor(
    _text_editor: *mut InputMethod_TextEditorProxy,
    number: i32,
    text: *mut char16_t,
    len: *mut usize,
) {
    let guard = OHOS_RS_IME_CALLBACKS
        .read()
        .expect("Failed to acquire read lock");
    if let Some(f) = &guard.get_left_text_of_cursor {
        let s = f(number);
        let utf16: Vec<u16> = s.encode_utf16().collect();

        if !text.is_null() && !len.is_null() && *len >= utf16.len() {
            std::ptr::copy_nonoverlapping(utf16.as_ptr(), text, utf16.len());
            *len = utf16.len();
        }
    }
}

pub unsafe extern "C" fn get_right_text_of_cursor(
    _text_editor: *mut InputMethod_TextEditorProxy,
    number: i32,
    text: *mut char16_t,
    len: *mut usize,
) {
    let guard = OHOS_RS_IME_CALLBACKS
        .read()
        .expect("Failed to acquire read lock");
    if let Some(f) = &guard.get_right_text_of_cursor {
        let s = f(number);
        let utf16: Vec<u16> = s.encode_utf16().collect();

        if !text.is_null() && !len.is_null() && *len >= utf16.len() {
            std::ptr::copy_nonoverlapping(utf16.as_ptr(), text, utf16.len());
            *len = utf16.len();
        }
    }
}

pub unsafe extern "C" fn get_text_config(
    _text_editor: *mut InputMethod_TextEditorProxy,
    config: *mut InputMethod_TextConfig,
) {
    let guard = OHOS_RS_IME_CALLBACKS
        .read()
        .expect("Failed to acquire read lock");
    if let Some(f) = &guard.get_text_config {
        f(TextConfig { raw: config });
    }
}

pub unsafe extern "C" fn get_text_index_at_cursor(
    _text_editor: *mut InputMethod_TextEditorProxy,
) -> i32 {
    let guard = OHOS_RS_IME_CALLBACKS
        .read()
        .expect("Failed to acquire read lock");
    match &guard.get_text_index_at_cursor {
        Some(f) => f(),
        None => 0,
    }
}

pub unsafe extern "C" fn handle_extend_action(
    _text_editor: *mut InputMethod_TextEditorProxy,
    action: InputMethod_ExtendAction,
) {
    let guard = OHOS_RS_IME_CALLBACKS
        .read()
        .expect("Failed to acquire read lock");
    if let Some(f) = &guard.handle_extend_action {
        f(Action::from(action));
    }
}

pub unsafe extern "C" fn handle_set_selection(
    _text_editor: *mut InputMethod_TextEditorProxy,
    start: i32,
    end: i32,
) {
    let guard = OHOS_RS_IME_CALLBACKS
        .read()
        .expect("Failed to acquire read lock");
    if let Some(f) = &guard.handle_set_selection {
        f(Selection { start, end });
    }
}

pub unsafe extern "C" fn move_cursor(
    _text_editor: *mut InputMethod_TextEditorProxy,
    direction: InputMethod_Direction,
) {
    let guard = OHOS_RS_IME_CALLBACKS
        .read()
        .expect("Failed to acquire read lock");
    if let Some(f) = &guard.move_cursor {
        f(Direction::from(direction));
    }
}

pub unsafe extern "C" fn receive_private_command(
    _text_editor: *mut InputMethod_TextEditorProxy,
    command: *mut *mut InputMethod_PrivateCommand,
    len: usize,
) -> i32 {
    let guard = OHOS_RS_IME_CALLBACKS
        .read()
        .expect("Failed to acquire read lock");
    if let Some(f) = &guard.receive_private_command {
        unsafe {
            let slice = std::slice::from_raw_parts_mut(command, len);

            let mut manual_array = Vec::new();
            for (i, command) in manual_array.iter_mut().enumerate().take(len) {
                *command = PrivateCommand {
                    raw: *slice.get_unchecked(i),
                };
            }
            f(manual_array);
        }
    }
    0
}

pub unsafe extern "C" fn send_enter_key(
    _text_editor: *mut InputMethod_TextEditorProxy,
    enter_key_type: InputMethod_EnterKeyType,
) {
    let guard = OHOS_RS_IME_CALLBACKS
        .read()
        .expect("Failed to acquire read lock");
    if let Some(f) = &guard.send_enter_key {
        f(EnterKey::from(enter_key_type));
    }
}

pub unsafe extern "C" fn send_keyboard_status(
    _text_editor: *mut InputMethod_TextEditorProxy,
    keyboard_status: InputMethod_KeyboardStatus,
) {
    let guard = OHOS_RS_IME_CALLBACKS
        .read()
        .expect("Failed to acquire read lock");
    if let Some(f) = &guard.send_keyboard_status {
        f(KeyboardStatus::from(keyboard_status));
    }
}

pub unsafe extern "C" fn set_preview_text(
    _text_editor: *mut InputMethod_TextEditorProxy,
    text: *const char16_t,
    length: usize,
    start: i32,
    end: i32,
) -> i32 {
    let guard = OHOS_RS_IME_CALLBACKS
        .read()
        .expect("Failed to acquire read lock");
    if let Some(f) = &guard.set_preview_text {
        let ret = char16_ptr_to_string(text, length);
        f(ret, start, end);
    }
    0
}
