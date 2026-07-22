use crate::error::{HiDebugError, Result};
use crate::r#type::CrashObjType;
use ohos_hidebug_sys as sys;

/// Handle to a crash object, as returned by [`set_crash_obj`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrashObjHandle(u64);

impl CrashObjHandle {
    /// The raw handle; `0` when no crash object was attached before.
    pub fn raw(self) -> u64 {
        self.0
    }
}

/// Buffer size the crash dumper reads for a given type.
fn required_len(kind: CrashObjType) -> usize {
    match kind {
        CrashObjType::String => 1,
        CrashObjType::Memory64B => 64,
        CrashObjType::Memory256B => 256,
        CrashObjType::Memory1024B => 1024,
        CrashObjType::Memory2048B => 2048,
        CrashObjType::Memory4096B => 4096,
    }
}

fn invalid_argument() -> HiDebugError {
    HiDebugError::from_code(sys::HiDebug_ErrorCode_HIDEBUG_INVALID_ARGUMENT)
}

/// Attach `data` to the crash context so it is dumped with the crash report.
///
/// hidebug stores the pointer, not a copy, and dereferences it only when the
/// process crashes — the buffer therefore has to stay valid for the rest of the
/// process lifetime, which is why only `'static` data is accepted. Passing a
/// borrow would leave a dangling pointer in the crash handler.
///
/// The crash dumper reads a fixed number of bytes for each type, so `data` must
/// be at least that large; [`CrashObjType::String`] additionally requires a
/// NUL terminator. Returns `Err` otherwise — the check is enforced in release
/// builds because an undersized buffer means an out-of-bounds read from the
/// crash handler.
///
/// On success returns the handle of the *previously* attached object (`0` if
/// none).
pub fn set_crash_obj(kind: CrashObjType, data: &'static mut [u8]) -> Result<CrashObjHandle> {
    if data.len() < required_len(kind) {
        return Err(invalid_argument());
    }
    if kind == CrashObjType::String && data.last() != Some(&0) {
        return Err(invalid_argument());
    }
    // SAFETY: data is 'static, so the pointer stays valid until the process
    // exits or crashes.
    Ok(CrashObjHandle(unsafe {
        sys::OH_HiDebug_SetCrashObj(kind.into(), data.as_mut_ptr().cast())
    }))
}

/// [`set_crash_obj`] taking ownership of `data` and leaking it to obtain the
/// required `'static` lifetime. The memory is never reclaimed.
pub fn set_crash_obj_leaked(kind: CrashObjType, data: Vec<u8>) -> Result<CrashObjHandle> {
    set_crash_obj(kind, Box::leak(data.into_boxed_slice()))
}

/// Detach a crash object previously attached with [`set_crash_obj`].
pub fn reset_crash_obj(handle: CrashObjHandle) {
    // SAFETY: handle comes from OH_HiDebug_SetCrashObj.
    unsafe { sys::OH_HiDebug_ResetCrashObj(handle.raw()) };
}
