//! Owned NativeImage consumers and fenced CPU mappings.
mod fence;
mod layout;
#[cfg(all(test, not(target_env = "ohos")))]
mod tests;
use ohos_native_buffer_binding::NativeBuffer;
use ohos_native_image_sys::*;
use ohos_native_window_sys::{
    OHNativeWindowBuffer, OH_NativeWindow_NativeObjectReference,
    OH_NativeWindow_NativeObjectUnreference,
};
use std::{
    cell::Cell,
    collections::BTreeMap,
    ffi::c_void,
    fmt,
    marker::PhantomData,
    os::fd::{AsRawFd, FromRawFd, OwnedFd},
    ptr::NonNull,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex, OnceLock,
    },
    time::Duration,
};

type Callback = Arc<dyn Fn() + Send + Sync>;
fn callbacks() -> &'static Mutex<BTreeMap<usize, Callback>> {
    static CALLBACKS: OnceLock<Mutex<BTreeMap<usize, Callback>>> = OnceLock::new();
    CALLBACKS.get_or_init(Mutex::default)
}
unsafe extern "C" fn available(context: *mut c_void) {
    // The context is a monotonic registry identity, never a borrowed address.
    let callback = callbacks()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .get(&(context as usize))
        .cloned();
    if let Some(callback) = callback {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || callback()));
    }
}
/// A failed NativeImage operation or rejected buffer layout.
#[derive(Debug, Clone, Copy)]
pub struct Error {
    operation: &'static str,
    code: i32,
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

/// Tightly packed RGBA pixels copied after their producer fence.
/// Alpha samples preserve the producer's representation; RGBX formats become opaque.
pub struct RgbaFrame {
    /// Physical width.
    pub width: u32,
    /// Physical height.
    pub height: u32,
    /// Four bytes per pixel, with no row padding.
    pub pixels: Vec<u8>,
}
/// A unique CPU consumer. It may move between threads; operations are serialized
/// by mutable borrowing and never use or depend on an EGL context.
///
/// ```compile_fail
/// fn needs_sync<T: Sync>() {}
/// needs_sync::<ohos_native_image_binding::ConsumerSurface>();
/// ```
pub struct ConsumerSurface {
    raw: NonNull<OH_NativeImage>,
    listener: usize,
    _not_sync: PhantomData<Cell<()>>,
}
// SAFETY: this CPU-only owner has no thread-affine EGL state. Its native APIs
// require serialization, which mutable borrowing enforces, including buffer guards.
unsafe impl Send for ConsumerSurface {}
impl ConsumerSurface {
    /// Creates a CPU consumer using native default usage on API 12.
    /// With `api-13`, requests CPU-readable, GPU-renderable buffers.
    /// The callback runs on a producer thread and must only enqueue a wake.
    /// It must not call NativeImage interfaces or retire consumers from the callback.
    pub fn new(callback: impl Fn() + Send + Sync + 'static) -> Result<Self, Error> {
        // SAFETY: creation takes no arguments and returns a unique native owner.
        let raw = NonNull::new(unsafe { OH_ConsumerSurface_Create() }).ok_or(Error {
            operation: "create",
            code: -1,
        })?;
        Self::with_raw(raw, callback, 1 | (1 << 8))
    }
    fn with_raw(
        raw: NonNull<OH_NativeImage>,
        callback: impl Fn() + Send + Sync + 'static,
        usage: u64,
    ) -> Result<Self, Error> {
        let mut surface = Self {
            raw,
            listener: 0,
            _not_sync: PhantomData,
        };
        #[cfg(feature = "api-13")]
        // SAFETY: surface retains the live, unique consumer.
        check("usage", unsafe {
            OH_ConsumerSurface_SetDefaultUsage(raw.as_ptr(), usage)
        })?;
        #[cfg(not(feature = "api-13"))]
        let _ = usage;
        static NEXT: AtomicUsize = AtomicUsize::new(1);
        #[allow(deprecated)] // Keep the binding's Rust 1.88 baseline.
        let id = NEXT
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |id| id.checked_add(1))
            .map_err(|_| Error {
                operation: "listener identity",
                code: -1,
            })?;
        callbacks()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .insert(id, Arc::new(callback));
        surface.listener = id;
        // SAFETY: the context is only converted back to a registry identity;
        // callbacks retain their own Arc while concurrent retirement removes it.
        check("listen", unsafe {
            OH_NativeImage_SetOnFrameAvailableListener(
                raw.as_ptr(),
                OH_OnFrameAvailableListener {
                    context: id as *mut c_void,
                    onFrameAvailable: Some(available),
                },
            )
        })?;
        Ok(surface)
    }
    /// Surface identity accepted by ArkUI BuilderNode's texture render options.
    pub fn id(&mut self) -> Result<u64, Error> {
        // Creating the producer window registers it with SurfaceUtils, which
        // ArkUI's texture exporter uses to resolve the consumer's surface ID.
        // The window is borrowed here and remains owned by NativeImage.
        // SAFETY: this mutable owner serializes initialization of its producer.
        if unsafe { OH_NativeImage_AcquireNativeWindow(self.raw.as_ptr()) }.is_null() {
            return Err(Error {
                operation: "producer window",
                code: -1,
            });
        }
        let mut id = 0;
        // SAFETY: this owner keeps the native image and output pointer live.
        check("surface ID", unsafe {
            OH_NativeImage_GetSurfaceId(self.raw.as_ptr(), &mut id)
        })?;
        Ok(id)
    }
    /// Update future producer-buffer dimensions. Outstanding frames retain their size.
    #[cfg(feature = "api-13")]
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), Error> {
        validate_size(width, height)?;
        // SAFETY: the mutable owner serializes this native operation.
        check("resize", unsafe {
            OH_ConsumerSurface_SetDefaultSize(self.raw.as_ptr(), width as i32, height as i32)
        })
    }
    /// Acquire a frame, or None if the producer has no pending buffer. No waiting.
    pub fn acquire(&mut self) -> Result<Option<Frame<'_>>, Error> {
        let mut buffer = std::ptr::null_mut();
        let mut fence = -1;
        // SAFETY: mutable borrowing serializes acquisition and owns both outputs.
        let code = unsafe {
            OH_NativeImage_AcquireNativeWindowBuffer(self.raw.as_ptr(), &mut buffer, &mut fence)
        };
        // SAFETY: a nonnegative returned descriptor transfers ownership to the caller.
        let fence = (fence >= 0).then(|| unsafe { OwnedFd::from_raw_fd(fence) });
        if code == 40601000 {
            return Ok(None);
        }
        check("acquire", code)?;
        let buffer = NonNull::new(buffer).ok_or(Error {
            operation: "null buffer",
            code: -1,
        })?;
        // SAFETY: acquisition returns a live window buffer. The NDK requires
        // retaining this wrapper until release and a matching unreference.
        let retain = unsafe { OH_NativeWindow_NativeObjectReference(buffer.as_ptr().cast()) };
        if retain != 0 {
            use std::os::fd::IntoRawFd;
            let release_fence = fence.map_or(-1, IntoRawFd::into_raw_fd);
            // SAFETY: acquisition still needs releasing when retaining fails.
            unsafe {
                OH_NativeImage_ReleaseNativeWindowBuffer(
                    self.raw.as_ptr(),
                    buffer.as_ptr(),
                    release_fence,
                );
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
            ready: false,
        }))
    }
}
impl Drop for ConsumerSurface {
    fn drop(&mut self) {
        // Removing the registry entry first also makes late callbacks harmless.
        let callback = {
            let mut callbacks = callbacks()
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            callbacks.remove(&self.listener)
        };
        // SAFETY: no frame can outlive its mutable consumer borrow. Registry
        // identities never dereference freed data, even during native teardown.
        unsafe {
            OH_NativeImage_UnsetOnFrameAvailableListener(self.raw.as_ptr());
            let mut raw = self.raw.as_ptr();
            OH_NativeImage_Destroy(&mut raw);
        }
        // Captured owner destructors may reenter the registry.
        drop(callback);
    }
}
/// A frame guard retains its consumer, acquire fence and native buffer until release.
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
    ready: bool,
}
impl fmt::Debug for Frame<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let layout = NativeBuffer::try_from_window_buffer_ptr(self.buffer.as_ptr())
            .map(|buffer| buffer.config())
            .ok();
        f.debug_struct("NativeImageFrame")
            .field("layout", &layout)
            .field("ready", &self.ready)
            .finish()
    }
}
impl Frame<'_> {
    /// Wait up to timeout for GPU writes. Use a worker for nonzero timeouts.
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
            if result == 0 {
                return Ok(false);
            }
            if !fence::ready(poll.revents)? {
                return Ok(false);
            }
        }
        self.ready = true;
        self.fence = None;
        Ok(true)
    }
    /// Copy RGBA/RGBX/BGRA/BGRX pixels, respecting the native row stride.
    /// Rejects unsignalled fences, unsupported formats and incomplete mappings.
    pub fn copy_rgba(&self) -> Result<RgbaFrame, Error> {
        if !self.ready {
            return Err(Error {
                operation: "unsignalled fence",
                code: -1,
            });
        }
        let buffer =
            NativeBuffer::try_from_window_buffer_ptr(self.buffer.as_ptr()).map_err(|_| Error {
                operation: "native buffer",
                code: -1,
            })?;
        let config = buffer.config();
        let width = u32::try_from(config.width).map_err(|_| Error {
            operation: "width",
            code: -1,
        })?;
        let height = u32::try_from(config.height).map_err(|_| Error {
            operation: "height",
            code: -1,
        })?;
        let layout = validate_size(width, height)?;
        layout::validate_format(config.format)?;
        layout.mapped_len(config.stride)?;
        let mapped = buffer.map_owned().map_err(|_| Error {
            operation: "map",
            code: -1,
        })?;
        let pixels = layout::copy(mapped.bytes(), config.stride, config.format, layout)?;
        Ok(RgbaFrame {
            width,
            height,
            pixels,
        })
    }
}
impl Drop for Frame<'_> {
    fn drop(&mut self) {
        use std::os::fd::IntoRawFd;
        // On a timeout, return the still-owned fence to the queue so the producer
        // cannot overwrite a buffer whose GPU writes have not finished.
        let fence = self.fence.take().map_or(-1, IntoRawFd::into_raw_fd);
        // SAFETY: this guard exclusively owns the acquisition; the system takes
        // ownership of a nonnegative release fence, as specified by the NDK.
        unsafe {
            OH_NativeImage_ReleaseNativeWindowBuffer(
                self.surface.raw.as_ptr(),
                self.buffer.as_ptr(),
                fence,
            );
        }
        // SAFETY: balances the explicit retain after acquisition, after the
        // consumer has finished using the wrapper in its release operation.
        unsafe {
            OH_NativeWindow_NativeObjectUnreference(self.buffer.as_ptr().cast());
        }
    }
}
fn validate_size(width: u32, height: u32) -> Result<layout::Layout, Error> {
    layout::Layout::new(width, height)
}
