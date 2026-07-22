use crate::alloc::with_allocator;
use crate::error::{check, IpcError, Result};
use crate::parcel::Parcel;
use ohos_ipc_sys as sys;
use std::ffi::CString;
use std::os::raw::{c_int, c_void};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;
use std::ptr::NonNull;

/// How a request is delivered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RequestMode {
    /// Wait for the stub to answer.
    #[default]
    Sync,
    /// Return as soon as the request has been dispatched.
    Async,
}

/// Options for [`RemoteProxy::send_request_with`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MessageOption {
    /// Whether to wait for the reply.
    pub mode: RequestMode,
    /// Reserved for RPC and ignored by IPC.
    pub timeout: u32,
}

impl MessageOption {
    /// A synchronous request with no timeout.
    pub fn sync() -> Self {
        MessageOption {
            mode: RequestMode::Sync,
            timeout: 0,
        }
    }

    /// An asynchronous request with no timeout.
    pub fn r#async() -> Self {
        MessageOption {
            mode: RequestMode::Async,
            timeout: 0,
        }
    }

    fn to_raw(self) -> sys::OH_IPC_MessageOption {
        let mode = match self.mode {
            RequestMode::Sync => sys::OH_IPC_RequestMode_OH_IPC_REQUEST_MODE_SYNC,
            RequestMode::Async => sys::OH_IPC_RequestMode_OH_IPC_REQUEST_MODE_ASYNC,
        };
        sys::OH_IPC_MessageOption {
            mode,
            timeout: self.timeout,
            reserved: ptr::null_mut(),
        }
    }
}

/// A handle to a remote service, used to send it requests.
///
/// Obtained from a parcel with [`Parcel::read_remote_proxy`]. The handle is
/// destroyed on drop.
pub struct RemoteProxy {
    raw: NonNull<sys::OHIPCRemoteProxy>,
}

impl RemoteProxy {
    /// Take ownership of a proxy handle produced by the native API.
    ///
    /// # Safety
    ///
    /// `raw` must be a live proxy handle that nothing else destroys.
    pub(crate) unsafe fn from_raw(raw: NonNull<sys::OHIPCRemoteProxy>) -> Self {
        RemoteProxy { raw }
    }

    pub(crate) fn as_ptr(&self) -> *mut sys::OHIPCRemoteProxy {
        self.raw.as_ptr()
    }

    /// Send a synchronous request and wait for the reply.
    ///
    /// `code` is the command word agreed with the stub and must be in the range
    /// \[0x01, 0x00ffffff\].
    pub fn send_request(&self, code: u32, data: &Parcel, reply: &mut Parcel) -> Result<()> {
        self.send_request_with(code, data, Some(reply), MessageOption::sync())
    }

    /// Send a request with explicit options.
    ///
    /// A reply parcel is required in [`RequestMode::Sync`] and optional in
    /// [`RequestMode::Async`].
    pub fn send_request_with(
        &self,
        code: u32,
        data: &Parcel,
        reply: Option<&mut Parcel>,
        option: MessageOption,
    ) -> Result<()> {
        if option.mode == RequestMode::Sync && reply.is_none() {
            return Err(IpcError::InvalidArgument(
                "a synchronous request needs a reply parcel",
            ));
        }
        let reply = match reply {
            Some(reply) => reply.as_ptr(),
            None => ptr::null_mut(),
        };
        let option = option.to_raw();
        // SAFETY: the proxy and the data parcel are live, `reply` is either null
        // or a live parcel, and `option` outlives the call.
        check(unsafe {
            sys::OH_IPCRemoteProxy_SendRequest(self.as_ptr(), code, data.as_ptr(), reply, &option)
        })
    }

    /// Obtain the interface descriptor the stub was created with.
    ///
    /// Bytes that are not valid UTF-8 are replaced.
    pub fn interface_descriptor(&mut self) -> Result<String> {
        let proxy = self.as_ptr();
        let descriptor = with_allocator(
            "failed to read the interface descriptor",
            |out, len, allocator| {
                // SAFETY: `proxy` is live, `out` and `len` are live locals, and
                // the allocator matches the deallocator used on the returned
                // buffer.
                unsafe { sys::OH_IPCRemoteProxy_GetInterfaceDescriptor(proxy, out, len, allocator) }
            },
        )?;
        Ok(descriptor.to_string_lossy().into_owned())
    }

    /// Subscribe `recipient` to the death of the remote stub.
    ///
    /// The recipient must stay alive for as long as it is subscribed; see
    /// [`DeathRecipient`] for what happens if it is not.
    pub fn add_death_recipient(&mut self, recipient: &DeathRecipient) -> Result<()> {
        // SAFETY: both objects are live for the duration of the call.
        check(unsafe {
            sys::OH_IPCRemoteProxy_AddDeathRecipient(self.as_ptr(), recipient.as_ptr())
        })
    }

    /// Unsubscribe a recipient previously passed to
    /// [`RemoteProxy::add_death_recipient`].
    pub fn remove_death_recipient(&mut self, recipient: &DeathRecipient) -> Result<()> {
        // SAFETY: both objects are live for the duration of the call.
        check(unsafe {
            sys::OH_IPCRemoteProxy_RemoveDeathRecipient(self.as_ptr(), recipient.as_ptr())
        })
    }

