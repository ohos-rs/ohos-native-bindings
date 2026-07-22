use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::os::raw::c_int;

use bitflags::bitflags;
use ohos_enum_derive::EnumFrom;
use ohos_hitrace_sys::*;

use crate::error::{cstring, HiTraceError, Result};

bitflags! {
    /// Behaviour flags carried by a [`TraceId`]. An empty set is
    /// `HITRACE_FLAG_DEFAULT`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TraceFlags: c_int {
        /// Trace asynchronous calls as well as synchronous ones.
        const INCLUDE_ASYNC = HiTrace_Flag_HITRACE_FLAG_INCLUDE_ASYNC as c_int;
        /// Do not create spans.
        const DONOT_CREATE_SPAN = HiTrace_Flag_HITRACE_FLAG_DONOT_CREATE_SPAN as c_int;
        /// Automatically add trace points to spans.
        const TP_INFO = HiTrace_Flag_HITRACE_FLAG_TP_INFO as c_int;
        /// Do not print the start/end of the trace task.
        const NO_BE_INFO = HiTrace_Flag_HITRACE_FLAG_NO_BE_INFO as c_int;
        /// Do not add the trace id to the log.
        const DONOT_ENABLE_LOG = HiTrace_Flag_HITRACE_FLAG_DONOT_ENABLE_LOG as c_int;
        /// Tracing is triggered by faults.
        const FAULT_TRIGGER = HiTrace_Flag_HITRACE_FLAG_FAULT_TRIGGER as c_int;
        /// Add trace points only for device-to-device call chains.
        const D2D_TP_INFO = HiTrace_Flag_HITRACE_FLAG_D2D_TP_INFO as c_int;
    }
}

/// Communication mode reported by a trace point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(HiTrace_Communication_Mode, "HiTrace_Communication_Mode_HITRACE_CM_")]
pub enum CommunicationMode {
    #[suffix("DEFAULT")]
    Default,
    #[suffix("THREAD")]
    Thread,
    #[suffix("PROCESS")]
    Process,
    #[suffix("DEVICE")]
    Device,
}

/// Trace point kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(HiTrace_Tracepoint_Type, "HiTrace_Tracepoint_Type_HITRACE_TP_")]
pub enum TracepointType {
    /// Client send.
    #[suffix("CS")]
    ClientSend,
    /// Client receive.
    #[suffix("CR")]
    ClientReceive,
    /// Server send.
    #[suffix("SS")]
    ServerSend,
    /// Server receive.
    #[suffix("SR")]
    ServerReceive,
    #[suffix("GENERAL")]
    General,
}

/// A call chain identifier: chain id, span id, parent span id and flags.
///
/// This is a plain value owned by the caller — the native API has no allocation
/// or destructor for it — so it is `Copy` and carries no lifetime.
#[derive(Debug, Clone, Copy)]
pub struct TraceId(HiTraceId);

impl Default for TraceId {
    fn default() -> Self {
        Self::invalid()
    }
}

impl TraceId {
    /// An initialized but invalid id.
    pub fn invalid() -> Self {
        let mut id = MaybeUninit::<HiTraceId>::uninit();
        // SAFETY: OH_HiTrace_InitId fully initializes the struct it is given.
        unsafe {
            OH_HiTrace_InitId(id.as_mut_ptr());
            TraceId(id.assume_init())
        }
    }

