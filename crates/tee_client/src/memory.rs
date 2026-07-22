use crate::context::Context;
use crate::error::{check, Result, TeeError};
use crate::param::SharedMemoryRef;
use crate::types::Direction;
use ohos_tee_client_sys as sys;
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::os::raw::c_void;
use std::slice;

/// A block of memory shared between the client application and the TEE.
///
/// A block is either *allocated* by the TEE — [`SharedMemory::allocate`] — or
/// *registered* from memory the caller already owns —
/// [`SharedMemory::register`]. Both are released on drop, which reclaims an
/// allocated block and un-registers a registered one.
///
/// A block borrows its context, so it cannot outlive it; a registered block
/// also holds the borrow of the buffer it was built from, so that buffer cannot
/// be touched or freed while the TEE may still reach it.
pub struct SharedMemory<'ctx> {
    // Boxed for a stable address: the native structure is linked into the
    // context's shared memory list and points back at the context.
    raw: Box<UnsafeCell<sys::TEEC_SharedMemory>>,
    _context: &'ctx Context,
    _buffer: PhantomData<&'ctx mut [u8]>,
}

impl<'ctx> SharedMemory<'ctx> {
    /// Ask the TEE to allocate a block of `size` bytes.
    ///
    /// `direction` states which way data flows through the block; the trusted
    /// application may only use it accordingly.
    pub fn allocate(context: &'ctx Context, size: usize, direction: Direction) -> Result<Self> {
        let size = u32::try_from(size).map_err(|_| {
            TeeError::invalid_argument("shared memory size does not fit in a 32-bit size")
        })?;
        let raw = Self::empty_raw();
        {
            // SAFETY: the block is owned by this frame and not yet handed to
            // the native API, so no other reference to it exists.
            let fields = unsafe { &mut *raw.get() };
            fields.size = size;
            fields.flags = direction.shared_memory_flag();
        }
        // SAFETY: the context is initialised and the block is a zeroed
        // structure with its size and flags set, as the call expects.
        let code = unsafe { sys::TEEC_AllocateSharedMemory(context.as_ptr(), raw.get()) };
        // Only wrap the block once it is allocated, so that a failed allocation
        // is not released by `Drop`.
        check(code, None)?;
        Ok(SharedMemory {
            raw,
            _context: context,
            _buffer: PhantomData,
        })
    }

    /// Register memory the caller owns with the TEE.
    ///
    /// The buffer stays borrowed for as long as the block lives, so it cannot
    /// be moved, reused or freed while the TEE holds a reference to it.
    pub fn register(
        context: &'ctx Context,
        buffer: &'ctx mut [u8],
        direction: Direction,
    ) -> Result<Self> {
        let size = u32::try_from(buffer.len()).map_err(|_| {
            TeeError::invalid_argument("shared memory size does not fit in a 32-bit size")
        })?;
        let address = buffer.as_mut_ptr();
        let raw = Self::empty_raw();
        {
            // SAFETY: the block is owned by this frame and not yet handed to
            // the native API, so no other reference to it exists.
            let fields = unsafe { &mut *raw.get() };
            fields.buffer = address.cast::<c_void>();
            fields.size = size;
            fields.flags = direction.shared_memory_flag();
        }
        // SAFETY: the context is initialised and the block describes the
        // caller's buffer, which stays borrowed for the lifetime of the block.
        let code = unsafe { sys::TEEC_RegisterSharedMemory(context.as_ptr(), raw.get()) };
        // Only wrap the block once it is registered, so that a failed
        // registration is not released by `Drop`.
        check(code, None)?;
        Ok(SharedMemory {
            raw,
            _context: context,
            _buffer: PhantomData,
        })
    }

    fn empty_raw() -> Box<UnsafeCell<sys::TEEC_SharedMemory>> {
        // SAFETY: TEEC_SharedMemory is a plain-old-data C struct; an all-zero
        // value is what the native API expects to be handed.
        Box::new(UnsafeCell::new(unsafe { std::mem::zeroed() }))
    }

    fn parts(&self) -> (*mut u8, usize) {
        // SAFETY: the block is initialised and only read here; no other
        // reference to the native structure is alive.
        let raw = unsafe { &*self.raw.get() };
        if raw.buffer.is_null() {
            (std::ptr::NonNull::<u8>::dangling().as_ptr(), 0)
        } else {
            (raw.buffer.cast::<u8>(), raw.size as usize)
        }
    }

    /// The size of the block, in bytes.
    pub fn len(&self) -> usize {
        self.parts().1
    }

    /// Whether the block is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The direction the block was created with.
    pub fn direction(&self) -> Option<Direction> {
        // SAFETY: the block is initialised and only read here.
        let raw = unsafe { &*self.raw.get() };
        Direction::from_shared_memory_flag(raw.flags)
    }

    /// Whether the TEE allocated the block, as opposed to it being registered
    /// from memory the caller owns.
    pub fn is_allocated(&self) -> bool {
        // SAFETY: the block is initialised and only read here.
        unsafe { &*self.raw.get() }.is_allocated
    }

    /// The content of the block.
    pub fn as_slice(&self) -> &[u8] {
        let (address, len) = self.parts();
        // SAFETY: the block owns `len` bytes at `address` until it is released,
        // and `&self` rules out a concurrent operation writing into it, since
        // handing the block to an operation takes an exclusive borrow.
        unsafe { slice::from_raw_parts(address, len) }
    }

    /// The content of the block, mutably.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        let (address, len) = self.parts();
        // SAFETY: as above, and the exclusive borrow rules out any other view
        // of the content.
        unsafe { slice::from_raw_parts_mut(address, len) }
    }

    /// Borrow the block so it can be placed in a
    /// [`Parameter`](crate::Parameter).
    ///
    /// The borrow is exclusive for the lifetime of the parameter, which is what
    /// keeps the content from being read while the trusted application writes
    /// into it.
    pub fn as_param(&mut self) -> SharedMemoryRef<'_> {
        SharedMemoryRef::new(self.raw.get())
    }
}

impl Drop for SharedMemory<'_> {
    fn drop(&mut self) {
        // SAFETY: the block was successfully registered or allocated and is
        // released exactly once, before its context is finalised.
        unsafe { sys::TEEC_ReleaseSharedMemory(self.raw.get()) };
    }
}
