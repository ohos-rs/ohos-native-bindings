//! Safe Rust bindings for OpenHarmony **purgeable memory**.
//!
//! A purgeable memory object owns a block of content that the system is free to
//! reclaim under memory pressure whenever the application is not actively using
//! it. To use the content, the application first asks for an access permit
//! (begin read / begin write); while the permit is held the system cannot
//! reclaim the block, and once the permit is released it may. If the content
//! was already reclaimed, the object rebuilds it from the rebuild function
//! supplied at construction time, plus every modification appended afterwards.
//!
//! This crate maps that protocol onto RAII guards: [`PurgeableMemory::read`]
//! and [`PurgeableMemory::write`] acquire the permit and hand out a
//! [`ReadGuard`] / [`WriteGuard`] that derefs to the content bytes and ends the
//! access on drop, so the content cannot be touched outside a permit and a
//! permit cannot be leaked by an early return. The object itself is freed on
//! drop.
//!
//! The whole API is available since API 10, so no `api-*` feature is required.
//! The raw bindings are re-exported as [`sys`].
//!
//! # Example
//!
//! ```no_run
//! use ohos_purgeable_memory_binding::{PurgeableMemory, PurgeableMemoryError};
//!
//! // The rebuild function fills the block whenever the system has reclaimed it.
//! // It runs while the object is locked, so it must not touch the object.
//! let mut mem = PurgeableMemory::new(4096, |content: &mut [u8]| {
//!     content.fill(0xAB);
//!     true
//! })?;
//!
//! // Exclusive access; the permit ends when the guard is dropped.
//! {
//!     let mut content = mem.write()?;
//!     content[0] = 1;
//! }
//!
//! // Keep the write above across rebuilds.
//! mem.append_modify(|content: &mut [u8]| {
//!     content[0] = 1;
//!     true
//! })?;
//!
//! // Shared access.
//! let content = mem.read()?;
//! assert_eq!(content[0], 1);
//! # Ok::<(), PurgeableMemoryError>(())
//! ```

pub use ohos_purgeable_memory_sys as sys;

mod error;

pub use error::{PurgeableMemoryError, Result};

use std::ops::{Deref, DerefMut};
use std::os::raw::c_void;
use std::panic::{self, AssertUnwindSafe};
use std::ptr::NonNull;
use std::slice;

/// A function that fills the content of a purgeable memory object.
///
/// It is handed the whole content block and returns whether it filled it
/// successfully. The system runs it from within a begin-access call while the
/// object is locked, so it must not call back into the same object.
///
/// The function is kept alive by the object it was registered on, hence the
/// `'static` bound; capture owned data or an [`std::rc::Rc`] / [`std::sync::Arc`]
/// handle instead of a borrow.
type ContentBuilder = dyn Fn(&mut [u8]) -> bool + 'static;

/// A registered content builder, kept behind a stable heap address so that the
/// native side can hold a pointer to it for the lifetime of the object.
struct Builder {
    func: Box<ContentBuilder>,
}

/// Trampoline handed to the native API as `OH_PurgeableMemory_ModifyFunc`.
///
/// `para` is the `*mut Builder` registered together with this function pointer.
unsafe extern "C" fn build_content(data: *mut c_void, size: usize, para: *mut c_void) -> bool {
    // SAFETY: `para` is the pointer to a `Builder` that was registered with this
    // trampoline and is kept alive until after the object is destroyed. Only
    // shared references to it are ever created.
    let Some(builder) = (unsafe { para.cast::<Builder>().as_ref() }) else {
        return false;
    };
    let mut empty: [u8; 0] = [];
    let content: &mut [u8] = if data.is_null() || size == 0 {
        &mut empty
    } else {
        // SAFETY: the native side passes the start address of the content block
        // and its size, and holds the object locked for the duration of the
        // call, so no other access to the block can be alive.
        unsafe { slice::from_raw_parts_mut(data.cast::<u8>(), size) }
    };
    // A panic must not unwind into C; report a failed build instead.
    panic::catch_unwind(AssertUnwindSafe(|| (builder.func)(content))).unwrap_or(false)
}

