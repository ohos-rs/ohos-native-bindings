//! Thin NativeImage consumer ownership, native listeners and acquire fences.

mod texture;
pub use texture::TextureImage;

use ohos_native_image_sys::*;
use ohos_native_window_sys::{
    OHNativeWindow, OHNativeWindowBuffer, OH_NativeWindow_NativeObjectReference,
    OH_NativeWindow_NativeObjectUnreference,
};
use std::{
    cell::Cell,
    fmt,
    marker::PhantomData,
    os::fd::{AsRawFd, FromRawFd, IntoRawFd, OwnedFd},
    ptr::NonNull,
    time::Duration,
};

/// The SDK listener descriptor; its context remains owned by the caller.
pub use ohos_native_image_sys::OH_OnFrameAvailableListener as FrameAvailableListener;

// The NativeImage header documents this status but does not declare its enum.
const NATIVE_ERROR_NO_BUFFER: i32 = 40_601_000;

/// A native operation failure, or a rejected null handle/dimension.
#[derive(Debug, Clone, Copy)]
pub struct Error {
    operation: &'static str,
    code: i32,
}

impl Error {
    /// Native status, or -1 for a rejected null handle/invalid dimension.
    pub const fn code(self) -> i32 {
        self.code
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NativeImage {} failed ({})", self.operation, self.code)
    }
}
impl std::error::Error for Error {}

fn check(operation: &'static str, code: i32) -> Result<(), Error> {
    if code == 0 {
        Ok(())
    } else {
        Err(Error { operation, code })
    }
}

fn fence_ready(events: i16) -> Result<bool, Error> {
    if events & (libc::POLLERR | libc::POLLNVAL | libc::POLLHUP) != 0 {
        return Err(Error {
            operation: "fence status",
            code: events as i32,
        });
    }
    Ok(events & libc::POLLIN != 0)
}

/// Native buffer-usage flags. No usage is selected implicitly by the binding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BufferUsage(u64);

impl BufferUsage {
    /// CPU-readable memory.
    pub const CPU_READ: Self =
        Self(ohos_native_buffer_sys::OH_NativeBuffer_Usage_NATIVEBUFFER_USAGE_CPU_READ as u64);
    /// CPU-writable memory.
    pub const CPU_WRITE: Self =
        Self(ohos_native_buffer_sys::OH_NativeBuffer_Usage_NATIVEBUFFER_USAGE_CPU_WRITE as u64);
    /// Native hardware render target.
    pub const HW_RENDER: Self =
        Self(ohos_native_buffer_sys::OH_NativeBuffer_Usage_NATIVEBUFFER_USAGE_HW_RENDER as u64);
    /// Native hardware texture.
    pub const HW_TEXTURE: Self =
        Self(ohos_native_buffer_sys::OH_NativeBuffer_Usage_NATIVEBUFFER_USAGE_HW_TEXTURE as u64);

    /// Wraps a caller-selected SDK usage mask, preserving all bits.
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    /// Returns the unmodified native usage mask.
    pub const fn bits(self) -> u64 {
        self.0
    }
}

impl std::ops::BitOr for BufferUsage {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// A unique native consumer with serialized operations and no EGL state.
///
/// ```compile_fail
/// fn needs_sync<T: Sync>() {}
/// needs_sync::<ohos_native_image_binding::ConsumerSurface>();
/// ```
pub struct ConsumerSurface {
    raw: NonNull<OH_NativeImage>,
    listening: bool,
    _not_sync: PhantomData<Cell<()>>,
}

// SAFETY: this consumer never owns an EGL context. Native calls require
// serialization, enforced by mutable ownership, including the frame borrow.
unsafe impl Send for ConsumerSurface {}

impl ConsumerSurface {
    /// Creates a consumer with the SDK's default usage (API 12).
    pub fn new() -> Result<Self, Error> {
        // SAFETY: creation takes no arguments and transfers a unique native owner.
        let raw = NonNull::new(unsafe { OH_ConsumerSurface_Create() }).ok_or(Error {
            operation: "create",
            code: -1,
        })?;
        Ok(Self {
            raw,
            listening: false,
            _not_sync: PhantomData,
        })
    }

