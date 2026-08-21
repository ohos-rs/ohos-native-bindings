use std::ptr::NonNull;

use ohos_web_sys::{
    ArkWeb_HttpBodyStream, OH_ArkWebHttpBodyStream_GetSize, OH_ArkWebHttpBodyStream_GetUserData,
    OH_ArkWebHttpBodyStream_Init, OH_ArkWebHttpBodyStream_IsChunked, OH_ArkWebHttpBodyStream_IsEof,
    OH_ArkWebHttpBodyStream_IsInMemory, OH_ArkWebHttpBodyStream_Read,
    OH_ArkWebHttpBodyStream_SetReadCallback, OH_ArkWebHttpBodyStream_SetUserData,
};

#[path = "read_buffer.rs"]
mod read_buffer;

use read_buffer::{finish_read_callback, ReadCallbackContext};

pub struct HttpBodyStream {
    raw: NonNull<ArkWeb_HttpBodyStream>,
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

    pub fn read<F>(&self, size: usize, mut callback: F)
    where
        F: FnMut(Vec<u8>),
    {
        let static_callback = unsafe {
            std::mem::transmute::<Box<dyn FnMut(Vec<u8>)>, Box<dyn FnMut(Vec<u8>) + 'static>>(
                Box::new(move |buf| {
                    callback(buf);
                }),
            )
        };

        let ctx = ReadCallbackContext::new(size, static_callback);
        let (ctx_ptr, buf_ptr) = ctx.into_raw_with_buffer();

        unsafe {
            OH_ArkWebHttpBodyStream_SetUserData(self.raw.as_ptr(), ctx_ptr);
            OH_ArkWebHttpBodyStream_SetReadCallback(self.raw.as_ptr(), Some(read_callback));
            OH_ArkWebHttpBodyStream_Read(self.raw.as_ptr(), buf_ptr, size as _);
        };
    }

    pub fn size(&self) -> u64 {
        unsafe { OH_ArkWebHttpBodyStream_GetSize(self.raw.as_ptr()) }
    }
}

extern "C" fn read_callback(
    http_body_stream: *const ArkWeb_HttpBodyStream,
    _buffer: *mut u8,
    bytes_read: i32,
) {
    unsafe {
        let user_data_ptr = OH_ArkWebHttpBodyStream_GetUserData(http_body_stream);
        finish_read_callback(user_data_ptr, bytes_read);
    }
}
