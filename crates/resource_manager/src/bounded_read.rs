//! Checked allocation and partial-read progress for an owned raw file.

use crate::RawFileError;

pub(crate) struct BoundedRead {
    remaining: usize,
}
impl BoundedRead {
    pub(crate) fn new(remaining: impl TryInto<usize>, limit: usize) -> Result<Self, RawFileError> {
        let remaining = remaining
            .try_into()
            .map_err(|_| RawFileError::FfiInnerError("invalid raw resource size".into()))?;
        if remaining > limit {
            return Err(RawFileError::FfiInnerError(
                "raw resource exceeds its byte limit".into(),
            ));
        }
        Ok(Self { remaining })
    }

    pub(crate) fn read(
        self,
        mut read: impl FnMut(&mut [u8]) -> i32,
    ) -> Result<Vec<u8>, RawFileError> {
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(self.remaining)
            .map_err(|error| RawFileError::FfiInnerError(error.to_string()))?;
        bytes.resize(self.remaining, 0);
        let mut offset = 0;
        while offset < bytes.len() {
            let length = (bytes.len() - offset).min(i32::MAX as usize);
            let count = usize::try_from(read(&mut bytes[offset..offset + length]))
                .ok()
                .filter(|count| *count > 0 && *count <= length)
                .ok_or_else(|| RawFileError::FfiInnerError("raw resource read failed".into()))?;
            offset += count;
        }
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_and_over_budget_sizes_before_reading() {
        assert!(BoundedRead::new(-1, 100).is_err());
        assert!(BoundedRead::new(101, 100).is_err());
        assert!(BoundedRead::new(i64::MAX, 100).is_err());
        assert!(BoundedRead::new(0, 0)
            .unwrap()
            .read(|_| panic!("empty resources need no native read"))
            .unwrap()
            .is_empty());
    }

    #[test]
    fn partial_native_reads_fill_the_remaining_resource() {
        let mut next = 0u8;
        let bytes = BoundedRead::new(7, 7)
            .unwrap()
            .read(|output| {
                let count = output.len().min(2);
                for byte in &mut output[..count] {
                    *byte = next;
                    next += 1;
                }
                count as i32
            })
            .unwrap();
        assert_eq!(bytes, [0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn rejects_eof_native_errors_and_impossible_progress() {
        for count in [-1, 0, 4] {
            assert!(BoundedRead::new(3, 3).unwrap().read(|_| count).is_err());
        }
    }
}
