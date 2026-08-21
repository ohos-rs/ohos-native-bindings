/// Copy an NDK media out-buffer into a Rust `Vec`.
///
/// `OH_ResourceManager_GetMedia*` writes `resultValue: *mut *mut u8`
/// (or `*mut *mut c_char` for base64) and allocates with malloc. This
/// helper only copies bytes and does not call NDK APIs, so it can be
/// unit-tested on the host with dummy pointers. The caller must free the
/// NDK allocation with `libc::free` after a successful non-empty adopt.
/// Do not use `Vec::from_raw_parts` on the NDK pointer (allocator mismatch).
///
/// # Safety
///
/// `ptr` must be null or point to at least `len` readable bytes.
pub(crate) unsafe fn adopt_ndk_media_buffer(ptr: *const u8, len: u64) -> Vec<u8> {
    if ptr.is_null() || len == 0 {
        Vec::new()
    } else {
        std::slice::from_raw_parts(ptr, len as usize).to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adopt_leftover_empty_null_len_zero_is_empty() {
        let ret = unsafe { adopt_ndk_media_buffer(std::ptr::null(), 0) };
        assert!(ret.is_empty());
    }

    #[test]
    fn adopt_leftover_dummy_four_byte_buffer() {
        let buf = [0xDEu8, 0xAD, 0xBE, 0xEF];
        let ret = unsafe { adopt_ndk_media_buffer(buf.as_ptr(), 4) };
        assert_eq!(ret.len(), 4);
        assert_eq!(ret, [0xDE, 0xAD, 0xBE, 0xEF]);
    }
}
