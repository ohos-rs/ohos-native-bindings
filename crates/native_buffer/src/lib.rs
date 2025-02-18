use std::{cell::RefCell, mem::MaybeUninit, os::raw::c_void, ptr::NonNull, rc::Rc};

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
        let mut buf = std::ptr::null_mut();
        let ret = unsafe { OH_NativeBuffer_FromNativeWindowBuffer(buffer, &mut buf) };
        assert!(ret == 0, "OH_NativeBuffer_FromNativeWindowBuffer failed");

        Self {
            buffer: NonNull::new(buf).expect("OHNativeWindowBuffer is null"),
            config: Rc::new(RefCell::new(None)),
        }
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
        self.config
            .borrow()
            .as_ref()
            .expect("OH_NativeBuffer_GetConfig failed")
            .clone()
    }

    /// Map ION memory to process space
    pub fn mmap(&self) -> NonNull<c_void> {
        let mut ptr = std::ptr::null_mut();
        let ret = unsafe { OH_NativeBuffer_Map(self.buffer.as_ptr(), &mut ptr) };
        #[cfg(debug_assertions)]
        assert!(ret == 0, "OH_NativeBuffer_Map failed");
        NonNull::new(ptr).expect("OH_NativeBuffer_Map failed")
    }

    /// Unmap ION memory from process space
    pub fn un_mmap(&self) {
        let ret = unsafe { OH_NativeBuffer_Unmap(self.buffer.as_ptr()) };
        #[cfg(debug_assertions)]
        assert!(ret == 0, "OH_NativeBuffer_Unmap failed");
    }
}

impl Clone for NativeBuffer {
    fn clone(&self) -> Self {
        let ret = unsafe { OH_NativeBuffer_Reference(self.buffer.as_ptr()) };
        #[cfg(debug_assertions)]
        assert!(ret == 0, "OH_NativeBuffer_Reference failed");
        Self {
            buffer: self.buffer,
            config: self.config.clone(),
        }
    }
}

impl Drop for NativeBuffer {
    fn drop(&mut self) {
        let ret = unsafe { OH_NativeBuffer_Unreference(self.buffer.as_ptr()) };
        #[cfg(debug_assertions)]
        assert!(ret == 0, "OH_NativeBuffer_Release failed");
    }
}
