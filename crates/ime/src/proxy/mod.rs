use ohos_input_method_sys::{char16_t, InputMethod_TextEditorProxy};

use crate::common::IME_INSTANCE;

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

fn char16_ptr_to_string(ptr: *const u16) -> String {
    let mut result = String::new();

    unsafe {
        let mut current = ptr;
        while !current.is_null() && *current != 0 {
            if let Some(c) = char::decode_utf16(std::iter::once(*current)).next() {
                if let Ok(c) = c {
                    result.push(c);
                }
            }
            current = current.add(1);
        }
    }

    result
}

pub unsafe extern "C" fn insert_text(
    text_editor: *mut InputMethod_TextEditorProxy,
    text: *const char16_t,
    _len: usize,
) {
    let guard = IME_INSTANCE.read().unwrap();
    let ime_option = guard.get(&(text_editor as usize));
    match ime_option {
        Some(ime) => {
            if let Some(f) = ime.insert_text.borrow_mut().as_ref() {
                let ret = char16_ptr_to_string(text);
                f(ret);
            }
        }
        None => {}
    }
}
