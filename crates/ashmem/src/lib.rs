//! Safe Rust access to OpenHarmony anonymous shared memory.
//!
//! This crate is a direct Rust implementation of the operations exposed by
//! OpenHarmony's
//! [`ashmem.cpp`](https://github.com/openharmony/commonlibrary_c_utils/blob/master/base/src/ashmem.cpp).
//! It owns the file descriptor, automatically unmaps memory on drop, and uses
//! copied byte buffers instead of exposing pointers into shared memory.
//!
//! Access shared with another process must be externally synchronized.

use std::{
    ffi::{c_void, CString},
    io,
    os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd, RawFd},
    ptr::NonNull,
};

use bitflags::bitflags;

mod error;
mod raw;

pub use error::AshmemError;

/// Result type used by this crate.
pub type Result<T> = std::result::Result<T, AshmemError>;

bitflags! {
    /// Protection flags for an Ashmem region or mapping.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Protection: i32 {
        /// No access.
        const NONE = libc::PROT_NONE;
        /// Memory may be read.
        const READ = libc::PROT_READ;
        /// Memory may be written.
        const WRITE = libc::PROT_WRITE;
        /// Memory may be executed.
        const EXECUTE = libc::PROT_EXEC;
    }
}

/// An owned OpenHarmony anonymous shared-memory region.
///
/// The region is automatically unmapped and its file descriptor is closed
/// when this value is dropped.
#[derive(Debug)]
pub struct Ashmem {
    // Keep the mapping before the descriptor so it is dropped first.
    mapping: Option<Mapping>,
    fd: OwnedFd,
    size: usize,
}

impl Ashmem {
    /// Creates a named Ashmem region.
    ///
    /// The size must be between 1 and [`i32::MAX`] bytes. The name may contain
    /// at most 255 bytes and may not contain an interior NUL byte.
    pub fn create(name: &str, size: usize) -> Result<Self> {
        validate_size(size)?;
        let name = validate_name(name)?;
        let fd = raw::create(name.as_c_str(), size)?;

        Ok(Self {
            mapping: None,
            fd,
            size,
        })
    }

    /// Adopts an owned descriptor received from another process.
    ///
    /// The descriptor is validated as an Ashmem character device and the
    /// region size is queried from the kernel.
    pub fn from_owned_fd(fd: OwnedFd) -> Result<Self> {
        raw::validate(fd.as_fd())?;
        let size = raw::get_size(fd.as_fd())?;

        Ok(Self {
            mapping: None,
            fd,
            size,
        })
    }

    /// Creates an independently owned descriptor for the same region.
    ///
    /// The returned value starts out unmapped.
    pub fn try_clone(&self) -> Result<Self> {
        Self::from_owned_fd(self.fd.try_clone()?)
    }

    /// Returns the region size in bytes.
    #[must_use]
    pub const fn size(&self) -> usize {
        self.size
    }

    /// Returns the current kernel-level protection.
    pub fn protection(&self) -> Result<Protection> {
        raw::get_protection(self.fd.as_fd())
    }

    /// Restricts the kernel-level protection.
    ///
    /// The Ashmem driver permits protection to be reduced, but not expanded.
    pub fn set_protection(&self, protection: Protection) -> Result<()> {
        validate_protection(protection)?;
        raw::set_protection(self.fd.as_fd(), protection)
    }