    /// Rebuild an id from the byte form produced by [`TraceId::to_bytes`].
    ///
    /// The slice must be exactly one `HiTraceId` wide; any other length is
    /// rejected, as the native call would otherwise return an invalid id without
    /// reporting an error.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        if bytes.len() != std::mem::size_of::<HiTraceId>() {
            return Err(HiTraceError::InvalidByteLength);
        }
        let mut id = Self::invalid();
        // SAFETY: `bytes` is exactly one HiTraceId wide, so its length fits `c_int`;
        // `id` points at an initialized struct.
        unsafe { OH_HiTrace_IdFromBytes(&mut id.0, bytes.as_ptr(), bytes.len() as c_int) };
        Ok(id)
    }

    /// Serialize the id for caching or cross-process propagation.
    ///
    /// An invalid id serializes to an empty `Vec` (the native call writes
    /// nothing for it), which [`TraceId::from_bytes`] rejects with
    /// [`HiTraceError::InvalidByteLength`] — only valid ids round-trip.
    pub fn to_bytes(self) -> Vec<u8> {
        let mut buf = vec![0u8; std::mem::size_of::<HiTraceId>()];
        // SAFETY: pIdArray/len describe `buf`, which is large enough for a HiTraceId.
        let len = unsafe { OH_HiTrace_IdToBytes(&self.0, buf.as_mut_ptr(), buf.len() as c_int) };
        buf.truncate(len.max(0) as usize);
        buf
    }

    /// Whether this id is valid.
    pub fn is_valid(self) -> bool {
        // SAFETY: `self.0` is an initialized HiTraceId.
        unsafe { OH_HiTrace_IsIdValid(&self.0) }
    }

    /// The chain id shared by every span of one call chain.
    pub fn chain_id(self) -> u64 {
        // SAFETY: `self.0` is an initialized HiTraceId.
        unsafe { OH_HiTrace_GetChainId(&self.0) }
    }

    /// Set the chain id.
    pub fn set_chain_id(&mut self, chain_id: u64) {
        // SAFETY: `self.0` is an initialized HiTraceId.
        unsafe { OH_HiTrace_SetChainId(&mut self.0, chain_id) };
    }

    /// The span id.
    pub fn span_id(self) -> u64 {
        // SAFETY: `self.0` is an initialized HiTraceId.
        unsafe { OH_HiTrace_GetSpanId(&self.0) }
    }

    /// Set the span id.
    pub fn set_span_id(&mut self, span_id: u64) {
        // SAFETY: `self.0` is an initialized HiTraceId.
        unsafe { OH_HiTrace_SetSpanId(&mut self.0, span_id) };
    }

    /// The parent span id.
    pub fn parent_span_id(self) -> u64 {
        // SAFETY: `self.0` is an initialized HiTraceId.
        unsafe { OH_HiTrace_GetParentSpanId(&self.0) }
    }

    /// Set the parent span id.
    pub fn set_parent_span_id(&mut self, parent_span_id: u64) {
        // SAFETY: `self.0` is an initialized HiTraceId.
        unsafe { OH_HiTrace_SetParentSpanId(&mut self.0, parent_span_id) };
    }

    /// The flags currently set.
    pub fn flags(self) -> TraceFlags {
        // SAFETY: `self.0` is an initialized HiTraceId.
        let raw = unsafe { OH_HiTrace_GetFlags(&self.0) };
        TraceFlags::from_bits_truncate(raw)
    }

    /// Replace the flags.
    pub fn set_flags(&mut self, flags: TraceFlags) {
        // SAFETY: `self.0` is an initialized HiTraceId.
        unsafe { OH_HiTrace_SetFlags(&mut self.0, flags.bits()) };
    }

    /// Add `flags` to the ones already set.
    pub fn enable_flags(&mut self, flags: TraceFlags) {
        // The native API mutates through a `const` pointer, so derive it from `&mut` to
        // keep write provenance.
        let id: *mut HiTraceId = &mut self.0;
        for flag in flags.iter() {
            // SAFETY: `id` points at an initialized HiTraceId we hold exclusively.
            unsafe { OH_HiTrace_EnableFlag(id.cast_const(), flag.bits() as HiTrace_Flag) };
        }
    }

    /// Whether every flag in `flags` is enabled.
    pub fn is_flag_enabled(self, flags: TraceFlags) -> bool {
        flags.iter().all(|flag| {
            // SAFETY: `self.0` is an initialized HiTraceId.
            unsafe { OH_HiTrace_IsFlagEnabled(&self.0, flag.bits() as HiTrace_Flag) }
        })
    }

    /// The underlying `HiTraceId`.
    pub fn as_raw(&self) -> &HiTraceId {
        &self.0
    }

    /// Wrap an id obtained from the raw bindings.
    pub fn from_raw(raw: HiTraceId) -> Self {
        TraceId(raw)
    }

    /// Print a trace point carrying this id and `text`.
    pub fn tracepoint(
        &self,
        mode: CommunicationMode,
        kind: TracepointType,
        text: &str,
    ) -> Result<()> {
        let text = cstring(text)?;
        // SAFETY: `text` is passed as a `%s` argument rather than as the format
        // string, so it cannot be interpreted as format directives.
        unsafe {
            OH_HiTrace_Tracepoint(
                mode.into(),
                kind.into(),
                &self.0,
                c"%s".as_ptr(),
                text.as_ptr(),
            );
        }
        Ok(())
    }
}

