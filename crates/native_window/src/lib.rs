use libc::pollfd;
use ohos_native_window_sys::{
    NativeWindow as NativeWindowRaw, OHNativeWindowBuffer as OHNativeWindowBufferRaw,
    OHScalingModeV2, OHScalingModeV2_OH_SCALING_MODE_FREEZE_V2,
    OHScalingModeV2_OH_SCALING_MODE_NO_SCALE_CROP_V2,
    OHScalingModeV2_OH_SCALING_MODE_SCALE_CROP_V2, OHScalingModeV2_OH_SCALING_MODE_SCALE_FIT_V2,
    OHScalingModeV2_OH_SCALING_MODE_SCALE_TO_WINDOW_V2, OH_NativeWindow_GetSurfaceId,
    OH_NativeWindow_NativeObjectReference, OH_NativeWindow_NativeObjectUnreference,
    OH_NativeWindow_NativeWindowAbortBuffer, OH_NativeWindow_NativeWindowFlushBuffer,
    OH_NativeWindow_NativeWindowHandleOpt, OH_NativeWindow_NativeWindowRequestBuffer,
    OH_NativeWindow_NativeWindowSetScalingModeV2, Region_Rect,
};
use std::{
    mem::MaybeUninit,
    os::{
        fd::{AsRawFd, FromRawFd, OwnedFd},
        raw::c_void,
    },
    ptr::NonNull,
};

/// flush region
pub type Region = Region_Rect;

mod error;
mod operation;

pub use error::*;
pub use ohos_native_buffer_binding::*;
pub use operation::*;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum NativeWindowScalingMode {
    Freeze,
    Stretch,
    Crop,
    NoScaleCrop,
    #[default]
    Fit,
}

impl NativeWindowScalingMode {
    fn raw(self) -> OHScalingModeV2 {
        match self {
            Self::Freeze => OHScalingModeV2_OH_SCALING_MODE_FREEZE_V2,
            Self::Stretch => OHScalingModeV2_OH_SCALING_MODE_SCALE_TO_WINDOW_V2,
            Self::Crop => OHScalingModeV2_OH_SCALING_MODE_SCALE_CROP_V2,
            Self::NoScaleCrop => OHScalingModeV2_OH_SCALING_MODE_NO_SCALE_CROP_V2,
            Self::Fit => OHScalingModeV2_OH_SCALING_MODE_SCALE_FIT_V2,
        }
    }
}

pub struct NativeWindow {
    window: NonNull<NativeWindowRaw>,
}

impl Clone for NativeWindow {
    fn clone(&self) -> Self {
        Self::clone_from_ptr(self.window.as_ptr().cast())
    }
}

impl NativeWindow {
    /// Create a new `NativeWindow` from a raw pointer and add a reference to it.
    pub fn clone_from_ptr(window: *mut c_void) -> Self {
        #[cfg(debug_assertions)]
        assert!(!window.is_null(), "The window pointer must not be null.");

        let ret = unsafe { OH_NativeWindow_NativeObjectReference(window) };
        assert_eq!(ret, 0, "OH_NativeWindow_NativeObjectReference failed");

        unsafe {
            NativeWindow {
                window: NonNull::new_unchecked(window as *mut NativeWindowRaw),
            }
        }
    }

    /// Returns the surface ID associated with this native window.
    ///
    /// The returned ID can be passed to APIs that associate work with this
    /// window, such as `OH_NativeVSync_Create_ForAssociatedWindow`.
    pub fn surface_id(&self) -> Result<u64, NativeWindowError> {
        let mut surface_id = 0_u64;
        let ret = unsafe { OH_NativeWindow_GetSurfaceId(self.window.as_ptr(), &mut surface_id) };
        if ret != 0 {
            return Err(NativeWindowError::InternalError(ret));
        }
        if surface_id == 0 {
            return Err(NativeWindowError::InternalError(-1));
        }
        Ok(surface_id)
    }

    /// Borrow the underlying native window pointer.
    ///
    /// The pointer remains valid for the lifetime of this owner. Consumers
    /// must not unreference or destroy it.
    pub fn as_ptr(&self) -> *mut ohos_native_window_sys::OHNativeWindow {
        self.window.as_ptr().cast()
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

    /// Select how producer frames are scaled into this output window.
    pub fn set_scaling_mode(&self, mode: NativeWindowScalingMode) -> Result<(), NativeWindowError> {
        let ret = unsafe {
            OH_NativeWindow_NativeWindowSetScalingModeV2(self.window.as_ptr(), mode.raw())
        };
        if ret != 0 {
            return Err(NativeWindowError::InternalError(ret));
        }
        Ok(())
    }

    pub fn request_buffer(
        &self,
        region: Option<Region>,
    ) -> Result<NativeWindowBuffer<'_>, NativeWindowError> {
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
        let Some(raw_buf) = NonNull::new(window_buf) else {
            return Err(NativeWindowError::InternalError(-1));
        };
        if let Err(error) = wait_for_release_fence(release_fd) {
            self.abort_buffer(raw_buf);
            return Err(error);
        }
        let native_buffer = match NativeBuffer::try_from_window_buffer_ptr(raw_buf.as_ptr()) {
            Ok(buffer) => buffer,
            Err(error) => {
                self.abort_buffer(raw_buf);
                return Err(NativeWindowError::InternalError(error.code()));
            }
        };
        let config = native_buffer.config();
        let format = NativeBufferFormat::from(config.format);
        let bytes_per_pixel = format.bytes_per_pixel();
        let geometry = validate_geometry(config, bytes_per_pixel);
        let (width, height, stride, stride_bytes) = match geometry {
            Ok(geometry) => geometry,
            Err(error) => {
                self.abort_buffer(raw_buf);
                return Err(error);
            }
        };
        let mapped = match native_buffer.map_owned() {
            Ok(mapped) => mapped,
            Err(error) => {
                self.abort_buffer(raw_buf);
                return Err(NativeWindowError::InternalError(error.code()));
            }
        };
        Ok(NativeWindowBuffer {
            window: self,
            raw_buf,
            mapped: Some(mapped),
            region,
            width,
            height,
            stride,
            stride_bytes,
            format,
        })
    }

