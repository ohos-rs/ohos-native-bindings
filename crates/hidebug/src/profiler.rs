use crate::error::{check_profiler, Result};
use crate::r#type::ResourceType;
use ohos_hidebug_sys as sys;
use std::ffi::CStr;

/// Resource profiler settings. Zero fields select the hidebug defaults.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ResProfilerConfig {
    /// Maximum collection duration, in seconds.
    pub max_duration: u32,
    /// Filter size, in bytes.
    pub filter_size: u32,
    /// Maximum stack trace depth, in frames.
    pub max_stack_depth: u32,
    /// Statistics interval, in seconds.
    pub statistics_interval: u32,
    /// Sample interval, in bytes. Allocations at or below it are sampled
    /// probabilistically, larger ones fully.
    pub sample_interval: u32,
}

/// A profiling result delivered to a [`ProfilingCallback`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfilingResult {
    /// The resource kind that was profiled.
    pub resource_type: ResourceType,
    /// Output file, or `None` when profiling produced no output.
    pub file_path: Option<String>,
}

impl ProfilingResult {
    /// Copy a native profiling result out of the callback payload.
    ///
    /// Returns `None` for a null pointer or an unrecognized resource type.
    ///
    /// # Safety
    ///
    /// `raw` must be the pointer hidebug passed to the callback, valid for the
    /// duration of that callback.
    pub unsafe fn from_raw(raw: *mut sys::OH_HiDebug_ProfilingResult) -> Option<Self> {
        // SAFETY: caller guarantees raw is the live callback payload.
        let raw = unsafe { raw.as_ref()? };
        let file_path = if raw.filePath.is_null() {
            None
        } else {
            // SAFETY: a non-null filePath is a NUL-terminated string valid for
            // the callback.
            Some(
                unsafe { CStr::from_ptr(raw.filePath) }
                    .to_string_lossy()
                    .into_owned(),
            )
        };
        Some(ProfilingResult {
            resource_type: ResourceType::try_from_raw(raw.resourceType)?,
            file_path,
        })
    }
}

/// Callback receiving the profiling result once profiling stops.
///
/// Turn the raw payload into a [`ProfilingResult`] with
/// [`ProfilingResult::from_raw`].
pub type ProfilingCallback = unsafe extern "C" fn(result: *mut sys::OH_HiDebug_ProfilingResult);

/// A running resource profiler session.
///
/// Profiling keeps running until this guard is stopped or dropped, or until it
/// reaches `max_duration`. A guard dropped without [`stop`](Self::stop) stops
/// profiling on a best-effort basis.
pub struct ProfilerSession {
    done: bool,
}

impl ProfilerSession {
    /// Stop profiling, returning the native error on failure. Consumes the
    /// guard.
    ///
    /// On failure the profiler may still be running, so the drop retry stays
    /// armed; stopping a profiler that already stopped is benign.
    pub fn stop(mut self) -> Result<()> {
        // SAFETY: no arguments, no out parameters.
        let result = unsafe { check_profiler(sys::OH_HiDebug_StopProfiler()) };
        self.done = result.is_ok();
        result
    }
}

impl Drop for ProfilerSession {
    fn drop(&mut self) {
        if self.done {
            return;
        }
        // SAFETY: no arguments, no out parameters; best-effort stop on drop.
        unsafe { sys::OH_HiDebug_StopProfiler() };
    }
}

/// Start profiling `kind` for the current process.
///
/// Profiling runs asynchronously; `callback` is invoked once it stops, either
/// through [`ProfilerSession::stop`], by dropping the guard, or by reaching
/// `max_duration`.
pub fn start_profiler(
    kind: ResourceType,
    config: &ResProfilerConfig,
    callback: ProfilingCallback,
) -> Result<ProfilerSession> {
    let mut raw = sys::OH_HiDebug_ResProfilerConfig {
        maxDuration: config.max_duration,
        filterSize: config.filter_size,
        maxStackDepth: config.max_stack_depth,
        statisticsInterval: config.statistics_interval,
        sampleInterval: config.sample_interval,
    };
    // SAFETY: raw is valid for the call and holds no pointers of its own.
    unsafe {
        check_profiler(sys::OH_HiDebug_StartProfiler(
            kind.into(),
            &mut raw,
            Some(callback),
        ))?;
    }
    Ok(ProfilerSession { done: false })
}
