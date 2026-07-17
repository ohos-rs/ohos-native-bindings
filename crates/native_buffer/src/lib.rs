use std::{
    cell::RefCell, marker::PhantomData, mem::MaybeUninit, os::raw::c_void, ptr::NonNull, rc::Rc,
};

use ohos_native_buffer_sys::{
    OH_NativeBuffer, OH_NativeBuffer_Alloc, OH_NativeBuffer_Config,
    OH_NativeBuffer_FromNativeWindowBuffer, OH_NativeBuffer_GetConfig, OH_NativeBuffer_Map,
    OH_NativeBuffer_Reference, OH_NativeBuffer_Unmap, OH_NativeBuffer_Unreference,
};
use ohos_native_window_sys::OHNativeWindowBuffer;

pub type NativeBufferConfig = OH_NativeBuffer_Config;

mod error;
mod format;

pub use error::*;
pub use format::*;

pub struct NativeBuffer {
    buffer: NonNull<OH_NativeBuffer>,
    config: Rc<RefCell<Option<NativeBufferConfig>>>,
}

/// Owned CPU mapping of a native buffer.
///
/// The mapping is released before the underlying [`NativeBuffer`] reference,
/// so consumers cannot accidentally retain a raw address after unmapping.
pub struct MappedNativeBuffer {
    buffer: NativeBuffer,
    address: NonNull<u8>,
    byte_len: usize,
}

/// Borrowed native buffer with a byte length supplied by its owning API.
pub struct NativeBufferRef<'a> {
    buffer: NonNull<OH_NativeBuffer>,
    byte_len: usize,
    _owner: PhantomData<&'a OH_NativeBuffer>,
}

impl<'a> NativeBufferRef<'a> {
    /// Borrow a native buffer whose owner guarantees `byte_len` readable bytes.
    ///
    /// # Safety
    ///
    /// `buffer` must remain valid for `'a`, and a successful native map must
    /// expose at least `byte_len` bytes.
    pub unsafe fn from_raw_parts(buffer: *mut OH_NativeBuffer, byte_len: usize) -> Option<Self> {
        NonNull::new(buffer).map(|buffer| Self {
            buffer,
            byte_len,
            _owner: PhantomData,
        })
    }

    /// Map the borrowed buffer and unmap it automatically when the guard drops.
    pub fn map(&mut self) -> Result<NativeBufferMap<'_>, NativeBufferError> {
        let mut address = std::ptr::null_mut();
        // SAFETY: this borrowed owner keeps the buffer live and the mutable
        // borrow prevents another map through the same wrapper.
        let code = unsafe { OH_NativeBuffer_Map(self.buffer.as_ptr(), &mut address) };
        if code != 0 {
            return Err(NativeBufferError::InternalError(code));
        }
        let Some(address) = NonNull::new(address.cast::<u8>()) else {
            // SAFETY: a successful map must be paired with an unmap.
            let _ = unsafe { OH_NativeBuffer_Unmap(self.buffer.as_ptr()) };
            return Err(NativeBufferError::InternalError(-1));
        };
        Ok(NativeBufferMap {
            buffer: self.buffer,
            address,
            byte_len: self.byte_len,
            _borrow: PhantomData,
        })
    }
}

/// RAII mapping of a borrowed native buffer.
pub struct NativeBufferMap<'a> {
    buffer: NonNull<OH_NativeBuffer>,
    address: NonNull<u8>,
    byte_len: usize,
    _borrow: PhantomData<&'a mut NativeBufferRef<'a>>,
}

impl NativeBufferMap<'_> {
    pub fn bytes(&self) -> &[u8] {
        // SAFETY: `NativeBufferRef::from_raw_parts` establishes the readable
        // length and this guard keeps the successful map alive.
        unsafe { std::slice::from_raw_parts(self.address.as_ptr(), self.byte_len) }
    }
}

impl Drop for NativeBufferMap<'_> {
    fn drop(&mut self) {
        // SAFETY: paired with the successful map that created this guard.
        let _ = unsafe { OH_NativeBuffer_Unmap(self.buffer.as_ptr()) };
    }
}

impl NativeBuffer {
    pub fn raw(&self) -> *mut OH_NativeBuffer {
        self.buffer.as_ptr()
    }

    pub fn new(config: NativeBufferConfig) -> Self {
        let ret = unsafe { OH_NativeBuffer_Alloc(&config) };
        #[cfg(debug_assertions)]
        assert!(!ret.is_null(), "OH_NativeBuffer_Alloc failed");

        Self {
            buffer: NonNull::new(ret).expect("OH_NativeBuffer_Alloc failed"),
            config: Rc::new(RefCell::new(None)),
        }
    }

    /// create NativeBuffer from OHNativeWindowBuffer
    pub fn from_window_buffer_ptr(buffer: *mut OHNativeWindowBuffer) -> Self {
        Self::try_from_window_buffer_ptr(buffer)
            .expect("OH_NativeBuffer_FromNativeWindowBuffer failed")
    }

