use ohos_input_method_sys::{
    char16_t, InputMethod_Direction, InputMethod_EnterKeyType, InputMethod_ExtendAction,
    InputMethod_KeyboardStatus, InputMethod_PrivateCommand, InputMethod_TextConfig,
    InputMethod_TextEditorProxy,
};

use crate::{
    common::IME_INSTANCE, private_command::PrivateCommand, Action, Direction, EnterKey,
    KeyboardStatus, Selection, TextConfig,
};

fn char16_ptr_to_string(ptr: *const u16, length: usize) -> String {
    let mut result = String::new();

    unsafe {
        let slice = std::slice::from_raw_parts(ptr, length);

        for &unit in slice.iter() {
            if let Some(c) = char::decode_utf16(std::iter::once(unit)).next() {
                if let Ok(c) = c {
                    result.push(c);
                }
            }
        }
    }

    result
}

pub unsafe extern "C" fn delete_backward(text_editor: *mut InputMethod_TextEditorProxy, len: i32) {
    let guard = IME_INSTANCE.read().unwrap();
    let ime_option = guard.get(&(text_editor as usize));
    match ime_option {
        Some(ime) => {
            if let Some(f) = ime.delete_backward.borrow_mut().as_ref() {
                f(len);
            }
        }
        None => {}
    }
}

pub unsafe extern "C" fn insert_text(
    text_editor: *mut InputMethod_TextEditorProxy,
    text: *const char16_t,
    len: usize,
) {
    let guard = IME_INSTANCE.read().unwrap();
    let ime_option = guard.get(&(text_editor as usize));
    match ime_option {
        Some(ime) => {
            if let Some(f) = ime.insert_text.borrow_mut().as_ref() {
                let ret = char16_ptr_to_string(text, len);
                f(ret);
            }
        }
        None => {}
    }
}

pub unsafe extern "C" fn delete_forward(text_editor: *mut InputMethod_TextEditorProxy, len: i32) {
    let guard = IME_INSTANCE.read().unwrap();
    let ime_option = guard.get(&(text_editor as usize));
    match ime_option {
        Some(ime) => {
            if let Some(f) = ime.delete_forward.borrow_mut().as_ref() {
                f(len);
            }
        }
        None => {}
    }
}

pub unsafe extern "C" fn finish_text_preview(text_editor: *mut InputMethod_TextEditorProxy) {
    let guard = IME_INSTANCE.read().unwrap();
    let ime_option = guard.get(&(text_editor as usize));
    match ime_option {
        Some(ime) => {
            if let Some(f) = ime.finish_text_preview.borrow_mut().as_ref() {
                f();
            }
        }
        None => {}
    }
}

pub unsafe extern "C" fn get_left_text_of_cursor(
    text_editor: *mut InputMethod_TextEditorProxy,
    number: i32,
    text: *mut char16_t,
    len: *mut usize,
) {
    let guard = IME_INSTANCE.read().unwrap();
    let ime_option = guard.get(&(text_editor as usize));
    match ime_option {
        Some(ime) => {
            if let Some(f) = ime.get_left_text_of_cursor.borrow_mut().as_ref() {
                let s = f(number);
                let utf16: Vec<u16> = s.encode_utf16().collect();

                if !text.is_null() && !len.is_null() && *len >= utf16.len() {
                    std::ptr::copy_nonoverlapping(utf16.as_ptr(), text, utf16.len());
                    *len = utf16.len();
                }
            }
        }
        None => {}
    }
}

pub unsafe extern "C" fn get_right_text_of_cursor(
    text_editor: *mut InputMethod_TextEditorProxy,
    number: i32,
    text: *mut char16_t,
    len: *mut usize,
) {
    let guard = IME_INSTANCE.read().unwrap();
    let ime_option = guard.get(&(text_editor as usize));
    match ime_option {
        Some(ime) => {
            if let Some(f) = ime.get_right_text_of_cursor.borrow_mut().as_ref() {
                let s = f(number);
                let utf16: Vec<u16> = s.encode_utf16().collect();

                if !text.is_null() && !len.is_null() && *len >= utf16.len() {
                    std::ptr::copy_nonoverlapping(utf16.as_ptr(), text, utf16.len());
                    *len = utf16.len();
                }
            }
        }
        None => {}
    }
}

pub unsafe extern "C" fn get_text_config(
    text_editor: *mut InputMethod_TextEditorProxy,
    config: *mut InputMethod_TextConfig,
) {
    let guard = IME_INSTANCE.read().unwrap();
    let ime_option = guard.get(&(text_editor as usize));
    match ime_option {
        Some(ime) => {
            if let Some(f) = ime.get_text_config.borrow_mut().as_ref() {
                f(TextConfig { raw: config });
            }
        }
        None => {}
    }
}

