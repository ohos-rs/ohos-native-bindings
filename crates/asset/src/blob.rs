pub type AssetBlob = Vec<u8>;

pub enum AssetValue {
    Boolean(bool),
    U32IntT(u32),
    Blob(Vec<u8>),
}

/// Copy native asset bytes into an owned `Vec<u8>`.
///
/// Query results are released by `OH_Asset_FreeResultSet`, so the binding
/// must own the copy. Host-testable: no Harmony FFI.
pub(crate) fn copy_asset_blob(data: *const u8, size: u32) -> Vec<u8> {
    unsafe { std::slice::from_raw_parts(data, size as usize).to_vec() }
}

#[cfg(test)]
mod tests {
    use super::{copy_asset_blob, AssetValue};

    #[test]
    fn copy_asset_blob_copies_dummy_bytes() {
        let dummy = [1u8, 2, 3];
        let copied = copy_asset_blob(dummy.as_ptr(), dummy.len() as u32);
        assert_eq!(copied.len(), 3);
        assert_eq!(copied, dummy);
    }

    #[test]
    fn dropping_blob_frees_owned_vec() {
        let value = AssetValue::Blob(vec![1, 2, 3]);
        let AssetValue::Blob(bytes) = value else {
            panic!("expected blob");
        };
        // Blob stores Vec<u8>, not ManuallyDrop, so Drop frees the copy.
        let _: Vec<u8> = bytes;
    }
}