    /// Whether the remote stub is dead.
    ///
    /// A proxy whose stub cannot be resolved counts as dead.
    pub fn is_remote_dead(&self) -> bool {
        // SAFETY: `self.raw` is a live proxy.
        unsafe { sys::OH_IPCRemoteProxy_IsRemoteDead(self.as_ptr()) != 0 }
    }
}

impl Drop for RemoteProxy {
    fn drop(&mut self) {
        // SAFETY: the handle is destroyed exactly once, here.
        unsafe { sys::OH_IPCRemoteProxy_Destroy(self.as_ptr()) };
    }
}

impl std::fmt::Debug for RemoteProxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RemoteProxy")
            .field("is_remote_dead", &self.is_remote_dead())
            .finish()
    }
}

/// A service object that answers requests sent to it over IPC.
///
/// Write it into a parcel with [`Parcel::write_remote_stub`] to hand a proxy for
/// it to the peer. The object is destroyed on drop, but the runtime keeps it
/// alive as long as a peer still holds a reference to it.
///
/// # Handler lifetime
///
/// The Rust handler is boxed and handed to the runtime as user data. It is
/// reclaimed in the native destroy callback rather than in `Drop`, because
/// dropping the handle only releases this process's reference: a request
/// dispatched on an IPC worker thread can still be running, and a peer can still
/// hold the object. The runtime invokes the destroy callback only once the
/// object is really gone, which is after the last request callback has returned,
/// so the handler cannot be observed after it is freed. If a runtime were never
/// to invoke that callback, the handler would leak, which is safe.
pub struct RemoteStub {
    raw: NonNull<sys::OHIPCRemoteStub>,
}

/// Trampoline invoked by the runtime for each incoming request.
unsafe extern "C" fn on_request<F>(
    code: u32,
    data: *const sys::OHIPCParcel,
    reply: *mut sys::OHIPCParcel,
    user_data: *mut c_void,
) -> c_int
where
    F: Fn(u32, &mut Parcel, &mut Parcel) -> Result<()> + Send + Sync + 'static,
{
    let inner_error = sys::OH_IPC_ErrorCode_OH_IPC_INNER_ERROR as c_int;
    if user_data.is_null() {
        return inner_error;
    }
    // A panic must not unwind into C, so it is turned into an internal error.
    let outcome = catch_unwind(AssertUnwindSafe(|| {
        // SAFETY: `user_data` is the `Box<F>` leaked in `RemoteStub::new`, which
        // is only reclaimed by `on_destroy` after the object is gone; `F` is
        // `Sync`, so sharing it across IPC worker threads is sound.
        let handler = unsafe { &*user_data.cast::<F>() };
        // SAFETY: both parcels are owned by the runtime and stay valid for the
        // duration of this call; the wrappers are borrowed, so they are not
        // destroyed here. The request parcel is passed as mutable because
        // reading advances its cursor.
        let borrowed = unsafe { (Parcel::borrowed(data.cast_mut()), Parcel::borrowed(reply)) };
        let (Some(mut data), Some(mut reply)) = borrowed else {
            return Err(IpcError::Native(
                sys::OH_IPC_ErrorCode_OH_IPC_CHECK_PARAM_ERROR as i32,
            ));
        };
        handler(code, &mut data, &mut reply)
    }));
    match outcome {
        Ok(Ok(())) => sys::OH_IPC_ErrorCode_OH_IPC_SUCCESS as c_int,
        Ok(Err(error)) => error.code().unwrap_or(inner_error),
        Err(_) => inner_error,
    }
}

/// Trampoline invoked by the runtime once the object it belongs to is gone.
unsafe extern "C" fn on_destroy<F>(user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }
    // SAFETY: `user_data` is the `Box<F>` leaked at creation time. The runtime
    // calls this once, after the object is destroyed and no further callback can
    // reference the box.
    let handler = unsafe { Box::from_raw(user_data.cast::<F>()) };
    // The value being dropped is user code, so a panic is contained here rather
    // than unwinding into C.
    let _ = catch_unwind(AssertUnwindSafe(move || drop(handler)));
}

