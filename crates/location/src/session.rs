use crate::config::RequestConfig;
use crate::error::{check, Result};
use crate::info::LocationInfo;
use ohos_location_sys as sys;
use std::collections::HashMap;
use std::os::raw::c_void;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, LazyLock, RwLock};

/// A reporting closure kept alive by the registry.
type SharedCallback = Arc<dyn Fn(LocationInfo) + Send + Sync + 'static>;

/// Reporting closures of the running sessions, keyed by session key.
///
/// The registry is the reason no Rust allocation is ever reachable through the
/// native `userData` pointer: the pointer only transports an integer key, and
/// the closure is reached through this map instead. Dereferencing a stale
/// pointer is therefore impossible by construction, whatever the native service
/// does with a callback registration after the session was stopped.
static CALLBACKS: LazyLock<RwLock<HashMap<u64, SharedCallback>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

/// Source of session keys. Keys are never reused, so a report that arrives late
/// cannot be routed to a session started afterwards.
static NEXT_KEY: AtomicU64 = AtomicU64::new(1);

fn register(callback: SharedCallback) -> u64 {
    let key = NEXT_KEY.fetch_add(1, Ordering::Relaxed);
    let mut guard = CALLBACKS.write().unwrap_or_else(|err| err.into_inner());
    guard.insert(key, callback);
    key
}

fn unregister(key: u64) {
    let mut guard = CALLBACKS.write().unwrap_or_else(|err| err.into_inner());
    guard.remove(&key);
}

fn lookup(key: u64) -> Option<SharedCallback> {
    let guard = CALLBACKS.read().unwrap_or_else(|err| err.into_inner());
    guard.get(&key).cloned()
}

/// Native reporting callback shared by every session.
///
/// # Safety
///
/// Registered with the location service through
/// `OH_LocationRequestConfig_SetCallback`, which is the only caller. `location`
/// is the instance the service reports and stays live until this function
/// returns; `user_data` is the integer key [`LocationSession::start`] passed in
/// and is never dereferenced.
unsafe extern "C" fn report_location(location: *mut sys::Location_Info, user_data: *mut c_void) {
    let key = user_data as usize as u64;
    // The registry lock is released before the closure runs, so a closure that
    // stops its own session cannot deadlock against the write lock taken there.
    let Some(callback) = lookup(key) else {
        return;
    };
    let Some(location) = NonNull::new(location) else {
        return;
    };
    // SAFETY: the service keeps `location` live for the duration of this
    // callback, which is exactly the scope in which it is read here.
    let info = unsafe { LocationInfo::from_raw(location) };
    callback(info);
}

/// A running location subscription.
///
/// Created by [`LocationSession::start`], which starts locating, and stopped
/// either explicitly through [`LocationSession::stop`] or on drop, so that
/// `OH_Location_StartLocating` and `OH_Location_StopLocating` are always paired.
/// The [`RequestConfig`] is owned by the session because the native API demands
/// that stopping passes the very instance that started the subscription.
///
/// # Permissions
///
/// Starting and stopping require `ohos.permission.APPROXIMATELY_LOCATION`
/// (`@permission` of `OH_Location_StartLocating` and `OH_Location_StopLocating`
/// in `oh_location.h`). Without it both calls fail with a
/// [`crate::LocationError`] for which
/// [`LocationError::is_permission_denied`](crate::LocationError::is_permission_denied)
/// holds. A location fix additionally requires the user to have the system
/// location switch on; otherwise the calls report the switch-off code.
///
/// # Example
///
/// ```no_run
/// use ohos_location_binding as location;
/// use location::{LocationSession, LocationUseScene, RequestConfig};
///
/// let config = RequestConfig::builder()
///     .use_scene(LocationUseScene::DailyLifeService)
///     .interval(5)
///     .build()?;
///
/// let session = LocationSession::start(config, |fix| {
///     println!("{} {}", fix.latitude, fix.longitude);
/// })?;
///
/// // ... later
/// session.stop()?;
/// # Ok::<(), location::LocationError>(())
/// ```
pub struct LocationSession {
    config: RequestConfig,
    key: u64,
    stopped: bool,
}

impl LocationSession {
    /// Start locating and report every fix to `callback`.
    ///
    /// `callback` runs on a thread owned by the location service, so it must be
    /// `Send + Sync`; it should return quickly, because the native location
    /// instance is recycled as soon as it does.
    ///
    /// Requires `ohos.permission.APPROXIMATELY_LOCATION`, see the type level
    /// documentation.
    pub fn start<F>(config: RequestConfig, callback: F) -> Result<Self>
    where
        F: Fn(LocationInfo) + Send + Sync + 'static,
    {
        let key = register(Arc::new(callback));
        // SAFETY: `config` is a live instance owned by the caller and moved
        // into the session below. `report_location` matches the native callback
        // signature, and the user data it is given is an integer key rather
        // than a pointer into Rust memory, so the service cannot make it
        // dereference anything.
        unsafe {
            sys::OH_LocationRequestConfig_SetCallback(
                config.as_ptr(),
                Some(report_location),
                key as usize as *mut c_void,
            )
        }
        // SAFETY: `config` is a live instance and stays alive in the session
        // returned below, or is destroyed only after the failed start.
        let code = unsafe { sys::OH_Location_StartLocating(config.as_ptr()) };
        if let Err(err) = check(code) {
            // Locating never started, so no report can be in flight.
            unregister(key);
            return Err(err);
        }
        Ok(LocationSession {
            config,
            key,
            stopped: false,
        })
    }

    /// Stop locating and release the request parameters.
    ///
    /// Reports the native failure if stopping failed; the subscription is
    /// considered finished either way and the reporting closure is dropped once
    /// no report is executing any more.
    pub fn stop(mut self) -> Result<()> {
        self.stop_inner()
    }

    fn stop_inner(&mut self) -> Result<()> {
        if self.stopped {
            return Ok(());
        }
        self.stopped = true;
        // SAFETY: `self.config` is the very instance passed to
        // `OH_Location_StartLocating` and is still alive, as the session owns
        // it and is dropped only after this call.
        let result = check(unsafe { sys::OH_Location_StopLocating(self.config.as_ptr()) });
        // Unregistering after the native stop keeps the closure reachable for
        // as long as reports can be delivered. A report that is already inside
        // `report_location` holds its own `Arc` clone, so dropping the registry
        // entry here cannot free a closure that is running, and a report that
        // arrives afterwards simply finds no entry and returns.
        unregister(self.key);
        result
    }
}

impl Drop for LocationSession {
    fn drop(&mut self) {
        // Stop before the owned `RequestConfig` is destroyed by its own `Drop`,
        // which runs after this.
        let _ = self.stop_inner();
    }
}

// SAFETY: the session owns its `RequestConfig` (itself `Send`) and a plain
// integer key; the reporting closure lives in the registry rather than in the
// session, and is required to be `Send + Sync`.
unsafe impl Send for LocationSession {}

/// Whether the system location switch is currently on.
///
/// Wraps `OH_Location_IsLocatingEnabled`. Requires no permission, but reports
/// the service-unavailable code when the location service failed to start.
pub fn is_locating_enabled() -> Result<bool> {
    let mut enabled = false;
    // SAFETY: the service writes a single `bool` through the pointer, which
    // points at an initialized local.
    let code = unsafe { sys::OH_Location_IsLocatingEnabled(&mut enabled) };
    check(code)?;
    Ok(enabled)
}
