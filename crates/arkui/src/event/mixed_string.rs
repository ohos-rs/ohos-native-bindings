//! Caller-owned indexed storage for ArkUI mixed-event strings.

use std::ffi::c_char;

pub(super) fn copy(
    index: i32,
    mut read: impl FnMut(i32, *mut *mut c_char, *mut i32) -> i32,
) -> Option<String> {
    let index = usize::try_from(index).ok().filter(|i| *i < 12)?;
    let mut capacity = 64;
    loop {
        let mut bytes = vec![0xffu8; capacity];
        let mut pointers = [std::ptr::null_mut(); 12];
        let mut sizes = [0i32; 12];
        pointers[index] = bytes.as_mut_ptr().cast();
        sizes[index] = capacity as i32;
        let status = read(index as i32, pointers.as_mut_ptr(), sizes.as_mut_ptr());
        // API 12 inverts strcpy_s's successful return code. Accept its copied
        // output, but never reinterpret untouched storage as a native string.
        if status != 0 && bytes[0] == 0xff {
            return None;
        }
        let length = bytes.iter().position(|b| *b == 0)?;
        if length < capacity - 1 {
            return Some(String::from_utf8_lossy(&bytes[..length]).into_owned());
        }
        if capacity >= 1024 * 1024 {
            return None;
        }
        capacity *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn native_copy(text: &[u8], index: i32, pointers: *mut *mut c_char, sizes: *mut i32) -> i32 {
        // SAFETY: copy supplies twelve entries and the advertised output buffer.
        unsafe {
            let capacity = *sizes.add(index as usize) as usize;
            let output = *pointers.add(index as usize);
            let count = text.len().min(capacity - 1);
            std::ptr::copy_nonoverlapping(text.as_ptr(), output.cast(), count);
            *output.add(count) = 0;
        }
        // Reproduce the API-12 implementation's successful-copy error status.
        401
    }

    #[test]
    fn indexed_copy_retries_truncation_and_owns_its_utf8() {
        let text = "hello \u{1f34a}".repeat(100);
        let mut calls = 0;
        let output = copy(7, |index, pointers, sizes| {
            calls += 1;
            native_copy(text.as_bytes(), index, pointers, sizes)
        });
        assert_eq!(output.as_deref(), Some(text.as_str()));
        assert!(calls > 1);
    }

    #[test]
    fn invalid_indices_fail_without_calling_native() {
        for index in [-1, 12, i32::MAX] {
            assert_eq!(
                copy(index, |_, _, _| panic!("invalid index reached native")),
                None
            );
        }
        assert_eq!(copy(0, |_, _, _| 401), None);
    }

    #[test]
    fn empty_values_and_unbounded_values_are_distinct() {
        assert_eq!(
            copy(0, |i, p, s| native_copy(b"", i, p, s)).as_deref(),
            Some("")
        );
        let text = vec![b'a'; 1024 * 1024];
        assert_eq!(copy(0, |i, p, s| native_copy(&text, i, p, s)), None);
    }
}
