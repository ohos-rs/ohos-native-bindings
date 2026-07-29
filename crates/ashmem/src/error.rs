use std::{fmt, io};

use crate::Protection;

/// Errors returned by Ashmem operations.
#[derive(Debug)]
#[non_exhaustive]
pub enum AshmemError {
    /// The region name contains an interior NUL byte.
    NameContainsNul,
    /// The region name is too long for the kernel Ashmem ABI.
    NameTooLong {
        /// Name length in bytes, excluding the trailing NUL byte.
        length: usize,
        /// Maximum accepted name length in bytes.
        maximum: usize,
    },
    /// The requested region size is zero or exceeds the supported range.
    InvalidSize {
        /// Requested size in bytes.
        size: usize,
        /// Maximum accepted size in bytes.
        maximum: usize,
    },
    /// The supplied protection value contains unsupported bits.
    InvalidProtection {
        /// Raw protection bits.
        bits: i32,
    },
    /// The file descriptor does not refer to an Ashmem character device.
    NotAshmemDevice,
    /// The kernel returned an invalid region size.
    InvalidKernelSize {
        /// Raw value returned by the kernel.
        size: i32,
    },
    /// The kernel returned unknown protection bits.
    InvalidKernelProtection {
        /// Raw value returned by the kernel.
        bits: i32,
    },
    /// `mmap` unexpectedly returned a null address.
    NullMapping,
    /// The region is already mapped.
    AlreadyMapped,
    /// The operation requires a mapped region.
    NotMapped,
    /// The requested range is outside the region.
    OutOfBounds {
        /// Byte offset from the start of the region.
        offset: usize,
        /// Requested byte length.
        length: usize,
        /// Region size in bytes.
        region_size: usize,
    },
    /// The mapped or kernel-level protection does not permit the operation.
    ProtectionDenied {
        /// Protection required by the operation.
        required: Protection,
        /// Protection used for the current mapping.
        mapping: Protection,
        /// Current kernel-level protection.
        region: Protection,
    },
    /// An operating-system call failed.
    Io(io::Error),
}

impl fmt::Display for AshmemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NameContainsNul => write!(f, "Ashmem name contains an interior NUL byte"),
            Self::NameTooLong { length, maximum } => write!(
                f,
                "Ashmem name is {length} bytes, but the maximum is {maximum}"
            ),
            Self::InvalidSize { size, maximum } => write!(
                f,
                "Ashmem size is {size} bytes, but it must be between 1 and {maximum}"
            ),
            Self::InvalidProtection { bits } => {
                write!(f, "unsupported Ashmem protection bits: {bits:#x}")
            }
            Self::NotAshmemDevice => {
                write!(f, "file descriptor does not refer to an Ashmem device")
            }
            Self::InvalidKernelSize { size } => {
                write!(f, "kernel returned invalid Ashmem size {size}")
            }
            Self::InvalidKernelProtection { bits } => {
                write!(f, "kernel returned unknown Ashmem protection bits {bits:#x}")
            }
            Self::NullMapping => write!(f, "mmap returned a null address"),
            Self::AlreadyMapped => write!(f, "Ashmem region is already mapped"),
            Self::NotMapped => write!(f, "Ashmem region is not mapped"),
            Self::OutOfBounds {
                offset,
                length,
                region_size,
            } => write!(
                f,
                "Ashmem range {offset}..{} exceeds region size {region_size}",
                offset.saturating_add(*length)
            ),
            Self::ProtectionDenied {
                required,
                mapping,
                region,
            } => write!(
                f,
                "Ashmem access requires {required:?}, mapping has {mapping:?}, region has {region:?}"
            ),
            Self::Io(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for AshmemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for AshmemError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
