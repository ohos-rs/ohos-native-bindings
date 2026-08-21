/// Decode an IME `char16_t` buffer to `String`.
///
/// The whole slice is decoded so UTF-16 surrogate pairs (emoji / non-BMP)
/// stay intact. Host-testable: no Harmony FFI.
pub(crate) fn char16_ptr_to_string(ptr: *const u16, length: usize) -> String {
    let slice = unsafe { std::slice::from_raw_parts(ptr, length) };
    String::from_utf16_lossy(slice)
}

#[cfg(test)]
mod tests {
    use super::char16_ptr_to_string;

    #[test]
    fn decodes_bmp_units() {
        let units = [b'h' as u16, b'i' as u16];
        assert_eq!(char16_ptr_to_string(units.as_ptr(), units.len()), "hi");
    }

    #[test]
    fn decodes_surrogate_pair() {
        let units = [0xD83D, 0xDE00];
        assert_eq!(char16_ptr_to_string(units.as_ptr(), units.len()), "😀");
    }
}
