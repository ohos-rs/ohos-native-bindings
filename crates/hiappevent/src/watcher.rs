use crate::array_len;
use crate::error::{check, HiAppEventError, Result};
use crate::r#type::AppEventType;
use ohos_hiappevent_sys as sys;
use std::ffi::{c_char, c_int, CStr, CString};

/// Callback invoked for every event a [`Watcher`] receives.
///
/// Turn the raw arguments into owned values with [`app_event_groups`].
pub type OnReceive = unsafe extern "C" fn(
    domain: *const c_char,
    groups: *const sys::HiAppEvent_AppEventGroup,
    group_len: u32,
);

/// Callback invoked when a [`Watcher`]'s trigger condition is met.
pub type OnTrigger = unsafe extern "C" fn(row: c_int, size: c_int);

/// Callback invoked with the events taken by [`Watcher::take_data`].
///
/// Turn the raw arguments into owned values with [`taken_events`].
pub type OnTake = unsafe extern "C" fn(events: *const *const c_char, event_len: u32);

/// A subscription to application events.
///
/// Configure the watcher, then [`add`](Self::add) it to start receiving events.
/// A watcher must be removed before its handle is destroyed, so dropping a
/// registered watcher removes it first.
pub struct Watcher {
    raw: *mut sys::HiAppEvent_Watcher,
    added: bool,
}

impl Watcher {
    /// Create a watcher identified by `name`.
    pub fn new(name: &str) -> Result<Self> {
        let name = CString::new(name)?;
        // SAFETY: name outlives the call.
        let raw = unsafe { sys::OH_HiAppEvent_CreateWatcher(name.as_ptr()) };
        if raw.is_null() {
            return Err(HiAppEventError::Alloc);
        }
        Ok(Watcher { raw, added: false })
    }

    /// Set the conditions that fire [`OnTrigger`]. Any condition set above zero
    /// fires the callback once it is met; the others stay disabled.
    pub fn set_trigger_condition(&mut self, row: i32, size: i32, timeout: i32) -> Result<()> {
        // SAFETY: raw is a live watcher handle.
        unsafe {
            check(sys::OH_HiAppEvent_SetTriggerCondition(
                self.raw, row, size, timeout,
            ))
        }
    }

    /// Restrict what the watcher receives to `domain`, the given event types and
    /// the given event names. An empty `types` or `names` means no restriction.
    pub fn set_app_event_filter(
        &mut self,
        domain: &str,
        types: &[AppEventType],
        names: &[&str],
    ) -> Result<()> {
        let domain = CString::new(domain)?;
        let mask = types.iter().fold(0u8, |mask, t| mask | t.filter_bit());
        let names = names
            .iter()
            .map(|name| CString::new(*name))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let ptrs: Vec<*const c_char> = names.iter().map(|name| name.as_ptr()).collect();
        // SAFETY: raw is a live watcher handle; domain and the name array outlive
        // the call.
        unsafe {
            check(sys::OH_HiAppEvent_SetAppEventFilter(
                self.raw,
                domain.as_ptr(),
                mask,
                ptrs.as_ptr(),
                array_len(ptrs.len()),
            ))
        }
    }

    /// Set the callback fired when the trigger condition is met.
    pub fn set_on_trigger(&mut self, on_trigger: OnTrigger) -> Result<()> {
        // SAFETY: raw is a live watcher handle; the callback is a plain function.
        unsafe {
            check(sys::OH_HiAppEvent_SetWatcherOnTrigger(
                self.raw,
                Some(on_trigger),
            ))
        }
    }

    /// Set the callback fired for every event received. Without it, events are
    /// stored for [`take_data`](Self::take_data) instead.
    pub fn set_on_receive(&mut self, on_receive: OnReceive) -> Result<()> {
        // SAFETY: raw is a live watcher handle; the callback is a plain function.
        unsafe {
            check(sys::OH_HiAppEvent_SetWatcherOnReceive(
                self.raw,
                Some(on_receive),
            ))
        }
    }

    /// Take up to `event_num` stored events. Only valid after [`add`](Self::add).
    pub fn take_data(&mut self, event_num: u32, on_take: OnTake) -> Result<()> {
        // SAFETY: raw is a live watcher handle; the callback is a plain function.
        unsafe {
            check(sys::OH_HiAppEvent_TakeWatcherData(
                self.raw,
                event_num,
                Some(on_take),
            ))
        }
    }

    /// Start receiving events.
    pub fn add(&mut self) -> Result<()> {
        // SAFETY: raw is a live watcher handle.
        unsafe { check(sys::OH_HiAppEvent_AddWatcher(self.raw))? };
        self.added = true;
        Ok(())
    }

    /// Stop receiving events.
    pub fn remove(&mut self) -> Result<()> {
        // SAFETY: raw is a live watcher handle.
        let result = unsafe { check(sys::OH_HiAppEvent_RemoveWatcher(self.raw)) };
        self.added = false;
        result
    }
}

impl Drop for Watcher {
    fn drop(&mut self) {
        // SAFETY: raw is a live handle, removed before it is destroyed and
        // destroyed once.
        unsafe {
            if self.added {
                sys::OH_HiAppEvent_RemoveWatcher(self.raw);
            }
            sys::OH_HiAppEvent_DestroyWatcher(self.raw);
        }
    }
}

/// One application event, as delivered to an [`OnReceive`] callback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppEventInfo {
    pub domain: String,
    pub name: String,
    /// `None` for a type this crate does not know.
    pub event_type: Option<AppEventType>,
    /// The event parameters, as a JSON object.
    pub params: String,
}

/// The events of one domain, grouped by event name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppEventGroup {
    pub name: String,
    pub events: Vec<AppEventInfo>,
}

/// Copy the event groups handed to an [`OnReceive`] callback into owned values.
///
/// # Safety
///
/// `groups` and `group_len` must be the arguments the callback was invoked with.
pub unsafe fn app_event_groups(
    groups: *const sys::HiAppEvent_AppEventGroup,
    group_len: u32,
) -> Vec<AppEventGroup> {
    if groups.is_null() {
        return Vec::new();
    }
    let groups = unsafe { std::slice::from_raw_parts(groups, group_len as usize) };
    groups
        .iter()
        .map(|group| AppEventGroup {
            name: unsafe { to_string(group.name) },
            events: unsafe { app_event_infos(group.appEventInfos, group.infoLen) },
        })
        .collect()
}

/// Copy the event strings handed to an [`OnTake`] callback into owned values.
///
/// # Safety
///
/// `events` and `event_len` must be the arguments the callback was invoked with.
pub unsafe fn taken_events(events: *const *const c_char, event_len: u32) -> Vec<String> {
    if events.is_null() {
        return Vec::new();
    }
    let events = unsafe { std::slice::from_raw_parts(events, event_len as usize) };
    events
        .iter()
        .map(|event| unsafe { to_string(*event) })
        .collect()
}

unsafe fn app_event_infos(
    infos: *const sys::HiAppEvent_AppEventInfo,
    info_len: u32,
) -> Vec<AppEventInfo> {
    if infos.is_null() {
        return Vec::new();
    }
    let infos = unsafe { std::slice::from_raw_parts(infos, info_len as usize) };
    infos
        .iter()
        .map(|info| AppEventInfo {
            domain: unsafe { to_string(info.domain) },
            name: unsafe { to_string(info.name) },
            event_type: AppEventType::try_from_raw(info.type_),
            params: unsafe { to_string(info.params) },
        })
        .collect()
}

unsafe fn to_string(text: *const c_char) -> String {
    if text.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(text) }
        .to_string_lossy()
        .into_owned()
}
