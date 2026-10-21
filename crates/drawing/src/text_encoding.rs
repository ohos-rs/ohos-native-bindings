use std::ffi::CString;

pub(super) fn nul_terminated(text: &str) -> CString {
    CString::new(text).unwrap_or_else(|error| {
        // Input was UTF-8; replacing only NUL preserves every other character.
        let text = String::from_utf8(error.into_vec()).expect("valid UTF-8 input");
        CString::new(text.replace('\0', "\u{fffd}")).expect("NULs replaced")
    })
}
