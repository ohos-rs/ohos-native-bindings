use std::os::raw::c_char;

/// Decode a fixed-size, NUL-terminated C character field into an owned
/// `String`.
///
/// The field is read up to the first NUL byte, or to its end if the native
/// layer filled it completely without terminating it. Bytes that are not valid
/// UTF-8 are replaced, so an SSID carrying arbitrary octets never fails to
/// decode.
pub(crate) fn c_field_to_string(field: &[c_char]) -> String {
    // `c_char` is `i8` or `u8` depending on the target; both have the size and
    // alignment of `u8`, so the field can be read as plain bytes.
    let bytes = unsafe { std::slice::from_raw_parts(field.as_ptr().cast::<u8>(), field.len()) };
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}
