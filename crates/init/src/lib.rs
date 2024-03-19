use std::ffi::CString;

mod sys;

/// check cap can be used, implement with canIUse
pub fn can_i_use(cap: &str) -> bool {
    let c_cap = CString::new(cap).expect("Create c_string failed");
    unsafe {
        sys::canIUse(c_cap.as_ptr().cast())
    }
}