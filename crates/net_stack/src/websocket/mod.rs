use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::ptr;

use ohos_net_stack_sys::*;

use crate::{NetStackError, Result};

pub type OnOpenCallback = WebSocket_OnOpenCallback;
pub type OnMessageCallback = WebSocket_OnMessageCallback;
pub type OnErrorCallback = WebSocket_OnErrorCallback;
pub type OnCloseCallback = WebSocket_OnCloseCallback;

#[derive(Debug, Clone, Copy, Default)]
pub struct WebSocketCallbacks {
    pub on_open: OnOpenCallback,
    pub on_message: OnMessageCallback,
    pub on_error: OnErrorCallback,
    pub on_close: OnCloseCallback,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WebSocketRequestOptions {
    pub headers: HashMap<String, String>,
}

impl WebSocketRequestOptions {
    fn to_raw_owned(&self) -> Result<RawWebSocketRequestOptionsOwned> {
        let mut names = Vec::with_capacity(self.headers.len());
        let mut values = Vec::with_capacity(self.headers.len());

        for (name, value) in &self.headers {
            names.push(CString::new(name.as_str()).map_err(|_| NetStackError::NullByte)?);
            values.push(CString::new(value.as_str()).map_err(|_| NetStackError::NullByte)?);
        }

        let mut headers = names
            .iter()
            .zip(values.iter())
            .map(|(name, value)| WebSocket_Header {
                fieldName: name.as_ptr(),
                fieldValue: value.as_ptr(),
                next: ptr::null_mut(),
            })
            .collect::<Vec<_>>();

        let headers_ptr = headers.as_mut_ptr();
        for index in 0..headers.len().saturating_sub(1) {
            headers[index].next = headers_ptr.wrapping_add(index + 1);
        }

        let raw = WebSocket_RequestOptions {
            headers: headers
                .first_mut()
                .map_or(ptr::null_mut(), |value| value as *mut _),
        };
        Ok(RawWebSocketRequestOptionsOwned {
            raw,
            _names: names,
            _values: values,
            _headers: headers,
        })
    }
}

struct RawWebSocketRequestOptionsOwned {
    raw: WebSocket_RequestOptions,
    _names: Vec<CString>,
    _values: Vec<CString>,
    _headers: Vec<WebSocket_Header>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebSocketCloseOption {
    pub code: u32,
    pub reason: Option<String>,
}

impl WebSocketCloseOption {
    pub fn close(&self, client: &mut WebSocketClient) -> Result<()> {
        client.close(self)
    }

    fn to_raw_owned(&self) -> Result<RawWebSocketCloseOptionOwned> {
        let reason = self
            .reason
            .as_deref()
            .map(CString::new)
            .transpose()
            .map_err(|_| NetStackError::NullByte)?;
        let raw = WebSocket_CloseOption {
            code: self.code,
            reason: reason.as_ref().map_or(ptr::null(), |value| value.as_ptr()),
        };
        Ok(RawWebSocketCloseOptionOwned {
            raw,
            _reason: reason,
        })
    }
}

struct RawWebSocketCloseOptionOwned {
    raw: WebSocket_CloseOption,
    _reason: Option<CString>,
}

#[derive(Debug)]
pub struct WebSocketClient {
    raw: *mut WebSocket,
}

impl WebSocketClient {
    pub fn new(callbacks: WebSocketCallbacks) -> Result<Self> {
        let raw = unsafe {
            OH_WebSocketClient_Constructor(
                callbacks.on_open,
                callbacks.on_message,
                callbacks.on_error,
                callbacks.on_close,
            )
        };
        if raw.is_null() {
            return Err(NetStackError::NullPointer);
        }
        Ok(Self { raw })
    }

    pub fn add_header(&mut self, name: &str, value: &str) -> Result<()> {
        let name = CString::new(name).map_err(|_| NetStackError::NullByte)?;
        let value = CString::new(value).map_err(|_| NetStackError::NullByte)?;
        let raw_header = WebSocket_Header {
            fieldName: name.as_ptr(),
            fieldValue: value.as_ptr(),
            next: ptr::null_mut(),
        };
        let code = unsafe { OH_WebSocketClient_AddHeader(self.raw, raw_header) };
        if code != 0 {
            return Err(NetStackError::Code(code));
        }
        Ok(())
    }

