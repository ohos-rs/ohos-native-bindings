use std::{collections::HashMap, ffi::c_void, mem::ManuallyDrop, ptr::NonNull};

use ohos_web_sys::{
    ArkWeb_HttpBodyStream, OH_ArkWebHttpBodyStream_GetUserData, OH_ArkWebHttpBodyStream_Init,
    OH_ArkWebHttpBodyStream_IsChunked, OH_ArkWebHttpBodyStream_IsEof,
    OH_ArkWebHttpBodyStream_IsInMemory, OH_ArkWebHttpBodyStream_Read,
    OH_ArkWebHttpBodyStream_SetReadCallback, OH_ArkWebHttpBodyStream_SetUserData,
};

pub struct HttpBodyStream {
    raw: NonNull<ArkWeb_HttpBodyStream>,
}

struct ReadCallbackContext {
    callback: Box<dyn FnMut(Vec<u8>)>,
}

impl HttpBodyStream {
    pub fn new(raw: *mut ArkWeb_HttpBodyStream) -> Self {
        unsafe {
            OH_ArkWebHttpBodyStream_Init(raw, None);
            Self {
                raw: NonNull::new_unchecked(raw),
            }
        }
    }

    pub fn is_chunked(&self) -> bool {
        unsafe { OH_ArkWebHttpBodyStream_IsChunked(self.raw.as_ptr()) }
    }

    pub fn is_eof(&self) -> bool {
        unsafe { OH_ArkWebHttpBodyStream_IsEof(self.raw.as_ptr()) }
    }

    pub fn is_in_memory(&self) -> bool {
        unsafe { OH_ArkWebHttpBodyStream_IsInMemory(self.raw.as_ptr()) }
    }

    pub fn read(&self, size: usize, callback: Box<dyn FnMut(Vec<u8>)>) {
        let mut buf: Vec<u8> = Vec::with_capacity(size);
        let buf_ptr = buf.as_mut_ptr();

        let ctx = ReadCallbackContext { callback };
        let ctx_ptr = Box::into_raw(Box::new(ctx)) as *mut c_void;

        unsafe {
            OH_ArkWebHttpBodyStream_SetUserData(self.raw.as_ptr(), ctx_ptr);
            OH_ArkWebHttpBodyStream_Read(self.raw.as_ptr(), buf_ptr, size as _);
            OH_ArkWebHttpBodyStream_SetReadCallback(self.raw.as_ptr(), Some(read_callback));
        };
    }
}

extern "C" fn read_callback(
    http_body_stream: *const ArkWeb_HttpBodyStream,
    buffer: *mut u8,
    bytes_read: i32,
) {
    unsafe {
        let user_data_ptr = OH_ArkWebHttpBodyStream_GetUserData(http_body_stream);
        let mut ctx = ManuallyDrop::new(Box::from_raw(user_data_ptr as *mut ReadCallbackContext));
        let data = std::slice::from_raw_parts(buffer, bytes_read as usize).to_vec();
        (ctx.callback)(data);
    }
}