pub unsafe extern "C" fn get_text_index_at_cursor(
    text_editor: *mut InputMethod_TextEditorProxy,
) -> i32 {
    let guard = IME_INSTANCE.read().unwrap();
    let ime_option = guard.get(&(text_editor as usize));
    match ime_option {
        Some(ime) => {
            if let Some(f) = ime.get_text_index_at_cursor.borrow_mut().as_ref() {
                let ret = f();
                return ret;
            }
            return 0;
        }
        None => 0,
    }
}

pub unsafe extern "C" fn handle_extend_action(
    text_editor: *mut InputMethod_TextEditorProxy,
    action: InputMethod_ExtendAction,
) {
    let guard = IME_INSTANCE.read().unwrap();
    let ime_option = guard.get(&(text_editor as usize));
    match ime_option {
        Some(ime) => {
            if let Some(f) = ime.handle_extend_action.borrow_mut().as_ref() {
                f(Action::from(action));
            }
        }
        None => {}
    }
}

pub unsafe extern "C" fn handle_set_selection(
    text_editor: *mut InputMethod_TextEditorProxy,
    start: i32,
    end: i32,
) {
    let guard = IME_INSTANCE.read().unwrap();
    let ime_option = guard.get(&(text_editor as usize));
    match ime_option {
        Some(ime) => {
            if let Some(f) = ime.handle_set_selection.borrow_mut().as_ref() {
                f(Selection { start, end });
            }
        }
        None => {}
    }
}

pub unsafe extern "C" fn move_cursor(
    text_editor: *mut InputMethod_TextEditorProxy,
    direction: InputMethod_Direction,
) {
    let guard = IME_INSTANCE.read().unwrap();
    let ime_option = guard.get(&(text_editor as usize));
    match ime_option {
        Some(ime) => {
            if let Some(f) = ime.move_cursor.borrow_mut().as_ref() {
                f(Direction::from(direction));
            }
        }
        None => {}
    }
}

pub unsafe extern "C" fn receive_private_command(
    text_editor: *mut InputMethod_TextEditorProxy,
    command: *mut *mut InputMethod_PrivateCommand,
    len: usize,
) -> i32 {
    let guard = IME_INSTANCE.read().unwrap();
    let ime_option = guard.get(&(text_editor as usize));
    match ime_option {
        Some(ime) => {
            if let Some(f) = ime.receive_private_command.borrow_mut().as_ref() {
                unsafe {
                    let slice = std::slice::from_raw_parts_mut(command, len);

                    let mut manual_array = Vec::new();
                    for i in 0..len {
                        manual_array[i] = PrivateCommand {
                            raw: *slice.get_unchecked(i),
                        };
                    }
                    f(manual_array);
                }
            }
            0
        }
        None => 0,
    }
}

pub unsafe extern "C" fn send_enter_key(
    text_editor: *mut InputMethod_TextEditorProxy,
    enter_key_type: InputMethod_EnterKeyType,
) {
    let guard = IME_INSTANCE.read().unwrap();
    let ime_option = guard.get(&(text_editor as usize));
    match ime_option {
        Some(ime) => {
            if let Some(f) = ime.send_enter_key.borrow_mut().as_ref() {
                f(EnterKey::from(enter_key_type));
            }
        }
        None => {}
    }
}

pub unsafe extern "C" fn send_keyboard_status(
    text_editor: *mut InputMethod_TextEditorProxy,
    keyboard_status: InputMethod_KeyboardStatus,
) {
    let guard = IME_INSTANCE.read().unwrap();
    let ime_option = guard.get(&(text_editor as usize));
    match ime_option {
        Some(ime) => {
            if let Some(f) = ime.send_keyboard_status.borrow_mut().as_ref() {
                f(KeyboardStatus::from(keyboard_status));
            }
        }
        None => {}
    }
}

pub unsafe extern "C" fn set_preview_text(
    text_editor: *mut InputMethod_TextEditorProxy,
    text: *const char16_t,
    length: usize,
    start: i32,
    end: i32,
) -> i32 {
    let guard = IME_INSTANCE.read().unwrap();
    let ime_option = guard.get(&(text_editor as usize));
    match ime_option {
        Some(ime) => {
            if let Some(f) = ime.set_preview_text.borrow_mut().as_ref() {
                let ret = char16_ptr_to_string(text, length);
                f(ret, start, end);
            }
            0
        }
        None => 0,
    }
}
