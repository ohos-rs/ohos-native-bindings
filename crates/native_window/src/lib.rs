use libc::{
    __errno_location, close, mmap, pollfd, EAGAIN, EINTR, MAP_SHARED, PROT_READ, PROT_WRITE,
};
use ohos_native_buffer_binding::NativeBufferFormat;
use ohos_native_buffer_sys::BufferHandle as BufferHandleRaw;
use ohos_native_window_sys::{
    NativeWindow as NativeWindowRaw, OHNativeWindowBuffer as OHNativeWindowBufferRaw,
    OH_NativeWindow_GetBufferHandleFromNative, OH_NativeWindow_NativeObjectReference,
    OH_NativeWindow_NativeObjectUnreference, OH_NativeWindow_NativeWindowFlushBuffer,
    OH_NativeWindow_NativeWindowHandleOpt, OH_NativeWindow_NativeWindowRequestBuffer,
    Region as RegionRaw, Region_Rect,
};
use std::{cell::RefCell, mem::MaybeUninit, os::raw::c_void, ptr::NonNull, rc::Rc};

/// flush region
pub type Region = Region_Rect;

mod error;
mod operation;

pub use error::*;
pub use operation::*;

pub struct NativeWindow {
    window: NonNull<NativeWindowRaw>,
    region: Rc<RefCell<Option<Region>>>,
}

impl NativeWindow {
    /// Create a new `NativeWindow` from a raw pointer and add a reference to it.
    pub fn clone_from_ptr(window: *mut c_void) -> Self {
        #[cfg(debug_assertions)]
        assert!(!window.is_null(), "The window pointer must not be null.");

        let ret = unsafe { OH_NativeWindow_NativeObjectReference(window) };
        #[cfg(debug_assertions)]
        assert!(ret == 0, "OH_NativeWindow_NativeObjectReference failed");

        unsafe {
            NativeWindow {
                window: NonNull::new_unchecked(window as *mut NativeWindowRaw),
                region: Rc::new(RefCell::new(None)),
            }
        }
    }

    pub fn set_buffer_geometry(&self, width: i32, height: i32) -> Result<(), NativeWindowError> {
        let ret = unsafe {
            OH_NativeWindow_NativeWindowHandleOpt(
                self.window.as_ptr(),
                Operation::SetBufferGeometry.into_i32(),
                width,
                height,
            )
        };
        if ret != 0 {
            return Err(NativeWindowError::InternalError(ret));
        }
        Ok(())
    }

    pub fn request_buffer(
        &self,
        region: Option<Region>,
    ) -> Result<NativeWindowBuffer, NativeWindowError> {
        self.region.replace(region);
        let mut window_buf = std::ptr::null_mut();
        let mut release_fd = -1;
        let ret = unsafe {
            OH_NativeWindow_NativeWindowRequestBuffer(
                self.window.as_ptr(),
                &mut window_buf,
                &mut release_fd,
            )
        };
        if ret != 0 {
            return Err(NativeWindowError::InternalError(ret));
        }

        unsafe {
            let handle = OH_NativeWindow_GetBufferHandleFromNative(window_buf);

            let addr = mmap(
                (*handle).virAddr,
                (*handle).size as _,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                (*handle).fd,
                0,
            );

            if addr == libc::MAP_FAILED {
                return Err(NativeWindowError::InternalError(-1));
            }

            if release_fd != -1 {
                let mut ret_code = -1;
                let mut fds = pollfd {
                    fd: release_fd,
                    events: libc::POLLIN,
                    revents: 0,
                };
                while ret_code == -1
                    && (*__errno_location() == EINTR || *__errno_location() == EAGAIN)
                {
                    ret_code = libc::poll(&mut fds, 1, 3000);
                }
                close(release_fd);
            }

            Ok(NativeWindowBuffer {
                window: self,
                raw_buf: NonNull::new_unchecked(window_buf),
                handle: NonNull::new_unchecked(handle),
                release_fd,
                window_buffer: NonNull::new_unchecked(addr),
            })
        }
    }
}

unsafe impl Send for NativeWindow {}

pub struct NativeWindowBuffer<'a> {
    window: &'a NativeWindow,
    // can be operate memory directly
    window_buffer: NonNull<c_void>,
    raw_buf: NonNull<OHNativeWindowBufferRaw>,
    handle: NonNull<BufferHandleRaw>,
    #[allow(dead_code)]
    release_fd: i32,
}

