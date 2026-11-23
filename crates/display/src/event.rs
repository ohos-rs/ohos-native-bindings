use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, LazyLock, Mutex};

use ohos_display_sys::{
    NativeDisplayManager_ErrorCode, NativeDisplayManager_FoldDisplayMode,
    OH_NativeDisplayManager_RegisterDisplayChangeListener,
    OH_NativeDisplayManager_RegisterFoldDisplayModeChangeListener,
    OH_NativeDisplayManager_UnregisterDisplayChangeListener,
    OH_NativeDisplayManager_UnregisterFoldDisplayModeChangeListener,
};

#[cfg(feature = "api-20")]
use ohos_display_sys::{
    OH_NativeDisplayManager_RegisterAvailableAreaChangeListener,
    OH_NativeDisplayManager_RegisterDisplayAddListener,
    OH_NativeDisplayManager_RegisterDisplayRemoveListener,
    OH_NativeDisplayManager_UnregisterAvailableAreaChangeListener,
    OH_NativeDisplayManager_UnregisterDisplayAddListener,
    OH_NativeDisplayManager_UnregisterDisplayRemoveListener,
};

use crate::{
    error::{check_status, Result},
    FoldMode,
};

type Callback<T> = Arc<dyn Fn(T) + Send + Sync + 'static>;

struct ListenerHub<T> {
    next_token: u64,
    native_index: Option<u32>,
    subscribers: Vec<(u64, Callback<T>)>,
}

impl<T: Copy> ListenerHub<T> {
    fn new() -> Self {
        Self {
            next_token: 1,
            native_index: None,
            subscribers: Vec::new(),
        }
    }

    fn register<R>(&mut self, callback: Callback<T>, register_native: R) -> Result<u64>
    where
        R: FnOnce() -> Result<u32>,
    {
        if self.native_index.is_none() {
            let index = register_native()?;
            self.native_index = Some(index);
        }
        let token = self.next_token;
        self.next_token = self.next_token.wrapping_add(1).max(1);
        self.subscribers.push((token, callback));
        Ok(token)
    }

    fn unregister<U>(&mut self, token: u64, unregister_native: U) -> Result<()>
    where
        U: FnOnce(u32) -> Result<()>,
    {
        let Some(pos) = self.subscribers.iter().position(|(id, _)| *id == token) else {
            return Ok(());
        };
        self.subscribers.swap_remove(pos);
        if self.subscribers.is_empty() {
            if let Some(index) = self.native_index.take() {
                if let Err(err) = unregister_native(index) {
                    self.native_index = Some(index);
                    return Err(err);
                }
            }
        }
        Ok(())
    }

    fn snapshot(&self) -> Vec<Callback<T>> {
        self.subscribers
            .iter()
            .map(|(_, cb)| Arc::clone(cb))
            .collect()
    }
}

fn dispatch_hub<T: Copy>(hub: &Mutex<ListenerHub<T>>, event: T) {
    let callbacks = match hub.lock() {
        Ok(guard) => guard.snapshot(),
        Err(poisoned) => poisoned.into_inner().snapshot(),
    };
    for callback in callbacks {
        let _ = catch_unwind(AssertUnwindSafe(|| callback(event)));
    }
}

static DISPLAY_CHANGE_HUB: LazyLock<Mutex<ListenerHub<u64>>> =
    LazyLock::new(|| Mutex::new(ListenerHub::new()));
static FOLD_DISPLAY_MODE_HUB: LazyLock<Mutex<ListenerHub<FoldMode>>> =
    LazyLock::new(|| Mutex::new(ListenerHub::new()));

#[cfg(feature = "api-20")]
static DISPLAY_ADD_HUB: LazyLock<Mutex<ListenerHub<u64>>> =
    LazyLock::new(|| Mutex::new(ListenerHub::new()));
#[cfg(feature = "api-20")]
static DISPLAY_REMOVE_HUB: LazyLock<Mutex<ListenerHub<u64>>> =
    LazyLock::new(|| Mutex::new(ListenerHub::new()));
#[cfg(feature = "api-20")]
static AVAILABLE_AREA_CHANGE_HUB: LazyLock<Mutex<ListenerHub<u64>>> =
    LazyLock::new(|| Mutex::new(ListenerHub::new()));

unsafe extern "C" fn on_display_change_trampoline(display_id: u64) {
    dispatch_hub(&DISPLAY_CHANGE_HUB, display_id);
}

unsafe extern "C" fn on_fold_display_mode_change_trampoline(
    mode: NativeDisplayManager_FoldDisplayMode,
) {
    dispatch_hub(&FOLD_DISPLAY_MODE_HUB, FoldMode::from(mode));
}

#[cfg(feature = "api-20")]
unsafe extern "C" fn on_display_add_trampoline(display_id: u64) {
    dispatch_hub(&DISPLAY_ADD_HUB, display_id);
}

#[cfg(feature = "api-20")]
unsafe extern "C" fn on_display_remove_trampoline(display_id: u64) {
    dispatch_hub(&DISPLAY_REMOVE_HUB, display_id);
}

