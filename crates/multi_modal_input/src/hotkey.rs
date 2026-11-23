use std::{
    collections::HashMap,
    panic::{catch_unwind, AssertUnwindSafe},
    ptr::{self, NonNull},
    sync::{Arc, LazyLock, Mutex},
};

use ohos_multi_modal_input_sys as sys;

use crate::error::{check_status, InputError, Result};

/// Owned description of a global hotkey combination (`@since 14`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HotkeySpec {
    /// Modifier key codes (OpenHarmony InputKit supports 1 or 2 modifiers).
    pub pre_keys: Vec<i32>,
    /// Final key code triggered with the modifiers.
    pub final_key: i32,
    /// Whether holding the hotkey reports repeated events.
    pub repeat: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct HotkeyKey {
    pre_keys: Vec<i32>,
    final_key: i32,
}

impl HotkeyKey {
    fn new(mut pre_keys: Vec<i32>, final_key: i32) -> Self {
        pre_keys.sort_unstable();
        pre_keys.dedup();
        Self {
            pre_keys,
            final_key,
        }
    }
}

impl HotkeySpec {
    pub fn new(pre_keys: Vec<i32>, final_key: i32) -> Self {
        Self {
            pre_keys,
            final_key,
            repeat: false,
        }
    }

    pub fn with_repeat(mut self, repeat: bool) -> Self {
        self.repeat = repeat;
        self
    }

    fn key(&self) -> HotkeyKey {
        HotkeyKey::new(self.pre_keys.clone(), self.final_key)
    }
}

struct OwnedHotkey(NonNull<sys::Input_Hotkey>);

// SAFETY: `Input_Hotkey` is an opaque value container synchronized by `HOTKEY_HUB`.
unsafe impl Send for OwnedHotkey {}

impl OwnedHotkey {
    fn from_spec(spec: &HotkeySpec) -> Result<Self> {
        let len = i32::try_from(spec.pre_keys.len())
            .map_err(|_| InputError::CommonError("too many pre_keys".into()))?;
        let raw = NonNull::new(unsafe { sys::OH_Input_CreateHotkey() })
            .ok_or_else(|| InputError::CommonError("OH_Input_CreateHotkey returned null".into()))?;
        let mut pre_keys = spec.pre_keys.clone();
        unsafe {
            sys::OH_Input_SetPreKeys(raw.as_ptr(), pre_keys.as_mut_ptr(), len);
            sys::OH_Input_SetFinalKey(raw.as_ptr(), spec.final_key);
            sys::OH_Input_SetRepeat(raw.as_ptr(), spec.repeat);
        }
        Ok(Self(raw))
    }

    fn as_ptr(&self) -> *const sys::Input_Hotkey {
        self.0.as_ptr()
    }
}

impl Drop for OwnedHotkey {
    fn drop(&mut self) {
        let mut raw = self.0.as_ptr();
        unsafe { sys::OH_Input_DestroyHotkey(&mut raw) };
    }
}

unsafe fn read_hotkey(raw: *const sys::Input_Hotkey) -> Result<HotkeySpec> {
    if raw.is_null() {
        return Err(InputError::CommonError(
            "Input_Hotkey pointer is null".into(),
        ));
    }
    let mut pre_keys_ptr: *mut i32 = ptr::null_mut();
    let mut pre_key_count: i32 = 0;
    check_status(unsafe { sys::OH_Input_GetPreKeys(raw, &mut pre_keys_ptr, &mut pre_key_count) })?;
    let pre_keys = if pre_key_count > 0 && !pre_keys_ptr.is_null() {
        unsafe { std::slice::from_raw_parts(pre_keys_ptr, pre_key_count as usize) }.to_vec()
    } else {
        Vec::new()
    };
    let mut final_key: i32 = 0;
    check_status(unsafe { sys::OH_Input_GetFinalKey(raw, &mut final_key) })?;
    let mut repeat = false;
    let _ = unsafe { sys::OH_Input_GetRepeat(raw, &mut repeat) };
    Ok(HotkeySpec {
        pre_keys,
        final_key,
        repeat,
    })
}

struct OwnedSystemHotkeys {
    raw: *mut *mut sys::Input_Hotkey,
    count: i32,
}

