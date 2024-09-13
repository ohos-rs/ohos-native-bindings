use std::ffi::CString;

use ohos_ability_access_control_sys::OH_AT_CheckSelfPermission;

/// Check if the application has been granted the specified permissions.
pub fn check_self_permission<T: AsRef<str>>(permission: T) -> bool {
    let p = CString::new(permission.as_ref()).expect("Create CString failed");
    unsafe { OH_AT_CheckSelfPermission(p.as_ptr().cast()) }
}