    pub fn connect(&mut self, url: &str, options: &WebSocketRequestOptions) -> Result<()> {
        let c_url = CString::new(url).map_err(|_| NetStackError::NullByte)?;
        let raw_options = options.to_raw_owned()?;
        let code = unsafe { OH_WebSocketClient_Connect(self.raw, c_url.as_ptr(), raw_options.raw) };
        if code != 0 {
            return Err(NetStackError::Code(code));
        }
        Ok(())
    }

    pub fn send(&mut self, data: &[u8]) -> Result<()> {
        let code = unsafe {
            OH_WebSocketClient_Send(self.raw, data.as_ptr().cast_mut().cast(), data.len())
        };
        if code != 0 {
            return Err(NetStackError::Code(code));
        }
        Ok(())
    }

    pub fn close(&mut self, option: &WebSocketCloseOption) -> Result<()> {
        let raw_option = option.to_raw_owned()?;
        let code = unsafe { OH_WebSocketClient_Close(self.raw, raw_option.raw) };
        if code != 0 {
            return Err(NetStackError::Code(code));
        }
        Ok(())
    }

    pub fn destroy(mut self) -> Result<()> {
        let raw = std::mem::replace(&mut self.raw, ptr::null_mut());
        let code = unsafe { OH_WebSocketClient_Destroy(raw) };
        if code != 0 {
            return Err(NetStackError::Code(code));
        }
        Ok(())
    }
}

impl Drop for WebSocketClient {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                OH_WebSocketClient_Destroy(self.raw);
            }
            self.raw = ptr::null_mut();
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebSocketOpenResult {
    pub code: u32,
    pub reason: Option<String>,
}

impl WebSocketOpenResult {
    /// Converts a raw WebSocket open callback payload into an owned Rust value.
    ///
    /// # Safety
    ///
    /// Any string pointers inside `value` must come from the WebSocket callback and remain valid for this call.
    pub unsafe fn from_raw(value: WebSocket_OpenResult) -> Result<Self> {
        unsafe {
            Ok(Self {
                code: value.code,
                reason: if value.reason.is_null() {
                    None
                } else {
                    Some(
                        CStr::from_ptr(value.reason)
                            .to_str()
                            .map_err(|_| NetStackError::Conversion)?
                            .to_owned(),
                    )
                },
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebSocketErrorResult {
    pub error_code: u32,
    pub error_message: Option<String>,
}

impl WebSocketErrorResult {
    /// Converts a raw WebSocket error callback payload into an owned Rust value.
    ///
    /// # Safety
    ///
    /// Any string pointers inside `value` must come from the WebSocket callback and remain valid for this call.
    pub unsafe fn from_raw(value: WebSocket_ErrorResult) -> Result<Self> {
        unsafe {
            Ok(Self {
                error_code: value.errorCode,
                error_message: if value.errorMessage.is_null() {
                    None
                } else {
                    Some(
                        CStr::from_ptr(value.errorMessage)
                            .to_str()
                            .map_err(|_| NetStackError::Conversion)?
                            .to_owned(),
                    )
                },
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebSocketCloseResult {
    pub code: u32,
    pub reason: Option<String>,
}

impl WebSocketCloseResult {
    /// Converts a raw WebSocket close callback payload into an owned Rust value.
    ///
    /// # Safety
    ///
    /// Any string pointers inside `value` must come from the WebSocket callback and remain valid for this call.
    pub unsafe fn from_raw(value: WebSocket_CloseResult) -> Result<Self> {
        unsafe {
            Ok(Self {
                code: value.code,
                reason: if value.reason.is_null() {
                    None
                } else {
                    Some(
                        CStr::from_ptr(value.reason)
                            .to_str()
                            .map_err(|_| NetStackError::Conversion)?
                            .to_owned(),
                    )
                },
            })
        }
    }
}