    /// Creates a consumer with exactly the caller's usage flags (API 13).
    #[cfg(feature = "api-13")]
    pub fn with_usage(usage: BufferUsage) -> Result<Self, Error> {
        let mut surface = Self::new()?;
        surface.set_usage(usage)?;
        Ok(surface)
    }

    /// Sets future producer-buffer usage without interpreting the flags (API 13).
    #[cfg(feature = "api-13")]
    pub fn set_usage(&mut self, usage: BufferUsage) -> Result<(), Error> {
        // SAFETY: mutable ownership serializes this live consumer's native call.
        check("usage", unsafe {
            OH_ConsumerSurface_SetDefaultUsage(self.raw.as_ptr(), usage.bits())
        })
    }

    /// Registers the native listener without a closure registry or context owner.
    ///
    /// # Safety
    ///
    /// The callback and context must remain valid for every possible invocation,
    /// including in-flight callbacks after replacement, unregistration or Drop.
    /// The caller must establish native callback quiescence before freeing the
    /// context. Calls may arrive on a producer thread; context access must be
    /// synchronized. The callback must not unwind or call/reenter NativeImage
    /// operations. The binding does not retain or destroy the context.
    pub unsafe fn set_frame_available_listener(
        &mut self,
        listener: FrameAvailableListener,
    ) -> Result<(), Error> {
        // SAFETY: the caller supplies the callback/context lifetime and thread contract.
        check("listen", unsafe {
            OH_NativeImage_SetOnFrameAvailableListener(self.raw.as_ptr(), listener)
        })?;
        self.listening = true;
        Ok(())
    }

    /// Unregisters the native listener. This does not prove that in-flight
    /// callbacks have completed; its context remains the caller's responsibility.
    pub fn unset_frame_available_listener(&mut self) -> Result<(), Error> {
        if self.listening {
            // SAFETY: mutable ownership serializes access to this live native image.
            check("unlisten", unsafe {
                OH_NativeImage_UnsetOnFrameAvailableListener(self.raw.as_ptr())
            })?;
            self.listening = false;
        }
        Ok(())
    }

    /// Acquires the SDK-owned producer-window handle, valid until consumer Drop.
    ///
    /// The caller must not destroy this borrowed handle.
    pub fn acquire_native_window(&mut self) -> Result<NonNull<OHNativeWindow>, Error> {
        // SAFETY: mutable ownership serializes initialization of the producer window.
        NonNull::new(unsafe { OH_NativeImage_AcquireNativeWindow(self.raw.as_ptr()) }).ok_or(
            Error {
                operation: "producer window",
                code: -1,
            },
        )
    }

    /// Queries the surface identity without acquiring a producer window.
    pub fn surface_id(&mut self) -> Result<u64, Error> {
        let mut id = 0;
        // SAFETY: the live consumer owns the native image and id is writable.
        check("surface ID", unsafe {
            OH_NativeImage_GetSurfaceId(self.raw.as_ptr(), &mut id)
        })?;
        Ok(id)
    }

    /// Sets future producer-buffer dimensions, retaining outstanding frames (API 13).
    #[cfg(feature = "api-13")]
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), Error> {
        if width == 0 || height == 0 || width > i32::MAX as u32 || height > i32::MAX as u32 {
            return Err(Error {
                operation: "native dimensions",
                code: -1,
            });
        }
        // SAFETY: dimensions fit the SDK's positive signed integers.
        check("resize", unsafe {
            OH_ConsumerSurface_SetDefaultSize(self.raw.as_ptr(), width as i32, height as i32)
        })
    }

