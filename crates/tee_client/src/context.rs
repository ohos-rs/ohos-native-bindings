use crate::error::{check, Result, TeeError};
use crate::param::Operation;
use crate::types::{Login, Uuid};
use ohos_tee_client_sys as sys;
use std::cell::UnsafeCell;
use std::ffi::CString;
use std::os::raw::c_void;

/// A logical connection between the client application and a TEE.
///
/// The context is the root of the three-level ownership chain of the TEE Client
/// API. Sessions and shared memory blocks borrow it, so the borrow checker
/// refuses any program in which the context would be finalised while one of
/// them is still alive — the ordering the C API requires but cannot enforce.
///
/// The context is finalised on drop.
///
/// A context is neither `Send` nor `Sync`: the native structure threads itself
/// into intrusive lists of the sessions and shared memory blocks opened on it,
/// with no documented locking.
///
/// # Example
///
/// ```no_run
/// use ohos_tee_client_binding::{Context, Direction, Login, Operation, Parameter, TeeError, Uuid};
///
/// let context = Context::new()?;
/// let mut session = context.open_session(
///     &"79b77788-9789-4a7a-a2be-b60155eef5f3".parse::<Uuid>()?,
///     Login::Public,
///     None,
/// )?;
///
/// let request = b"ping";
/// let mut response = [0u8; 32];
/// let mut operation = Operation::with_params([
///     Parameter::TempMemoryInput(request),
///     Parameter::TempMemoryOutput(&mut response),
///     Parameter::Value { direction: Direction::Output, a: 0, b: 0 },
///     Parameter::None,
/// ]);
///
/// session.invoke_command(1, Some(&mut operation))?;
///
/// let written = operation.output_size(1).unwrap_or(0);
/// let (status, _) = operation.value(2).unwrap_or((0, 0));
/// println!("{status}: {:?}", &response[..written]);
/// # Ok::<(), TeeError>(())
/// ```
pub struct Context {
    // Boxed so the address stays put: the native structure holds the heads of
    // intrusive lists that point back into itself, and sessions and shared
    // memory blocks keep a pointer to it. Behind an UnsafeCell because the
    // native API mutates it through calls that this crate models as shared
    // borrows.
    raw: Box<UnsafeCell<sys::TEEC_Context>>,
}

impl Context {
    /// Connect to the default TEE.
    pub fn new() -> Result<Self> {
        Self::open(None)
    }

    /// Connect to the TEE reachable under `path`.
    pub fn with_path(path: &str) -> Result<Self> {
        Self::open(Some(path))
    }

    fn open(path: Option<&str>) -> Result<Self> {
        let path = match path {
            Some(path) => Some(CString::new(path).map_err(|_| {
                TeeError::invalid_argument("the TEE path contains an interior NUL byte")
            })?),
            None => None,
        };
        // SAFETY: TEEC_Context is a plain-old-data C struct; an all-zero value
        // is what the native API expects to be handed for initialisation.
        let raw: Box<UnsafeCell<sys::TEEC_Context>> =
            Box::new(UnsafeCell::new(unsafe { std::mem::zeroed() }));
        let path = path.as_ref().map_or(std::ptr::null(), |path| path.as_ptr());
        // SAFETY: `path` is either null or a NUL-terminated string alive for
        // the call, and the context points at an owned, zeroed structure.
        let code = unsafe { sys::TEEC_InitializeContext(path, raw.get()) };
        // Only wrap the handle once it is initialised, so that a failed
        // initialisation is not finalised by `Drop`.
        check(code, None)?;
        Ok(Context { raw })
    }

    pub(crate) fn as_ptr(&self) -> *mut sys::TEEC_Context {
        self.raw.get()
    }

    /// Open a session with the trusted application identified by `destination`.
    ///
    /// `login` selects how the client identifies itself and carries the
    /// connection data the method needs. `operation` transports parameters to
    /// the trusted application's session-open handler; pass `None` when it
    /// takes none.
    ///
    /// The returned session borrows the context, so it must be dropped — and
    /// therefore closed — before the context is finalised.
    pub fn open_session(
        &self,
        destination: &Uuid,
        login: Login,
        mut operation: Option<&mut Operation<'_>>,
    ) -> Result<Session<'_>> {
        let destination = destination.to_raw();
        let (method, connection_data) = login.to_raw();
        let operation_ptr = match operation.as_mut() {
            Some(operation) => operation.prepare()?,
            None => std::ptr::null_mut(),
        };
        // SAFETY: TEEC_Session is a plain-old-data C struct; the native API
        // fills it in and expects a zeroed structure at a stable address.
        let raw: Box<UnsafeCell<sys::TEEC_Session>> =
            Box::new(UnsafeCell::new(unsafe { std::mem::zeroed() }));
        let mut origin: u32 = 0;
        // SAFETY: every pointer is either null or points at a live value owned
        // by this frame: the context, the freshly zeroed session, the UUID, the
        // connection data and the prepared operation.
        let code = unsafe {
            sys::TEEC_OpenSession(
                self.as_ptr(),
                raw.get(),
                &destination,
                method,
                connection_data.as_ref().map_or(std::ptr::null(), |group| {
                    std::ptr::from_ref(group).cast::<c_void>()
                }),
                operation_ptr,
                &mut origin,
            )
        };
        if let Some(operation) = operation.as_mut() {
            operation.absorb();
        }
        check(code, Some(origin))?;
        Ok(Session {
            raw,
            _context: self,
        })
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        // SAFETY: the context was successfully initialised and is finalised
        // exactly once. Every session and shared memory block borrowed it, so
        // all of them have already been dropped.
        unsafe { sys::TEEC_FinalizeContext(self.as_ptr()) };
    }
}

/// An open connection to a trusted application.
///
/// Created with [`Context::open_session`] and closed on drop. A session borrows
/// its context, which is what makes "close every session before finalising the
/// context" a compile-time rule rather than a convention.
pub struct Session<'ctx> {
    // Boxed for the same reason as the context: the native structure is linked
    // into the context's session list and points back at the context.
    raw: Box<UnsafeCell<sys::TEEC_Session>>,
    _context: &'ctx Context,
}

impl Session<'_> {
    /// Send a command to the trusted application.
    ///
    /// `command_id` is defined by the trusted application. `operation` carries
    /// the parameters; pass `None` for a command that takes none. Whatever the
    /// trusted application writes back is read from the operation once the call
    /// returns.
    pub fn invoke_command(
        &mut self,
        command_id: u32,
        mut operation: Option<&mut Operation<'_>>,
    ) -> Result<()> {
        let operation_ptr = match operation.as_mut() {
            Some(operation) => operation.prepare()?,
            None => std::ptr::null_mut(),
        };
        let mut origin: u32 = 0;
        // SAFETY: the session is open and owned by `self`, and the operation
        // pointer is either null or points at the prepared operation.
        let code = unsafe {
            sys::TEEC_InvokeCommand(self.raw.get(), command_id, operation_ptr, &mut origin)
        };
        if let Some(operation) = operation.as_mut() {
            operation.absorb();
        }
        check(code, Some(origin))
    }
}

impl Drop for Session<'_> {
    fn drop(&mut self) {
        // SAFETY: the session was successfully opened and is closed exactly
        // once, before its context is finalised.
        unsafe { sys::TEEC_CloseSession(self.raw.get()) };
    }
}