    fn abort_buffer(&self, buffer: NonNull<OHNativeWindowBufferRaw>) {
        let _ = unsafe {
            OH_NativeWindow_NativeWindowAbortBuffer(self.window.as_ptr(), buffer.as_ptr())
        };
    }
}

unsafe impl Send for NativeWindow {}

pub struct NativeWindowBuffer<'a> {
    window: &'a NativeWindow,
    raw_buf: NonNull<OHNativeWindowBufferRaw>,
    mapped: Option<MappedNativeBuffer>,
    region: Option<Region>,
    width: usize,
    height: usize,
    stride: usize,
    stride_bytes: usize,
    format: NativeBufferFormat,
}

/// create native window buffer allow you to flush
/// some codes are forked from [ndk](https://github.com/rust-mobile/ndk/blob/master/ndk/src/hardware_buffer.rs)
/// use MIT license
impl NativeWindowBuffer<'_> {
    /// The number of pixels that are shown horizontally.
    pub fn width(&self) -> usize {
        self.width
    }

    // The number of pixels that are shown vertically.
    pub fn height(&self) -> usize {
        self.height
    }

    /// The number of _pixels_ that a line in the buffer takes in memory.
    ///
    /// This may be `>= width`.
    pub fn stride(&self) -> usize {
        self.stride
    }

    pub fn format(&self) -> NativeBufferFormat {
        self.format
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
        self.mapped
            .as_mut()
            .expect("native window buffer is mapped")
            .bytes_mut()
            .as_mut_ptr()
            .cast()
    }

    /// Safe write access to likely uninitialized pixel buffer data.
    pub fn bytes(&mut self) -> Option<&mut [MaybeUninit<u8>]> {
        let bytes = self
            .mapped
            .as_mut()
            .expect("native window buffer is mapped")
            .bytes_mut();
        Some(unsafe { std::slice::from_raw_parts_mut(bytes.as_mut_ptr().cast(), bytes.len()) })
    }

    /// Returns a slice of bytes for each line of visible pixels in the buffer, ignoring any
    /// padding pixels incurred by the stride.
    ///
    /// See [`bits()`][Self::bits()] and [`bytes()`][Self::bytes()] for contiguous access to the
    /// underlying buffer.
    pub fn lines(&mut self) -> Option<impl Iterator<Item = &mut [MaybeUninit<u8>]>> {
        let bpp = self.format().bytes_per_pixel();
        let scanline_bytes = self.stride_bytes;
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
        self.mapped.take();
        let mut region = ohos_native_window_sys::Region {
            rectNumber: 0,
            rects: std::ptr::null_mut(),
        };
        if let Some(r) = self.region.as_ref() {
            region = ohos_native_window_sys::Region {
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
        let _ = ret;
    }
}

fn wait_for_release_fence(fd: i32) -> Result<(), NativeWindowError> {
    if fd < 0 {
        return Ok(());
    }
    let fence = unsafe { OwnedFd::from_raw_fd(fd) };
    let mut descriptor = pollfd {
        fd: fence.as_raw_fd(),
        events: libc::POLLIN,
        revents: 0,
    };
    loop {
        let result = unsafe { libc::poll(&mut descriptor, 1, 3000) };
        if result > 0 {
            return Ok(());
        }
        if result == 0 {
            return Err(NativeWindowError::InternalError(libc::ETIMEDOUT));
        }
        let error = std::io::Error::last_os_error();
        if error.kind() != std::io::ErrorKind::Interrupted {
            return Err(NativeWindowError::InternalError(
                error.raw_os_error().unwrap_or(-1),
            ));
        }
    }
}

fn validate_geometry(
    config: NativeBufferConfig,
    bytes_per_pixel: usize,
) -> Result<(usize, usize, usize, usize), NativeWindowError> {
    let width = positive_usize(config.width)?;
    let height = positive_usize(config.height)?;
    let stride_bytes = positive_usize(config.stride)?;
    if bytes_per_pixel == 0
        || stride_bytes % bytes_per_pixel != 0
        || stride_bytes < width.saturating_mul(bytes_per_pixel)
    {
        return Err(NativeWindowError::InternalError(-1));
    }
    Ok((width, height, stride_bytes / bytes_per_pixel, stride_bytes))
}

fn positive_usize(value: i32) -> Result<usize, NativeWindowError> {
    usize::try_from(value)
        .ok()
        .filter(|value| *value > 0)
        .ok_or(NativeWindowError::InternalError(-1))
}

impl Drop for NativeWindow {
    fn drop(&mut self) {
        let _ = unsafe { OH_NativeWindow_NativeObjectUnreference(self.window.as_ptr().cast()) };
    }
}
