use crate::error::{ChildProcessError, Result};
use ohos_child_process_sys as sys;
use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw::c_char;
use std::ptr;

/// Isolation mode of a child process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IsolationMode {
    /// The parent process shares its sandbox and network with the child
    /// process.
    #[default]
    Normal,
    /// The child process gets its own sandbox and network.
    Isolated,
}

impl IsolationMode {
    pub(crate) fn to_raw(self) -> sys::NativeChildProcess_IsolationMode {
        match self {
            IsolationMode::Normal => {
                sys::NativeChildProcess_IsolationMode_NCP_ISOLATION_MODE_NORMAL
            }
            IsolationMode::Isolated => {
                sys::NativeChildProcess_IsolationMode_NCP_ISOLATION_MODE_ISOLATED
            }
        }
    }
}

/// Options applied to a child process started with
/// [`start_child_process`](crate::start_child_process).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChildProcessOptions {
    isolation_mode: IsolationMode,
}

impl ChildProcessOptions {
    /// Options using the given isolation mode.
    pub fn new(isolation_mode: IsolationMode) -> Self {
        Self { isolation_mode }
    }

    /// The isolation mode these options request.
    pub fn isolation_mode(&self) -> IsolationMode {
        self.isolation_mode
    }

    /// Set the isolation mode.
    pub fn set_isolation_mode(&mut self, isolation_mode: IsolationMode) {
        self.isolation_mode = isolation_mode;
    }

    pub(crate) fn to_raw(self) -> sys::NativeChildProcess_Options {
        sys::NativeChildProcess_Options {
            isolationMode: self.isolation_mode.to_raw(),
            reserved: 0,
        }
    }
}

/// Arguments handed to the entry function of a child process: a free-form
/// parameter string and a set of named file descriptors.
///
/// The child process reads them back with
/// [`current_child_process_args`](crate::current_child_process_args).
///
/// ```no_run
/// use ohos_child_process_binding::ChildProcessArgs;
///
/// let args = ChildProcessArgs::new()
///     .with_entry_params("--mode=worker")?
///     .with_fd("log", 3)?;
/// # Ok::<(), ohos_child_process_binding::ChildProcessError>(())
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChildProcessArgs {
    entry_params: Option<CString>,
    fds: Vec<(CString, i32)>,
}

impl ChildProcessArgs {
    /// Empty arguments: no entry parameter and no file descriptor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the entry parameter string.
    ///
    /// Returns [`ChildProcessError::InteriorNul`] if the string contains a NUL
    /// byte.
    pub fn set_entry_params(&mut self, entry_params: &str) -> Result<()> {
        self.entry_params =
            Some(CString::new(entry_params).map_err(|_| ChildProcessError::InteriorNul)?);
        Ok(())
    }

    /// Builder form of [`set_entry_params`](Self::set_entry_params).
    pub fn with_entry_params(mut self, entry_params: &str) -> Result<Self> {
        self.set_entry_params(entry_params)?;
        Ok(self)
    }

    /// Drop the entry parameter string.
    pub fn clear_entry_params(&mut self) {
        self.entry_params = None;
    }

    /// The entry parameter string, if one is set.
    pub fn entry_params(&self) -> Option<Cow<'_, str>> {
        self.entry_params.as_deref().map(CStr::to_string_lossy)
    }

    /// Pass a file descriptor to the child process under `name`.
    ///
    /// The descriptor stays owned by the caller; it must be kept open until the
    /// child process has been started. Returns
    /// [`ChildProcessError::InteriorNul`] if `name` contains a NUL byte.
    pub fn add_fd(&mut self, name: &str, fd: i32) -> Result<()> {
        let name = CString::new(name).map_err(|_| ChildProcessError::InteriorNul)?;
        self.fds.push((name, fd));
        Ok(())
    }

    /// Builder form of [`add_fd`](Self::add_fd).
    pub fn with_fd(mut self, name: &str, fd: i32) -> Result<Self> {
        self.add_fd(name, fd)?;
        Ok(self)
    }

    /// The named file descriptors, in insertion order.
    pub fn fds(&self) -> impl Iterator<Item = (Cow<'_, str>, i32)> {
        self.fds
            .iter()
            .map(|(name, fd)| (name.to_string_lossy(), *fd))
    }

    /// Number of named file descriptors.
    pub fn fd_count(&self) -> usize {
        self.fds.len()
    }

    /// Build the C view of these arguments. The returned value borrows the
    /// strings owned by `self` and must outlive the native call.
    pub(crate) fn to_raw(&self) -> RawArgs<'_> {
        let mut nodes: Vec<sys::NativeChildProcess_Fd> = self
            .fds
            .iter()
            .map(|(name, fd)| sys::NativeChildProcess_Fd {
                fdName: name.as_ptr() as *mut c_char,
                fd: *fd,
                next: ptr::null_mut(),
            })
            .collect();

        let base = nodes.as_mut_ptr();
        // Link the nodes through the stable heap buffer of `nodes`, which the
        // returned `RawArgs` keeps alive and never reallocates.
        for index in 0..nodes.len().saturating_sub(1) {
            unsafe { (*base.add(index)).next = base.add(index + 1) };
        }
        let head = if nodes.is_empty() {
            ptr::null_mut()
        } else {
            base
        };

        let args = sys::NativeChildProcess_Args {
            entryParams: self
                .entry_params
                .as_ref()
                .map_or(ptr::null_mut(), |params| params.as_ptr() as *mut c_char),
            fdList: sys::NativeChildProcess_FdList { head },
        };

        RawArgs {
            _nodes: nodes,
            args,
            _owner: PhantomData,
        }
    }

    /// Copy arguments out of the C representation owned by the runtime.
    ///
    /// # Safety
    ///
    /// `raw` must point to a valid `NativeChildProcess_Args` whose strings and
    /// descriptor list stay valid for the duration of the call.
    #[cfg(feature = "api-17")]
    pub(crate) unsafe fn from_raw(raw: &sys::NativeChildProcess_Args) -> Self {
        let entry_params = if raw.entryParams.is_null() {
            None
        } else {
            Some(CStr::from_ptr(raw.entryParams).to_owned())
        };

        let mut fds = Vec::new();
        let mut node = raw.fdList.head;
        while !node.is_null() {
            let current = &*node;
            if !current.fdName.is_null() {
                fds.push((CStr::from_ptr(current.fdName).to_owned(), current.fd));
            }
            node = current.next;
        }

        Self { entry_params, fds }
    }
}

/// Keeps the C linked list of file descriptors alive for the duration of a
/// native call.
///
/// `NativeChildProcess_Args` points at the entry parameter string owned by the
/// [`ChildProcessArgs`] it was built from, so the lifetime keeps this view from
/// outliving that owner.
pub(crate) struct RawArgs<'a> {
    _nodes: Vec<sys::NativeChildProcess_Fd>,
    args: sys::NativeChildProcess_Args,
    _owner: PhantomData<&'a ChildProcessArgs>,
}

impl RawArgs<'_> {
    /// The raw view. It borrows `self` and must not outlive the call it is
    /// passed to.
    pub(crate) fn args(&self) -> sys::NativeChildProcess_Args {
        self.args
    }
}
