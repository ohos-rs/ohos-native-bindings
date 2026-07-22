use crate::error::{check, CommonEventError, Result};
use crate::info::SubscribeInfo;
use crate::parameters::ParametersRef;
use ohos_common_event_sys as sys;
use std::ffi::{c_char, CStr};
use std::marker::PhantomData;
use std::ptr::NonNull;

#[cfg(feature = "api-18")]
use crate::error::check_bool;
#[cfg(feature = "api-18")]
use std::ffi::CString;

/// One received common event, borrowed for the duration of the callback.
///
/// The common event service allocates the data right before it calls the
/// handler and frees it as soon as the handler returns, so this is a view and
/// not an owned value: it has no `Drop`, and its lifetime is tied to the
/// `&RcvData` the handler is given. Anything to be kept must be copied out —
/// the string accessors borrow the native buffers, so call `to_owned` on what
/// is needed.
pub struct RcvData<'a> {
    raw: NonNull<sys::CommonEvent_RcvData>,
    _marker: PhantomData<&'a sys::CommonEvent_RcvData>,
}

impl RcvData<'_> {
    /// The name of the event, for example
    /// [`event::SCREEN_ON`](crate::event::SCREEN_ON).
    ///
    /// `None` when the name is not valid UTF-8.
    pub fn event(&self) -> Option<&str> {
        // SAFETY: the data is borrowed for the callback and the pointer, when not
        // null, is a NUL-terminated string owned by it.
        self.text(unsafe { sys::OH_CommonEvent_GetEventFromRcvData(self.raw.as_ptr()) })
    }

    /// The result code carried by the event, `0` when it carries none.
    pub fn code(&self) -> i32 {
        // SAFETY: the data is borrowed for the callback.
        unsafe { sys::OH_CommonEvent_GetCodeFromRcvData(self.raw.as_ptr()) }
    }

    /// The string payload of the event, if it has one that is valid UTF-8.
    pub fn data(&self) -> Option<&str> {
        // SAFETY: as in `event`.
        self.text(unsafe { sys::OH_CommonEvent_GetDataStrFromRcvData(self.raw.as_ptr()) })
    }

    /// The bundle name the event was published for, if it names one.
    pub fn bundle_name(&self) -> Option<&str> {
        // SAFETY: as in `event`.
        self.text(unsafe { sys::OH_CommonEvent_GetBundleNameFromRcvData(self.raw.as_ptr()) })
    }

    /// The key/value payload of the event, if it has one.
    ///
    /// The view borrows `self`, which keeps it inside the callback along with
    /// every buffer its array getters hand out.
    pub fn parameters(&self) -> Option<ParametersRef<'_>> {
        // SAFETY: the data is borrowed for the callback; the payload it returns
        // is owned by the data and released with it.
        let raw = unsafe { sys::OH_CommonEvent_GetParametersFromRcvData(self.raw.as_ptr()) };
        NonNull::new(raw.cast_mut()).map(ParametersRef::from_raw)
    }

    fn text(&self, pointer: *const c_char) -> Option<&str> {
        if pointer.is_null() {
            return None;
        }
        // SAFETY: the caller passes a NUL-terminated string owned by the borrowed
        // data, so it lives at least as long as `&self`.
        unsafe { CStr::from_ptr(pointer) }.to_str().ok()
    }
}

/// The handler a [`Subscriber`] runs for every event it receives.
///
/// The native callback is a bare C function pointer with no user-data argument,
/// so there is nowhere to keep the captures of a closure. The handler is a type
/// instead: [`Subscriber::new`] instantiates one C trampoline per handler type,
/// which is what tells the callback which `on_receive` to run. State the
/// handler needs has to reach it some other way, such as a channel in a
/// `static`.
///
/// The handler runs on a thread owned by the common event service, and a panic
/// crossing back into it aborts the process, so it should not panic.
///
/// ```no_run
/// use ohos_common_event_binding::{event, ReceiveHandler, RcvData, SubscribeInfo, Subscriber};
///
/// struct OnScreenOn;
///
/// impl ReceiveHandler for OnScreenOn {
///     fn on_receive(data: &RcvData<'_>) {
///         let _name = data.event().unwrap_or_default();
///     }
/// }
///
/// let info = SubscribeInfo::new(&[event::SCREEN_ON])?;
/// let mut subscriber = Subscriber::new::<OnScreenOn>(&info)?;
/// subscriber.subscribe()?;
/// # Ok::<(), ohos_common_event_binding::CommonEventError>(())
/// ```
pub trait ReceiveHandler {
    /// Called once per received event, with the event borrowed for the call.
    fn on_receive(data: &RcvData<'_>);
}

/// The C callback handed to the native layer, one instantiation per handler.
unsafe extern "C" fn trampoline<H: ReceiveHandler>(data: *const sys::CommonEvent_RcvData) {
    let Some(raw) = NonNull::new(data.cast_mut()) else {
        return;
    };
    // The view borrows for the body of this function only, which is exactly how
    // long the common event service keeps `data` alive.
    let data = RcvData {
        raw,
        _marker: PhantomData,
    };
    H::on_receive(&data);
}

/// A subscription to a set of common events.
///
/// The subscriber is created from a [`SubscribeInfo`] together with the
/// [`ReceiveHandler`] to run, and starts receiving events once
/// [`subscribe`](Self::subscribe) is called. Dropping it unsubscribes on a
/// best-effort basis and then releases the native handle.
///
/// The order matters: the common event service holds its own reference to the
/// object that carries the callback, so destroying the handle alone would leave
/// the service dispatching into it. Unsubscribing first is what stops the
/// dispatch.
pub struct Subscriber {
    raw: NonNull<sys::CommonEvent_Subscriber>,
    subscribed: bool,
}

impl Subscriber {
    /// Create a subscriber for the events described by `info`, dispatching to
    /// `H`.
    ///
    /// The subscriber does not receive anything until
    /// [`subscribe`](Self::subscribe) is called. Everything it needs is copied
    /// out of `info` here, so `info` may be dropped or reused afterwards.
    pub fn new<H: ReceiveHandler>(info: &SubscribeInfo) -> Result<Self> {
        // SAFETY: `info` is a live handle, and the callback is a `'static` C
        // function which is valid for as long as the process runs.
        let raw =
            unsafe { sys::OH_CommonEvent_CreateSubscriber(info.as_ptr(), Some(trampoline::<H>)) };
        NonNull::new(raw)
            .map(|raw| Self {
                raw,
                subscribed: false,
            })
            .ok_or(CommonEventError::Alloc)
    }

