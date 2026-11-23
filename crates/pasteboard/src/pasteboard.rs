use std::{
    ffi::{c_int, c_void, CString},
    panic::{catch_unwind, AssertUnwindSafe},
    ptr::NonNull,
};

use ohos_enum_derive::EnumFrom;
use ohos_native_pasteboard_sys::*;
use ohos_udmf_binding::{UdmfData, UdsValue};

use crate::error::PasteboardError;

/// Pasteboard change source for observer subscriptions (API 13).
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(Pasteboard_NotifyType, "Pasteboard_NotifyType_NOTIFY_", c_int)]
pub enum PasteboardNotifyType {
    /// Local device pasteboard data changed.
    LocalDataChange,
    /// Remote device pasteboard data changed.
    RemoteDataChange,
}

struct ObserverContext {
    callback: Box<dyn Fn(PasteboardNotifyType) + Send + Sync>,
}

unsafe extern "C" fn observer_notify_trampoline(
    context: *mut c_void,
    notify_type: Pasteboard_NotifyType,
) {
    if context.is_null() {
        return;
    }
    // SAFETY: `context` stays live until `OH_PasteboardObserver_Destroy` invokes
    // `observer_finalize_trampoline`.
    let state = unsafe { &*context.cast::<ObserverContext>() };
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if let Some(kind) = PasteboardNotifyType::try_from_raw(notify_type) {
            (state.callback)(kind);
        }
    }));
}

unsafe extern "C" fn observer_finalize_trampoline(context: *mut c_void) {
    if context.is_null() {
        return;
    }
    // SAFETY: `OH_PasteboardObserver_SetData` transfers ownership of the boxed
    // context to the native observer, which calls finalize once on destroy.
    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
        drop(Box::from_raw(context.cast::<ObserverContext>()));
    }));
}

/// Owned native pasteboard observer (API 13).
pub struct PasteboardObserver {
    raw: NonNull<OH_PasteboardObserver>,
}

// SAFETY: native observer handles are synchronized by the pasteboard service and
// the stored callback is `Send + Sync`.
unsafe impl Send for PasteboardObserver {}
unsafe impl Sync for PasteboardObserver {}

impl PasteboardObserver {
    /// Creates an observer invoking `callback` on pasteboard changes (API 13).
    pub fn new<F>(callback: F) -> Result<Self, PasteboardError>
    where
        F: Fn(PasteboardNotifyType) + Send + Sync + 'static,
    {
        let raw = NonNull::new(unsafe { OH_PasteboardObserver_Create() }).ok_or_else(|| {
            PasteboardError::CommonError("OH_PasteboardObserver_Create returned null".into())
        })?;
        let observer = Self { raw };
        let ctx = Box::into_raw(Box::new(ObserverContext {
            callback: Box::new(callback),
        }))
        .cast::<c_void>();
        // SAFETY: `observer.raw` is live and `ctx` is owned by the observer if
        // registration succeeds; on error we reclaim `ctx` immediately.
        let status = unsafe {
            OH_PasteboardObserver_SetData(
                observer.raw.as_ptr(),
                ctx,
                Some(observer_notify_trampoline),
                Some(observer_finalize_trampoline),
            )
        };
        if status != 0 {
            unsafe {
                drop(Box::from_raw(ctx.cast::<ObserverContext>()));
            }
            return Err(PasteboardError::InternalError(status));
        }
        Ok(observer)
    }
}

impl Drop for PasteboardObserver {
    fn drop(&mut self) {
        // SAFETY: uniquely owned observer; destroys it and triggers finalize.
        let _ = unsafe { OH_PasteboardObserver_Destroy(self.raw.as_ptr()) };
    }
}

/// RAII subscription that unsubscribes from a [`Pasteboard`] before destroying
/// its [`PasteboardObserver`] (API 13).
#[must_use = "dropping a PasteboardSubscription unsubscribes immediately"]
pub struct PasteboardSubscription<'a> {
    pasteboard: &'a Pasteboard,
    notify_type: PasteboardNotifyType,
    observer: PasteboardObserver,
    active: bool,
}

impl PasteboardSubscription<'_> {
    /// Explicitly unsubscribes and surfaces any native status error.
    pub fn unsubscribe(mut self) -> Result<(), PasteboardError> {
        self.active = false;
        self.pasteboard
            .unsubscribe(self.notify_type, &self.observer)
    }
}

