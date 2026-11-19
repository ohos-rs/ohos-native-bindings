//! Synchronous HiTrace scopes and counters on the calling thread.
use std::{ffi::CStr, marker::PhantomData, rc::Rc};

/// A trace section ending on the same thread where it began.
#[must_use]
pub struct Scope(PhantomData<Rc<()>>);

impl Scope {
    /// Starts a named trace section. The C API consumes the name synchronously.
    pub fn new(name: &CStr) -> Self {
        unsafe { ohos_hitrace_sys::OH_HiTrace_StartTrace(name.as_ptr()) };
        Self(PhantomData)
    }
}

impl Drop for Scope {
    fn drop(&mut self) {
        unsafe { ohos_hitrace_sys::OH_HiTrace_FinishTrace() };
    }
}

/// Records one named counter sample.
pub fn count(name: &CStr, value: i64) {
    unsafe { ohos_hitrace_sys::OH_HiTrace_CountTrace(name.as_ptr(), value) };
}
