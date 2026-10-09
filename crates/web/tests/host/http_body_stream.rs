//! Host regression tests use the production module with a deterministic ArkWeb
//! test double. No OHOS library or device is needed:
//! crates/web/tests/run-http-body-stream.sh
#![allow(
    non_snake_case,
    non_camel_case_types,
    dead_code,
    clippy::missing_safety_doc
)]

extern crate self as ohos_web_sys;

#[path = "../../src/error.rs"]
mod error;
pub use error::HttpBodyStreamError;
#[path = "../../src/protocol/http_body_stream.rs"]
mod http_body_stream;
pub use http_body_stream::HttpBodyStream;

use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    ffi::c_void,
    rc::Rc,
};

type InitCallback = unsafe extern "C" fn(*const ArkWeb_HttpBodyStream, i32);
type ReadCallback = unsafe extern "C" fn(*const ArkWeb_HttpBodyStream, *mut u8, i32);
type BodyResult = Result<Vec<u8>, HttpBodyStreamError>;

#[derive(Clone, Copy, Default)]
enum Timing {
    #[default]
    Inline,
    Deferred,
    Alternating,
}

#[derive(Default)]
struct Config {
    body: Vec<u8>,
    chunked: bool,
    max_chunk: Option<usize>,
    init_timing: Timing,
    read_timing: Timing,
    init_return: i32,
    init_result: i32,
    user_data_error: i32,
    read_callback_error: i32,
}

#[derive(Clone, Copy)]
struct PendingRead {
    buffer: *mut u8,
    length: usize,
}

struct NativeState {
    config: Config,
    user_data: Cell<*mut c_void>,
    read_callback: Cell<Option<ReadCallback>>,
    init_callback: Cell<Option<InitCallback>>,
    reads: RefCell<VecDeque<PendingRead>>,
    initialized: Cell<bool>,
    position: Cell<usize>,
    eof: Cell<bool>,
    init_calls: Cell<usize>,
    read_calls: Cell<usize>,
    user_data_calls: Cell<usize>,
    read_depth: Cell<usize>,
    max_read_depth: Cell<usize>,
    destroy_calls: Cell<usize>,
}

pub struct ArkWeb_HttpBodyStream {
    state: Rc<NativeState>,
}

struct Native {
    raw: *mut ArkWeb_HttpBodyStream,
    state: Rc<NativeState>,
}

impl Native {
    fn create(config: Config) -> (Self, Result<HttpBodyStream, HttpBodyStreamError>) {
        let state = Rc::new(NativeState {
            config,
            user_data: Cell::new(std::ptr::null_mut()),
            read_callback: Cell::new(None),
            init_callback: Cell::new(None),
            reads: RefCell::new(VecDeque::new()),
            initialized: Cell::new(false),
            position: Cell::new(0),
            eof: Cell::new(false),
            init_calls: Cell::new(0),
            read_calls: Cell::new(0),
            user_data_calls: Cell::new(0),
            read_depth: Cell::new(0),
            max_read_depth: Cell::new(0),
            destroy_calls: Cell::new(0),
        });
        let raw = Box::into_raw(Box::new(ArkWeb_HttpBodyStream {
            state: state.clone(),
        }));
        // Each test thread stands in for the ArkWeb I/O thread; all callbacks
        // run on that same thread. Ownership is transferred to the subject.
        let stream = unsafe { HttpBodyStream::new(raw) };
        (Self { raw, state }, stream)
    }

    fn finish_init(&self) {
        self.state
            .initialized
            .set(self.state.config.init_result == 0);
        self.state.position.set(0);
        self.state
            .eof
            .set(!self.state.config.chunked && self.state.config.body.is_empty());
        let callback = self.state.init_callback.take().unwrap();
        unsafe { callback(self.raw, self.state.config.init_result) };
    }

