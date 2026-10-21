//! Production-owner tests using native symbol substitutes on the host.
#![allow(non_snake_case)]

use super::*;
use std::{io::Write, os::unix::net::UnixStream};

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
});

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
    0
}
#[no_mangle]
unsafe extern "C" fn OH_NativeImage_Destroy(raw: *mut *mut OH_NativeImage) {
    STATE.lock().unwrap().destroyed += 1;
    // SAFETY: the production owner supplies its live local pointer slot.
    unsafe { *raw = std::ptr::null_mut() };
}
#[no_mangle]
unsafe extern "C" fn OH_NativeImage_AcquireNativeWindowBuffer(
    _: *mut OH_NativeImage,
    buffer: *mut *mut OHNativeWindowBuffer,
    fence: *mut i32,
) -> i32 {
    let mut state = STATE.lock().unwrap();
    if !std::mem::take(&mut state.pending) {
        return 40601000;
    }
    // SAFETY: the production acquisition supplies two writable output slots.
    unsafe {
        *buffer = NonNull::dangling().as_ptr();
        *fence = std::mem::replace(&mut state.fence, -1);
    }
    0
}
#[no_mangle]
unsafe extern "C" fn OH_NativeWindow_NativeObjectReference(_: *mut c_void) -> i32 {
    STATE.lock().unwrap().references += 1;
    0
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
        // SAFETY: production Frame transfers this descriptor to the native release.
        drop(unsafe { OwnedFd::from_raw_fd(fence) });
    }
    0
}

#[test]
fn guards_return_unsignalled_fences_and_retire_callback_identities() {
    let _serial = TEST_LOCK.lock().unwrap();
    *STATE.lock().unwrap() = State {
        fence: -1,
        ..State::default()
    };
    use std::os::fd::IntoRawFd;
    let calls = Arc::new(AtomicUsize::new(0));
    let captured = calls.clone();
    let mut surface = ConsumerSurface::new(move || {
        captured.fetch_add(1, Ordering::Relaxed);
    })
    .unwrap();
    let (callback, context) = {
        let state = STATE.lock().unwrap();
        (state.callback.unwrap(), state.context)
    };
    // SAFETY: the substitute calls the registered callback with its unchanged context.
    unsafe { callback(context as *mut c_void) };
    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert!(surface.acquire().unwrap().is_none());

    let (fence, _pending_producer) = UnixStream::pair().unwrap();
    {
        let mut state = STATE.lock().unwrap();
        state.pending = true;
        state.fence = fence.into_raw_fd();
    }
    let mut frame = surface.acquire().unwrap().unwrap();
    assert!(!frame.wait(Duration::ZERO).unwrap());
    drop(frame);
    {
        let state = STATE.lock().unwrap();
        assert!(state.returned_fence);
        assert_eq!(
            (state.references, state.unreferences, state.releases),
            (1, 1, 1)
        );
    }

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
    assert!(!STATE.lock().unwrap().returned_fence);
    drop(surface);
    assert_eq!(STATE.lock().unwrap().destroyed, 1);
    // SAFETY: late native callbacks carry the retired integer identity, not an address.
    unsafe { callback(context as *mut c_void) };
    assert_eq!(calls.load(Ordering::Relaxed), 1);
}

#[test]
fn callback_owner_destruction_runs_outside_the_registry_and_inside_ffi_catch() {
    use std::sync::atomic::AtomicBool;
    let _serial = TEST_LOCK.lock().unwrap();
    struct Reenter(Arc<AtomicBool>);
    impl Drop for Reenter {
        fn drop(&mut self) {
            self.0
                .store(callbacks().try_lock().is_ok(), Ordering::Relaxed);
        }
    }
    let unlocked = Arc::new(AtomicBool::new(false));
    let owner = Reenter(unlocked.clone());
    let surface = ConsumerSurface::new(move || {
        let _ = &owner;
    })
    .unwrap();
    drop(surface);
    assert!(unlocked.load(Ordering::Relaxed));

    struct PanicOnDrop;
    impl Drop for PanicOnDrop {
        fn drop(&mut self) {
            panic!("callback owner destructor");
        }
    }
    let owner = PanicOnDrop;
    let identity = usize::MAX;
    callbacks().lock().unwrap().insert(
        identity,
        Arc::new(move || {
            let _ = &owner;
            // Model concurrent registry retirement while available owns its clone.
            callbacks().lock().unwrap().remove(&identity);
        }),
    );
    // SAFETY: the callback context is only an integer registry identity. Its
    // final captured-owner panic must be contained before crossing this C ABI.
    unsafe { available(identity as *mut c_void) };
    assert!(!callbacks().lock().unwrap().contains_key(&identity));
}