/// A block of memory whose content the system may reclaim when it is not in use.
///
/// Create one with [`PurgeableMemory::new`], then access its content through
/// [`read`](Self::read) or [`write`](Self::write). The object is destroyed on
/// drop.
///
/// The handle is neither `Send` nor `Sync`: the native header documents nothing
/// about using one object from several threads, so it stays on the thread that
/// created it.
pub struct PurgeableMemory {
    handle: NonNull<sys::OH_PurgeableMemory>,
    /// The rebuild function plus every appended modification. The native object
    /// keeps raw pointers to these, so they are freed only after it is
    /// destroyed.
    builders: Vec<*mut Builder>,
}

impl PurgeableMemory {
    /// Create an object holding `size` bytes of purgeable content.
    ///
    /// `rebuild` is the function the system calls to fill the block, both
    /// initially and every time the content has been reclaimed and is needed
    /// again. Returning `false` from it makes the pending begin-access call
    /// fail with [`PurgeableMemoryError::ContentPurged`].
    pub fn new<F>(size: usize, rebuild: F) -> Result<Self>
    where
        F: Fn(&mut [u8]) -> bool + 'static,
    {
        let builder = Box::into_raw(Box::new(Builder {
            func: Box::new(rebuild),
        }));
        // SAFETY: `builder` points at a live `Builder`; it is released below if
        // the object is not created, and in `Drop` otherwise.
        let raw = unsafe {
            sys::OH_PurgeableMemory_Create(size, Some(build_content), builder.cast::<c_void>())
        };
        match NonNull::new(raw) {
            Some(handle) => Ok(Self {
                handle,
                builders: vec![builder],
            }),
            None => {
                // SAFETY: the native side did not take the pointer, and nothing
                // else holds it.
                drop(unsafe { Box::from_raw(builder) });
                Err(PurgeableMemoryError::Create)
            }
        }
    }

    /// The size of the content block, in bytes.
    pub fn len(&self) -> usize {
        // SAFETY: the handle is non-null and valid for the lifetime of `self`.
        unsafe { sys::OH_PurgeableMemory_ContentSize(self.handle.as_ptr()) }
    }

    /// Whether the content block is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Acquire shared access to the content.
    ///
    /// If the content has been reclaimed, the system rebuilds it first. The
    /// system cannot reclaim the block while the returned guard is alive.
    ///
    /// Returns [`PurgeableMemoryError::ContentPurged`] when the content was
    /// reclaimed and could not be rebuilt.
    pub fn read(&self) -> Result<ReadGuard<'_>> {
        // SAFETY: the handle is non-null and valid for the lifetime of `self`.
        if !unsafe { sys::OH_PurgeableMemory_BeginRead(self.handle.as_ptr()) } {
            return Err(PurgeableMemoryError::ContentPurged);
        }
        let (ptr, len) = self.content();
        Ok(ReadGuard {
            memory: self,
            ptr,
            len,
        })
    }

    /// Acquire exclusive access to the content.
    ///
    /// If the content has been reclaimed, the system rebuilds it first. The
    /// system cannot reclaim the block while the returned guard is alive.
    ///
    /// A write performed through the guard is lost if the content is later
    /// reclaimed; use [`append_modify`](Self::append_modify) to make it part of
    /// the rebuilt content.
    ///
    /// Returns [`PurgeableMemoryError::ContentPurged`] when the content was
    /// reclaimed and could not be rebuilt.
    pub fn write(&mut self) -> Result<WriteGuard<'_>> {
        // SAFETY: the handle is non-null and valid for the lifetime of `self`.
        if !unsafe { sys::OH_PurgeableMemory_BeginWrite(self.handle.as_ptr()) } {
            return Err(PurgeableMemoryError::ContentPurged);
        }
        let (ptr, len) = self.content();
        Ok(WriteGuard {
            memory: self,
            ptr,
            len,
        })
    }

    /// Append a modification to the content.
    ///
    /// The modification is applied to the current content and replayed after
    /// every rebuild, on top of the function given to [`new`](Self::new) and of
    /// the modifications appended before it. Use it for changes that must
    /// survive the content being reclaimed.
    pub fn append_modify<F>(&mut self, modify: F) -> Result<()>
    where
        F: Fn(&mut [u8]) -> bool + 'static,
    {
        let builder = Box::into_raw(Box::new(Builder {
            func: Box::new(modify),
        }));
        // SAFETY: the handle is valid and `builder` points at a live `Builder`,
        // released below if the native side did not take it.
        let appended = unsafe {
            sys::OH_PurgeableMemory_AppendModify(
                self.handle.as_ptr(),
                Some(build_content),
                builder.cast::<c_void>(),
            )
        };
        if appended {
            self.builders.push(builder);
            Ok(())
        } else {
            // SAFETY: the append failed, so nothing else holds the pointer.
            drop(unsafe { Box::from_raw(builder) });
            Err(PurgeableMemoryError::AppendModify)
        }
    }

    /// Start address and size of the content block. Only meaningful while an
    /// access permit is held; a null address is reported as an empty block.
    fn content(&self) -> (*mut u8, usize) {
        // SAFETY: the handle is non-null and valid for the lifetime of `self`.
        let ptr = unsafe { sys::OH_PurgeableMemory_GetContent(self.handle.as_ptr()) };
        if ptr.is_null() {
            (std::ptr::NonNull::<u8>::dangling().as_ptr(), 0)
        } else {
            (ptr.cast::<u8>(), self.len())
        }
    }
}

