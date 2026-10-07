use std::{ffi::c_void, ptr::NonNull};

use ohos_web_sys::{
    ArkWeb_HttpBodyStream, OH_ArkWebHttpBodyStream_GetSize, OH_ArkWebHttpBodyStream_GetUserData,
    OH_ArkWebHttpBodyStream_Init, OH_ArkWebHttpBodyStream_IsChunked, OH_ArkWebHttpBodyStream_IsEof,
    OH_ArkWebHttpBodyStream_IsInMemory, OH_ArkWebHttpBodyStream_Read,
    OH_ArkWebHttpBodyStream_SetReadCallback, OH_ArkWebHttpBodyStream_SetUserData,
};

pub struct HttpBodyStream {
    raw: NonNull<ArkWeb_HttpBodyStream>,
}

struct ReadCallbackContext {
    callback: Box<dyn FnMut(Vec<u8>)>,
    buffer: Vec<u8>,
}

impl HttpBodyStream {
    pub fn new(raw: *mut ArkWeb_HttpBodyStream) -> Self {
        unsafe {
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

    pub fn read<F>(&self, size: usize, callback: F)
    where
        F: FnMut(Vec<u8>) + 'static,
    {
        let buf = vec![0; size.min(i32::MAX as usize)];

        let ctx = ReadCallbackContext {
            callback: Box::new(callback),
            buffer: buf,
        };
        let ctx_ptr = Box::into_raw(Box::new(ctx)) as *mut c_void;

        unsafe {
            OH_ArkWebHttpBodyStream_SetUserData(self.raw.as_ptr(), ctx_ptr);
            OH_ArkWebHttpBodyStream_SetReadCallback(self.raw.as_ptr(), Some(read_callback));
            let status = OH_ArkWebHttpBodyStream_Init(self.raw.as_ptr(), Some(read_initialized));
            if status != 0 {
                read_callback(self.raw.as_ptr(), std::ptr::null_mut(), -1);
            }
        };
    }

    /// Read through EOF, retaining each native output buffer until its callback.
    pub fn read_to_end<F>(&self, callback: F)
    where
        F: FnOnce(Result<Vec<u8>, String>) + 'static,
    {
        let context = Box::new(BodyReader {
            callback: Some(Box::new(callback)),
            buffer: vec![0; 64 * 1024],
            data: Vec::new(),
        });
        unsafe {
            OH_ArkWebHttpBodyStream_SetUserData(self.raw.as_ptr(), Box::into_raw(context).cast());
            OH_ArkWebHttpBodyStream_SetReadCallback(self.raw.as_ptr(), Some(body_read));
            let status = OH_ArkWebHttpBodyStream_Init(self.raw.as_ptr(), Some(body_initialized));
            if status != 0 {
                finish_body(
                    self.raw.as_ptr(),
                    Some(format!("Body initialization failed: {status}")),
                );
            }
        }
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
        if user_data_ptr.is_null() {
            return;
        }
        let mut ctx = Box::from_raw(user_data_ptr as *mut ReadCallbackContext);
        OH_ArkWebHttpBodyStream_SetUserData(http_body_stream as *mut _, std::ptr::null_mut());
        OH_ArkWebHttpBodyStream_SetReadCallback(http_body_stream as *mut _, None);
        let data = if bytes_read > 0 && (bytes_read as usize) <= ctx.buffer.len() {
            ctx.buffer[..bytes_read as usize].to_vec()
        } else {
            Vec::new()
        };
        (ctx.callback)(data);
    }
}

type BodyCallback = Box<dyn FnOnce(Result<Vec<u8>, String>)>;
struct BodyReader {
    callback: Option<BodyCallback>,
    buffer: Vec<u8>,
    data: Vec<u8>,
}

unsafe fn finish_body(stream: *const ArkWeb_HttpBodyStream, error: Option<String>) {
    let raw = OH_ArkWebHttpBodyStream_GetUserData(stream) as *mut BodyReader;
    if raw.is_null() {
        return;
    }
    OH_ArkWebHttpBodyStream_SetUserData(stream as *mut _, std::ptr::null_mut());
    OH_ArkWebHttpBodyStream_SetReadCallback(stream as *mut _, None);
    let mut reader = Box::from_raw(raw);
    let result = match error {
        Some(error) => Err(error),
        None => Ok(std::mem::take(&mut reader.data)),
    };
    reader.callback.take().unwrap()(result);
}

unsafe fn read_next(stream: *const ArkWeb_HttpBodyStream) {
    let reader = &mut *(OH_ArkWebHttpBodyStream_GetUserData(stream) as *mut BodyReader);
    let buffer = reader.buffer.as_mut_ptr();
    let length = reader.buffer.len() as i32;
    OH_ArkWebHttpBodyStream_Read(stream, buffer, length);
}

unsafe extern "C" fn body_initialized(stream: *const ArkWeb_HttpBodyStream, status: i32) {
    if status != 0 {
        finish_body(
            stream,
            Some(format!("Body initialization failed: {status}")),
        );
    } else if OH_ArkWebHttpBodyStream_IsEof(stream) {
        finish_body(stream, None);
    } else {
        read_next(stream);
    }
}

unsafe extern "C" fn body_read(stream: *const ArkWeb_HttpBodyStream, _buffer: *mut u8, count: i32) {
    let reader = &mut *(OH_ArkWebHttpBodyStream_GetUserData(stream) as *mut BodyReader);
    if count < 0 || count as usize > reader.buffer.len() {
        finish_body(stream, Some(format!("Body read failed: {count}")));
        return;
    }
    reader
        .data
        .extend_from_slice(&reader.buffer[..count as usize]);
    if OH_ArkWebHttpBodyStream_IsEof(stream) {
        finish_body(stream, None);
    } else if count == 0 {
        finish_body(stream, Some("Body stream stopped before EOF".into()));
    } else {
        read_next(stream);
    }
}

unsafe extern "C" fn read_initialized(stream: *const ArkWeb_HttpBodyStream, status: i32) {
    let ctx = &mut *(OH_ArkWebHttpBodyStream_GetUserData(stream) as *mut ReadCallbackContext);
    if status != 0 {
        read_callback(stream, ctx.buffer.as_mut_ptr(), -1);
    } else {
        OH_ArkWebHttpBodyStream_Read(stream, ctx.buffer.as_mut_ptr(), ctx.buffer.len() as i32);
    }
}