    /// Create a native buffer from an OHNativeWindowBuffer.
    pub fn try_from_window_buffer_ptr(
        buffer: *mut OHNativeWindowBuffer,
    ) -> Result<Self, NativeBufferError> {
        if buffer.is_null() {
            return Err(NativeBufferError::InternalError(-1));
        }
        let mut buf = std::ptr::null_mut();
        let ret = unsafe { OH_NativeBuffer_FromNativeWindowBuffer(buffer, &mut buf) };
        if ret != 0 {
            return Err(NativeBufferError::InternalError(ret));
        }

        let buffer = NonNull::new(buf).ok_or(NativeBufferError::InternalError(-1))?;
        // `OH_NativeBuffer_FromNativeWindowBuffer` only exposes the native
        // buffer owned by the dequeued window buffer; unlike allocation and
        // parcel deserialization, it does not acquire a reference for the
        // caller. Acquire one here so this owning wrapper can always balance
        // its Drop without invalidating the window buffer before it is flushed.
        let ret = unsafe { OH_NativeBuffer_Reference(buffer.as_ptr()) };
        if ret != 0 {
            return Err(NativeBufferError::InternalError(ret));
        }

        Ok(Self {
            buffer,
            config: Rc::new(RefCell::new(None)),
        })
    }

    /// Get current buffer config
    pub fn config(&self) -> NativeBufferConfig {
        if self.config.borrow().is_none() {
            let mut config: MaybeUninit<OH_NativeBuffer_Config> = MaybeUninit::uninit();
            let config = unsafe {
                OH_NativeBuffer_GetConfig(self.buffer.as_ptr(), config.as_mut_ptr());
                config.assume_init()
            };
            self.config.replace(Some(config));
        }
        *self
            .config
            .borrow()
            .as_ref()
            .expect("OH_NativeBuffer_GetConfig failed")
    }

    /// Map ION memory to process space
    pub fn mmap(&self) -> NonNull<c_void> {
        let mut ptr = std::ptr::null_mut();
        let ret = unsafe { OH_NativeBuffer_Map(self.buffer.as_ptr(), &mut ptr) };
        assert_eq!(ret, 0, "OH_NativeBuffer_Map failed");
        NonNull::new(ptr).expect("OH_NativeBuffer_Map failed")
    }

    /// Unmap ION memory from process space
    pub fn un_mmap(&self) {
        let ret = unsafe { OH_NativeBuffer_Unmap(self.buffer.as_ptr()) };
        assert_eq!(ret, 0, "OH_NativeBuffer_Unmap failed");
    }

    /// Map this buffer for CPU access and transfer ownership into an RAII
    /// mapping guard.
    pub fn map_owned(self) -> Result<MappedNativeBuffer, NativeBufferError> {
        let byte_len = mapped_byte_len(self.config())?;
        let mut address = std::ptr::null_mut();
        let code = unsafe { OH_NativeBuffer_Map(self.buffer.as_ptr(), &mut address) };
        if code != 0 {
            return Err(NativeBufferError::InternalError(code));
        }
        let Some(address) = NonNull::new(address.cast::<u8>()) else {
            let _ = unsafe { OH_NativeBuffer_Unmap(self.buffer.as_ptr()) };
            return Err(NativeBufferError::InternalError(-1));
        };
        Ok(MappedNativeBuffer {
            buffer: self,
            address,
            byte_len,
        })
    }
}

impl MappedNativeBuffer {
    pub fn config(&self) -> NativeBufferConfig {
        self.buffer.config()
    }

    pub fn bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.address.as_ptr(), self.byte_len) }
    }

    pub fn bytes_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.address.as_ptr(), self.byte_len) }
    }
}

impl Drop for MappedNativeBuffer {
    fn drop(&mut self) {
        let _ = unsafe { OH_NativeBuffer_Unmap(self.buffer.buffer.as_ptr()) };
    }
}

fn mapped_byte_len(config: NativeBufferConfig) -> Result<usize, NativeBufferError> {
    let height = usize::try_from(config.height)
        .ok()
        .filter(|height| *height > 0)
        .ok_or(NativeBufferError::InternalError(-1))?;
    let stride = usize::try_from(config.stride)
        .ok()
        .filter(|stride| *stride > 0)
        .ok_or(NativeBufferError::InternalError(-1))?;
    stride
        .checked_mul(height)
        .ok_or(NativeBufferError::InternalError(-1))
}

impl Clone for NativeBuffer {
    fn clone(&self) -> Self {
        let ret = unsafe { OH_NativeBuffer_Reference(self.buffer.as_ptr()) };
        assert_eq!(ret, 0, "OH_NativeBuffer_Reference failed");
        Self {
            buffer: self.buffer,
            config: self.config.clone(),
        }
    }
}

impl Drop for NativeBuffer {
    fn drop(&mut self) {
        let _ = unsafe { OH_NativeBuffer_Unreference(self.buffer.as_ptr()) };
    }
}