/// create native window buffer allow you to flush
/// some codes are forked from [ndk](https://github.com/rust-mobile/ndk/blob/master/ndk/src/hardware_buffer.rs)
/// use MIT license
impl NativeWindowBuffer<'_> {
    /// The number of pixels that are shown horizontally.
    pub fn width(&self) -> usize {
        let width = unsafe { (*self.handle.as_ptr()).width };
        usize::try_from(width).unwrap()
    }

    // The number of pixels that are shown vertically.
    pub fn height(&self) -> usize {
        let height = unsafe { (*self.handle.as_ptr()).height };
        usize::try_from(height).unwrap()
    }

    /// The number of _pixels_ that a line in the buffer takes in memory.
    ///
    /// This may be `>= width`.
    pub fn stride(&self) -> usize {
        let stride = unsafe { (*self.handle.as_ptr()).stride };
        usize::try_from(stride).unwrap()
    }

    pub fn format(&self) -> NativeBufferFormat {
        let format = unsafe { (*self.handle.as_ptr()).format };
        NativeBufferFormat::from(format)
    }

    /// The actual bits.
    ///
    /// This points to a memory segment of [`stride()`][Self::stride()] *
    /// [`height()`][Self::height()] * [`NativeBufferFormat::bytes_per_pixel()`] bytes.
    ///
    /// Only [`width()`][Self::width()] pixels are visible for each [`stride()`][Self::stride()]
    /// line of pixels in the buffer.
    ///
    /// See [`bytes()`][Self::bytes()] for safe access to these bytes.
    pub fn bits(&mut self) -> *mut c_void {
        self.window_buffer.as_ptr()
    }

    /// Safe write access to likely uninitialized pixel buffer data.
    pub fn bytes(&mut self) -> Option<&mut [MaybeUninit<u8>]> {
        let num_pixels = self.height() * self.stride();
        let bytes_nums = num_pixels * NativeBufferFormat::from(self.format()).bytes_per_pixel();
        Some(unsafe { std::slice::from_raw_parts_mut(self.bits().cast(), bytes_nums) })
    }

    /// Returns a slice of bytes for each line of visible pixels in the buffer, ignoring any
    /// padding pixels incurred by the stride.
    ///
    /// See [`bits()`][Self::bits()] and [`bytes()`][Self::bytes()] for contiguous access to the
    /// underlying buffer.
    pub fn lines(&mut self) -> Option<impl Iterator<Item = &mut [MaybeUninit<u8>]>> {
        let bpp = NativeBufferFormat::from(self.format()).bytes_per_pixel();
        let scanline_bytes = self.stride();
        let width_bytes = bpp * self.width();
        let bytes = self.bytes()?;

        Some(
            bytes
                .chunks_exact_mut(scanline_bytes)
                .map(move |scanline| &mut scanline[..width_bytes]),
        )
    }
}

impl<'a> Drop for NativeWindowBuffer<'a> {
    fn drop(&mut self) {
        let r = self.window.region.borrow();
        let mut region = RegionRaw {
            rectNumber: 0,
            rects: std::ptr::null_mut(),
        };
        if let Some(r) = r.as_ref() {
            region = RegionRaw {
                rectNumber: 1,
                rects: r as *const _ as *mut _,
            };
        }

        let ret = unsafe {
            OH_NativeWindow_NativeWindowFlushBuffer(
                self.window.window.as_ptr(),
                self.raw_buf.as_ptr(),
                self.release_fd,
                region,
            )
        };
        #[cfg(debug_assertions)]
        assert!(ret == 0, "OH_NativeWindow_NativeWindowFlushBuffer failed");

        // self.buffer.un_mmap();
        unsafe {
            libc::munmap(
                self.window_buffer.as_ptr(),
                (*self.handle.as_ptr()).size as _,
            );
        }
    }
}

impl Drop for NativeWindow {
    fn drop(&mut self) {
        let ret = unsafe { OH_NativeWindow_NativeObjectUnreference(self.window.as_ptr().cast()) };
        #[cfg(debug_assertions)]
        assert!(ret == 0, "OH_NativeWindow_NativeObjectUnreference failed");
    }
}
