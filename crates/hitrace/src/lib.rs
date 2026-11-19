//! OpenHarmony HiTraceMeter and HiTraceChain native bindings.

use std::{ffi::CStr, marker::PhantomData, rc::Rc};

pub use ohos_hitrace_sys as sys;

/// Starts a synchronous trace section on the calling thread (`OH_HiTrace_StartTrace`).
///
/// Must be paired with [`finish_trace`] on the same thread.
pub fn start_trace(name: &CStr) {
    // SAFETY: `name` is a valid NUL-terminated C string consumed synchronously.
    unsafe { sys::OH_HiTrace_StartTrace(name.as_ptr()) };
}

/// Finishes the most recent synchronous trace section on the calling thread (`OH_HiTrace_FinishTrace`).
pub fn finish_trace() {
    // SAFETY: pairing is enforced by the caller (or via [`Scope`]).
    unsafe { sys::OH_HiTrace_FinishTrace() };
}

/// Starts an asynchronous trace task (`OH_HiTrace_StartAsyncTrace`).
pub fn start_async_trace(name: &CStr, task_id: i32) {
    // SAFETY: `name` is a valid NUL-terminated C string consumed synchronously.
    unsafe { sys::OH_HiTrace_StartAsyncTrace(name.as_ptr(), task_id) };
}

/// Finishes an asynchronous trace task (`OH_HiTrace_FinishAsyncTrace`).
pub fn finish_async_trace(name: &CStr, task_id: i32) {
    // SAFETY: `name` is a valid NUL-terminated C string consumed synchronously.
    unsafe { sys::OH_HiTrace_FinishAsyncTrace(name.as_ptr(), task_id) };
}

/// Records one named integer counter sample (`OH_HiTrace_CountTrace`).
pub fn count_trace(name: &CStr, value: i64) {
    // SAFETY: `name` is a valid NUL-terminated C string consumed synchronously.
    unsafe { sys::OH_HiTrace_CountTrace(name.as_ptr(), value) };
}

/// Alias for [`count_trace`].
pub fn count(name: &CStr, value: i64) {
    count_trace(name, value);
}

/// RAII guard for a synchronous HiTrace section on the calling thread.
///
/// `!Send` ensures the section ends on the same thread where it began.
#[must_use]
pub struct Scope(PhantomData<Rc<()>>);

impl Scope {
    /// Starts a named synchronous trace section that finishes when dropped.
    pub fn new(name: &CStr) -> Self {
        start_trace(name);
        Self(PhantomData)
    }
}

impl Drop for Scope {
    fn drop(&mut self) {
        finish_trace();
    }
}

/// Trace output level for HiTraceMeter `Ex` APIs (`HiTrace_Output_Level`, API 19).
#[cfg(feature = "api-19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum OutputLevel {
    /// Debugging level (lowest priority).
    Debug = sys::HiTrace_Output_Level_HITRACE_LEVEL_DEBUG,
    /// Log-version info level.
    Info = sys::HiTrace_Output_Level_HITRACE_LEVEL_INFO,
    /// Log-version critical level.
    Critical = sys::HiTrace_Output_Level_HITRACE_LEVEL_CRITICAL,
    /// Commercial/nolog-version level (highest priority).
    Commercial = sys::HiTrace_Output_Level_HITRACE_LEVEL_COMMERCIAL,
}

#[cfg(feature = "api-19")]
impl From<OutputLevel> for sys::HiTrace_Output_Level {
    fn from(level: OutputLevel) -> Self {
        level as Self
    }
}

/// Starts a synchronous trace section with output level and optional args (`OH_HiTrace_StartTraceEx`, API 19).
#[cfg(feature = "api-19")]
pub fn start_trace_ex(level: OutputLevel, name: &CStr, custom_args: Option<&CStr>) {
    let args = custom_args.map_or(std::ptr::null(), CStr::as_ptr);
    // SAFETY: `name` and optional `custom_args` are valid NUL-terminated C strings.
    unsafe { sys::OH_HiTrace_StartTraceEx(level.into(), name.as_ptr(), args) };
}

/// Finishes a synchronous trace section with output level (`OH_HiTrace_FinishTraceEx`, API 19).
#[cfg(feature = "api-19")]
pub fn finish_trace_ex(level: OutputLevel) {
    // SAFETY: pairing with `start_trace_ex` at the same level is enforced by the caller.
    unsafe { sys::OH_HiTrace_FinishTraceEx(level.into()) };
}

/// Starts an asynchronous trace task with output level, category, and args (`OH_HiTrace_StartAsyncTraceEx`, API 19).
#[cfg(feature = "api-19")]
pub fn start_async_trace_ex(
    level: OutputLevel,
    name: &CStr,
    task_id: i32,
    custom_category: Option<&CStr>,
    custom_args: Option<&CStr>,
) {
    let category = custom_category.map_or(std::ptr::null(), CStr::as_ptr);
    let args = custom_args.map_or(std::ptr::null(), CStr::as_ptr);
    // SAFETY: all non-null pointers come from valid `CStr` references.
    unsafe {
        sys::OH_HiTrace_StartAsyncTraceEx(level.into(), name.as_ptr(), task_id, category, args)
    };
}

