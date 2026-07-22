use std::ffi::CString;
use std::marker::PhantomData;

use ohos_hitrace_sys::*;

use crate::error::{cstring, Result};

/// An open synchronous trace slice, closed when dropped.
///
/// Slices are matched against a per-thread stack, so this guard is `!Send` and
/// nested guards must be dropped in reverse order of creation.
#[must_use = "the trace slice ends as soon as the guard is dropped"]
pub struct TraceScope {
    _not_send: PhantomData<*const ()>,
}

impl Drop for TraceScope {
    fn drop(&mut self) {
        // SAFETY: the guard is bound to the thread that opened the slice.
        unsafe { OH_HiTrace_FinishTrace() };
    }
}

/// Open a synchronous trace slice named `name`.
///
/// ```no_run
/// use ohos_hitrace_binding as hitrace;
///
/// let _slice = hitrace::start_trace("decode_frame")?;
/// # Ok::<(), hitrace::HiTraceError>(())
/// ```
pub fn start_trace(name: &str) -> Result<TraceScope> {
    let name = cstring(name)?;
    // SAFETY: `name` is a valid NUL-terminated string for the duration of the call.
    unsafe { OH_HiTrace_StartTrace(name.as_ptr()) };
    Ok(TraceScope {
        _not_send: PhantomData,
    })
}

/// An open asynchronous trace slice, closed when dropped.
///
/// Asynchronous slices are matched by name and task id rather than by a stack,
/// so this guard may be moved across threads and held across await points. The
/// name is owned by the guard because closing the slice needs it again.
#[must_use = "the trace slice ends as soon as the guard is dropped"]
pub struct AsyncTraceScope {
    name: CString,
    task_id: i32,
}

impl AsyncTraceScope {
    /// The task id distinguishing this slice from others of the same name.
    pub fn task_id(&self) -> i32 {
        self.task_id
    }
}

impl Drop for AsyncTraceScope {
    fn drop(&mut self) {
        // SAFETY: `name` is owned by this guard and outlives the call.
        unsafe { OH_HiTrace_FinishAsyncTrace(self.name.as_ptr(), self.task_id) };
    }
}

/// Open an asynchronous trace slice.
///
/// Concurrent slices sharing a name must use different `task_id`s.
pub fn start_async_trace(name: &str, task_id: i32) -> Result<AsyncTraceScope> {
    let name = cstring(name)?;
    // SAFETY: `name` is a valid NUL-terminated string for the duration of the call.
    unsafe { OH_HiTrace_StartAsyncTrace(name.as_ptr(), task_id) };
    Ok(AsyncTraceScope { name, task_id })
}

/// Record the current value of a named integer counter.
pub fn count_trace(name: &str, count: i64) -> Result<()> {
    let name = cstring(name)?;
    // SAFETY: `name` is a valid NUL-terminated string for the duration of the call.
    unsafe { OH_HiTrace_CountTrace(name.as_ptr(), count) };
    Ok(())
}

#[cfg(feature = "api-19")]
pub use level::*;

#[cfg(feature = "api-19")]
mod level {
    use super::*;
    use ohos_enum_derive::EnumFrom;
    use std::os::raw::c_char;

    /// Optional `const char*` argument: absent means null.
    fn opt_ptr(value: &Option<CString>) -> *const c_char {
        value.as_ref().map_or(std::ptr::null(), |v| v.as_ptr())
    }

    /// Output priority of a trace slice; the system threshold decides which
    /// levels are recorded.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
    #[config(HiTrace_Output_Level, "HiTrace_Output_Level_HITRACE_LEVEL_")]
    pub enum OutputLevel {
        #[suffix("DEBUG")]
        Debug,
        #[suffix("INFO")]
        Info,
        #[suffix("CRITICAL")]
        Critical,
        #[suffix("COMMERCIAL")]
        Commercial,
    }

    /// An open synchronous trace slice with an output level, closed when dropped.
    ///
    /// `!Send` for the same reason as [`TraceScope`]; the level is kept because
    /// closing the slice needs it again.
    #[must_use = "the trace slice ends as soon as the guard is dropped"]
    pub struct TraceScopeEx {
        level: OutputLevel,
        _not_send: PhantomData<*const ()>,
    }

    impl Drop for TraceScopeEx {
        fn drop(&mut self) {
            // SAFETY: the guard is bound to the thread that opened the slice.
            unsafe { OH_HiTrace_FinishTraceEx(self.level.into()) };
        }
    }