impl RemoteStub {
    /// Create a service object that answers with `handler`.
    ///
    /// `descriptor` identifies the interface and is what
    /// [`RemoteProxy::interface_descriptor`] returns on the peer side; it is
    /// conventionally also the interface token written into each request.
    ///
    /// The handler runs on IPC worker threads, possibly several at a time, hence
    /// the `Send + Sync` bound. Returning `Err` sends that error code back to
    /// the caller; use [`IpcError::user`] for a code of your own. A panic inside
    /// the handler is caught and reported as an internal error instead of
    /// unwinding into the runtime.
    pub fn new<F>(descriptor: &str, handler: F) -> Result<Self>
    where
        F: Fn(u32, &mut Parcel, &mut Parcel) -> Result<()> + Send + Sync + 'static,
    {
        let descriptor = CString::new(descriptor).map_err(|_| {
            IpcError::InvalidArgument("the descriptor contains an interior NUL byte")
        })?;
        let user_data = Box::into_raw(Box::new(handler)).cast::<c_void>();
        // SAFETY: the descriptor is NUL-terminated and only read during the
        // call; the trampolines are monomorphized for `F` and `user_data` is the
        // matching leaked box.
        let raw = unsafe {
            sys::OH_IPCRemoteStub_Create(
                descriptor.as_ptr(),
                Some(on_request::<F>),
                Some(on_destroy::<F>),
                user_data,
            )
        };
        match NonNull::new(raw) {
            Some(raw) => Ok(RemoteStub { raw }),
            None => {
                // Creation failed, so no object exists and no callback can ever
                // run: the box is reclaimed here instead.
                // SAFETY: `user_data` is the box leaked just above and no
                // callback has seen it.
                drop(unsafe { Box::from_raw(user_data.cast::<F>()) });
                Err(IpcError::Failed("failed to create a remote stub"))
            }
        }
    }

    /// Take ownership of a stub handle produced by the native API.
    ///
    /// # Safety
    ///
    /// `raw` must be a live stub handle that nothing else destroys.
    pub(crate) unsafe fn from_raw(raw: NonNull<sys::OHIPCRemoteStub>) -> Self {
        RemoteStub { raw }
    }

    pub(crate) fn as_ptr(&self) -> *mut sys::OHIPCRemoteStub {
        self.raw.as_ptr()
    }
}

impl Drop for RemoteStub {
    fn drop(&mut self) {
        // SAFETY: the handle is destroyed exactly once, here. Any boxed handler
        // is reclaimed by `on_destroy`, not here, so a request still running on
        // an IPC worker thread keeps seeing a live handler.
        unsafe { sys::OH_IPCRemoteStub_Destroy(self.as_ptr()) };
    }
}

impl std::fmt::Debug for RemoteStub {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RemoteStub").finish_non_exhaustive()
    }
}

/// A subscription object that reports the unexpected death of a remote stub.
///
/// Register it on a proxy with [`RemoteProxy::add_death_recipient`]. The object
/// is destroyed on drop.
///
/// # Callback lifetime
///
/// As for [`RemoteStub`], the boxed callback is reclaimed in the native destroy
/// callback rather than in `Drop`: dropping the recipient while it is still
/// registered, or while a death notification is being delivered on another
/// thread, only releases this process's reference. The runtime invokes the
/// destroy callback once nothing can call back any more, so the closure cannot
/// be observed after it is freed. Unsubscribe with
/// [`RemoteProxy::remove_death_recipient`] before dropping the recipient to stop
/// notifications at a defined point.
pub struct DeathRecipient {
    raw: NonNull<sys::OHIPCDeathRecipient>,
}

/// Trampoline invoked by the runtime when the observed stub dies.
unsafe extern "C" fn on_death<F>(user_data: *mut c_void)
where
    F: Fn() + Send + Sync + 'static,
{
    if user_data.is_null() {
        return;
    }
    // SAFETY: `user_data` is the `Box<F>` leaked in `DeathRecipient::new`, which
    // is only reclaimed by `on_destroy` after the object is gone; `F` is `Sync`,
    // so calling it from a runtime thread is sound.
    let callback = unsafe { &*user_data.cast::<F>() };
    // A panic must not unwind into C.
    let _ = catch_unwind(AssertUnwindSafe(callback));
}

impl DeathRecipient {
    /// Create a recipient that runs `callback` when the observed stub dies.
    ///
    /// The callback runs on a runtime thread, hence the `Send + Sync` bound. A
    /// panic inside it is caught rather than unwinding into the runtime.
    pub fn new<F>(callback: F) -> Result<Self>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let user_data = Box::into_raw(Box::new(callback)).cast::<c_void>();
        // SAFETY: the trampolines are monomorphized for `F` and `user_data` is
        // the matching leaked box.
        let raw = unsafe {
            sys::OH_IPCDeathRecipient_Create(Some(on_death::<F>), Some(on_destroy::<F>), user_data)
        };
        match NonNull::new(raw) {
            Some(raw) => Ok(DeathRecipient { raw }),
            None => {
                // Creation failed, so no callback can ever run.
                // SAFETY: `user_data` is the box leaked just above and no
                // callback has seen it.
                drop(unsafe { Box::from_raw(user_data.cast::<F>()) });
                Err(IpcError::Failed("failed to create a death recipient"))
            }
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut sys::OHIPCDeathRecipient {
        self.raw.as_ptr()
    }
}

impl Drop for DeathRecipient {
    fn drop(&mut self) {
        // SAFETY: the handle is destroyed exactly once, here. The boxed callback
        // is reclaimed by `on_destroy`, not here, so a notification in flight
        // keeps seeing a live closure.
        unsafe { sys::OH_IPCDeathRecipient_Destroy(self.as_ptr()) };
    }
}

impl std::fmt::Debug for DeathRecipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeathRecipient").finish_non_exhaustive()
    }
}