#[cfg(feature = "api-20")]
unsafe extern "C" fn on_available_area_change_trampoline(display_id: u64) {
    dispatch_hub(&AVAILABLE_AREA_CHANGE_HUB, display_id);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ListenerKind {
    DisplayChange,
    FoldDisplayModeChange,
    #[cfg(feature = "api-20")]
    DisplayAdd,
    #[cfg(feature = "api-20")]
    DisplayRemove,
    #[cfg(feature = "api-20")]
    AvailableAreaChange,
}

/// RAII handle for a registered native display listener.
///
/// Dropping this handle removes the Rust callback and unregisters the underlying
/// native listener once no subscribers remain for that event kind.
#[must_use = "dropping a DisplayListener unregisters it immediately"]
pub struct DisplayListener {
    kind: ListenerKind,
    token: Option<u64>,
}

impl DisplayListener {
    /// Explicitly unregisters this listener and surfaces any native unregister error.
    pub fn unregister(mut self) -> Result<()> {
        self.unregister_inner()
    }

    fn unregister_inner(&mut self) -> Result<()> {
        let Some(token) = self.token.take() else {
            return Ok(());
        };
        match self.kind {
            ListenerKind::DisplayChange => unregister_from(&DISPLAY_CHANGE_HUB, token, |idx| {
                check_status(unsafe {
                    OH_NativeDisplayManager_UnregisterDisplayChangeListener(idx)
                })
            }),
            ListenerKind::FoldDisplayModeChange => {
                unregister_from(&FOLD_DISPLAY_MODE_HUB, token, |idx| {
                    check_status(unsafe {
                        OH_NativeDisplayManager_UnregisterFoldDisplayModeChangeListener(idx)
                    })
                })
            }
            #[cfg(feature = "api-20")]
            ListenerKind::DisplayAdd => unregister_from(&DISPLAY_ADD_HUB, token, |idx| {
                check_status(unsafe { OH_NativeDisplayManager_UnregisterDisplayAddListener(idx) })
            }),
            #[cfg(feature = "api-20")]
            ListenerKind::DisplayRemove => unregister_from(&DISPLAY_REMOVE_HUB, token, |idx| {
                check_status(unsafe {
                    OH_NativeDisplayManager_UnregisterDisplayRemoveListener(idx)
                })
            }),
            #[cfg(feature = "api-20")]
            ListenerKind::AvailableAreaChange => {
                unregister_from(&AVAILABLE_AREA_CHANGE_HUB, token, |idx| {
                    check_status(unsafe {
                        OH_NativeDisplayManager_UnregisterAvailableAreaChangeListener(idx)
                    })
                })
            }
        }
    }
}

impl Drop for DisplayListener {
    fn drop(&mut self) {
        let _ = self.unregister_inner();
    }
}

fn register_into<T: Copy, R>(
    hub: &Mutex<ListenerHub<T>>,
    kind: ListenerKind,
    callback: Callback<T>,
    register_native: R,
) -> Result<DisplayListener>
where
    R: FnOnce(&mut u32) -> NativeDisplayManager_ErrorCode,
{
    let mut guard = match hub.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let token = guard.register(callback, || {
        let mut index = 0;
        check_status(register_native(&mut index))?;
        Ok(index)
    })?;
    Ok(DisplayListener {
        kind,
        token: Some(token),
    })
}

fn unregister_from<T: Copy, U>(
    hub: &Mutex<ListenerHub<T>>,
    token: u64,
    unregister_native: U,
) -> Result<()>
where
    U: FnOnce(u32) -> Result<()>,
{
    let mut guard = match hub.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    guard.unregister(token, unregister_native)
}

/// Registers a listener for display status changes such as rotation, DPI, refresh
/// rate, or resolution changes (`@since 12`).
pub fn on_display_change<F>(callback: F) -> Result<DisplayListener>
where
    F: Fn(u64) + Send + Sync + 'static,
{
    register_into(
        &DISPLAY_CHANGE_HUB,
        ListenerKind::DisplayChange,
        Arc::new(callback),
        |index| unsafe {
            OH_NativeDisplayManager_RegisterDisplayChangeListener(
                Some(on_display_change_trampoline),
                index,
            )
        },
    )
}

/// Registers a listener for foldable display mode changes (`@since 12`).
pub fn on_fold_display_mode_change<F>(callback: F) -> Result<DisplayListener>
where
    F: Fn(FoldMode) + Send + Sync + 'static,
{
    register_into(
        &FOLD_DISPLAY_MODE_HUB,
        ListenerKind::FoldDisplayModeChange,
        Arc::new(callback),
        |index| unsafe {
            OH_NativeDisplayManager_RegisterFoldDisplayModeChangeListener(
                Some(on_fold_display_mode_change_trampoline),
                index,
            )
        },
    )
}

/// Registers a listener for display hot-plug addition events (`@since 20`).
#[cfg(feature = "api-20")]
pub fn on_display_add<F>(callback: F) -> Result<DisplayListener>
where
    F: Fn(u64) + Send + Sync + 'static,
{
    register_into(
        &DISPLAY_ADD_HUB,
        ListenerKind::DisplayAdd,
        Arc::new(callback),
        |index| unsafe {
            OH_NativeDisplayManager_RegisterDisplayAddListener(
                Some(on_display_add_trampoline),
                index,
            )
        },
    )
}

/// Registers a listener for display hot-plug removal events (`@since 20`).
#[cfg(feature = "api-20")]
pub fn on_display_remove<F>(callback: F) -> Result<DisplayListener>
where
    F: Fn(u64) + Send + Sync + 'static,
{
    register_into(
        &DISPLAY_REMOVE_HUB,
        ListenerKind::DisplayRemove,
        Arc::new(callback),
        |index| unsafe {
            OH_NativeDisplayManager_RegisterDisplayRemoveListener(
                Some(on_display_remove_trampoline),
                index,
            )
        },
    )
}

/// Registers a listener for display available-area changes (`@since 20`).
#[cfg(feature = "api-20")]
pub fn on_available_area_change<F>(callback: F) -> Result<DisplayListener>
where
    F: Fn(u64) + Send + Sync + 'static,
{
    register_into(
        &AVAILABLE_AREA_CHANGE_HUB,
        ListenerKind::AvailableAreaChange,
        Arc::new(callback),
        |index| unsafe {
            OH_NativeDisplayManager_RegisterAvailableAreaChangeListener(
                Some(on_available_area_change_trampoline),
                index,
            )
        },
    )
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;
    use crate::DisplayError;

    #[test]
    fn listener_hub_registers_once_fans_out_and_unregisters_on_last_drop() {
        let hub = Arc::new(Mutex::new(ListenerHub::<u64>::new()));
        let native_reg = Arc::new(AtomicUsize::new(0));
        let native_unreg = Arc::new(AtomicUsize::new(0));
        let received_a = Arc::new(Mutex::new(Vec::new()));
        let received_b = Arc::new(Mutex::new(Vec::new()));

        let t1 = {
            let mut guard = hub.lock().unwrap();
            let reg = Arc::clone(&native_reg);
            let dst = Arc::clone(&received_a);
            guard
                .register(Arc::new(move |id| dst.lock().unwrap().push(id)), || {
                    reg.fetch_add(1, Ordering::SeqCst);
                    Ok(7)
                })
                .unwrap()
        };
        let t2 = {
            let mut guard = hub.lock().unwrap();
            let reg = Arc::clone(&native_reg);
            let dst = Arc::clone(&received_b);
            guard
                .register(Arc::new(move |id| dst.lock().unwrap().push(id)), || {
                    reg.fetch_add(1, Ordering::SeqCst);
                    Ok(99)
                })
                .unwrap()
        };
        assert_eq!(native_reg.load(Ordering::SeqCst), 1);

        dispatch_hub(&hub, 42);
        assert_eq!(*received_a.lock().unwrap(), vec![42]);
        assert_eq!(*received_b.lock().unwrap(), vec![42]);

        {
            let mut guard = hub.lock().unwrap();
            guard
                .unregister(t1, |_| {
                    native_unreg.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                })
                .unwrap();
        }
        assert_eq!(native_unreg.load(Ordering::SeqCst), 0);

        dispatch_hub(&hub, 99);
        assert_eq!(*received_a.lock().unwrap(), vec![42]);
        assert_eq!(*received_b.lock().unwrap(), vec![42, 99]);

        {
            let mut guard = hub.lock().unwrap();
            guard
                .unregister(t2, |idx| {
                    assert_eq!(idx, 7);
                    native_unreg.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                })
                .unwrap();
        }
        assert_eq!(native_unreg.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn listener_hub_isolates_panics_and_allows_reentrant_unregister() {
        let hub = Arc::new(Mutex::new(ListenerHub::<u64>::new()));
        let seen = Arc::new(AtomicUsize::new(0));

        let mut guard = hub.lock().unwrap();
        guard
            .register(Arc::new(|_| panic!("boom")), || Ok(1))
            .unwrap();
        let hub_clone = Arc::clone(&hub);
        let token_holder = Arc::new(Mutex::new(0u64));
        let token_in_cb = Arc::clone(&token_holder);
        let seen_in_cb = Arc::clone(&seen);
        let token = guard
            .register(
                Arc::new(move |_| {
                    seen_in_cb.fetch_add(1, Ordering::SeqCst);
                    let tok = *token_in_cb.lock().unwrap();
                    let _ = hub_clone.lock().unwrap().unregister(tok, |_| Ok(()));
                }),
                || Ok(1),
            )
            .unwrap();
        *token_holder.lock().unwrap() = token;
        drop(guard);

        dispatch_hub(&hub, 1);
        dispatch_hub(&hub, 2);
        assert_eq!(seen.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn listener_hub_propagates_native_registration_error() {
        let mut hub = ListenerHub::<u64>::new();
        let err = hub
            .register(Arc::new(|_| {}), || Err(DisplayError::InternalError(401)))
            .unwrap_err();
        assert_eq!(err, DisplayError::InternalError(401));
        assert!(hub.native_index.is_none());
        assert!(hub.subscribers.is_empty());
    }
}
