use std::ffi::{CString, NulError};

pub(super) fn nul_terminated(text: &str) -> Result<CString, NulError> {
    CString::new(text)
}