    fn complete_read(&self, forced_count: Option<i32>, wrong_buffer: bool) {
        let pending = self.state.reads.borrow_mut().pop_front().unwrap();
        let remaining = self.state.config.body.len() - self.state.position.get();
        let count = forced_count.unwrap_or_else(|| {
            remaining
                .min(pending.length)
                .min(self.state.config.max_chunk.unwrap_or(usize::MAX)) as i32
        });
        if count >= 0 && count as usize <= pending.length && count as usize <= remaining {
            let start = self.state.position.get();
            // This write happens after read() has returned in deferred mode.
            unsafe {
                std::ptr::copy_nonoverlapping(
                    self.state.config.body.as_ptr().add(start),
                    pending.buffer,
                    count as usize,
                )
            };
            self.state.position.set(start + count as usize);
            // A chunked stream may only discover EOF on a final zero read.
            self.state.eof.set(
                self.state.position.get() == self.state.config.body.len()
                    && (!self.state.config.chunked || count == 0),
            );
        }
        let callback = self.state.read_callback.get().unwrap();
        let buffer = if wrong_buffer {
            std::ptr::null_mut()
        } else {
            pending.buffer
        };
        unsafe { callback(self.raw, buffer, count) };
    }

    fn drain(&self) {
        while !self.state.reads.borrow().is_empty() {
            self.complete_read(None, false);
        }
    }
}

pub unsafe fn OH_ArkWebHttpBodyStream_SetUserData(
    raw: *mut ArkWeb_HttpBodyStream,
    data: *mut c_void,
) -> i32 {
    let state = &(*raw).state;
    state.user_data_calls.set(state.user_data_calls.get() + 1);
    // Some ArkWeb implementations ignore null userData. Cleanup must not rely
    // on clearing the pointer before freeing a per-operation context.
    if !data.is_null() && state.config.user_data_error == 0 {
        state.user_data.set(data);
    }
    state.config.user_data_error
}

pub unsafe fn OH_ArkWebHttpBodyStream_GetUserData(
    raw: *const ArkWeb_HttpBodyStream,
) -> *mut c_void {
    let state = &(*raw).state;
    state.user_data.get()
}

pub unsafe fn OH_ArkWebHttpBodyStream_SetReadCallback(
    raw: *mut ArkWeb_HttpBodyStream,
    callback: Option<ReadCallback>,
) -> i32 {
    let state = &(*raw).state;
    if state.config.read_callback_error == 0 {
        state.read_callback.set(callback);
    }
    state.config.read_callback_error
}

pub unsafe fn OH_ArkWebHttpBodyStream_Init(
    raw: *mut ArkWeb_HttpBodyStream,
    callback: Option<InitCallback>,
) -> i32 {
    let state = (*raw).state.clone();
    state.init_calls.set(state.init_calls.get() + 1);
    if state.config.init_return != 0 {
        return state.config.init_return;
    }
    state.init_callback.set(callback);
    if matches!(state.config.init_timing, Timing::Inline) {
        Native {
            raw,
            state: state.clone(),
        }
        .finish_init();
    }
    0
}

pub unsafe fn OH_ArkWebHttpBodyStream_Read(
    raw: *const ArkWeb_HttpBodyStream,
    buffer: *mut u8,
    length: i32,
) {
    let state = (*raw).state.clone();
    assert!(state.initialized.get());
    assert!(state.reads.borrow().is_empty(), "overlapping native reads");
    assert!(length > 0);
    state.reads.borrow_mut().push_back(PendingRead {
        buffer,
        length: length as usize,
    });
    let calls = state.read_calls.get() + 1;
    state.read_calls.set(calls);
    let inline = match state.config.read_timing {
        Timing::Inline => true,
        Timing::Deferred => false,
        Timing::Alternating => calls % 2 == 1,
    };
    if inline {
        state.read_depth.set(state.read_depth.get() + 1);
        state
            .max_read_depth
            .set(state.max_read_depth.get().max(state.read_depth.get()));
        Native {
            raw: raw.cast_mut(),
            state: state.clone(),
        }
        .complete_read(None, false);
        state.read_depth.set(state.read_depth.get() - 1);
    }
}