    /// Start receiving events.
    pub fn subscribe(&mut self) -> Result<()> {
        // SAFETY: the handle is live.
        unsafe { check(sys::OH_CommonEvent_Subscribe(self.raw.as_ptr()))? };
        self.subscribed = true;
        Ok(())
    }

    /// Stop receiving events.
    ///
    /// A failed call leaves the subscription in place, and dropping the
    /// subscriber tries again.
    pub fn unsubscribe(&mut self) -> Result<()> {
        // SAFETY: the handle is live.
        unsafe { check(sys::OH_CommonEvent_UnSubscribe(self.raw.as_ptr()))? };
        self.subscribed = false;
        Ok(())
    }

    /// Whether the event being handled is an ordered one.
    #[cfg(feature = "api-18")]
    pub fn is_ordered(&self) -> bool {
        // SAFETY: the handle is live.
        unsafe { sys::OH_CommonEvent_IsOrderedCommonEvent(self.raw.as_ptr()) }
    }

    /// Finish the ordered event being handled, releasing it to the next
    /// subscriber in line.
    #[cfg(feature = "api-18")]
    pub fn finish(&mut self) -> Result<()> {
        // SAFETY: the handle is live.
        check_bool(unsafe { sys::OH_CommonEvent_FinishCommonEvent(self.raw.as_ptr()) })
    }

    /// Whether the ordered event being handled has been aborted.
    #[cfg(feature = "api-18")]
    pub fn is_aborted(&self) -> bool {
        // SAFETY: the handle is live.
        unsafe { sys::OH_CommonEvent_GetAbortCommonEvent(self.raw.as_ptr()) }
    }

    /// Abort the ordered event being handled, so it reaches no further
    /// subscriber.
    #[cfg(feature = "api-18")]
    pub fn abort(&mut self) -> Result<()> {
        // SAFETY: the handle is live.
        check_bool(unsafe { sys::OH_CommonEvent_AbortCommonEvent(self.raw.as_ptr()) })
    }

    /// Clear the aborted flag of the ordered event being handled.
    #[cfg(feature = "api-18")]
    pub fn clear_abort(&mut self) -> Result<()> {
        // SAFETY: the handle is live.
        check_bool(unsafe { sys::OH_CommonEvent_ClearAbortCommonEvent(self.raw.as_ptr()) })
    }

    /// The result code of the ordered event being handled, `0` when there is
    /// none.
    #[cfg(feature = "api-18")]
    pub fn code(&self) -> i32 {
        // SAFETY: the handle is live.
        unsafe { sys::OH_CommonEvent_GetCodeFromSubscriber(self.raw.as_ptr()) }
    }

    /// Set the result code passed on to the next subscriber of the ordered
    /// event being handled.
    #[cfg(feature = "api-18")]
    pub fn set_code(&mut self, code: i32) -> Result<()> {
        // SAFETY: the handle is live.
        check_bool(unsafe { sys::OH_CommonEvent_SetCodeToSubscriber(self.raw.as_ptr(), code) })
    }

    /// The result data of the ordered event being handled.
    ///
    /// The value is copied out: the native buffer is reused by the next call.
    /// `None` when there is no result, or it is not valid UTF-8.
    #[cfg(feature = "api-18")]
    pub fn data(&self) -> Option<String> {
        // SAFETY: the handle is live; the pointer, when not null, is a
        // NUL-terminated string that stays valid until the next call.
        let pointer = unsafe { sys::OH_CommonEvent_GetDataFromSubscriber(self.raw.as_ptr()) };
        if pointer.is_null() {
            return None;
        }
        // SAFETY: as above; the string is copied before returning.
        unsafe { CStr::from_ptr(pointer) }
            .to_str()
            .ok()
            .map(str::to_owned)
    }

    /// Set the result data passed on to the next subscriber of the ordered
    /// event being handled.
    #[cfg(feature = "api-18")]
    pub fn set_data(&mut self, data: &str) -> Result<()> {
        let data = CString::new(data)?;
        let length = data.as_bytes().len();
        // SAFETY: the handle is live and the string of `length` bytes outlives
        // the call, which copies it.
        check_bool(unsafe {
            sys::OH_CommonEvent_SetDataToSubscriber(self.raw.as_ptr(), data.as_ptr(), length)
        })
    }
}

impl Drop for Subscriber {
    fn drop(&mut self) {
        if self.subscribed {
            // SAFETY: the handle is live. Unsubscribing first is what makes
            // destroying it safe: the common event service keeps its own
            // reference to the object holding the callback, and only drops it
            // here, so no further dispatch can be in flight afterwards.
            unsafe { sys::OH_CommonEvent_UnSubscribe(self.raw.as_ptr()) };
        }
        // SAFETY: the handle came from OH_CommonEvent_CreateSubscriber, is owned
        // here and is released once.
        unsafe { sys::OH_CommonEvent_DestroySubscriber(self.raw.as_ptr()) };
    }
}
