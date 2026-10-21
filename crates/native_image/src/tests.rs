//! Production owners exercised through statically linked native substitutes.
#![allow(non_snake_case)]

use super::*;
use std::{
    ffi::c_void,
    io::Write,
    os::unix::net::UnixStream,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

static TEST_LOCK: Mutex<()> = Mutex::new(());

#[derive(Default)]
struct State {
    context: usize,
    callback: OH_OnFrameAvailable,
    pending: bool,
    fence: i32,
    releases: usize,
    references: usize,
    unreferences: usize,
    destroyed: usize,
    returned_fence: bool,
    windows: usize,
    #[cfg_attr(not(feature = "api-13"), expect(dead_code))]
    usage: u64,
    #[cfg_attr(not(feature = "api-13"), expect(dead_code))]
    usage_status: i32,
    retain_status: i32,
    #[cfg_attr(not(feature = "api-13"), expect(dead_code))]
    size: (i32, i32),
}
static STATE: Mutex<State> = Mutex::new(State {
    context: 0,
    callback: None,
    pending: false,
    fence: -1,
    releases: 0,
    references: 0,
    unreferences: 0,
    destroyed: 0,
    returned_fence: false,
    windows: 0,
    usage: 0,
    usage_status: 0,
    retain_status: 0,
    size: (0, 0),
});

fn reset() {
    *STATE.lock().unwrap() = State {
        fence: -1,
        ..State::default()
    };
}

#[no_mangle]
unsafe extern "C" fn OH_ConsumerSurface_Create() -> *mut OH_NativeImage {
    NonNull::dangling().as_ptr()
}

#[no_mangle]
unsafe extern "C" fn OH_NativeImage_SetOnFrameAvailableListener(
    _: *mut OH_NativeImage,
    listener: OH_OnFrameAvailableListener,
) -> i32 {
    let mut state = STATE.lock().unwrap();
    state.context = listener.context as usize;
    state.callback = listener.onFrameAvailable;
    0
}

#[no_mangle]
unsafe extern "C" fn OH_NativeImage_UnsetOnFrameAvailableListener(_: *mut OH_NativeImage) -> i32 {
    STATE.lock().unwrap().callback = None;
    0
}

#[no_mangle]
unsafe extern "C" fn OH_NativeImage_Destroy(raw: *mut *mut OH_NativeImage) {
    STATE.lock().unwrap().destroyed += 1;
    // SAFETY: the production owner supplies a live local pointer slot.
    unsafe { *raw = std::ptr::null_mut() };
}

#[no_mangle]
unsafe extern "C" fn OH_NativeImage_AcquireNativeWindow(
    _: *mut OH_NativeImage,
) -> *mut OHNativeWindow {
    STATE.lock().unwrap().windows += 1;
    NonNull::dangling().as_ptr()
}

#[no_mangle]
unsafe extern "C" fn OH_NativeImage_GetSurfaceId(_: *mut OH_NativeImage, id: *mut u64) -> i32 {
    // SAFETY: the production query provides a writable u64.
    unsafe { *id = 1234 };
    0
}

#[cfg(feature = "api-13")]
#[no_mangle]
unsafe extern "C" fn OH_ConsumerSurface_SetDefaultUsage(_: *mut OH_NativeImage, usage: u64) -> i32 {
    let mut state = STATE.lock().unwrap();
    state.usage = usage;
    state.usage_status
}

#[cfg(feature = "api-13")]
#[no_mangle]
unsafe extern "C" fn OH_ConsumerSurface_SetDefaultSize(
    _: *mut OH_NativeImage,
    w: i32,
    h: i32,
) -> i32 {
    STATE.lock().unwrap().size = (w, h);
    0
}

#[no_mangle]
unsafe extern "C" fn OH_NativeImage_AcquireNativeWindowBuffer(
    _: *mut OH_NativeImage,
    buffer: *mut *mut OHNativeWindowBuffer,
    fence: *mut i32,
) -> i32 {
    let mut state = STATE.lock().unwrap();
    if !std::mem::take(&mut state.pending) {
        return NATIVE_ERROR_NO_BUFFER;
    }
    // SAFETY: the production acquisition provides writable output slots.
    unsafe {
        *buffer = NonNull::dangling().as_ptr();
        *fence = std::mem::replace(&mut state.fence, -1);
    }
    0
}

#[no_mangle]
unsafe extern "C" fn OH_NativeWindow_NativeObjectReference(_: *mut c_void) -> i32 {
    let mut state = STATE.lock().unwrap();
    state.references += 1;
    state.retain_status
}

#[no_mangle]
unsafe extern "C" fn OH_NativeWindow_NativeObjectUnreference(_: *mut c_void) -> i32 {
    STATE.lock().unwrap().unreferences += 1;
    0
}

#[no_mangle]
unsafe extern "C" fn OH_NativeImage_ReleaseNativeWindowBuffer(
    _: *mut OH_NativeImage,
    _: *mut OHNativeWindowBuffer,
    fence: i32,
) -> i32 {
    let mut state = STATE.lock().unwrap();
    state.releases += 1;
    state.returned_fence = fence >= 0;
    if fence >= 0 {
        // SAFETY: Frame transfers its uniquely owned descriptor to native release.
        drop(unsafe { OwnedFd::from_raw_fd(fence) });
    }
    0
}

unsafe extern "C" fn count_frame(context: *mut c_void) {
    // SAFETY: the test retains its Arc until this synchronous callback completes.
    unsafe { &*context.cast::<AtomicUsize>() }.fetch_add(1, Ordering::Relaxed);
}

#[test]
fn caller_owns_listener_context_and_identity_query_has_no_window_side_effect() {
    let _serial = TEST_LOCK.lock().unwrap();
    reset();
    let calls = Arc::new(AtomicUsize::new(0));
    let mut surface = ConsumerSurface::new().unwrap();
    assert_eq!(surface.surface_id().unwrap(), 1234);
    assert_eq!(STATE.lock().unwrap().windows, 0);
    surface.acquire_native_window().unwrap();
    assert_eq!(STATE.lock().unwrap().windows, 1);
    // SAFETY: Arc storage outlives registration and mock callbacks cannot outlive
    // the synchronous invocation below. count_frame neither panics nor reenters.
    unsafe {
        surface.set_frame_available_listener(FrameAvailableListener {
            context: Arc::as_ptr(&calls).cast_mut().cast(),
            onFrameAvailable: Some(count_frame),
        })
    }
    .unwrap();
    let (callback, context) = {
        let state = STATE.lock().unwrap();
        (state.callback.unwrap(), state.context)
    };
    // SAFETY: the callback receives its unchanged, still-live context.
    unsafe { callback(context as *mut c_void) };
    assert_eq!(calls.load(Ordering::Relaxed), 1);
    surface.unset_frame_available_listener().unwrap();
    assert!(STATE.lock().unwrap().callback.is_none());
    drop(surface);
    assert_eq!(STATE.lock().unwrap().destroyed, 1);
    assert_eq!(Arc::strong_count(&calls), 1);
}

#[test]
fn frame_guards_return_unsignalled_fences_and_release_after_wait() {
    let _serial = TEST_LOCK.lock().unwrap();
    reset();
    let mut surface = ConsumerSurface::new().unwrap();
    assert!(surface.acquire().unwrap().is_none());
    let (fence, _producer) = UnixStream::pair().unwrap();
    {
        let mut state = STATE.lock().unwrap();
        state.pending = true;
        state.fence = fence.into_raw_fd();
    }
    let mut frame = surface.acquire().unwrap().unwrap();
    assert!(!frame.wait(Duration::ZERO).unwrap());
    assert_eq!(frame.native_window_buffer(), NonNull::dangling());
    drop(frame);
    assert!(STATE.lock().unwrap().returned_fence);
    let (fence, mut producer) = UnixStream::pair().unwrap();
    {
        let mut state = STATE.lock().unwrap();
        state.pending = true;
        state.fence = fence.into_raw_fd();
    }
    producer.write_all(&[1]).unwrap();
    let mut frame = surface.acquire().unwrap().unwrap();
    assert!(frame.wait(Duration::ZERO).unwrap());
    drop(frame);
    let state = STATE.lock().unwrap();
    assert!(!state.returned_fence);
    assert_eq!(
        (state.references, state.unreferences, state.releases),
        (2, 2, 2)
    );
}

#[test]
fn retain_failure_returns_the_acquisition_and_transfers_its_fence() {
    let _serial = TEST_LOCK.lock().unwrap();
    reset();
    let mut surface = ConsumerSurface::new().unwrap();
    let (fence, _producer) = UnixStream::pair().unwrap();
    {
        let mut state = STATE.lock().unwrap();
        state.pending = true;
        state.fence = fence.into_raw_fd();
        state.retain_status = -77;
    }
    assert_eq!(surface.acquire().unwrap_err().code(), -77);
    let state = STATE.lock().unwrap();
    assert_eq!(
        (state.references, state.unreferences, state.releases),
        (1, 0, 1)
    );
    assert!(state.returned_fence);
}

#[cfg(feature = "api-13")]
#[test]
fn usage_is_caller_selected_and_failed_configuration_disposes_the_owner() {
    let _serial = TEST_LOCK.lock().unwrap();
    reset();
    let usage = BufferUsage::CPU_READ | BufferUsage::HW_TEXTURE | BufferUsage::from_bits(1 << 40);
    let surface = ConsumerSurface::with_usage(usage).unwrap();
    assert_eq!(STATE.lock().unwrap().usage, usage.bits());
    drop(surface);
    STATE.lock().unwrap().usage_status = -12;
    assert!(matches!(ConsumerSurface::with_usage(usage), Err(e) if e.code() == -12));
    assert_eq!(STATE.lock().unwrap().destroyed, 2);
}

#[cfg(feature = "api-13")]
#[test]
fn resize_checks_only_native_dimension_representability() {
    let _serial = TEST_LOCK.lock().unwrap();
    reset();
    let mut surface = ConsumerSurface::new().unwrap();
    assert!(surface.resize(0, 1).is_err());
    assert!(surface.resize(u32::MAX, 1).is_err());
    surface.resize(i32::MAX as u32, 1).unwrap();
    assert_eq!(STATE.lock().unwrap().size, (i32::MAX, 1));
}