    /// Maps the region into this process with the requested protection.
    pub fn map(&mut self, protection: Protection) -> Result<()> {
        if self.mapping.is_some() {
            return Err(AshmemError::AlreadyMapped);
        }
        validate_protection(protection)?;

        let address = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                self.size,
                protection.bits(),
                libc::MAP_SHARED,
                self.fd.as_raw_fd(),
                0,
            )
        };
        if address == libc::MAP_FAILED {
            return Err(io::Error::last_os_error().into());
        }

        let Some(address) = NonNull::new(address) else {
            unsafe {
                libc::munmap(address, self.size);
            }
            return Err(AshmemError::NullMapping);
        };

        self.mapping = Some(Mapping {
            address: Some(address),
            length: self.size,
            protection,
        });
        Ok(())
    }

    /// Maps the region for reading and writing.
    pub fn map_read_write(&mut self) -> Result<()> {
        self.map(Protection::READ | Protection::WRITE)
    }

    /// Maps the region for reading only.
    pub fn map_read_only(&mut self) -> Result<()> {
        self.map(Protection::READ)
    }

    /// Returns whether the region is currently mapped.
    #[must_use]
    pub fn is_mapped(&self) -> bool {
        self.mapping.is_some()
    }

    /// Returns the current mapping protection, or `None` when unmapped.
    #[must_use]
    pub fn mapped_protection(&self) -> Option<Protection> {
        self.mapping.as_ref().map(|mapping| mapping.protection)
    }

    /// Unmaps the region.
    ///
    /// Calling this method when the region is already unmapped succeeds.
    pub fn unmap(&mut self) -> Result<()> {
        let Some(mut mapping) = self.mapping.take() else {
            return Ok(());
        };

        if let Err(error) = mapping.unmap() {
            self.mapping = Some(mapping);
            return Err(error.into());
        }

        Ok(())
    }

    /// Copies bytes into the mapped region at `offset`.
    pub fn write(&mut self, offset: usize, data: &[u8]) -> Result<()> {
        let address = self.checked_address(offset, data.len(), Protection::WRITE)?;
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), address, data.len());
        }
        Ok(())
    }

    /// Copies `length` bytes from the mapped region at `offset`.
    pub fn read(&mut self, offset: usize, length: usize) -> Result<Vec<u8>> {
        let address = self.checked_address(offset, length, Protection::READ)?;
        let mut output = vec![0; length];
        unsafe {
            std::ptr::copy_nonoverlapping(address.cast_const(), output.as_mut_ptr(), length);
        }
        Ok(output)
    }

    /// Copies bytes from the mapped region at `offset` into `output`.
    pub fn read_into(&mut self, offset: usize, output: &mut [u8]) -> Result<()> {
        let address = self.checked_address(offset, output.len(), Protection::READ)?;
        unsafe {
            std::ptr::copy_nonoverlapping(address.cast_const(), output.as_mut_ptr(), output.len());
        }
        Ok(())
    }

    /// Unmaps the region and returns its owned descriptor.
    ///
    /// This is useful when transferring ownership to another IPC abstraction.
    pub fn into_owned_fd(mut self) -> Result<OwnedFd> {
        self.unmap()?;
        Ok(self.fd)
    }

    /// Explicitly unmaps and closes the region.
    ///
    /// Dropping the value performs the same cleanup.
    pub fn close(mut self) -> Result<()> {
        self.unmap()
    }

    fn checked_address(
        &self,
        offset: usize,
        length: usize,
        required: Protection,
    ) -> Result<*mut u8> {
        let mapping = self.mapping.as_ref().ok_or(AshmemError::NotMapped)?;
        validate_range(offset, length, self.size)?;

        let region = self.protection()?;
        if !mapping.protection.contains(required) || !region.contains(required) {
            return Err(AshmemError::ProtectionDenied {
                required,
                mapping: mapping.protection,
                region,
            });
        }

        let address = mapping
            .address
            .expect("a live Mapping always has an address")
            .as_ptr()
            .cast::<u8>();
        Ok(unsafe { address.add(offset) })
    }
}

impl AsFd for Ashmem {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}

impl AsRawFd for Ashmem {
    fn as_raw_fd(&self) -> RawFd {
        self.fd.as_raw_fd()
    }
}

#[derive(Debug)]
struct Mapping {
    address: Option<NonNull<c_void>>,
    length: usize,
    protection: Protection,
}

// Moving ownership of an mmap to another thread is valid. Mapping deliberately
// remains !Sync, and the public read/write operations require exclusive access.
unsafe impl Send for Mapping {}