    /// Open a synchronous trace slice with an output level.
    ///
    /// `custom_args` is a comma-separated list of `key=value` pairs.
    pub fn start_trace_ex(
        level: OutputLevel,
        name: &str,
        custom_args: Option<&str>,
    ) -> Result<TraceScopeEx> {
        let name = cstring(name)?;
        let custom_args = custom_args.map(cstring).transpose()?;
        // SAFETY: both strings are valid NUL-terminated strings for the duration of
        // the call; a null `customArgs` means "none".
        unsafe {
            OH_HiTrace_StartTraceEx(level.into(), name.as_ptr(), opt_ptr(&custom_args));
        }
        Ok(TraceScopeEx {
            level,
            _not_send: PhantomData,
        })
    }

    /// An open asynchronous trace slice with an output level, closed when dropped.
    #[must_use = "the trace slice ends as soon as the guard is dropped"]
    pub struct AsyncTraceScopeEx {
        level: OutputLevel,
        name: CString,
        task_id: i32,
    }

    impl AsyncTraceScopeEx {
        /// The task id distinguishing this slice from others of the same name.
        pub fn task_id(&self) -> i32 {
            self.task_id
        }
    }

    impl Drop for AsyncTraceScopeEx {
        fn drop(&mut self) {
            // SAFETY: `name` is owned by this guard and outlives the call.
            unsafe {
                OH_HiTrace_FinishAsyncTraceEx(self.level.into(), self.name.as_ptr(), self.task_id)
            };
        }
    }

    /// Open an asynchronous trace slice with an output level.
    ///
    /// Slices sharing a `custom_category` are grouped together for display.
    pub fn start_async_trace_ex(
        level: OutputLevel,
        name: &str,
        task_id: i32,
        custom_category: Option<&str>,
        custom_args: Option<&str>,
    ) -> Result<AsyncTraceScopeEx> {
        let name = cstring(name)?;
        let custom_category = custom_category.map(cstring).transpose()?;
        let custom_args = custom_args.map(cstring).transpose()?;
        // SAFETY: every string is a valid NUL-terminated string for the duration of
        // the call; null means "none" for the optional ones.
        unsafe {
            OH_HiTrace_StartAsyncTraceEx(
                level.into(),
                name.as_ptr(),
                task_id,
                opt_ptr(&custom_category),
                opt_ptr(&custom_args),
            );
        }
        Ok(AsyncTraceScopeEx {
            level,
            name,
            task_id,
        })
    }

    /// Record the current value of a named integer counter at an output level.
    pub fn count_trace_ex(level: OutputLevel, name: &str, count: i64) -> Result<()> {
        let name = cstring(name)?;
        // SAFETY: `name` is a valid NUL-terminated string for the duration of the call.
        unsafe { OH_HiTrace_CountTraceEx(level.into(), name.as_ptr(), count) };
        Ok(())
    }

    /// Whether the calling process is currently allowed to emit trace data.
    pub fn is_trace_enabled() -> bool {
        // SAFETY: no arguments, no shared state.
        unsafe { OH_HiTrace_IsTraceEnabled() }
    }
}

#[cfg(feature = "api-22")]
pub use listener::*;

#[cfg(feature = "api-22")]
mod listener {
    use super::*;
    use crate::error::HiTraceError;

    /// Callback invoked when the process-wide trace switch changes.
    ///
    /// The NDK callback has no user-data parameter, so a Rust closure cannot be
    /// carried into it; only plain function pointers can be registered.
    pub type TraceEventListener = unsafe extern "C" fn(trace_status: bool);

    /// Handle to a registered [`TraceEventListener`].
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[must_use = "the listener stays registered until TraceListenerId::unregister is called"]
    pub struct TraceListenerId(i32);

    impl TraceListenerId {
        /// Unregister this listener.
        pub fn unregister(self) -> Result<()> {
            // SAFETY: `self` can only be obtained from a successful registration.
            let status = unsafe { OH_HiTrace_UnregisterTraceListener(self.0) };
            if status == 0 {
                Ok(())
            } else {
                Err(HiTraceError::InvalidListener)
            }
        }
    }

    /// Register a trace switch listener. At most 10 may be registered at once.
    ///
    /// Registration is not RAII: the returned id owns no memory, and listeners are
    /// normally kept for the lifetime of the process.
    ///
    /// # Safety
    ///
    /// `listener` may be invoked from any thread at any point until it is
    /// unregistered, so it must be sound to call concurrently with the rest of the
    /// program.
    pub unsafe fn register_trace_listener(listener: TraceEventListener) -> Result<TraceListenerId> {
        let index = unsafe { OH_HiTrace_RegisterTraceListener(Some(listener)) };
        match index {
            -1 => Err(HiTraceError::ListenerLimit),
            index if index < 0 => Err(HiTraceError::InvalidListener),
            index => Ok(TraceListenerId(index)),
        }
    }
}
