use std::ffi::c_void;

/// Destination buffer for `OH_ArkWebHttpBodyStream_Read`.
///
/// The NDK fills this allocation from a worker thread. The leftover local
/// `Vec` was dropped when `HttpBodyStream::read` returned, while the worker
/// could still write (UAF). Keep the allocation here until the read callback
/// copies `bytes_read` and the context `Box` is dropped.
pub(crate) struct ReadBufferOwner {
    buf: Vec<u8>,
}

impl ReadBufferOwner {
    pub(crate) fn with_capacity(size: usize) -> Self {
        Self {
            buf: Vec::with_capacity(size),
        }
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut u8 {
        self.buf.as_mut_ptr()
    }

    /// Copy `bytes_read` from the context-owned buffer.
    ///
    /// Length is applied only after the worker reports how many bytes were
    /// written. Copy via `from_raw_parts` so uninitialized capacity is never
    /// exposed as a `Vec` length.
    ///
    /// # Safety
    ///
    /// The first `bytes_read` bytes of this buffer must have been written.
    pub(crate) unsafe fn copy_bytes_read(&self, bytes_read: usize) -> Vec<u8> {
        let n = bytes_read.min(self.buf.capacity());
        if n == 0 {
            Vec::new()
        } else {
            std::slice::from_raw_parts(self.buf.as_ptr(), n).to_vec()
        }
    }
}

/// Per-read user data: callback plus the leftover-buffer owner.
///
/// Leftover `ManuallyDrop` around `Box::from_raw` leaked this `Box` on every
/// read. Reconstruct the `Box` in the callback and drop it normally.
pub(crate) struct ReadCallbackContext {
    callback: Box<dyn FnMut(Vec<u8>) + 'static>,
    buffer: ReadBufferOwner,
}

impl ReadCallbackContext {
    pub(crate) fn new(size: usize, callback: Box<dyn FnMut(Vec<u8>) + 'static>) -> Self {
        Self {
            callback,
            buffer: ReadBufferOwner::with_capacity(size),
        }
    }

    /// Heap-allocate the context and return `(user_data, buffer_ptr)`.
    ///
    /// `buffer_ptr` stays valid until [`finish_read_callback`] drops the `Box`.
    pub(crate) fn into_raw_with_buffer(self) -> (*mut c_void, *mut u8) {
        let mut boxed = Box::new(self);
        let buf_ptr = boxed.buffer.as_mut_ptr();
        (Box::into_raw(boxed) as *mut c_void, buf_ptr)
    }
}

/// Copy `bytes_read` from the owned buffer, invoke the callback, then drop
/// the context `Box` (no `ManuallyDrop`).
///
/// # Safety
///
/// `user_data` must come from [`ReadCallbackContext::into_raw_with_buffer`]
/// and must not have been taken already. The first `bytes_read` bytes of the
/// owned buffer must have been written.
pub(crate) unsafe fn finish_read_callback(user_data: *mut c_void, bytes_read: i32) {
    let mut ctx = Box::from_raw(user_data as *mut ReadCallbackContext);
    let data = ctx.buffer.copy_bytes_read(bytes_read.max(0) as usize);
    (ctx.callback)(data);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::mem::ManuallyDrop;
    use std::rc::Rc;

    struct DropSpy {
        dropped: Rc<Cell<bool>>,
    }

    impl Drop for DropSpy {
        fn drop(&mut self) {
            self.dropped.set(true);
        }
    }

    fn spy_ctx(size: usize) -> (ReadCallbackContext, Rc<Cell<bool>>) {
        let dropped = Rc::new(Cell::new(false));
        let spy = DropSpy {
            dropped: dropped.clone(),
        };
        let ctx = ReadCallbackContext::new(
            size,
            Box::new(move |_| {
                let _keep = &spy;
            }),
        );
        (ctx, dropped)
    }

    /// Stub Read that writes after the caller returns.
    /// Leftover drop of a local `Vec` is UAF; the owner keeps the buffer.
    #[test]
    fn stub_read_writes_after_return() {
        let ctx = ReadCallbackContext::new(4, Box::new(|_| {}));
        let (user_data, buf_ptr) = ctx.into_raw_with_buffer();
        // `read()` has returned. Worker writes into the leftover-buffer owner.
        unsafe {
            std::ptr::copy_nonoverlapping([0xAAu8, 0xBB, 0xCC, 0xDD].as_ptr(), buf_ptr, 4);
            let ctx = &mut *user_data.cast::<ReadCallbackContext>();
            assert_eq!(ctx.buffer.as_mut_ptr(), buf_ptr);
            let data = ctx.buffer.copy_bytes_read(4);
            assert_eq!(data, [0xAA, 0xBB, 0xCC, 0xDD]);
            drop(Box::from_raw(user_data as *mut ReadCallbackContext));
        }
    }

    /// Leftover `ManuallyDrop` around `Box::from_raw` never runs `Drop`.
    #[test]
    fn leftover_manually_drop_skips_ctx_drop() {
        let (ctx, dropped) = spy_ctx(1);
        let (user_data, _) = ctx.into_raw_with_buffer();
        unsafe {
            let _ctx = ManuallyDrop::new(Box::from_raw(user_data as *mut ReadCallbackContext));
        }
        assert!(
            !dropped.get(),
            "leftover ManuallyDrop must skip Drop of ctx"
        );
        unsafe {
            drop(Box::from_raw(user_data as *mut ReadCallbackContext));
        }
        assert!(dropped.get());
    }

    /// Stub callback: dropping the `Box` normally runs leftover `Drop` of ctx.
    #[test]
    fn finish_read_callback_drops_ctx() {
        let (ctx, dropped) = spy_ctx(3);
        let (user_data, buf_ptr) = ctx.into_raw_with_buffer();
        unsafe {
            std::ptr::copy_nonoverlapping([1u8, 2, 3].as_ptr(), buf_ptr, 3);
            finish_read_callback(user_data, 3);
        }
        assert!(
            dropped.get(),
            "ctx Box must drop; leftover ManuallyDrop leaked it"
        );
    }

    #[test]
    fn finish_read_callback_copies_bytes_read() {
        let got = Rc::new(RefCell::new(None));
        let got2 = got.clone();
        let ctx =
            ReadCallbackContext::new(8, Box::new(move |data| *got2.borrow_mut() = Some(data)));
        let (user_data, buf_ptr) = ctx.into_raw_with_buffer();
        unsafe {
            std::ptr::copy_nonoverlapping(b"hello".as_ptr(), buf_ptr, 5);
            finish_read_callback(user_data, 5);
        }
        assert_eq!(got.borrow().as_deref(), Some(&b"hello"[..]));
    }
}
