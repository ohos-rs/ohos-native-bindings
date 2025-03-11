use ohos_native_window_sys::{
    NativeWindow as NativeWindowRaw, OHNativeWindowBuffer as OHNativeWindowBufferRaw,
    OH_NativeWindow_NativeObjectReference, OH_NativeWindow_NativeObjectUnreference,
    OH_NativeWindow_NativeWindowFlushBuffer, OH_NativeWindow_NativeWindowHandleOpt,
    OH_NativeWindow_NativeWindowRequestBuffer, Region as RegionRaw, Region_Rect,
};
use std::{cell::RefCell, mem::MaybeUninit, os::raw::c_void, ptr::NonNull, rc::Rc};

/// flush region
pub type Region = Region_Rect;

mod error;
mod operation;

pub use error::*;
pub use ohos_native_buffer_binding::*;
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

        let buf = NativeBuffer::from_window_buffer_ptr(window_buf);

        let window_buffer = buf.mmap();

        Ok(NativeWindowBuffer {
            window: self,
            raw_buf: NonNull::new(window_buf).expect("NonNull::new failed"),
            buffer: buf,
            release_fd,
            window_buffer,
        })
    }
}

unsafe impl Send for NativeWindow {}

pub struct NativeWindowBuffer<'a> {
    window: &'a NativeWindow,
    // can be operate memory directly
    window_buffer: NonNull<c_void>,
    raw_buf: NonNull<OHNativeWindowBufferRaw>,
    buffer: NativeBuffer,
    #[allow(dead_code)]
    release_fd: i32,
}

/// create native window buffer allow you to flush
/// some codes are forked from [ndk](https://github.com/rust-mobile/ndk/blob/master/ndk/src/hardware_buffer.rs)
/// use MIT license
impl NativeWindowBuffer<'_> {
    /// The number of pixels that are shown horizontally.
    pub fn width(&self) -> usize {
        usize::try_from(self.buffer.config().width).unwrap()
    }

    // The number of pixels that are shown vertically.
    pub fn height(&self) -> usize {
        usize::try_from(self.buffer.config().height).unwrap()
    }

    /// The number of _pixels_ that a line in the buffer takes in memory.
    ///
    /// This may be `>= width`.
    pub fn stride(&self) -> usize {
        usize::try_from(self.buffer.config().stride).unwrap()
    }

    pub fn format(&self) -> NativeBufferFormat {
        self.buffer.config().format.into()
    }

    /// The actual bits.
    ///
    /// This points to a memory segment of [`stride()`][Self::stride()] *
    /// [`height()`][Self::height()] * [`HardwareBufferFormat::bytes_per_pixel()`] bytes.
    ///
    /// Only [`width()`][Self::width()] pixels are visible for each [`stride()`][Self::stride()]
    /// line of pixels in the buffer.
    ///
    /// See [`bytes()`][Self::bytes()] for safe access to these bytes.
    pub fn bits(&mut self) -> *mut c_void {
        self.window_buffer.as_ptr()
    }

    /// Safe write access to likely uninitialized pixel buffer data.
    ///
    /// Returns [`None`] when there is no [`HardwareBufferFormat::bytes_per_pixel()`] size
    /// available for this [`format()`][Self::format()].
    ///
    /// The returned slice consists of [`stride()`][Self::stride()] * [`height()`][Self::height()]
    /// \* [`HardwareBufferFormat::bytes_per_pixel()`] bytes.
    ///
    /// Only [`width()`][Self::width()] pixels are visible for each [`stride()`][Self::stride()]
    /// line of pixels in the buffer.
    pub fn bytes(&mut self) -> Option<&mut [MaybeUninit<u8>]> {
        let config = self.buffer.config();
        let num_pixels = self.height() * self.stride();
        let num_bytes = num_pixels * NativeBufferFormat::from(config.format).bytes_per_pixel();
        Some(unsafe { std::slice::from_raw_parts_mut(self.bits().cast(), num_bytes) })
    }

    /// Returns a slice of bytes for each line of visible pixels in the buffer, ignoring any
    /// padding pixels incurred by the stride.
    ///
    /// See [`bits()`][Self::bits()] and [`bytes()`][Self::bytes()] for contiguous access to the
    /// underlying buffer.
    pub fn lines(&mut self) -> Option<impl Iterator<Item = &mut [MaybeUninit<u8>]>> {
        let config = self.buffer.config();
        let bpp = NativeBufferFormat::from(config.format).bytes_per_pixel();
        let scanline_bytes = bpp * self.stride();
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
                -1,
                region,
            )
        };
        #[cfg(debug_assertions)]
        assert!(ret == 0, "OH_NativeWindow_NativeWindowFlushBuffer failed");

        self.buffer.un_mmap();
    }
}

impl Drop for NativeWindow {
    fn drop(&mut self) {
        let ret = unsafe { OH_NativeWindow_NativeObjectUnreference(self.window.as_ptr().cast()) };
        #[cfg(debug_assertions)]
        assert!(ret == 0, "OH_NativeWindow_NativeObjectUnreference failed");
    }
}
