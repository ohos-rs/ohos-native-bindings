use std::ffi::c_char;

extern "C" {
    pub fn canIUse(cap: *const c_char) -> bool;
}