impl Drop for PasteboardSubscription<'_> {
    fn drop(&mut self) {
        if self.active {
            self.active = false;
            let _ = self
                .pasteboard
                .unsubscribe(self.notify_type, &self.observer);
        }
    }
}

pub struct Pasteboard {
    raw: NonNull<OH_Pasteboard>,
}

impl Pasteboard {
    /// Clears the native system pasteboard (API 13).
    pub fn clear(&self) -> Result<(), PasteboardError> {
        // SAFETY: the owner keeps the pasteboard instance live for this call.
        let status = unsafe { OH_Pasteboard_ClearData(self.raw.as_ptr()) };
        if status == 0 {
            Ok(())
        } else {
            Err(PasteboardError::InternalError(status))
        }
    }

    /// Copies the native MIME-type list (API 14).
    ///
    /// The native array and strings belong to this pasteboard instance and
    /// are released by the SDK, not by the caller.
    #[cfg(feature = "api-14")]
    pub fn mime_types(&self) -> Result<Vec<String>, PasteboardError> {
        let mut count = 0;
        // SAFETY: the pasteboard instance is live and count is writable.
        let types = unsafe { OH_Pasteboard_GetMimeTypes(self.raw.as_ptr(), &mut count) };
        if count == 0 {
            return Ok(Vec::new());
        }
        let len = count as usize;
        if types.is_null()
            || len > isize::MAX as usize / std::mem::size_of::<*mut std::os::raw::c_char>()
        {
            return Err(PasteboardError::CommonError(
                "invalid native MIME-type array".into(),
            ));
        }
        let mut copy = Vec::new();
        copy.try_reserve_exact(len)
            .map_err(|e| PasteboardError::CommonError(e.to_string()))?;
        // SAFETY: the SDK returns count pointers owned by this pasteboard.
        // The SDK replaces/frees this cache on later queries/destruction; copy
        // both the strings and the array without freeing either native owner.
        for &value in unsafe { std::slice::from_raw_parts(types, len) } {
            if value.is_null() {
                return Err(PasteboardError::CommonError(
                    "null native MIME-type string".into(),
                ));
            }
            // SAFETY: every SDK MIME-type pointer names a live NUL-terminated string.
            let value = unsafe { std::ffi::CStr::from_ptr(value) }
                .to_str()
                .map_err(|e| PasteboardError::CommonError(e.to_string()))?;
            copy.push(value.to_owned());
        }
        Ok(copy)
    }

    /// Returns the native content-change counter (API 18).
    ///
    /// Zero also represents a failed query. The counter resets when the
    /// service/system restarts; clearing data does not reset it.
    #[cfg(feature = "api-18")]
    pub fn change_count(&self) -> u32 {
        // SAFETY: the owner keeps the pasteboard instance live for this query.
        unsafe { OH_Pasteboard_GetChangeCount(self.raw.as_ptr()) }
    }

    pub fn new() -> Self {
        Self::try_new().expect("OH_Pasteboard_Create failed")
    }

    /// Creates a native pasteboard handle, returning an error on allocation failure (API 13).
    pub fn try_new() -> Result<Self, PasteboardError> {
        let raw = unsafe { OH_Pasteboard_Create() };
        NonNull::new(raw).map(|raw| Self { raw }).ok_or_else(|| {
            PasteboardError::CommonError("OH_Pasteboard_Create returned null".into())
        })
    }

    pub fn data(&self) -> Result<UdmfData, PasteboardError> {
        let mut ret = 0;
        let raw = unsafe { OH_Pasteboard_GetData(self.raw.as_ptr(), &mut ret) };
        if ret != 0 {
            return Err(PasteboardError::InternalError(ret));
        }
        if raw.is_null() {
            return Err(PasteboardError::CommonError(
                "OH_Pasteboard_GetData returned null".into(),
            ));
        }
        let udmf_data = UdmfData::from_raw(raw);
        Ok(udmf_data)
    }

    // pub fn source(&self) -> Result<String, PasteboardError> {
    //     let mut len: u32 = 0;
    //     let ret = unsafe {
    //         OH_Pasteboard_GetDataSource(self.raw.as_ptr(), std::ptr::null_mut(), &mut len)
    //     };
    //     if ret != 0 {
    //         return Err(PasteboardError::InternalError(ret));
    //     }

    //     let mut buf = vec![0; len as usize];
    //     let ret =
    //         unsafe { OH_Pasteboard_GetDataSource(self.raw.as_ptr(), buf.as_mut_ptr(), &mut len) };
    //     if ret != 0 {
    //         return Err(PasteboardError::InternalError(ret));
    //     }
    //     Ok(String::from_utf8(buf).expect("Invalid UTF-8"))
    // }