impl Drop for OwnedSystemHotkeys {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { sys::OH_Input_DestroyAllSystemHotkeys(self.raw, self.count) };
        }
    }
}

/// Queries all system-reserved hotkeys (`@since 14`).
pub fn system_hotkeys() -> Result<Vec<HotkeySpec>> {
    let mut count: i32 = 0;
    check_status(unsafe { sys::OH_Input_GetAllSystemHotkeys(ptr::null_mut(), &mut count) })?;
    if count <= 0 {
        return Ok(Vec::new());
    }
    let raw = unsafe { sys::OH_Input_CreateAllSystemHotkeys(count) };
    if raw.is_null() {
        return Err(InputError::CommonError(
            "OH_Input_CreateAllSystemHotkeys returned null".into(),
        ));
    }
    let guard = OwnedSystemHotkeys { raw, count };
    let mut actual_count = count;
    check_status(unsafe { sys::OH_Input_GetAllSystemHotkeys(guard.raw, &mut actual_count) })?;
    let len = actual_count.clamp(0, count) as usize;
    let slice = unsafe { std::slice::from_raw_parts(guard.raw, len) };
    let mut out = Vec::with_capacity(len);
    for &item in slice {
        if !item.is_null() {
            out.push(unsafe { read_hotkey(item) }?);
        }
    }
    Ok(out)
}

type HotkeyCallback = Arc<dyn Fn(&HotkeySpec) + Send + Sync + 'static>;

struct HotkeySlot<H> {
    handle: H,
    subscribers: Vec<(u64, HotkeyCallback)>,
}

struct HotkeyHub<H> {
    next_token: u64,
    slots: HashMap<HotkeyKey, HotkeySlot<H>>,
}

impl<H> HotkeyHub<H> {
    fn new() -> Self {
        Self {
            next_token: 1,
            slots: HashMap::new(),
        }
    }

    fn register<F>(
        &mut self,
        spec: &HotkeySpec,
        callback: HotkeyCallback,
        install: F,
    ) -> Result<u64>
    where
        F: FnOnce() -> Result<H>,
    {
        let key = spec.key();
        let token = self.next_token;
        self.next_token = self.next_token.wrapping_add(1).max(1);
        if let Some(slot) = self.slots.get_mut(&key) {
            slot.subscribers.push((token, callback));
            return Ok(token);
        }
        let handle = install()?;
        self.slots.insert(
            key,
            HotkeySlot {
                handle,
                subscribers: vec![(token, callback)],
            },
        );
        Ok(token)
    }

    fn unregister<U>(&mut self, key: &HotkeyKey, token: u64, uninstall: U) -> Result<()>
    where
        U: FnOnce(&H) -> Result<()>,
    {
        let Some(slot) = self.slots.get_mut(key) else {
            return Ok(());
        };
        let Some(pos) = slot.subscribers.iter().position(|(id, _)| *id == token) else {
            return Ok(());
        };
        slot.subscribers.swap_remove(pos);
        if slot.subscribers.is_empty() {
            let result = uninstall(&slot.handle);
            if result.is_ok() {
                self.slots.remove(key);
            }
            return result;
        }
        Ok(())
    }

    fn snapshot(&self, key: &HotkeyKey) -> Vec<HotkeyCallback> {
        self.slots
            .get(key)
            .map(|slot| {
                slot.subscribers
                    .iter()
                    .map(|(_, cb)| Arc::clone(cb))
                    .collect()
            })
            .unwrap_or_default()
    }
}

static HOTKEY_HUB: LazyLock<Mutex<HotkeyHub<OwnedHotkey>>> =
    LazyLock::new(|| Mutex::new(HotkeyHub::new()));

unsafe extern "C" fn hotkey_trampoline(raw: *mut sys::Input_Hotkey) {
    let Ok(spec) = (unsafe { read_hotkey(raw) }) else {
        return;
    };
    let key = spec.key();
    let callbacks = match HOTKEY_HUB.lock() {
        Ok(guard) => guard.snapshot(&key),
        Err(poisoned) => poisoned.into_inner().snapshot(&key),
    };
    for callback in callbacks {
        let _ = catch_unwind(AssertUnwindSafe(|| callback(&spec)));
    }
}

