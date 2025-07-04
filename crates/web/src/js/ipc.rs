use std::ffi::{c_char, c_void, CStr};
use std::mem::ManuallyDrop;
use std::slice;

use ohos_web_sys::ArkWeb_JavaScriptBridgeData;

#[repr(C)]
pub(crate) struct JsApiCallbackContext {
    pub(crate) callback: Box<dyn FnMut(String, Vec<String>)>,
}

fn convert_data_to_strings(
    data: *const ArkWeb_JavaScriptBridgeData,
    data_length: usize,
) -> Vec<String> {
    if data.is_null() || data_length == 0 {
        return Vec::new();
    }

    let data_slice = unsafe { slice::from_raw_parts(data, data_length) };
    let mut strings = Vec::with_capacity(data_length);

    for bridge_data in data_slice {
        if bridge_data.buffer.is_null() || bridge_data.size == 0 {
            strings.push(String::new());
            continue;
        }

        let bytes = unsafe { slice::from_raw_parts(bridge_data.buffer, bridge_data.size) };

        match String::from_utf8(bytes.to_vec()) {
            Ok(s) => strings.push(s),
            Err(_) => {
                strings.push(String::from_utf8_lossy(bytes).to_string());
            }
        }
    }

    strings
}

pub extern "C" fn ark_web_proxy_method(
    web_tag: *const c_char,
    data: *const ArkWeb_JavaScriptBridgeData,
    data_length: usize,
    user_data: *mut c_void,
) {
    let web_tag_str = unsafe { CStr::from_ptr(web_tag as _) };
    let web_tag = web_tag_str.to_string_lossy().to_string();
    let data = convert_data_to_strings(data, data_length);

    let mut ctx =
        unsafe { ManuallyDrop::new(Box::from_raw(user_data as *mut JsApiCallbackContext)) };
    let cb = &mut ctx.callback;
    cb(web_tag, data);
}

pub extern "C" fn ark_web_proxy_method_with_result(
    web_tag: *const c_char,
    data: *const ArkWeb_JavaScriptBridgeData,
    data_length: usize,
    user_data: *mut c_void,
) {
    let web_tag_str = unsafe { CStr::from_ptr(web_tag as _) };
    let web_tag = web_tag_str.to_string_lossy().to_string();
    let data = convert_data_to_strings(data, data_length);

    let mut ctx =
        unsafe { ManuallyDrop::new(Box::from_raw(user_data as *mut JsApiCallbackContext)) };
    let cb = &mut ctx.callback;
    cb(web_tag, data);
}
