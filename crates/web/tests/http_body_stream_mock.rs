//! Host-side callback/lifetime regression checks, without an ArkWeb shared library.
//! Run with `rustc --edition=2021 --test crates/web/tests/http_body_stream_mock.rs -o /tmp/web-body-tests`.
#![allow(non_camel_case_types, non_snake_case, dead_code)]
extern crate self as ohos_web_sys;

use std::{
    cell::{Cell, RefCell},
    ffi::c_void,
    rc::Rc,
};

type ReadCallback = Option<unsafe extern "C" fn(*const ArkWeb_HttpBodyStream, *mut u8, i32)>;
type InitCallback = Option<unsafe extern "C" fn(*const ArkWeb_HttpBodyStream, i32)>;

#[derive(Default)]
pub struct ArkWeb_HttpBodyStream {
    user_data: Cell<*mut c_void>,
    read: Cell<ReadCallback>,
    init: Cell<InitCallback>,
    buffer: Cell<*mut u8>,
    length: Cell<i32>,
    eof: Cell<bool>,
    init_result: Cell<i32>,
}

impl ArkWeb_HttpBodyStream {
    fn initialize(&self, status: i32) {
        unsafe { self.init.take().unwrap()(self, status) }
    }
    fn deliver(&self, data: &[u8], eof: bool) {
        assert!(data.len() <= self.length.get() as usize);
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), self.buffer.get(), data.len());
            self.eof.set(eof);
            self.read.get().unwrap()(self, self.buffer.get(), data.len() as i32);
        }
    }
    fn detached(&self) -> bool {
        self.user_data.get().is_null() && self.read.get().is_none()
    }
}

pub unsafe fn OH_ArkWebHttpBodyStream_SetUserData(
    s: *mut ArkWeb_HttpBodyStream,
    data: *mut c_void,
) -> i32 {
    (*s).user_data.set(data);
    0
}
pub unsafe fn OH_ArkWebHttpBodyStream_GetUserData(s: *const ArkWeb_HttpBodyStream) -> *mut c_void {
    (*s).user_data.get()
}
pub unsafe fn OH_ArkWebHttpBodyStream_SetReadCallback(
    s: *mut ArkWeb_HttpBodyStream,
    callback: ReadCallback,
) -> i32 {
    (*s).read.set(callback);
    0
}
pub unsafe fn OH_ArkWebHttpBodyStream_Init(
    s: *mut ArkWeb_HttpBodyStream,
    callback: InitCallback,
) -> i32 {
    (*s).init.set(callback);
    (*s).init_result.get()
}
pub unsafe fn OH_ArkWebHttpBodyStream_Read(
    s: *const ArkWeb_HttpBodyStream,
    buffer: *mut u8,
    len: i32,
) {
    (*s).buffer.set(buffer);
    (*s).length.set(len);
}
pub unsafe fn OH_ArkWebHttpBodyStream_IsEof(s: *const ArkWeb_HttpBodyStream) -> bool {
    (*s).eof.get()
}
pub unsafe fn OH_ArkWebHttpBodyStream_IsChunked(_: *const ArkWeb_HttpBodyStream) -> bool {
    true
}
pub unsafe fn OH_ArkWebHttpBodyStream_IsInMemory(_: *const ArkWeb_HttpBodyStream) -> bool {
    false
}
pub unsafe fn OH_ArkWebHttpBodyStream_GetSize(_: *const ArkWeb_HttpBodyStream) -> u64 {
    0
}

#[path = "../src/protocol/http_body_stream.rs"]
mod http_body_stream;
use http_body_stream::HttpBodyStream;

#[test]
fn single_read_owns_buffer_and_releases_callback_after_async_completion() {
    let mut native = ArkWeb_HttpBodyStream::default();
    let stream = HttpBodyStream::new(&mut native);
    let result = Rc::new(RefCell::new(None));
    let captured = result.clone();
    stream.read(32, move |bytes| *captured.borrow_mut() = Some(bytes));
    assert!(result.borrow().is_none());
    native.initialize(0);
    native.deliver(b"later", true);
    assert_eq!(result.borrow().as_deref(), Some(b"later".as_slice()));
    assert!(native.detached());
    assert_eq!(Rc::strong_count(&result), 1);
}

#[test]
fn unknown_length_body_is_read_across_chunks_until_eof() {
    let mut native = ArkWeb_HttpBodyStream::default();
    let stream = HttpBodyStream::new(&mut native);
    let result = Rc::new(RefCell::new(None));
    let captured = result.clone();
    stream.read_to_end(move |body| *captured.borrow_mut() = Some(body));
    native.initialize(0);
    native.deliver(b"first", false);
    assert!(result.borrow().is_none());
    native.deliver(b"second", true);
    assert_eq!(*result.borrow(), Some(Ok(b"firstsecond".to_vec())));
    assert!(native.detached());
    assert_eq!(Rc::strong_count(&result), 1);
}

#[test]
fn immediate_and_async_initialization_errors_complete_and_release() {
    for immediate in [true, false] {
        let mut native = ArkWeb_HttpBodyStream::default();
        native.init_result.set(if immediate { 17 } else { 0 });
        let stream = HttpBodyStream::new(&mut native);
        let result = Rc::new(RefCell::new(None));
        let captured = result.clone();
        stream.read_to_end(move |body| *captured.borrow_mut() = Some(body));
        if !immediate {
            native.initialize(17);
        }
        assert!(matches!(*result.borrow(), Some(Err(_))));
        assert!(native.detached());
        assert_eq!(Rc::strong_count(&result), 1);
    }
}

#[test]
fn empty_eof_succeeds_but_zero_progress_without_eof_fails() {
    for eof in [true, false] {
        let mut native = ArkWeb_HttpBodyStream::default();
        let stream = HttpBodyStream::new(&mut native);
        let result = Rc::new(RefCell::new(None));
        let captured = result.clone();
        stream.read_to_end(move |body| *captured.borrow_mut() = Some(body));
        native.eof.set(eof);
        native.initialize(0);
        if !eof {
            native.deliver(b"", false);
        }
        assert_eq!(result.borrow().as_ref().unwrap().is_ok(), eof);
        assert!(native.detached());
    }
}
