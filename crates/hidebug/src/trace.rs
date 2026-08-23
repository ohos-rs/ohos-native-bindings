use crate::error::{check, Result};
use crate::r#type::TraceFlag;
use ohos_hidebug_sys as sys;
use std::ffi::CStr;
use std::ops::{BitOr, BitOrAssign};

/// Room for the trace file name hidebug writes back.
const FILE_NAME_CAP: usize = 1024;

/// A set of trace tags, selecting which subsystems a capture records.
///
/// Values come from [`trace_tag`](crate::trace_tag) and combine with `|`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TraceTags(u64);

impl TraceTags {
    /// An empty tag set.
    pub const fn empty() -> Self {
        TraceTags(0)
    }

    /// Wrap a raw tag bit mask.
    pub const fn from_bits(bits: u64) -> Self {
        TraceTags(bits)
    }

    /// The raw tag bit mask.
    pub const fn bits(self) -> u64 {
        self.0
    }

    /// Whether every tag in `other` is set.
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl BitOr for TraceTags {
    type Output = TraceTags;
    fn bitor(self, rhs: Self) -> Self {
        TraceTags(self.0 | rhs.0)
    }
}

impl BitOrAssign for TraceTags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// An in-progress application trace capture.
///
/// Capture keeps writing (up to the configured size limit) until this guard is
/// stopped or dropped. Only one capture can run at a time, so a guard dropped
/// without [`stop`](Self::stop) still stops the capture on a best-effort basis.
pub struct AppTraceCapture {
    file_name: String,
    done: bool,
}

impl AppTraceCapture {
    /// Name of the file the trace is being written into.
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Stop the capture, returning the native error on failure. Consumes the
    /// guard.
    ///
    /// On failure the capture may still be running, so the drop retry stays
    /// armed; stopping a capture that already stopped is benign.
    pub fn stop(mut self) -> Result<()> {
        // SAFETY: no arguments, no out parameters.
        let result = unsafe { check(sys::OH_HiDebug_StopAppTraceCapture()) };
        self.done = result.is_ok();
        result
    }
}

impl Drop for AppTraceCapture {
    fn drop(&mut self) {
        if self.done {
            return;
        }
        // SAFETY: no arguments, no out parameters; best-effort stop on drop.
        unsafe { sys::OH_HiDebug_StopAppTraceCapture() };
    }
}

/// Start capturing an application trace into a file.
///
/// `tags` selects the subsystems to record (see [`trace_tag`](crate::trace_tag)),
/// `limit_size` the maximum file size in bytes (at most 500 MiB). The returned
/// guard stops the capture when dropped; call [`AppTraceCapture::stop`] to
/// observe the stop result.
pub fn start_app_trace_capture(
    flag: TraceFlag,
    tags: TraceTags,
    limit_size: u32,
) -> Result<AppTraceCapture> {
    let mut name = vec![0u8; FILE_NAME_CAP];
    // SAFETY: fileName points at a `length`-byte owned buffer.
    unsafe {
        check(sys::OH_HiDebug_StartAppTraceCapture(
            flag.into(),
            tags.bits(),
            limit_size,
            name.as_mut_ptr().cast(),
            name.len() as u32,
        ))?;
    }
    // `length` is input-only — hidebug never reports how much it wrote, so the
    // NUL terminator is the only length signal.
    // SAFETY: on success hidebug wrote a NUL-terminated name into the buffer.
    let file_name = unsafe { CStr::from_ptr(name.as_ptr().cast()) }
        .to_string_lossy()
        .into_owned();
    Ok(AppTraceCapture {
        file_name,
        done: false,
    })
}

/// Callback delivering the result of [`request_trace`].
///
/// `file_path` is the generated trace file, or NULL on failure.
#[cfg(feature = "api-24")]
pub type RequestTraceCallback = unsafe extern "C" fn(
    error_code: sys::HiDebug_ErrorCode,
    file_path: *const std::os::raw::c_char,
);

/// Request a trace capture with the given configuration; `callback` is invoked
/// asynchronously once the trace file is available.
///
/// `identifier` prefixes the generated file name. The request outlives this
/// call and the native header does not state that the identifier is copied, so
/// only `'static` identifiers are accepted.
#[cfg(feature = "api-24")]
pub fn request_trace(
    identifier: &'static CStr,
    buffer_size_kb: u32,
    duration_ms: u32,
    callback: RequestTraceCallback,
) -> Result<()> {
    let mut config = sys::OH_HiDebug_RequestTraceConfig {
        identifier: identifier.as_ptr(),
        bufferSizeKb: buffer_size_kb,
        durationMs: duration_ms,
        reserved: 0,
    };
    // SAFETY: config is valid for the call and the identifier it points at is
    // 'static.
    unsafe { check(sys::OH_HiDebug_RequestTrace(&mut config, Some(callback))) }
}
