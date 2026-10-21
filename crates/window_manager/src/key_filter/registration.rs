use super::{KeyEvent, CALLBACKS};
use crate::{Error, Result, Window};
use std::{
    cell::Cell,
    sync::{mpsc, Arc, LazyLock, Mutex},
};

type Handler = Arc<dyn Fn(KeyEvent) -> bool + Send + Sync>;
struct Entry {
    window: Window,
    handler: Option<Handler>,
}
// API 12 has no callback user data or event-window getter. Each live or retiring
// registration reserves one trampoline until successful native unregister.
static FILTERS: LazyLock<Mutex<[Option<Entry>; 32]>> =
    LazyLock::new(|| Mutex::new(std::array::from_fn(|_| None)));
thread_local! {
    // The OHOS emulated-TLS expansion triggers this lint despite the const block.
    #[allow(clippy::missing_const_for_thread_local)]
    static CALLBACK_DEPTH: Cell<usize> = const { Cell::new(0) };
}

struct CallbackScope;
impl CallbackScope {
    fn enter() -> Self {
        CALLBACK_DEPTH.set(CALLBACK_DEPTH.get() + 1);
        Self
    }
}
impl Drop for CallbackScope {
    fn drop(&mut self) {
        CALLBACK_DEPTH.set(CALLBACK_DEPTH.get() - 1);
    }
}

pub(super) fn dispatch(slot: usize, event: KeyEvent) -> bool {
    let handler = FILTERS.lock().unwrap_or_else(|e| e.into_inner())[slot]
        .as_ref()
        .and_then(|entry| entry.handler.clone());
    let Some(handler) = handler else { return false };
    let _scope = CallbackScope::enter();
    // No registry lock or borrowed event survives invocation. Panics cannot
    // cross the native callback boundary, and scope also covers handler Drop.
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || handler(event))).unwrap_or(false)
}

fn complete_retirement(slot: usize) {
    let window = FILTERS.lock().unwrap_or_else(|e| e.into_inner())[slot]
        .as_ref()
        .map(|entry| entry.window);
    let Some(window) = window else { return };
    // API-20 WindowSessionImpl holds keyEventFilterMutex_ throughout callbacks
    // and ClearKeyEventFilter. Success waits for active callbacks and prevents
    // queued input from reading the old filter, so this trampoline can be reused.
    if window.unregister_key_event_filter().is_ok() {
        let entry = FILTERS.lock().unwrap_or_else(|e| e.into_inner())[slot].take();
        drop(entry);
    }
}

// Native unregister takes a nonrecursive filter mutex. Callback-thread Drop
// cannot acquire it; one lazy worker handles all such retirements. Startup or
// delivery failure leaves the handler-free slot reserved, just as native failure.
static CLEANUP: LazyLock<Option<mpsc::Sender<usize>>> = LazyLock::new(|| {
    let (sender, receiver) = mpsc::channel();
    std::thread::Builder::new()
        .name("ohos-key-filter-cleanup".into())
        .spawn(move || {
            for slot in receiver {
                complete_retirement(slot);
            }
        })
        .ok()
        .map(|_| sender)
});

/// Exclusive API-12 window filter with owned key snapshots.
///
/// At most 32 windows can be live or retiring concurrently; successful
/// unregister releases the slot for reuse. Failed unregister quarantines the
/// slot with delivery disabled. Do not replace this window's native filter
/// through another API while its owner is live.
pub struct WindowKeyFilter {
    slot: usize,
}
impl WindowKeyFilter {
    /// Installs a filter that returns true to consume a key.
    ///
    /// Callbacks may run off the ArkTS thread and must only use owned data.
    /// Drop normally unregisters synchronously. Drop from any key callback
    /// disables delivery immediately and completes native retirement on the
    /// shared cleanup worker to avoid the native callback mutex.
    pub fn register(
        window: Window,
        handler: impl Fn(KeyEvent) -> bool + Send + Sync + 'static,
    ) -> Result<Self> {
        let slot = {
            let mut entries = FILTERS.lock().unwrap_or_else(|e| e.into_inner());
            if entries.iter().flatten().any(|entry| entry.window == window) {
                return Err(Error::KeyFilterInUse);
            }
            let slot = entries
                .iter()
                .position(Option::is_none)
                .ok_or(Error::KeyFilterLimit)?;
            entries[slot] = Some(Entry {
                window,
                handler: Some(Arc::new(handler)),
            });
            slot
        };
        if let Err(error) = window.register_key_event_filter(CALLBACKS[slot]) {
            let entry = FILTERS.lock().unwrap_or_else(|e| e.into_inner())[slot].take();
            drop(entry);
            return Err(error);
        }
        Ok(Self { slot })
    }
}
impl Drop for WindowKeyFilter {
    fn drop(&mut self) {
        let handler = FILTERS.lock().unwrap_or_else(|e| e.into_inner())[self.slot]
            .as_mut()
            .and_then(|entry| entry.handler.take());
        drop(handler);
        if CALLBACK_DEPTH.get() == 0 {
            complete_retirement(self.slot);
        } else if let Some(sender) = &*CLEANUP {
            let _ = sender.send(self.slot);
        }
    }
}