impl Drop for PurgeableMemory {
    fn drop(&mut self) {
        // SAFETY: the handle is valid and destroyed exactly once. The native
        // object no longer references the builders afterwards, so they can be
        // released.
        unsafe { sys::OH_PurgeableMemory_Destroy(self.handle.as_ptr()) };
        for builder in self.builders.drain(..) {
            // SAFETY: each pointer came from `Box::into_raw` and is freed once.
            drop(unsafe { Box::from_raw(builder) });
        }
    }
}

/// Shared access to the content of a [`PurgeableMemory`].
///
/// The system cannot reclaim the content while this guard is alive; dropping it
/// ends the access and allows reclaiming again.
pub struct ReadGuard<'a> {
    memory: &'a PurgeableMemory,
    ptr: *mut u8,
    len: usize,
}

impl ReadGuard<'_> {
    /// The content bytes.
    pub fn as_slice(&self) -> &[u8] {
        // SAFETY: the access permit is held for as long as this guard lives, so
        // the block stays present at `ptr` with `len` bytes.
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl Deref for ReadGuard<'_> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Drop for ReadGuard<'_> {
    fn drop(&mut self) {
        // SAFETY: a read access was successfully begun on this handle and is
        // ended exactly once.
        unsafe { sys::OH_PurgeableMemory_EndRead(self.memory.handle.as_ptr()) };
    }
}

/// Exclusive access to the content of a [`PurgeableMemory`].
///
/// The system cannot reclaim the content while this guard is alive; dropping it
/// ends the access and allows reclaiming again.
pub struct WriteGuard<'a> {
    memory: &'a mut PurgeableMemory,
    ptr: *mut u8,
    len: usize,
}

impl WriteGuard<'_> {
    /// The content bytes.
    pub fn as_slice(&self) -> &[u8] {
        // SAFETY: the access permit is held for as long as this guard lives, so
        // the block stays present at `ptr` with `len` bytes.
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }

    /// The content bytes, mutably.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        // SAFETY: as above, and the guard borrows the object exclusively, so no
        // other access to the block is alive.
        unsafe { slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl Deref for WriteGuard<'_> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl DerefMut for WriteGuard<'_> {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

impl Drop for WriteGuard<'_> {
    fn drop(&mut self) {
        // SAFETY: a write access was successfully begun on this handle and is
        // ended exactly once.
        unsafe { sys::OH_PurgeableMemory_EndWrite(self.memory.handle.as_ptr()) };
    }
}