pub unsafe fn OH_ArkWebHttpBodyStream_GetSize(raw: *const ArkWeb_HttpBodyStream) -> u64 {
    let state = &(*raw).state;
    assert!(
        state.initialized.get(),
        "metadata queried before initialization"
    );
    if state.config.chunked {
        0
    } else {
        state.config.body.len() as u64
    }
}
pub unsafe fn OH_ArkWebHttpBodyStream_IsChunked(raw: *const ArkWeb_HttpBodyStream) -> bool {
    let state = &(*raw).state;
    assert!(state.initialized.get());
    state.config.chunked
}
pub unsafe fn OH_ArkWebHttpBodyStream_IsEof(raw: *const ArkWeb_HttpBodyStream) -> bool {
    let state = &(*raw).state;
    assert!(state.initialized.get());
    state.eof.get()
}
pub unsafe fn OH_ArkWebHttpBodyStream_IsInMemory(raw: *const ArkWeb_HttpBodyStream) -> bool {
    let state = &(*raw).state;
    assert!(state.initialized.get());
    matches!(state.config.read_timing, Timing::Inline)
}
pub unsafe fn OH_ArkWebResourceRequest_DestroyHttpBodyStream(raw: *mut ArkWeb_HttpBodyStream) {
    let native = Box::from_raw(raw);
    assert_eq!(
        native.state.read_depth.get(),
        0,
        "destroyed before native Read returned"
    );
    assert!(
        native.state.reads.borrow().is_empty(),
        "destroyed with a native write outstanding"
    );
    assert!(
        native.state.init_callback.get().is_none(),
        "destroyed before initialization callback"
    );
    native
        .state
        .destroy_calls
        .set(native.state.destroy_calls.get() + 1);
}

#[derive(Clone, Default)]
struct Results(Rc<RefCell<Vec<BodyResult>>>);
impl Results {
    fn callback(&self) -> impl FnOnce(BodyResult) + 'static {
        let results = self.clone();
        move |result| results.0.borrow_mut().push(result)
    }
    fn take(&self) -> Vec<BodyResult> {
        std::mem::take(&mut *self.0.borrow_mut())
    }
}

#[test]
fn initialize_returns_ownership_for_metadata_without_reading() {
    let (native, stream) = Native::create(Config {
        body: b"abc".to_vec(),
        init_timing: Timing::Deferred,
        ..Config::default()
    });
    let stream = stream.unwrap();
    assert_eq!(stream.size(), Err(HttpBodyStreamError::NotInitialized));
    let returned = Rc::new(RefCell::new(None));
    let slot = returned.clone();
    stream.initialize(move |result| *slot.borrow_mut() = Some(result.unwrap()));
    assert!(returned.borrow().is_none());
    assert_eq!(native.state.destroy_calls.get(), 0);
    native.finish_init();
    let stream = returned.borrow_mut().take().unwrap();
    assert_eq!(stream.size(), Ok(Some(3)));
    assert_eq!(stream.is_in_memory(), Ok(true));
    assert_eq!(stream.is_chunked(), Ok(false));
    assert_eq!(stream.is_eof(), Ok(false));
    assert_eq!(native.state.read_calls.get(), 0);
    drop(stream);
    assert_eq!(native.state.destroy_calls.get(), 1);
}

#[test]
fn reads_initialize_automatically_and_preserve_position() {
    let (native, stream) = Native::create(Config {
        body: b"abcdefghij".to_vec(),
        ..Config::default()
    });
    let output = Results::default();
    let cb = output.callback();
    stream.unwrap().read(3, move |result| {
        let (stream, first) = result.unwrap();
        assert_eq!(first, b"abc");
        stream.initialize(move |result| {
            result.unwrap().read(2, move |result| {
                let (stream, second) = result.unwrap();
                assert_eq!(second, b"de");
                stream.read_to_end(cb);
            });
        });
    });
    assert_eq!(output.take(), [Ok(b"fghij".to_vec())]);
    assert_eq!(native.state.init_calls.get(), 1);
    assert_eq!(native.state.destroy_calls.get(), 1);
}

