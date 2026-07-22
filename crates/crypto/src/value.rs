//! Typed values for the otherwise untyped `SetParam` byte blobs.

/// A value that can be stored into a crypto parameter set.
///
/// The native `SetParam` calls take an opaque byte blob whose interpretation is
/// fixed by the parameter type — see the `_DATABLOB` / `_INT` / `_UINT64` /
/// `_STR` suffixes on the parameter enums. This trait encodes each Rust type
/// into that blob, so callers pass `10_000u32` instead of hand-rolling the
/// little-endian bytes.
///
/// Integers are encoded in native byte order, matching how the framework reads
/// them back as C integers on the little-endian OpenHarmony targets.
pub trait IntoCryptoValue {
    fn into_crypto_value(self) -> Vec<u8>;
}

impl IntoCryptoValue for &[u8] {
    fn into_crypto_value(self) -> Vec<u8> {
        self.to_vec()
    }
}

impl IntoCryptoValue for &str {
    fn into_crypto_value(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

macro_rules! int_crypto_value {
    ($($ty:ty),* $(,)?) => {
        $(impl IntoCryptoValue for $ty {
            fn into_crypto_value(self) -> Vec<u8> {
                self.to_ne_bytes().to_vec()
            }
        })*
    };
}

int_crypto_value!(i32, u32, u64);
