use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    ptr::{self, NonNull},
    sync::{Arc, LazyLock, Mutex},
};

use ohos_audio_sys as sys;

use crate::{
    error::{check_common, AudioError, Result},
    types::{AudioSessionConcurrencyMode, AudioSessionDeactivatedReason},
};

#[cfg(feature = "api-20")]
use crate::types::{AudioSessionScene, AudioSessionStateChangeHint};

type Callback<T> = Arc<dyn Fn(T) + Send + Sync + 'static>;

struct SessionHub<T> {
    next_token: u64,
    registered: bool,
    subscribers: Vec<(u64, Callback<T>)>,
}

impl<T: Copy> SessionHub<T> {
    fn new() -> Self {
        Self {
            next_token: 1,
            registered: false,
            subscribers: Vec::new(),
        }
    }

    fn register<R>(&mut self, callback: Callback<T>, register_native: R) -> Result<u64>
    where
        R: FnOnce() -> Result<()>,
    {
        if !self.registered {
            register_native()?;
            self.registered = true;
        }
        let token = self.next_token;
        self.next_token = self.next_token.wrapping_add(1).max(1);
        self.subscribers.push((token, callback));
        Ok(token)
    }

    fn unregister<U>(&mut self, token: u64, unregister_native: U) -> Result<()>
    where
        U: FnOnce() -> Result<()>,
    {
        let Some(pos) = self.subscribers.iter().position(|(id, _)| *id == token) else {
            return Ok(());
        };
        self.subscribers.swap_remove(pos);
        if self.subscribers.is_empty() && self.registered {
            unregister_native()?;
            self.registered = false;
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

fn dispatch_hub<T: Copy>(hub: &Mutex<SessionHub<T>>, event: T) {
    let callbacks = match hub.lock() {
        Ok(guard) => guard.snapshot(),
        Err(poisoned) => poisoned.into_inner().snapshot(),
    };
    for callback in callbacks {
        let _ = catch_unwind(AssertUnwindSafe(|| callback(event)));
    }
}

static DEACTIVATED_HUB: LazyLock<Mutex<SessionHub<AudioSessionDeactivatedReason>>> =
    LazyLock::new(|| Mutex::new(SessionHub::new()));

unsafe extern "C" fn on_deactivated_trampoline(
    event: sys::OH_AudioSession_DeactivatedEvent,
) -> i32 {
    if let Some(reason) = AudioSessionDeactivatedReason::try_from_raw(event.reason) {
        dispatch_hub(&DEACTIVATED_HUB, reason);
    }
    0
}

#[cfg(feature = "api-20")]
static STATE_CHANGE_HUB: LazyLock<Mutex<SessionHub<AudioSessionStateChangeHint>>> =
    LazyLock::new(|| Mutex::new(SessionHub::new()));

#[cfg(feature = "api-20")]
unsafe extern "C" fn on_state_change_trampoline(event: sys::OH_AudioSession_StateChangedEvent) {
    if let Some(hint) = AudioSessionStateChangeHint::try_from_raw(event.stateChangeHint) {
        dispatch_hub(&STATE_CHANGE_HUB, hint);
    }
}

/// Handle to the process-global OpenHarmony `OH_AudioSessionManager` singleton (`@since 12`).
pub struct AudioSessionManager {
    raw: NonNull<sys::OH_AudioSessionManager>,
}

// SAFETY: `OH_AudioSessionManager` is a process-global service singleton.
unsafe impl Send for AudioSessionManager {}
unsafe impl Sync for AudioSessionManager {}

impl AudioSessionManager {
    /// Retrieves the process-global audio session manager singleton (`@since 12`).
    pub fn get() -> Result<Self> {
        let mut raw: *mut sys::OH_AudioSessionManager = ptr::null_mut();
        check_common(unsafe { sys::OH_AudioManager_GetAudioSessionManager(&mut raw) })?;
        NonNull::new(raw).map(|raw| Self { raw }).ok_or_else(|| {
            AudioError::CommonError("OH_AudioManager_GetAudioSessionManager returned null".into())
        })
    }

    /// Activates the application's audio session with the given concurrency mode (`@since 12`).
    pub fn activate(&self, mode: AudioSessionConcurrencyMode) -> Result<()> {
        let strategy = sys::OH_AudioSession_Strategy {
            concurrencyMode: mode.into(),
        };
        check_common(unsafe {
            sys::OH_AudioSessionManager_ActivateAudioSession(self.raw.as_ptr(), &strategy)
        })
    }

    /// Deactivates the application's audio session (`@since 12`).
    pub fn deactivate(&self) -> Result<()> {
        check_common(unsafe {
            sys::OH_AudioSessionManager_DeactivateAudioSession(self.raw.as_ptr())
        })
    }

    /// Returns whether the application's audio session is currently active (`@since 12`).
    pub fn is_activated(&self) -> bool {
        unsafe { sys::OH_AudioSessionManager_IsAudioSessionActivated(self.raw.as_ptr()) }
    }

    /// Subscribes to system audio session deactivation events (`@since 12`).
    pub fn on_deactivated<F>(&self, callback: F) -> Result<AudioSessionDeactivatedSubscription>
    where
        F: Fn(AudioSessionDeactivatedReason) + Send + Sync + 'static,
    {
        let mut guard = match DEACTIVATED_HUB.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let raw = self.raw;
        let token = guard.register(Arc::new(callback), || {
            check_common(unsafe {
                sys::OH_AudioSessionManager_RegisterSessionDeactivatedCallback(
                    raw.as_ptr(),
                    Some(on_deactivated_trampoline),
                )
            })
        })?;
        Ok(AudioSessionDeactivatedSubscription {
            raw,
            token: Some(token),
        })
    }

    /// Configures the audio session scene (`@since 20`).
    #[cfg(feature = "api-20")]
    pub fn set_scene(&self, scene: AudioSessionScene) -> Result<()> {
        check_common(unsafe {
            sys::OH_AudioSessionManager_SetScene(self.raw.as_ptr(), scene.into())
        })
    }

    /// Subscribes to audio session state change hints (`@since 20`).
    #[cfg(feature = "api-20")]
    pub fn on_state_change<F>(&self, callback: F) -> Result<AudioSessionStateSubscription>
    where
        F: Fn(AudioSessionStateChangeHint) + Send + Sync + 'static,
    {
        let mut guard = match STATE_CHANGE_HUB.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        let raw = self.raw;
        let token = guard.register(Arc::new(callback), || {
            check_common(unsafe {
                sys::OH_AudioSessionManager_RegisterStateChangeCallback(
                    raw.as_ptr(),
                    Some(on_state_change_trampoline),
                )
            })
        })?;
        Ok(AudioSessionStateSubscription {
            raw,
            token: Some(token),
        })
    }

    /// Enables or disables mute suggestion hints when mixing with other apps (`@since 23`).
    #[cfg(feature = "api-23")]
    pub fn enable_mute_suggestion_when_mix_with_others(&self, enable: bool) -> Result<()> {
        check_common(unsafe {
            sys::OH_AudioSessionManager_EnableMuteSuggestionWhenMixWithOthers(
                self.raw.as_ptr(),
                enable,
            )
        })
    }

    /// Returns whether another application is playing media audio (`@since 23`).
    #[cfg(feature = "api-23")]
    pub fn is_other_media_playing(&self) -> bool {
        unsafe { sys::OH_AudioSessionManager_IsOtherMediaPlaying(self.raw.as_ptr()) }
    }
}

/// RAII subscription for audio session deactivation notifications (`@since 12`).
#[must_use = "dropping an AudioSessionDeactivatedSubscription unregisters it immediately"]
pub struct AudioSessionDeactivatedSubscription {
    raw: NonNull<sys::OH_AudioSessionManager>,
    token: Option<u64>,
}

// SAFETY: `OH_AudioSessionManager` is a process-global service singleton.
unsafe impl Send for AudioSessionDeactivatedSubscription {}
unsafe impl Sync for AudioSessionDeactivatedSubscription {}

impl AudioSessionDeactivatedSubscription {
    pub fn unregister(mut self) -> Result<()> {
        self.unregister_inner()
    }

    fn unregister_inner(&mut self) -> Result<()> {
        let Some(token) = self.token.take() else {
            return Ok(());
        };
        let mut guard = match DEACTIVATED_HUB.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        guard.unregister(token, || {
            check_common(unsafe {
                sys::OH_AudioSessionManager_UnregisterSessionDeactivatedCallback(
                    self.raw.as_ptr(),
                    Some(on_deactivated_trampoline),
                )
            })
        })
    }
}

impl Drop for AudioSessionDeactivatedSubscription {
    fn drop(&mut self) {
        let _ = self.unregister_inner();
    }
}

/// RAII subscription for audio session state change notifications (`@since 20`).
#[cfg(feature = "api-20")]
#[must_use = "dropping an AudioSessionStateSubscription unregisters it immediately"]
pub struct AudioSessionStateSubscription {
    raw: NonNull<sys::OH_AudioSessionManager>,
    token: Option<u64>,
}

#[cfg(feature = "api-20")]
// SAFETY: `OH_AudioSessionManager` is a process-global service singleton.
unsafe impl Send for AudioSessionStateSubscription {}
#[cfg(feature = "api-20")]
unsafe impl Sync for AudioSessionStateSubscription {}

#[cfg(feature = "api-20")]
impl AudioSessionStateSubscription {
    pub fn unregister(mut self) -> Result<()> {
        self.unregister_inner()
    }

    fn unregister_inner(&mut self) -> Result<()> {
        let Some(token) = self.token.take() else {
            return Ok(());
        };
        let mut guard = match STATE_CHANGE_HUB.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        };
        guard.unregister(token, || {
            check_common(unsafe {
                sys::OH_AudioSessionManager_UnregisterStateChangeCallback(
                    self.raw.as_ptr(),
                    Some(on_state_change_trampoline),
                )
            })
        })
    }
}

#[cfg(feature = "api-20")]
impl Drop for AudioSessionStateSubscription {
    fn drop(&mut self) {
        let _ = self.unregister_inner();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    #[test]
    fn session_hub_tracks_refcount_and_isolates_panics() {
        let hub = Arc::new(Mutex::new(
            SessionHub::<AudioSessionDeactivatedReason>::new(),
        ));
        let reg = AtomicUsize::new(0);
        let unreg = AtomicUsize::new(0);
        let received = Arc::new(Mutex::new(Vec::new()));

        let t1 = hub
            .lock()
            .unwrap()
            .register(Arc::new(|_| panic!("boom")), || {
                reg.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
            .unwrap();
        let dst = Arc::clone(&received);
        let t2 = hub
            .lock()
            .unwrap()
            .register(
                Arc::new(move |reason| dst.lock().unwrap().push(reason)),
                || {
                    reg.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                },
            )
            .unwrap();
        assert_eq!(reg.load(Ordering::SeqCst), 1);

        dispatch_hub(&hub, AudioSessionDeactivatedReason::LowerPriority);
        assert_eq!(
            *received.lock().unwrap(),
            vec![AudioSessionDeactivatedReason::LowerPriority]
        );

        hub.lock()
            .unwrap()
            .unregister(t1, || {
                unreg.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
            .unwrap();
        assert_eq!(unreg.load(Ordering::SeqCst), 0);

        hub.lock()
            .unwrap()
            .unregister(t2, || {
                unreg.fetch_add(1, Ordering::SeqCst);
                Ok(())
            })
            .unwrap();
        assert_eq!(unreg.load(Ordering::SeqCst), 1);
    }
}