impl Mapping {
    fn unmap(&mut self) -> io::Result<()> {
        let Some(address) = self.address else {
            return Ok(());
        };

        let result = unsafe { libc::munmap(address.as_ptr(), self.length) };
        if result < 0 {
            return Err(io::Error::last_os_error());
        }

        self.address = None;
        Ok(())
    }
}

impl Drop for Mapping {
    fn drop(&mut self) {
        let _ = self.unmap();
    }
}

fn validate_name(name: &str) -> Result<CString> {
    let name = CString::new(name).map_err(|_| AshmemError::NameContainsNul)?;
    let length = name.as_bytes().len();
    if length >= raw::NAME_LENGTH {
        return Err(AshmemError::NameTooLong {
            length,
            maximum: raw::NAME_LENGTH - 1,
        });
    }
    Ok(name)
}

fn validate_size(size: usize) -> Result<()> {
    const MAXIMUM: usize = i32::MAX as usize;
    if size == 0 || size > MAXIMUM {
        return Err(AshmemError::InvalidSize {
            size,
            maximum: MAXIMUM,
        });
    }
    Ok(())
}

fn validate_protection(protection: Protection) -> Result<()> {
    if Protection::from_bits(protection.bits()).is_none() {
        return Err(AshmemError::InvalidProtection {
            bits: protection.bits(),
        });
    }
    Ok(())
}

fn validate_range(offset: usize, length: usize, region_size: usize) -> Result<()> {
    let Some(end) = offset.checked_add(length) else {
        return Err(AshmemError::OutOfBounds {
            offset,
            length,
            region_size,
        });
    };
    if offset > region_size || end > region_size {
        return Err(AshmemError::OutOfBounds {
            offset,
            length,
            region_size,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_name_before_opening_device() {
        let error = Ashmem::create("contains\0nul", 1).unwrap_err();
        assert!(matches!(error, AshmemError::NameContainsNul));

        let error = Ashmem::create(&"a".repeat(raw::NAME_LENGTH), 1).unwrap_err();
        assert!(matches!(
            error,
            AshmemError::NameTooLong {
                length,
                maximum,
            } if length == raw::NAME_LENGTH && maximum == raw::NAME_LENGTH - 1
        ));
    }

    #[test]
    fn validates_size_before_opening_device() {
        let error = Ashmem::create("test", 0).unwrap_err();
        assert!(matches!(error, AshmemError::InvalidSize { size: 0, .. }));
    }

    #[test]
    fn validates_ranges_without_overflow() {
        assert!(validate_range(0, 8, 8).is_ok());
        assert!(validate_range(8, 0, 8).is_ok());
        assert!(matches!(
            validate_range(8, 1, 8),
            Err(AshmemError::OutOfBounds { .. })
        ));
        assert!(matches!(
            validate_range(usize::MAX, 2, usize::MAX),
            Err(AshmemError::OutOfBounds { .. })
        ));
    }

    #[test]
    fn rejects_unknown_protection_bits() {
        let protection = Protection::from_bits_retain(1 << 30);
        assert!(matches!(
            validate_protection(protection),
            Err(AshmemError::InvalidProtection { bits }) if bits == 1 << 30
        ));
    }
}

#[cfg(all(test, target_env = "ohos"))]
mod device_tests {
    use super::*;

    #[test]
    fn creates_maps_writes_and_restricts_region() -> Result<()> {
        let mut ashmem = Ashmem::create("ohos-ashmem-binding-test", 1024)?;
        assert_eq!(ashmem.size(), 1024);

        ashmem.map_read_write()?;
        ashmem.write(0, b"hello")?;
        assert_eq!(ashmem.read(0, 5)?, b"hello");
        ashmem.unmap()?;

        ashmem.set_protection(Protection::READ)?;
        assert!(ashmem.map_read_write().is_err());
        ashmem.map_read_only()?;
        assert_eq!(ashmem.read(0, 5)?, b"hello");
        assert!(matches!(
            ashmem.write(5, b"!"),
            Err(AshmemError::ProtectionDenied { .. })
        ));

        Ok(())
    }
}
