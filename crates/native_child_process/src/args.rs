use std::{
    ffi::{CStr, CString},
    os::fd::{AsRawFd, BorrowedFd},
    ptr,
};

use crate::{sys, NativeChildProcessError as Error, Result};

/// Parent-side launch arguments. Strings are owned; descriptors are borrowed
/// until the synchronous start call finishes. Native transfers the descriptors
/// to the child; this wrapper neither duplicates nor closes the parent's FDs.
#[derive(Debug, Default)]
pub struct ChildProcessArgs<'fd> {
    params: CString,
    fds: Vec<(CString, BorrowedFd<'fd>)>,
}

impl<'fd> ChildProcessArgs<'fd> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_entry_params(&mut self, params: &str) -> Result<&mut Self> {
        self.params = CString::new(params)?;
        Ok(self)
    }

    /// Adds a named FD. Native validates names; the documented list limit is 16.
    pub fn add_fd(&mut self, name: &str, fd: BorrowedFd<'fd>) -> Result<&mut Self> {
        if self.fds.len() == 16 {
            return Err(Error::TooManyFileDescriptors);
        }
        self.fds.push((CString::new(name)?, fd));
        Ok(self)
    }

    pub fn entry_params(&self) -> &CStr {
        &self.params
    }

    pub fn fd_count(&self) -> usize {
        self.fds.len()
    }
}

// Prepare the C list only for the call. Nodes live in one Vec allocation and
// are linked after allocation completes, so moving this owner preserves links.
pub(crate) struct PreparedArgs<'args, 'fd> {
    args: &'args ChildProcessArgs<'fd>,
    nodes: Vec<sys::NativeChildProcess_Fd>,
}

impl<'args, 'fd> PreparedArgs<'args, 'fd> {
    pub(crate) fn new(args: &'args ChildProcessArgs<'fd>) -> Self {
        let mut nodes: Vec<_> = args
            .fds
            .iter()
            .map(|(name, fd)| sys::NativeChildProcess_Fd {
                fdName: name.as_ptr().cast_mut(),
                fd: fd.as_raw_fd(),
                next: ptr::null_mut(),
            })
            .collect();
        let base = nodes.as_mut_ptr();
        for index in 0..nodes.len().saturating_sub(1) {
            // SAFETY: Both indices are in the fully allocated Vec. Linking
            // through its raw base avoids mutable slice reborrows invalidating
            // previously stored node pointers. The Vec is never resized.
            unsafe { (*base.add(index)).next = base.add(index + 1) };
        }
        Self { args, nodes }
    }

    pub(crate) fn raw(&mut self) -> sys::NativeChildProcess_Args {
        sys::NativeChildProcess_Args {
            entryParams: self.args.params.as_ptr().cast_mut(),
            fdList: sys::NativeChildProcess_FdList {
                head: if self.nodes.is_empty() {
                    ptr::null_mut()
                } else {
                    self.nodes.as_mut_ptr()
                },
            },
        }
    }
}

/// A named descriptor borrowed from native child arguments.
#[derive(Debug, Clone, Copy)]
pub struct ChildProcessFd<'a> {
    pub name: &'a CStr,
    pub fd: BorrowedFd<'a>,
}

/// Read-only view of system-owned arguments. Clone a borrowed FD to obtain an
/// independent owner; this view never adopts or closes native descriptors.
#[derive(Debug, Clone, Copy)]
pub struct ChildProcessArgsRef<'a> {
    raw: &'a sys::NativeChildProcess_Args,
}

impl<'a> ChildProcessArgsRef<'a> {
    /// Borrows argument storage supplied by the native runtime.
    ///
    /// # Safety
    /// For `'a`, all non-null pointers must refer to valid, immutable native
    /// storage. Strings must be NUL-terminated, the FD list finite and acyclic,
    /// FD names non-null, and every FD valid and kept open. The caller must not
    /// free or modify the argument storage or close its FDs during that borrow.
    pub unsafe fn from_raw(raw: &'a sys::NativeChildProcess_Args) -> Self {
        Self { raw }
    }

    pub fn entry_params(&self) -> Option<&'a CStr> {
        if self.raw.entryParams.is_null() {
            None
        } else {
            // SAFETY: The constructor guarantees the string's lifetime and format.
            Some(unsafe { CStr::from_ptr(self.raw.entryParams) })
        }
    }

    pub fn fds(&self) -> impl Iterator<Item = ChildProcessFd<'a>> + 'a {
        let mut next = self.raw.fdList.head;
        std::iter::from_fn(move || {
            if next.is_null() {
                return None;
            }
            // SAFETY: The constructor guarantees valid immutable list nodes,
            // names and live descriptors for the entire view lifetime.
            let node = unsafe { &*next };
            next = node.next;
            Some(ChildProcessFd {
                name: unsafe { CStr::from_ptr(node.fdName) },
                fd: unsafe { BorrowedFd::borrow_raw(node.fd) },
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, os::fd::AsFd};

    #[test]
    fn prepared_list_survives_move_and_borrows_original_descriptors() {
        let file = File::open("/dev/null").unwrap();
        let original = file.as_raw_fd();
        let mut args = ChildProcessArgs::new();
        args.set_entry_params("payload")
            .unwrap()
            .add_fd("first", file.as_fd())
            .unwrap()
            .add_fd("second", file.as_fd())
            .unwrap();
        let mut prepared = PreparedArgs::new(&args);
        let first = prepared.raw().fdList.head;
        let mut moved = prepared;
        let raw = moved.raw();
        assert_eq!(raw.fdList.head, first);
        {
            // SAFETY: Prepared storage and the original descriptor remain live
            // for all uses of the view and the descriptors collected from it.
            let view = unsafe { ChildProcessArgsRef::from_raw(&raw) };
            assert_eq!(view.entry_params(), Some(c"payload"));
            let fds: Vec<_> = view.fds().collect();
            assert_eq!(fds.len(), 2);
            assert_eq!(fds[0].name, c"first");
            assert_eq!(fds[1].name, c"second");
            assert_eq!(fds[0].fd.as_raw_fd(), original);
            assert_eq!(fds[1].fd.as_raw_fd(), original);
        }
        drop(moved);
        drop(args);
        assert_eq!(file.metadata().unwrap().len(), 0);
    }

    #[test]
    fn empty_arguments_have_a_terminated_string_and_null_list() {
        let args = ChildProcessArgs::new();
        let mut prepared = PreparedArgs::new(&args);
        let raw = prepared.raw();
        assert!(raw.fdList.head.is_null());
        // SAFETY: Prepared storage stays live while viewed.
        let view = unsafe { ChildProcessArgsRef::from_raw(&raw) };
        assert_eq!(view.entry_params(), Some(c""));
        assert_eq!(view.fds().count(), 0);
    }
}