/// The active call chain of the current thread, ended when dropped.
///
/// The chain id lives in thread-local storage, so this guard is `!Send`: ending
/// the chain from another thread would clear that thread's TLS instead. On drop
/// the chain is ended only if it is still the thread's current one; a chain
/// installed later via [`set_current_id`] or cleared with [`clear_current_id`]
/// is left untouched.
#[must_use = "the call chain ends as soon as the guard is dropped"]
pub struct ChainScope {
    id: TraceId,
    _not_send: PhantomData<*const ()>,
}

impl ChainScope {
    /// The id created for this chain, or an invalid id if a chain was already
    /// active and this call was ignored.
    pub fn id(&self) -> TraceId {
        self.id
    }
}

impl Drop for ChainScope {
    fn drop(&mut self) {
        // `EndChain` clears whatever id is in this thread's TLS, without the
        // chain-id match check the innerkits `HiTraceChainEnd` performs. Two
        // ways the TLS chain can differ from ours: a nested `begin_chain` is
        // ignored by the runtime and yields an invalid id, and
        // `clear_current_id`/`set_current_id` can replace the chain underneath
        // the guard. Only end the chain this guard actually opened.
        if self.id.is_valid() && current_id().chain_id() == self.id.chain_id() {
            // SAFETY: the guard is bound to the thread that began the chain.
            unsafe { OH_HiTrace_EndChain() };
        }
    }
}

/// Start a call chain on the current thread and store its id in TLS.
///
/// Only the first call on a thread takes effect. When a chain is already active
/// the runtime ignores the call and the returned guard holds an invalid
/// [`id`](ChainScope::id) and ends nothing when dropped.
pub fn begin_chain(name: &str, flags: TraceFlags) -> Result<ChainScope> {
    let name = cstring(name)?;
    // SAFETY: `name` is a valid NUL-terminated string for the duration of the call.
    let id = unsafe { OH_HiTrace_BeginChain(name.as_ptr(), flags.bits()) };
    Ok(ChainScope {
        id: TraceId(id),
        _not_send: PhantomData,
    })
}

/// The trace id of the current thread, or an invalid id if it has none.
pub fn current_id() -> TraceId {
    // SAFETY: reads the calling thread's TLS.
    TraceId(unsafe { OH_HiTrace_GetId() })
}

/// Set the trace id of the current thread. Invalid ids are ignored.
pub fn set_current_id(id: TraceId) {
    // SAFETY: `id.0` is an initialized HiTraceId.
    unsafe { OH_HiTrace_SetId(&id.0) };
}

/// Clear and invalidate the trace id of the current thread.
pub fn clear_current_id() {
    // SAFETY: clears the calling thread's TLS.
    unsafe { OH_HiTrace_ClearId() };
}

/// Derive a new span from the trace id of the current thread.
///
/// Returns the thread's own id when span creation is disabled by
/// [`TraceFlags::DONOT_CREATE_SPAN`].
pub fn create_span() -> TraceId {
    // SAFETY: reads the calling thread's TLS.
    TraceId(unsafe { OH_HiTrace_CreateSpan() })
}