#[test]
fn short_reads_handle_inline_deferred_and_mixed_completion() {
    for timing in [Timing::Inline, Timing::Deferred, Timing::Alternating] {
        for chunked in [false, true] {
            let body: Vec<u8> = (0..200_003).map(|i| (i % 251) as u8).collect();
            let (native, stream) = Native::create(Config {
                body: body.clone(),
                max_chunk: Some(37),
                read_timing: timing,
                chunked,
                ..Config::default()
            });
            let output = Results::default();
            stream.unwrap().read_to_end(output.callback());
            native.drain();
            assert_eq!(output.take(), [Ok(body)]);
            assert_eq!(native.state.init_calls.get(), 1);
            assert_eq!(
                native.state.max_read_depth.get(),
                usize::from(!matches!(timing, Timing::Deferred))
            );
            assert_eq!(native.state.destroy_calls.get(), 1);
        }
    }
}

#[test]
fn synchronous_short_reads_do_not_grow_the_stack() {
    let (native, stream) = Native::create(Config {
        body: vec![42; 100_000],
        max_chunk: Some(1),
        ..Config::default()
    });
    let output = Results::default();
    stream.unwrap().read_to_end(output.callback());
    assert_eq!(output.take(), [Ok(vec![42; 100_000])]);
    assert_eq!(native.state.read_calls.get(), 100_000);
    assert_eq!(native.state.max_read_depth.get(), 1);
    assert_eq!(native.state.destroy_calls.get(), 1);
}

#[test]
fn empty_and_unknown_length_bodies_are_distinct() {
    for chunked in [false, true] {
        let (native, stream) = Native::create(Config {
            chunked,
            ..Config::default()
        });
        let output = Results::default();
        let cb = output.callback();
        stream.unwrap().initialize(move |result| {
            let stream = result.unwrap();
            assert_eq!(stream.size(), Ok(if chunked { None } else { Some(0) }));
            assert_eq!(stream.is_eof(), Ok(!chunked));
            stream.read_to_end(cb);
        });
        assert_eq!(output.take(), [Ok(vec![])]);
        assert_eq!(native.state.read_calls.get(), usize::from(chunked));
        assert_eq!(native.state.destroy_calls.get(), 1);
    }
}

#[test]
fn zero_read_initializes_without_consuming_data() {
    let (native, stream) = Native::create(Config {
        body: b"abc".to_vec(),
        ..Config::default()
    });
    let output = Results::default();
    let cb = output.callback();
    stream.unwrap().read(0, move |result| {
        let (stream, bytes) = result.unwrap();
        assert!(bytes.is_empty());
        stream.read_to_end(cb);
    });
    assert_eq!(output.take(), [Ok(b"abc".to_vec())]);
    assert_eq!(native.state.read_calls.get(), 1);
    assert_eq!(native.state.init_calls.get(), 1);
}

#[test]
fn oversized_read_reports_error_and_closes_without_native_work() {
    let (native, stream) = Native::create(Config::default());
    let output = Results::default();
    let cb = output.callback();
    let size = i32::MAX as usize + 1;
    stream
        .unwrap()
        .read(size, move |result| cb(result.map(|(_, data)| data)));
    assert_eq!(
        output.take(),
        [Err(HttpBodyStreamError::InvalidReadSize(size))]
    );
    assert_eq!(native.state.init_calls.get(), 0);
    assert_eq!(native.state.read_calls.get(), 0);
    assert_eq!(native.state.destroy_calls.get(), 1);
}

#[test]
fn operation_owns_stream_and_buffer_until_deferred_completion() {
    let (native, stream) = Native::create(Config {
        body: b"native writes after submission returns".to_vec(),
        init_timing: Timing::Deferred,
        read_timing: Timing::Deferred,
        ..Config::default()
    });
    let output = Results::default();
    stream.unwrap().read_to_end(output.callback());
    // No public handle exists from this point. The operation owns the stream.
    assert!(output.take().is_empty());
    assert_eq!(native.state.destroy_calls.get(), 0);
    native.finish_init();
    assert!(output.take().is_empty());
    assert_eq!(native.state.destroy_calls.get(), 0);
    native.drain();
    assert_eq!(
        output.take(),
        [Ok(b"native writes after submission returns".to_vec())]
    );
    assert_eq!(native.state.destroy_calls.get(), 1);
}

