//! Production registration lifecycle with the native callback mutex substituted.
#![allow(dead_code)]
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc, Arc, Barrier, LazyLock, Mutex,
    },
    time::{Duration, Instant},
};
#[derive(Clone, Copy)]
struct KeyEvent(u32);
const CALLBACKS: [usize; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31,
];
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Window(u32);
#[derive(Debug, PartialEq)]
enum Error {
    KeyFilterInUse,
    KeyFilterLimit,
    Native,
}
type Result<T> = std::result::Result<T, Error>;
#[derive(Default)]
struct Native {
    filter: Mutex<Option<usize>>,
    fail: AtomicBool,
    retired: AtomicUsize,
}
static WINDOWS: LazyLock<Mutex<HashMap<Window, Arc<Native>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
impl Window {
    fn native(self) -> Arc<Native> {
        WINDOWS.lock().unwrap().entry(self).or_default().clone()
    }
    fn register_key_event_filter(self, slot: usize) -> Result<()> {
        if self.0 == 999 {
            return Err(Error::Native);
        }
        *self.native().filter.lock().unwrap() = Some(slot);
        Ok(())
    }
    fn unregister_key_event_filter(self) -> Result<()> {
        let native = self.native();
        let mut filter = native.filter.lock().unwrap();
        if native.fail.load(Ordering::SeqCst) {
            return Err(Error::Native);
        }
        *filter = None;
        native.retired.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
    fn emit(self) -> bool {
        let native = self.native();
        let filter = native.filter.lock().unwrap();
        filter.is_some_and(|slot| registration::dispatch(slot, KeyEvent(7)))
    }
}
#[path = "../src/key_filter/registration.rs"]
mod registration;
use registration::WindowKeyFilter;
fn wait_for(mut ready: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(3);
    while !ready() {
        assert!(Instant::now() < deadline, "cleanup did not complete");
        std::thread::yield_now();
    }
}
#[test]
fn unregister_waits_for_callback_and_slots_reuse_across_more_than_32_lifetimes() {
    let window = Window(1);
    let entered = Arc::new(Barrier::new(2));
    let release = Arc::new(Barrier::new(2));
    let guard = WindowKeyFilter::register(window, {
        let entered = entered.clone();
        let release = release.clone();
        move |event| {
            assert_eq!(event.0, 7);
            entered.wait();
            release.wait();
            true
        }
    })
    .unwrap();
    assert!(matches!(
        WindowKeyFilter::register(window, |_| false),
        Err(Error::KeyFilterInUse)
    ));
    let callback = std::thread::spawn(move || window.emit());
    entered.wait();
    let (sender, receiver) = mpsc::channel();
    let retiring = std::thread::spawn(move || {
        drop(guard);
        sender.send(()).unwrap()
    });
    assert!(receiver.recv_timeout(Duration::from_millis(30)).is_err());
    release.wait();
    assert!(callback.join().unwrap());
    retiring.join().unwrap();
    receiver.recv_timeout(Duration::from_secs(1)).unwrap();
    for _ in 0..96 {
        drop(WindowKeyFilter::register(window, |_| false).unwrap());
    }
    assert_eq!(window.native().retired.load(Ordering::SeqCst), 97);
    assert!(matches!(
        WindowKeyFilter::register(Window(999), |_| false),
        Err(Error::Native)
    ));
}
#[test]
fn callback_drop_uses_worker_and_panic_does_not_escape_or_poison_retirement() {
    let window = Window(2);
    let owner = Arc::new(Mutex::new(None));
    let guard = WindowKeyFilter::register(window, {
        let owner = owner.clone();
        move |_| {
            drop(owner.lock().unwrap().take());
            true
        }
    })
    .unwrap();
    *owner.lock().unwrap() = Some(guard);
    assert!(window.emit());
    wait_for(|| window.native().retired.load(Ordering::SeqCst) == 1);
    wait_for(|| WindowKeyFilter::register(window, |_| false).is_ok());
    let panic_window = Window(3);
    let guard = WindowKeyFilter::register(panic_window, |_| panic!("callback failure")).unwrap();
    assert!(!panic_window.emit());
    drop(guard);
    assert_eq!(panic_window.native().retired.load(Ordering::SeqCst), 1);
}
#[test]
fn native_unregister_failure_disables_delivery_and_quarantines_its_slot() {
    let window = Window(4);
    let calls = Arc::new(AtomicUsize::new(0));
    let guard = WindowKeyFilter::register(window, {
        let calls = calls.clone();
        move |_| {
            calls.fetch_add(1, Ordering::SeqCst);
            true
        }
    })
    .unwrap();
    assert!(window.emit());
    window.native().fail.store(true, Ordering::SeqCst);
    drop(guard);
    assert!(!window.emit());
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(matches!(
        WindowKeyFilter::register(window, |_| true),
        Err(Error::KeyFilterInUse)
    ));
}