/// Finishes an asynchronous trace task with output level (`OH_HiTrace_FinishAsyncTraceEx`, API 19).
#[cfg(feature = "api-19")]
pub fn finish_async_trace_ex(level: OutputLevel, name: &CStr, task_id: i32) {
    // SAFETY: `name` is a valid NUL-terminated C string.
    unsafe { sys::OH_HiTrace_FinishAsyncTraceEx(level.into(), name.as_ptr(), task_id) };
}

/// Records an integer counter sample with output level (`OH_HiTrace_CountTraceEx`, API 19).
#[cfg(feature = "api-19")]
pub fn count_trace_ex(level: OutputLevel, name: &CStr, value: i64) {
    // SAFETY: `name` is a valid NUL-terminated C string.
    unsafe { sys::OH_HiTrace_CountTraceEx(level.into(), name.as_ptr(), value) };
}

/// Returns whether application trace capture is currently enabled (`OH_HiTrace_IsTraceEnabled`, API 19).
#[cfg(feature = "api-19")]
pub fn is_trace_enabled() -> bool {
    // SAFETY: thread-safe query taking no arguments.
    unsafe { sys::OH_HiTrace_IsTraceEnabled() }
}

/// RAII guard for a synchronous `Ex` HiTrace section on the calling thread (API 19).
#[cfg(feature = "api-19")]
#[must_use]
pub struct ScopeEx {
    level: OutputLevel,
    _not_send: PhantomData<Rc<()>>,
}

#[cfg(feature = "api-19")]
impl ScopeEx {
    /// Starts a named synchronous trace section at `level` that finishes when dropped.
    pub fn new(level: OutputLevel, name: &CStr, custom_args: Option<&CStr>) -> Self {
        start_trace_ex(level, name, custom_args);
        Self {
            level,
            _not_send: PhantomData,
        }
    }
}

#[cfg(feature = "api-19")]
impl Drop for ScopeEx {
    fn drop(&mut self) {
        finish_trace_ex(self.level);
    }
}

/// Starts a distributed trace chain in the calling thread's TLS (`OH_HiTrace_BeginChain`).
pub fn begin_chain(name: &CStr, flags: i32) -> sys::HiTraceId {
    // SAFETY: `name` is a valid NUL-terminated C string.
    unsafe { sys::OH_HiTrace_BeginChain(name.as_ptr(), flags) }
}

/// Ends the active distributed trace chain in the calling thread's TLS (`OH_HiTrace_EndChain`).
pub fn end_chain() {
    // SAFETY: clears thread-local HiTrace chain state.
    unsafe { sys::OH_HiTrace_EndChain() };
}

/// Returns the calling thread's current `HiTraceId` (`OH_HiTrace_GetId`).
pub fn current_id() -> sys::HiTraceId {
    // SAFETY: reads thread-local HiTrace chain state.
    unsafe { sys::OH_HiTrace_GetId() }
}

/// Sets the calling thread's current `HiTraceId` (`OH_HiTrace_SetId`).
pub fn set_id(id: &sys::HiTraceId) {
    // SAFETY: `id` points to a valid `HiTraceId`.
    unsafe { sys::OH_HiTrace_SetId(id) };
}

/// Clears the calling thread's current `HiTraceId` (`OH_HiTrace_ClearId`).
pub fn clear_id() {
    // SAFETY: clears thread-local HiTrace chain state.
    unsafe { sys::OH_HiTrace_ClearId() };
}

/// Creates a child span derived from the calling thread's active `HiTraceId` (`OH_HiTrace_CreateSpan`).
pub fn create_span() -> sys::HiTraceId {
    // SAFETY: reads thread-local HiTrace chain state.
    unsafe { sys::OH_HiTrace_CreateSpan() }
}

/// Returns whether `id` is a valid `HiTraceId` (`OH_HiTrace_IsIdValid`).
pub fn is_id_valid(id: &sys::HiTraceId) -> bool {
    // SAFETY: `id` points to a valid `HiTraceId`.
    unsafe { sys::OH_HiTrace_IsIdValid(id) }
}

/// Registers a listener for application trace capture state changes (`OH_HiTrace_RegisterTraceListener`, API 22).
///
/// On success, returns the listener index (`0..=9`) for [`unregister_trace_listener`].
/// On failure, returns the negative HiTrace status code.
#[cfg(feature = "api-22")]
pub fn register_trace_listener(callback: extern "C" fn(bool)) -> Result<i32, i32> {
    // SAFETY: `callback` is a valid C-ABI function pointer with no borrowed user data.
    let status = unsafe { sys::OH_HiTrace_RegisterTraceListener(Some(callback)) };
    if status >= 0 {
        Ok(status)
    } else {
        Err(status)
    }
}

/// Unregisters a listener previously returned by [`register_trace_listener`] (`OH_HiTrace_UnregisterTraceListener`, API 22).
#[cfg(feature = "api-22")]
pub fn unregister_trace_listener(index: i32) -> Result<(), i32> {
    // SAFETY: integer handle validated by the runtime.
    let status = unsafe { sys::OH_HiTrace_UnregisterTraceListener(index) };
    if status == 0 {
        Ok(())
    } else {
        Err(status)
    }
}
