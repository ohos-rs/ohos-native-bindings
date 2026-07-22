//! Memory-allocator plumbing for the native APIs that hand back a
//! caller-allocated buffer.
//!
//! `OH_IPCParcel_ReadInterfaceToken`, `OH_IPCRemoteProxy_GetInterfaceDescriptor`
//! and `OH_IPCSkeleton_ResetCallingIdentity` take an `OH_IPC_MemAllocator` and
//! write into memory obtained from it; the header states that the caller must
//! release that memory, and must do so even when the call reports an error.
//! Nothing in the header names the matching deallocator, so this module both
//! allocates and frees, and never lets the buffer escape the safe layer.
//!
//! To free with the exact `Layout` it allocated, the allocator over-allocates a
//! header and stores the total size there, returning the address just past it.
//! The reported length is therefore never used for deallocation.

use crate::error::{check, IpcError, Result};
use ohos_ipc_sys as sys;
use std::alloc::Layout;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

/// Size and alignment of the bookkeeping header. 16 bytes is at least
/// `align_of::<usize>()` on every supported target and keeps the returned
/// pointer suitably aligned for any C scalar type.
const HEADER: usize = 16;

/// Allocator handed to the native side.
unsafe extern "C" fn ipc_alloc(len: i32) -> *mut c_void {
    let Ok(len) = usize::try_from(len) else {
        return ptr::null_mut();
    };
    let Some(total) = len.checked_add(HEADER) else {
        return ptr::null_mut();
    };
    let Ok(layout) = Layout::from_size_align(total, HEADER) else {
        return ptr::null_mut();
    };
    // SAFETY: `total` is at least `HEADER`, so the layout has a non-zero size.
    let base = unsafe { std::alloc::alloc(layout) };
    if base.is_null() {
        return ptr::null_mut();
    }
    // SAFETY: `base` is a fresh allocation of `total >= HEADER` bytes aligned
    // to `HEADER`, so writing a `usize` at its start and offsetting by `HEADER`
    // both stay in bounds.
    unsafe {
        base.cast::<usize>().write(total);
        base.add(HEADER).cast()
    }
}

/// Owns a buffer produced by [`ipc_alloc`] and releases it on drop.
struct Allocated(*mut c_char);

impl Drop for Allocated {
    fn drop(&mut self) {
        if self.0.is_null() {
            return;
        }
        // SAFETY: a non-null pointer here came from `ipc_alloc`, so it is
        // `HEADER` bytes past a live allocation whose total size is stored in
        // that header.
        unsafe {
            let base = self.0.cast::<u8>().sub(HEADER);
            let total = base.cast::<usize>().read();
            if let Ok(layout) = Layout::from_size_align(total, HEADER) {
                std::alloc::dealloc(base, layout);
            }
        }
    }
}

/// Run a native call of the `(out_ptr, out_len, allocator)` shape and copy the
/// string it produced into an owned [`CString`].
///
/// The buffer is released on every path, including the error path, as the
/// header requires.
pub(crate) fn with_allocator(
    context: &'static str,
    call: impl FnOnce(*mut *mut c_char, *mut i32, sys::OH_IPC_MemAllocator) -> c_int,
) -> Result<CString> {
    let mut out: *mut c_char = ptr::null_mut();
    let mut len: i32 = 0;
    let code = call(&mut out, &mut len, Some(ipc_alloc));
    let buffer = Allocated(out);
    check(code)?;

    if buffer.0.is_null() {
        return Err(IpcError::Failed(context));
    }
    let len = usize::try_from(len).map_err(|_| IpcError::Failed(context))?;
    // SAFETY: on success the native side wrote `len` bytes, terminator
    // included, into the buffer that is still owned by `buffer`.
    let bytes = unsafe { std::slice::from_raw_parts(buffer.0.cast::<u8>(), len) };
    let value = CStr::from_bytes_until_nul(bytes).map_err(|_| IpcError::Failed(context))?;
    Ok(value.to_owned())
}
