//! Caller-owned indexed storage for ArkUI mixed-event strings.

use std::ffi::c_char;

/// Failure when copying a callback-scoped mixed-event string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum StringValueError {
    /// The mixed-event index is outside its twelve-element union.
    InvalidIndex,
    /// The native getter returned an error.
    Native(i32),
    /// The native output did not contain a terminating NUL.
    Unterminated,
    /// The native output is not UTF-8.
    InvalidUtf8,
    /// A buffer size cannot be represented by the native i32 interface.
    Capacity,
    /// Allocation of the requested output buffer failed.
    Allocation,
}

impl std::fmt::Display for StringValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArkUI mixed-event string: {self:?}")
    }
}

impl std::error::Error for StringValueError {}

pub(super) fn copy(
    index: i32,
    mut read: impl FnMut(i32, *mut *mut c_char, *mut i32) -> i32,
) -> Result<String, StringValueError> {
    let index = usize::try_from(index)
        .ok()
        .filter(|i| *i < 12)
        .ok_or(StringValueError::InvalidIndex)?;
    let mut capacity = 64usize;
    loop {
        let native_capacity = i32::try_from(capacity).map_err(|_| StringValueError::Capacity)?;
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(capacity)
            .map_err(|_| StringValueError::Allocation)?;
        bytes.resize(capacity, 0xff);
        let mut pointers = [std::ptr::null_mut(); 12];
        let mut sizes = [0i32; 12];
        pointers[index] = bytes.as_mut_ptr().cast();
        sizes[index] = native_capacity;
        let status = read(index as i32, pointers.as_mut_ptr(), sizes.as_mut_ptr());
        // Legacy API-12 strcpy_s inversion is confined to API-12 builds. A
        // written buffer is required; untouched error output is never decoded.
        let legacy_copied = cfg!(not(feature = "api-13")) && status == 401 && bytes[0] != 0xff;
        if status != 0 && !legacy_copied {
            return Err(StringValueError::Native(status));
        }
        let length = bytes
            .iter()
            .position(|b| *b == 0)
            .ok_or(StringValueError::Unterminated)?;
        if length < capacity - 1 {
            bytes.truncate(length);
            return String::from_utf8(bytes).map_err(|_| StringValueError::InvalidUtf8);
        }
        capacity = capacity.checked_mul(2).ok_or(StringValueError::Capacity)?;
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
        0
    }

    #[test]
    fn indexed_copy_retries_truncation_and_owns_its_utf8() {
        let text = "hello \u{1f34a}".repeat(100);
        let mut calls = 0;
        let output = copy(7, |index, pointers, sizes| {
            calls += 1;
            native_copy(text.as_bytes(), index, pointers, sizes)
        });
        assert_eq!(output.as_deref(), Ok(text.as_str()));
        assert!(calls > 1);
    }

    #[test]
    fn invalid_indices_and_native_errors_are_distinct() {
        for index in [-1, 12, i32::MAX] {
            assert_eq!(
                copy(index, |_, _, _| panic!("invalid index")),
                Err(StringValueError::InvalidIndex)
            );
        }
        assert_eq!(copy(0, |_, _, _| 401), Err(StringValueError::Native(401)));
        assert_eq!(copy(0, |_, _, _| 0), Err(StringValueError::Unterminated));
        assert_eq!(
            copy(0, |i, p, s| native_copy(&[0xfe], i, p, s)),
            Err(StringValueError::InvalidUtf8)
        );
    }

    #[test]
    fn large_strings_are_not_silently_capped() {
        assert_eq!(
            copy(0, |i, p, s| native_copy(b"", i, p, s)).as_deref(),
            Ok("")
        );
        let text = vec![b'a'; 1024 * 1024 + 7];
        assert_eq!(
            copy(0, |i, p, s| native_copy(&text, i, p, s))
                .unwrap()
                .len(),
            text.len()
        );
    }

    #[test]
    fn strcpy_inversion_is_only_accepted_on_api_12() {
        let output = copy(0, |i, p, s| {
            native_copy(b"legacy", i, p, s);
            401
        });
        if cfg!(feature = "api-13") {
            assert_eq!(output, Err(StringValueError::Native(401)));
        } else {
            assert_eq!(output.as_deref(), Ok("legacy"));
        }
    }
}