    pub fn has_data(&self) -> bool {
        unsafe { OH_Pasteboard_HasData(self.raw.as_ptr()) }
    }

    /// Current pasteboard has some types
    /// https://developer.huawei.com/consumer/cn/doc/harmonyos-guides-V14/use_pasteboard_to_copy_and_paste-V14#js%E6%8E%A5%E5%8F%A3%E4%B8%8Endk%E6%8E%A5%E5%8F%A3%E6%95%B0%E6%8D%AE%E7%B1%BB%E5%9E%8B%E5%AF%B9%E5%BA%94%E5%85%B3%E7%B3%BB
    /// ```no_run
    /// let pasteboard = Pasteboard::new();
    /// let t = UdsPlainText::new();
    /// let has_type = pasteboard.has_type(t).unwrap();
    /// ```
    pub fn has_type<T: UdsValue>(&self, t: T) -> Result<bool, PasteboardError> {
        let types = t
            .get_type()
            .map_err(|e| PasteboardError::CommonError(e.to_string()))?;
        self.has_type_str(&types.to_string())
    }

    /// Checks whether the pasteboard contains data of the specified MIME or UTD type string (API 13).
    pub fn has_type_str(&self, mime_or_utd: &str) -> Result<bool, PasteboardError> {
        let s =
            CString::new(mime_or_utd).map_err(|e| PasteboardError::CommonError(e.to_string()))?;
        // SAFETY: both pointers stay live for the synchronous query.
        Ok(unsafe { OH_Pasteboard_HasType(self.raw.as_ptr(), s.as_ptr()) })
    }

    pub fn is_remote_data(&self) -> bool {
        unsafe { OH_Pasteboard_IsRemoteData(self.raw.as_ptr()) }
    }

    /// Checks whether pasteboard data resides on a remote device without fetching it (API 24).
    #[cfg(feature = "api-24")]
    pub fn has_remote_data(&self) -> bool {
        unsafe { OH_Pasteboard_HasRemoteData(self.raw.as_ptr()) }
    }

    /// Subscribes an existing observer to pasteboard changes (API 13).
    ///
    /// Prefer [`Self::on_change`] so unsubscription is guaranteed before the
    /// observer is destroyed.
    pub fn subscribe(
        &self,
        notify_type: PasteboardNotifyType,
        observer: &PasteboardObserver,
    ) -> Result<(), PasteboardError> {
        let status = unsafe {
            OH_Pasteboard_Subscribe(self.raw.as_ptr(), notify_type.into(), observer.raw.as_ptr())
        };
        if status == 0 {
            Ok(())
        } else {
            Err(PasteboardError::InternalError(status))
        }
    }

    /// Unsubscribes a previously subscribed observer (API 13).
    pub fn unsubscribe(
        &self,
        notify_type: PasteboardNotifyType,
        observer: &PasteboardObserver,
    ) -> Result<(), PasteboardError> {
        let status = unsafe {
            OH_Pasteboard_Unsubscribe(self.raw.as_ptr(), notify_type.into(), observer.raw.as_ptr())
        };
        if status == 0 {
            Ok(())
        } else {
            Err(PasteboardError::InternalError(status))
        }
    }

    /// Subscribes a callback to pasteboard changes and returns an RAII guard (API 13).
    pub fn on_change<F>(
        &self,
        notify_type: PasteboardNotifyType,
        callback: F,
    ) -> Result<PasteboardSubscription<'_>, PasteboardError>
    where
        F: Fn(PasteboardNotifyType) + Send + Sync + 'static,
    {
        let observer = PasteboardObserver::new(callback)?;
        self.subscribe(notify_type, &observer)?;
        Ok(PasteboardSubscription {
            pasteboard: self,
            notify_type,
            observer,
            active: true,
        })
    }

    pub fn set_data(&self, data: &UdmfData) -> Result<(), PasteboardError> {
        let ret = unsafe { OH_Pasteboard_SetData(self.raw.as_ptr(), data.raw().as_ptr()) };
        if ret != 0 {
            return Err(PasteboardError::InternalError(ret));
        }
        Ok(())
    }
}

impl Drop for Pasteboard {
    fn drop(&mut self) {
        unsafe { OH_Pasteboard_Destroy(self.raw.as_ptr()) }
    }
}

impl Default for Pasteboard {
    fn default() -> Self {
        Self::new()
    }
}
