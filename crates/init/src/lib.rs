use std::ffi::CString;

mod sys;

/// check cap can be used, implement with canIUse
#[allow(non_snake_case)]
pub fn canIUse(cap: &str) -> bool {
    let c_cap = CString::new(cap).expect("Create c_string failed");
    unsafe { sys::canIUse(c_cap.as_ptr().cast()) }
}
