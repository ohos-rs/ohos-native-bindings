use ohos_dlp_permission_sys as sys;
use std::ffi::{c_char, c_void, CStr, CString};
use std::ptr::NonNull;

use crate::error::{check, DlpError, Result};
use crate::types::{Actions, FileAccess, PermissionInfo};

/// Owns a `char *` buffer handed out by the DLP kit and releases it on drop.
///
/// # Ownership contract
///
/// `libohdlp_permission.so` exports no deallocator, so the release contract is
/// not expressed in the headers. It is taken from the native implementation,
/// which allocates both buffers with `malloc`, and from the official NDK usage
/// guide, which releases them with `free`. Should the upstream allocator ever
/// change, this is the single place to adjust.
struct OwnedNativeString(NonNull<c_char>);

impl OwnedNativeString {
    /// Take ownership of a buffer produced by the native API.
    ///
    /// Returns `None` for a null pointer.
    ///
    /// # Safety
    ///
    /// `ptr` must be null or point to a NUL-terminated buffer allocated by the
    /// DLP kit that no one else releases.
    unsafe fn from_raw(ptr: *mut c_char) -> Option<Self> {
        NonNull::new(ptr).map(OwnedNativeString)
    }

    /// Copy the buffer into a `String`, rejecting non-UTF-8 content.
    fn to_utf8(&self) -> Result<String> {
        // SAFETY: the pointer is non-null and, per the ownership contract
        // above, points to a NUL-terminated buffer owned by `self`.
        let bytes = unsafe { CStr::from_ptr(self.0.as_ptr()) }.to_bytes();
        std::str::from_utf8(bytes)
            .map(str::to_owned)
            .map_err(|_| DlpError::InvalidUtf8)
    }
}

impl Drop for OwnedNativeString {
    fn drop(&mut self) {
        // SAFETY: the buffer was allocated with `malloc` by the DLP kit and is
        // released exactly once, here, because `OwnedNativeString` is not
        // `Copy` and hands out no raw pointer.
        unsafe { libc::free(self.0.as_ptr().cast::<c_void>()) }
    }
}

/// Convert a Rust string into a C string, rejecting interior NUL bytes.
fn to_c_string(value: &str) -> Result<CString> {
    CString::new(value).map_err(|_| DlpError::InteriorNul)
}

/// Obtain the permission information of the DLP file opened by this sandbox
/// application.
///
/// Only callable from a DLP sandbox application; otherwise the call fails with
/// `DLP_ErrCode` 19100006.
pub fn permission_info() -> Result<PermissionInfo> {
    let mut access: sys::DLP_FileAccess = 0;
    let mut flags: u32 = 0;
    check(unsafe { sys::OH_DLP_GetDlpPermissionInfo(&mut access, &mut flags) })?;
    let access = FileAccess::from_raw(access).ok_or(DlpError::UnknownFileAccess(access))?;
    Ok(PermissionInfo {
        access,
        actions: Actions::from_bits_retain(flags),
    })
}

/// Derive the original file name from a DLP file name by stripping the DLP
/// extension.
pub fn original_file_name(file_name: impl AsRef<str>) -> Result<String> {
    let file_name = to_c_string(file_name.as_ref())?;
    let mut out: *mut c_char = std::ptr::null_mut();
    check(unsafe { sys::OH_DLP_GetOriginalFileName(file_name.as_ptr(), &mut out) })?;
    // SAFETY: on success the native call stored a freshly allocated,
    // NUL-terminated buffer in `out` and transferred its ownership.
    let owned = unsafe { OwnedNativeString::from_raw(out) }.ok_or(DlpError::MissingOutput)?;
    owned.to_utf8()
}

/// Check whether the current application runs inside a DLP sandbox.
pub fn is_in_sandbox() -> Result<bool> {
    let mut in_sandbox = false;
    check(unsafe { sys::OH_DLP_IsInSandbox(&mut in_sandbox) })?;
    Ok(in_sandbox)
}

/// Set the sandbox application configuration.
///
/// Only callable from a non-sandbox application; otherwise the call fails with
/// `DLP_ErrCode` 19100007.
pub fn set_sandbox_app_config(config: impl AsRef<str>) -> Result<()> {
    let config = to_c_string(config.as_ref())?;
    check(unsafe { sys::OH_DLP_SetSandboxAppConfig(config.as_ptr()) })
}

/// Obtain the sandbox application configuration.
pub fn sandbox_app_config() -> Result<String> {
    let mut out: *mut c_char = std::ptr::null_mut();
    check(unsafe { sys::OH_DLP_GetSandboxAppConfig(&mut out) })?;
    // SAFETY: on success the native call stored a freshly allocated,
    // NUL-terminated buffer in `out` and transferred its ownership.
    let owned = unsafe { OwnedNativeString::from_raw(out) }.ok_or(DlpError::MissingOutput)?;
    owned.to_utf8()
}

/// Clear the sandbox application configuration.
///
/// Only callable from a non-sandbox application; otherwise the call fails with
/// `DLP_ErrCode` 19100007.
pub fn clean_sandbox_app_config() -> Result<()> {
    check(unsafe { sys::OH_DLP_CleanSandboxAppConfig() })
}