#[test]
fn returned_stream_can_be_read_again_after_deferred_completion() {
    let (native, stream) = Native::create(Config {
        body: b"abcdef".to_vec(),
        read_timing: Timing::Deferred,
        ..Config::default()
    });
    let output = Results::default();
    let cb = output.callback();
    stream.unwrap().read(2, move |result| {
        let (stream, prefix) = result.unwrap();
        assert_eq!(prefix, b"ab");
        stream.read_to_end(cb);
    });
    native.drain();
    assert_eq!(output.take(), [Ok(b"cdef".to_vec())]);
    assert_eq!(native.state.init_calls.get(), 1);
    assert_eq!(native.state.destroy_calls.get(), 1);
}

#[test]
fn read_after_eof_returns_empty_without_native_read() {
    let (native, stream) = Native::create(Config {
        body: b"abc".to_vec(),
        ..Config::default()
    });
    stream.unwrap().read(10, |result| {
        let (stream, bytes) = result.unwrap();
        assert_eq!(bytes, b"abc");
        assert_eq!(stream.is_eof(), Ok(true));
        stream.read(10, |result| {
            let (stream, bytes) = result.unwrap();
            assert!(bytes.is_empty());
            drop(stream);
        });
    });
    assert_eq!(native.state.read_calls.get(), 1);
    assert_eq!(native.state.destroy_calls.get(), 1);
}

#[test]
fn initialization_errors_complete_once_and_release_operation() {
    for (init_return, init_result, timing) in [
        (17100101, 0, Timing::Inline),
        (0, -7, Timing::Inline),
        (0, -7, Timing::Deferred),
    ] {
        let (native, stream) = Native::create(Config {
            init_return,
            init_result,
            init_timing: timing,
            ..Config::default()
        });
        let output = Results::default();
        stream.unwrap().read_to_end(output.callback());
        if matches!(timing, Timing::Deferred) {
            native.finish_init();
        }
        let code = if init_return == 0 {
            init_result
        } else {
            init_return
        };
        assert_eq!(
            output.take(),
            [Err(HttpBodyStreamError::InitializationFailed(code))]
        );
        assert_eq!(native.state.read_calls.get(), 0);
        assert_eq!(native.state.destroy_calls.get(), 1);
    }
}

#[test]
fn native_read_errors_and_invalid_results_close_the_stream() {
    for (count, wrong_buffer, error) in [
        (-7, false, HttpBodyStreamError::ReadFailed(-7)),
        (
            65_537,
            false,
            HttpBodyStreamError::InvalidReadCount {
                count: 65_537,
                capacity: 65_536,
            },
        ),
        (0, false, HttpBodyStreamError::NoProgress),
        (2, true, HttpBodyStreamError::UnexpectedBuffer),
    ] {
        let (native, stream) = Native::create(Config {
            body: b"abcdef".to_vec(),
            read_timing: Timing::Deferred,
            ..Config::default()
        });
        let output = Results::default();
        stream.unwrap().read_to_end(output.callback());
        native.complete_read(Some(count), wrong_buffer);
        assert_eq!(output.take(), [Err(error)]);
        assert_eq!(native.state.destroy_calls.get(), 1);
    }
}

#[test]
fn registration_errors_report_once_and_destroy_the_owned_stream() {
    for (config, error) in [
        (
            Config {
                user_data_error: 17100101,
                ..Config::default()
            },
            HttpBodyStreamError::SetUserDataFailed(17100101),
        ),
        (
            Config {
                read_callback_error: 17100101,
                ..Config::default()
            },
            HttpBodyStreamError::SetReadCallbackFailed(17100101),
        ),
    ] {
        let (native, stream) = Native::create(config);
        let output = Results::default();
        stream.unwrap().read_to_end(output.callback());
        assert_eq!(output.take(), [Err(error)]);
        assert_eq!(native.state.init_calls.get(), 0);
        assert_eq!(native.state.destroy_calls.get(), 1);
    }
    assert!(matches!(
        unsafe { HttpBodyStream::new(std::ptr::null_mut()) },
        Err(HttpBodyStreamError::NullStream)
    ));
}

#[test]
fn unused_stream_is_destroyed_without_initialization() {
    let (native, stream) = Native::create(Config::default());
    drop(stream.unwrap());
    assert_eq!(native.state.init_calls.get(), 0);
    assert_eq!(native.state.destroy_calls.get(), 1);
}