/// RAII handle for a registered global hotkey monitor (`@since 14`).
///
/// Dropping this monitor removes the Rust callback and unregisters the native
/// hotkey monitor once no subscribers remain for that key combination.
#[must_use = "dropping a HotkeyMonitor unregisters the hotkey immediately"]
pub struct HotkeyMonitor {
    key: HotkeyKey,
    token: Option<u64>,
}

impl HotkeyMonitor {
    /// Registers a global hotkey monitor (`@since 14`).
    pub fn register<F>(spec: &HotkeySpec, callback: F) -> Result<Self>
    where
        F: Fn(&HotkeySpec) + Send + Sync + 'static,
    {
        let mut hub = match HOTKEY_HUB.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let token = hub.register(spec, Arc::new(callback), || {
            let owned = OwnedHotkey::from_spec(spec)?;
            check_status(unsafe {
                sys::OH_Input_AddHotkeyMonitor(owned.as_ptr(), Some(hotkey_trampoline))
            })?;
            Ok(owned)
        })?;
        Ok(Self {
            key: spec.key(),
            token: Some(token),
        })
    }

    /// Explicitly unregisters this hotkey monitor and surfaces any native error.
    pub fn unregister(mut self) -> Result<()> {
        self.unregister_inner()
    }

    fn unregister_inner(&mut self) -> Result<()> {
        let Some(token) = self.token.take() else {
            return Ok(());
        };
        let mut hub = match HOTKEY_HUB.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        hub.unregister(&self.key, token, |owned| {
            check_status(unsafe {
                sys::OH_Input_RemoveHotkeyMonitor(owned.as_ptr(), Some(hotkey_trampoline))
            })
        })
    }
}

impl Drop for HotkeyMonitor {
    fn drop(&mut self) {
        let _ = self.unregister_inner();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    #[test]
    fn hotkey_hub_canonicalizes_modifiers_and_refcounts_native_registration() {
        let mut hub = HotkeyHub::<usize>::new();
        let installs = AtomicUsize::new(0);
        let uninstalls = AtomicUsize::new(0);
        let hits = Arc::new(AtomicUsize::new(0));

        let spec_a = HotkeySpec::new(vec![2072, 2047], 2017);
        let spec_b = HotkeySpec::new(vec![2047, 2072], 2017);

        let h1 = Arc::clone(&hits);
        let t1 = hub
            .register(
                &spec_a,
                Arc::new(move |_| {
                    h1.fetch_add(1, Ordering::SeqCst);
                }),
                || {
                    installs.fetch_add(1, Ordering::SeqCst);
                    Ok(1)
                },
            )
            .unwrap();
        let h2 = Arc::clone(&hits);
        let t2 = hub
            .register(
                &spec_b,
                Arc::new(move |_| {
                    h2.fetch_add(10, Ordering::SeqCst);
                }),
                || {
                    installs.fetch_add(1, Ordering::SeqCst);
                    Ok(2)
                },
            )
            .unwrap();

        assert_eq!(installs.load(Ordering::SeqCst), 1);
        for cb in hub.snapshot(&spec_a.key()) {
            cb(&spec_a);
        }
        assert_eq!(hits.load(Ordering::SeqCst), 11);

        hub.unregister(&spec_a.key(), t1, |_| {
            uninstalls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
        .unwrap();
        assert_eq!(uninstalls.load(Ordering::SeqCst), 0);

        hub.unregister(&spec_b.key(), t2, |_| {
            uninstalls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        })
        .unwrap();
        assert_eq!(uninstalls.load(Ordering::SeqCst), 1);
        assert!(hub.slots.is_empty());
    }

    #[test]
    fn hotkey_hub_does_not_insert_slot_on_native_error() {
        let mut hub = HotkeyHub::<usize>::new();
        let spec = HotkeySpec::new(vec![2072], 2017);
        let err = hub
            .register(&spec, Arc::new(|_| {}), || {
                Err(InputError::InternalError(
                    sys::Input_Result_INPUT_OCCUPIED_BY_SYSTEM,
                ))
            })
            .unwrap_err();
        assert_eq!(
            err,
            InputError::InternalError(sys::Input_Result_INPUT_OCCUPIED_BY_SYSTEM)
        );
        assert!(hub.slots.is_empty());
    }
}
