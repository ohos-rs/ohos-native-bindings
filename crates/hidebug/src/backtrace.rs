use crate::error::{check, HiDebugError, Result};
use ohos_hidebug_sys as sys;
use std::any::Any;
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use std::panic::{self, AssertUnwindSafe};
use std::ptr::NonNull;

/// A symbolized stack frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StackFrame {
    Js(JsStackFrame),
    Native(NativeStackFrame),
}

/// An ArkTS / JS frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsStackFrame {
    /// pc relative to the start of the mapped file.
    pub relative_pc: u64,
    pub line: i32,
    pub column: i32,
    pub map_name: Option<String>,
    pub function_name: Option<String>,
    pub url: Option<String>,
    pub package_name: Option<String>,
}

/// A native frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeStackFrame {
    /// pc relative to the start of the mapped file.
    pub relative_pc: u64,
    /// pc relative to the start of the enclosing function.
    pub func_offset: u64,
    pub map_name: Option<String>,
    pub function_name: Option<String>,
    pub build_id: Option<String>,
    pub reserved: Option<String>,
}

fn owned(text: *const c_char) -> Option<String> {
    if text.is_null() {
        return None;
    }
    // SAFETY: non-null frame strings are NUL-terminated and valid until the
    // symbolization callback returns.
    Some(
        unsafe { CStr::from_ptr(text) }
            .to_string_lossy()
            .into_owned(),
    )
}

impl StackFrame {
    /// Frame strings are only valid while the callback runs, so everything is
    /// copied here. Returns `None` for frame types this crate does not know.
    fn from_raw(frame: &sys::HiDebug_StackFrame) -> Option<Self> {
        match frame.type_ {
            sys::HiDebug_StackFrameType_HIDEBUG_STACK_FRAME_TYPE_JS => {
                // SAFETY: the union discriminant says this is a JS frame.
                let js = unsafe { frame.frame.js };
                Some(StackFrame::Js(JsStackFrame {
                    relative_pc: js.relativePc,
                    line: js.line,
                    column: js.column,
                    map_name: owned(js.mapName),
                    function_name: owned(js.functionName),
                    url: owned(js.url),
                    package_name: owned(js.packageName),
                }))
            }
            sys::HiDebug_StackFrameType_HIDEBUG_STACK_FRAME_TYPE_NATIVE => {
                // SAFETY: the union discriminant says this is a native frame.
                let native = unsafe { frame.frame.native };
                Some(StackFrame::Native(NativeStackFrame {
                    relative_pc: native.relativePc,
                    func_offset: native.funcOffset,
                    map_name: owned(native.mapName),
                    function_name: owned(native.functionName),
                    build_id: owned(native.buildId),
                    reserved: owned(native.reserved),
                }))
            }
            _ => None,
        }
    }
}

/// Sink state passed through the symbolization callback: the user closure plus
/// any panic payload caught from it.
struct SymbolicSink<'a> {
    sink: &'a mut dyn FnMut(&StackFrame),
    panic_payload: Option<Box<dyn Any + Send>>,
}

unsafe extern "C" fn symbolic_trampoline(
    _pc: *mut c_void,
    arg: *mut c_void,
    frame: *const sys::HiDebug_StackFrame,
) {
    if arg.is_null() || frame.is_null() {
        return;
    }
    // SAFETY: arg is the sink state handed to OH_HiDebug_SymbolicAddress.
    let state = unsafe { &mut *(arg as *mut SymbolicSink) };
    if state.panic_payload.is_some() {
        return;
    }
    // SAFETY: frame is valid until this callback returns.
    if let Some(frame) = StackFrame::from_raw(unsafe { &*frame }) {
        // A panic must not unwind through the extern "C" boundary (that would
        // abort); catch it here and resume it once the native call returns.
        if let Err(payload) = panic::catch_unwind(AssertUnwindSafe(|| (state.sink)(&frame))) {
            state.panic_payload = Some(payload);
        }
    }
}

/// Native unwinder state, needed to walk stacks and symbolize addresses.
pub struct BacktraceObject {
    raw: NonNull<sys::HiDebug_Backtrace_Object__>,
}

impl BacktraceObject {
    /// Create a backtrace object; `None` on architectures without backtrace
    /// support.
    pub fn new() -> Option<Self> {
        // SAFETY: no arguments; NULL signals an unsupported architecture.
        let raw = unsafe { sys::OH_HiDebug_CreateBacktraceObject() };
        NonNull::new(raw).map(|raw| BacktraceObject { raw })
    }

    /// Walk up to `max_frames` frames starting at `start_fp` and return their
    /// program counters. Signal-safe.
    ///
    /// At most `i32::MAX` frames are walked; a larger `max_frames` is capped.
    ///
    /// # Safety
    ///
    /// `start_fp` must be the frame pointer of a live frame whose stack stays
    /// valid for the duration of the call.
    pub unsafe fn backtrace_from_fp(
        &self,
        start_fp: *mut c_void,
        max_frames: usize,
    ) -> Vec<*mut c_void> {
        let max_frames = max_frames.min(i32::MAX as usize);
        let mut pcs: Vec<*mut c_void> = Vec::with_capacity(max_frames);
        // SAFETY: pcArray points at `max_frames` writable slots; the return
        // value is the number of frames actually written.
        let written = unsafe {
            sys::OH_HiDebug_BacktraceFromFp(
                self.raw.as_ptr(),
                start_fp,
                pcs.as_mut_ptr(),
                max_frames as i32,
            )
        };
        let written = (written.max(0) as usize).min(max_frames);
        // SAFETY: the call initialized `written` leading elements.
        unsafe { pcs.set_len(written) };
        pcs
    }

    /// Symbolize `pc` and pass the frame to `sink`. Not signal-safe.
    ///
    /// The frame is only borrowed for the duration of `sink`; copy out whatever
    /// is needed. A panic in `sink` is caught at the callback boundary and
    /// resumed after the native call returns.
    pub fn symbolic_address_with<F: FnMut(&StackFrame)>(
        &self,
        pc: *mut c_void,
        mut sink: F,
    ) -> Result<()> {
        let mut state = SymbolicSink {
            sink: &mut sink,
            panic_payload: None,
        };
        let arg: *mut SymbolicSink = &mut state;
        // SAFETY: hidebug invokes the callback before returning, so arg stays
        // valid for as long as it is used.
        let result = unsafe {
            check(sys::OH_HiDebug_SymbolicAddress(
                self.raw.as_ptr(),
                pc,
                arg.cast(),
                Some(symbolic_trampoline),
            ))
        };
        if let Some(payload) = state.panic_payload {
            panic::resume_unwind(payload);
        }
        result
    }

    /// Symbolize `pc` into an owned frame. Not signal-safe.
    pub fn symbolic_address(&self, pc: *mut c_void) -> Result<StackFrame> {
        let mut frame = None;
        self.symbolic_address_with(pc, |parsed| frame = Some(parsed.clone()))?;
        frame.ok_or_else(|| {
            HiDebugError::from_code(sys::HiDebug_ErrorCode_HIDEBUG_INVALID_SYMBOLIC_PC_ADDRESS)
        })
    }
}

impl Drop for BacktraceObject {
    fn drop(&mut self) {
        // SAFETY: raw comes from OH_HiDebug_CreateBacktraceObject and is
        // destroyed exactly once.
        unsafe { sys::OH_HiDebug_DestroyBacktraceObject(self.raw.as_ptr()) };
    }
}
