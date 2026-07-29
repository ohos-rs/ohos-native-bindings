use std::{
    ffi::{c_void, CStr},
    io,
    mem::{self, MaybeUninit},
    os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd},
};

use crate::{AshmemError, Protection, Result};

pub(crate) const NAME_LENGTH: usize = 256;

const DEVICE_PATH: &[u8] = b"/dev/ashmem\0";
const ASHMEM_IOC: u32 = 0x77;
const IOC_NONE: u32 = 0;
const IOC_WRITE: u32 = 1;
const IOC_DIRECTION_SHIFT: u32 = 30;
const IOC_SIZE_SHIFT: u32 = 16;
const IOC_TYPE_SHIFT: u32 = 8;

const fn ioctl_request(direction: u32, number: u32, size: u32) -> libc::c_ulong {
    ((direction << IOC_DIRECTION_SHIFT)
        | (size << IOC_SIZE_SHIFT)
        | (ASHMEM_IOC << IOC_TYPE_SHIFT)
        | number) as libc::c_ulong
}

const ASHMEM_SET_NAME: libc::c_ulong = ioctl_request(IOC_WRITE, 1, NAME_LENGTH as u32);
const ASHMEM_SET_SIZE: libc::c_ulong = ioctl_request(IOC_WRITE, 3, mem::size_of::<usize>() as u32);
const ASHMEM_GET_SIZE: libc::c_ulong = ioctl_request(IOC_NONE, 4, 0);
const ASHMEM_SET_PROT_MASK: libc::c_ulong =
    ioctl_request(IOC_WRITE, 5, mem::size_of::<libc::c_ulong>() as u32);
const ASHMEM_GET_PROT_MASK: libc::c_ulong = ioctl_request(IOC_NONE, 6, 0);

pub(crate) fn create(name: &CStr, size: usize) -> Result<OwnedFd> {
    let fd = open_device()?;
    validate(fd.as_fd())?;

    let name_bytes = name.to_bytes_with_nul();
    debug_assert!(name_bytes.len() <= NAME_LENGTH);

    let mut name_buffer = [0 as libc::c_char; NAME_LENGTH];
    for (destination, source) in name_buffer.iter_mut().zip(name_bytes) {
        *destination = *source as libc::c_char;
    }

    ioctl_pointer(
        fd.as_fd(),
        ASHMEM_SET_NAME,
        name_buffer.as_ptr().cast::<c_void>(),
    )?;
    ioctl_value(fd.as_fd(), ASHMEM_SET_SIZE, size as libc::c_ulong)?;

    Ok(fd)
}

pub(crate) fn validate(fd: BorrowedFd<'_>) -> Result<()> {
    let mut status = MaybeUninit::<libc::stat>::uninit();
    retry_syscall(|| unsafe { libc::fstat(fd.as_raw_fd(), status.as_mut_ptr()) })?;
    let status = unsafe { status.assume_init() };

    if (status.st_mode & libc::S_IFMT) != libc::S_IFCHR || status.st_rdev == 0 {
        return Err(AshmemError::NotAshmemDevice);
    }

    Ok(())
}

pub(crate) fn get_size(fd: BorrowedFd<'_>) -> Result<usize> {
    let size = ioctl_no_argument(fd, ASHMEM_GET_SIZE)?;
    if size <= 0 {
        return Err(AshmemError::InvalidKernelSize { size });
    }
    Ok(size as usize)
}

pub(crate) fn set_protection(fd: BorrowedFd<'_>, protection: Protection) -> Result<()> {
    ioctl_value(fd, ASHMEM_SET_PROT_MASK, protection.bits() as libc::c_ulong)?;
    Ok(())
}

pub(crate) fn get_protection(fd: BorrowedFd<'_>) -> Result<Protection> {
    let bits = ioctl_no_argument(fd, ASHMEM_GET_PROT_MASK)?;
    Protection::from_bits(bits).ok_or(AshmemError::InvalidKernelProtection { bits })
}

fn open_device() -> io::Result<OwnedFd> {
    let raw_fd = retry_syscall(|| unsafe {
        libc::open(DEVICE_PATH.as_ptr().cast(), libc::O_RDWR | libc::O_CLOEXEC)
    })?;
    Ok(unsafe { OwnedFd::from_raw_fd(raw_fd) })
}

fn ioctl_no_argument(fd: BorrowedFd<'_>, request: libc::c_ulong) -> io::Result<libc::c_int> {
    retry_syscall(|| unsafe { libc::ioctl(fd.as_raw_fd(), request as _, 0 as libc::c_ulong) })
}

fn ioctl_value(
    fd: BorrowedFd<'_>,
    request: libc::c_ulong,
    value: libc::c_ulong,
) -> io::Result<libc::c_int> {
    retry_syscall(|| unsafe { libc::ioctl(fd.as_raw_fd(), request as _, value) })
}

fn ioctl_pointer(
    fd: BorrowedFd<'_>,
    request: libc::c_ulong,
    value: *const c_void,
) -> io::Result<libc::c_int> {
    retry_syscall(|| unsafe { libc::ioctl(fd.as_raw_fd(), request as _, value) })
}

fn retry_syscall(mut call: impl FnMut() -> libc::c_int) -> io::Result<libc::c_int> {
    loop {
        let result = call();
        if result >= 0 {
            return Ok(result);
        }

        let error = io::Error::last_os_error();
        if error.kind() != io::ErrorKind::Interrupted {
            return Err(error);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ioctl_numbers_match_linux_ashmem_abi() {
        assert_eq!(ASHMEM_SET_NAME, 0x4100_7701);
        assert_eq!(ASHMEM_GET_SIZE, 0x0000_7704);
        assert_eq!(ASHMEM_GET_PROT_MASK, 0x0000_7706);

        if mem::size_of::<usize>() == 8 {
            assert_eq!(ASHMEM_SET_SIZE, 0x4008_7703);
            assert_eq!(ASHMEM_SET_PROT_MASK, 0x4008_7705);
        } else {
            assert_eq!(ASHMEM_SET_SIZE, 0x4004_7703);
            assert_eq!(ASHMEM_SET_PROT_MASK, 0x4004_7705);
        }
    }
}