    /// Acquires a native buffer/fence, or None when the SDK reports no buffer.
    pub fn acquire(&mut self) -> Result<Option<Frame<'_>>, Error> {
        let mut buffer = std::ptr::null_mut();
        let mut fence = -1;
        // SAFETY: mutable borrowing serializes acquisition; outputs are writable.
        let code = unsafe {
            OH_NativeImage_AcquireNativeWindowBuffer(self.raw.as_ptr(), &mut buffer, &mut fence)
        };
        // SAFETY: the SDK transfers ownership of any nonnegative output descriptor.
        let fence = (fence >= 0).then(|| unsafe { OwnedFd::from_raw_fd(fence) });
        if code == NATIVE_ERROR_NO_BUFFER {
            return Ok(None);
        }
        check("acquire", code)?;
        let buffer = NonNull::new(buffer).ok_or(Error {
            operation: "null buffer",
            code: -1,
        })?;
        // SAFETY: the SDK requires this explicit retain while using the window buffer.
        let retain = unsafe { OH_NativeWindow_NativeObjectReference(buffer.as_ptr().cast()) };
        if retain != 0 {
            // SAFETY: return the acquisition and its owned fence before propagating failure.
            let released = unsafe {
                OH_NativeImage_ReleaseNativeWindowBuffer(
                    self.raw.as_ptr(),
                    buffer.as_ptr(),
                    fence.map_or(-1, IntoRawFd::into_raw_fd),
                )
            };
            if released != 0 {
                log::error!("NativeImage release after retain failure: {released}");
            }
            return Err(Error {
                operation: "retain buffer",
                code: retain,
            });
        }
        Ok(Some(Frame {
            surface: self,
            buffer,
            fence,
        }))
    }
}

impl Drop for ConsumerSurface {
    fn drop(&mut self) {
        if let Err(error) = self.unset_frame_available_listener() {
            log::error!("NativeImage cleanup: {error}");
        }
        // SAFETY: all frame borrows have ended. Native destruction does not
        // release caller-owned listener contexts.
        unsafe {
            let mut raw = self.raw.as_ptr();
            OH_NativeImage_Destroy(&mut raw);
        }
    }
}

/// An acquisition guard retaining the consumer, buffer wrapper and fence.
///
/// ```compile_fail
/// use ohos_native_image_binding::{ConsumerSurface, Frame};
/// fn escape(surface: &mut ConsumerSurface) -> Frame<'static> {
///     surface.acquire().unwrap().unwrap()
/// }
/// ```
pub struct Frame<'a> {
    surface: &'a mut ConsumerSurface,
    buffer: NonNull<OHNativeWindowBuffer>,
    fence: Option<OwnedFd>,
}

impl fmt::Debug for Frame<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NativeImageFrame")
            .field("buffer", &self.buffer)
            .field("pending_fence", &self.fence.is_some())
            .finish()
    }
}

impl Frame<'_> {
    /// Borrows the opaque window-buffer handle until this frame is released.
    ///
    /// The caller must not unreference or release it. Reading its contents through
    /// unsafe native APIs requires first observing a successful acquire-fence wait.
    pub fn native_window_buffer(&self) -> NonNull<OHNativeWindowBuffer> {
        self.buffer
    }

    /// Waits within the caller's timeout for the acquire fence; performs no mapping.
    pub fn wait(&mut self, timeout: Duration) -> Result<bool, Error> {
        if let Some(fence) = &self.fence {
            let mut poll = libc::pollfd {
                fd: fence.as_raw_fd(),
                events: libc::POLLIN,
                revents: 0,
            };
            // SAFETY: the owned descriptor and stack pollfd remain live for poll.
            let result = unsafe {
                libc::poll(
                    &mut poll,
                    1,
                    timeout.as_millis().min(i32::MAX as u128) as i32,
                )
            };
            if result < 0 {
                return Err(Error {
                    operation: "fence wait",
                    code: std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                });
            }
            if result == 0 || !fence_ready(poll.revents)? {
                return Ok(false);
            }
        }
        self.fence = None;
        Ok(true)
    }
}

impl Drop for Frame<'_> {
    fn drop(&mut self) {
        let fence = self.fence.take().map_or(-1, IntoRawFd::into_raw_fd);
        // SAFETY: this guard owns the acquisition; the SDK consumes the release
        // fence, including an unsignalled acquire fence returned without a wait.
        let released = unsafe {
            OH_NativeImage_ReleaseNativeWindowBuffer(
                self.surface.raw.as_ptr(),
                self.buffer.as_ptr(),
                fence,
            )
        };
        if released != 0 {
            log::error!("NativeImage frame release: {released}");
        }
        // SAFETY: balances our retain after the native release finished using the wrapper.
        let unreferenced =
            unsafe { OH_NativeWindow_NativeObjectUnreference(self.buffer.as_ptr().cast()) };
        if unreferenced != 0 {
            log::error!("NativeImage frame unreference: {unreferenced}");
        }
    }
}
