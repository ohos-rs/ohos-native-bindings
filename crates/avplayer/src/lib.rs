//! Owned AVPlayer operations and callback-borrowed metadata.
//!
//! Native calls are serialized by mutable ownership. Preparation is asynchronous:
//! observe the native prepared-state notification before starting playback.
//! Explicit close and Drop can block while releasing surface buffers; run them
//! on an application-owned media worker when UI latency matters.
use ohos_avplayer_sys as sys;
use std::{
    cell::Cell,
    ffi::{c_void, CStr},
    marker::PhantomData,
    os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd},
    ptr::NonNull,
};

/// Metadata borrowed by a native callback; never destroy or retain it.
pub use sys::OH_AVFormat as RawFormat;
/// Native player pointer supplied to callbacks; never destroy it in a callback.
pub use sys::OH_AVPlayer as RawPlayer;
/// Native error callback. Message text is valid only during the callback.
pub use sys::OH_AVPlayerOnErrorCallback as ErrorCallback;
/// Native info callback. Arguments are valid only during the callback.
pub use sys::OH_AVPlayerOnInfoCallback as InfoCallback;
/// Buffering notifications defined by the SDK.
pub use sys::{
    AVPlayerBufferingType_AVPLAYER_BUFFERING_END as BUFFERING_END,
    AVPlayerBufferingType_AVPLAYER_BUFFERING_START as BUFFERING_START,
};
/// Native message kinds, preserving SDK values.
pub use sys::{
    AVPlayerOnInfoType as InfoType,
    AVPlayerOnInfoType_AV_INFO_TYPE_BUFFERING_UPDATE as INFO_BUFFERING,
    AVPlayerOnInfoType_AV_INFO_TYPE_EOS as INFO_END,
    AVPlayerOnInfoType_AV_INFO_TYPE_POSITION_UPDATE as INFO_POSITION,
    AVPlayerOnInfoType_AV_INFO_TYPE_SEEKDONE as INFO_SEEK_DONE,
    AVPlayerOnInfoType_AV_INFO_TYPE_STATE_CHANGE as INFO_STATE_CHANGE,
};
/// Native player states, preserving SDK values.
pub use sys::{
    AVPlayerState as State, AVPlayerState_AV_COMPLETED as STATE_COMPLETED,
    AVPlayerState_AV_ERROR as STATE_ERROR, AVPlayerState_AV_IDLE as STATE_IDLE,
    AVPlayerState_AV_INITIALIZED as STATE_INITIALIZED, AVPlayerState_AV_PAUSED as STATE_PAUSED,
    AVPlayerState_AV_PLAYING as STATE_PLAYING, AVPlayerState_AV_PREPARED as STATE_PREPARED,
    AVPlayerState_AV_RELEASED as STATE_RELEASED, AVPlayerState_AV_STOPPED as STATE_STOPPED,
};

/// A rejected argument, closed owner or native AVPlayer status.
#[derive(Clone, Copy, Debug)]
pub struct Error {
    operation: &'static str,
    code: u32,
}
impl Error {
    /// The unmodified native status code.
    pub const fn code(self) -> u32 {
        self.code
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AVPlayer {} failed ({})", self.operation, self.code)
    }
}
impl std::error::Error for Error {}
fn check(operation: &'static str, code: u32) -> Result<(), Error> {
    if code == sys::OH_AVErrCode_AV_ERR_OK {
        Ok(())
    } else {
        Err(Error { operation, code })
    }
}
fn invalid(operation: &'static str) -> Error {
    Error {
        operation,
        code: sys::OH_AVErrCode_AV_ERR_INVALID_VAL,
    }
}

/// Native keyframe selection used by seek.
#[derive(Clone, Copy, Debug)]
pub enum SeekMode {
    /// Seeks to the preceding sync frame.
    PreviousSync,
    /// Seeks to the next sync frame.
    NextSync,
    /// Seeks to the closest frame.
    Closest,
}
impl SeekMode {
    fn raw(self) -> sys::AVPlayerSeekMode {
        match self {
            Self::PreviousSync => sys::AVPlayerSeekMode_AV_SEEK_PREVIOUS_SYNC,
            Self::NextSync => sys::AVPlayerSeekMode_AV_SEEK_NEXT_SYNC,
            Self::Closest => sys::AVPlayerSeekMode_AV_SEEK_CLOSEST,
        }
    }
}

/// A uniquely owned player. Moving it to a media worker is supported; concurrent
/// operations are forbidden. Its borrowed native video window must outlive close.
pub struct AvPlayer {
    raw: Option<NonNull<RawPlayer>>,
    source: Option<OwnedFd>,
    close_failed: Option<Error>,
    info_registered: bool,
    error_registered: bool,
    _not_sync: PhantomData<Cell<()>>,
}
// SAFETY: the native player supports serialized calls across threads; mutable
// methods and !Sync prohibit overlapping operations through this owner.
unsafe impl Send for AvPlayer {}
impl AvPlayer {
    /// Creates a native player (API 11).
    pub fn new() -> Result<Self, Error> {
        // SAFETY: creation has no input pointers and returns a unique handle.
        let raw = NonNull::new(unsafe { sys::OH_AVPlayer_Create() }).ok_or(invalid("create"))?;
        Ok(Self {
            raw: Some(raw),
            source: None,
            close_failed: None,
            info_registered: false,
            error_registered: false,
            _not_sync: PhantomData,
        })
    }
    fn raw(&self) -> Result<*mut RawPlayer, Error> {
        if let Some(error) = self.close_failed {
            return Err(error);
        }
        self.raw
            .map(NonNull::as_ptr)
            .ok_or(invalid("closed player"))
    }
    /// Sets a URL source. The SDK copies the URL during this call (API 11).
    pub fn set_url(&mut self, url: &CStr) -> Result<(), Error> {
        // SAFETY: both handle and NUL-terminated URL remain live for this call.
        check("URL source", unsafe {
            sys::OH_AVPlayer_SetURLSource(self.raw()?, url.as_ptr())
        })
    }
    /// Duplicates and retains a source descriptor until successful reset or close.
    /// Offset and size describe a nonempty checked region of that file (API 11).
    /// A failed native source call conservatively retains the descriptor too.
    pub fn set_fd(&mut self, fd: BorrowedFd<'_>, offset: i64, size: i64) -> Result<(), Error> {
        if self.source.is_some() || offset < 0 || size <= 0 || offset.checked_add(size).is_none() {
            return Err(invalid("FD region"));
        }
        let owned = fd
            .try_clone_to_owned()
            .map_err(|_| invalid("duplicate FD"))?;
        let raw = self.raw()?;
        self.source = Some(owned);
        // SAFETY: this owner retains the duplicated descriptor across native use.
        check("FD source", unsafe {
            sys::OH_AVPlayer_SetFDSource(
                raw,
                self.source.as_ref().unwrap().as_fd().as_raw_fd(),
                offset,
                size,
            )
        })
    }
    /// Associates a borrowed producer window after source selection, before prepare.
    ///
    /// # Safety
    /// The window and its native surface must remain live until successful
    /// synchronous close. Reset is not a surface-buffer retirement barrier. Serialize producer retirement with close. A native
    /// window reference alone does not prove its XComponent surface is live.
    pub unsafe fn set_video_surface(&mut self, window: NonNull<c_void>) -> Result<(), Error> {
        // SAFETY: the caller supplies the native window/surface lifetime.
        check("video surface", unsafe {
            sys::OH_AVPlayer_SetVideoSurface(self.raw()?, window.as_ptr().cast())
        })
    }
    /// Registers a native info callback (API 12).
    ///
    /// # Safety
    /// Callback and userdata must remain valid and thread safe until a successful
    /// explicit clear. Callback payload pointers are borrowed for that invocation.
    /// Do not unwind or call AVPlayer APIs synchronously from a callback: native
    /// registration and delivery hold the same callback mutex.
    pub unsafe fn set_info_callback(
        &mut self,
        callback: InfoCallback,
        userdata: *mut c_void,
    ) -> Result<(), Error> {
        let raw = self.raw()?;
        if callback.is_none() && !self.info_registered {
            return Ok(());
        }
        // Retain the potential registration after any failed install; only a
        // successful native clear proves that userdata can be reclaimed.
        self.info_registered |= callback.is_some();
        // SAFETY: the caller retains and synchronizes callback userdata.
        check("info callback", unsafe {
            sys::OH_AVPlayer_SetOnInfoCallback(raw, callback, userdata)
        })?;
        self.info_registered = callback.is_some();
        Ok(())
    }
    /// Registers an error callback with the same contract as `set_info_callback`.
    ///
    /// # Safety
    /// The callback/userdata must satisfy [`Self::set_info_callback`]'s contract.
    pub unsafe fn set_error_callback(
        &mut self,
        callback: ErrorCallback,
        userdata: *mut c_void,
    ) -> Result<(), Error> {
        let raw = self.raw()?;
        if callback.is_none() && !self.error_registered {
            return Ok(());
        }
        self.error_registered |= callback.is_some();
        // SAFETY: the caller retains and synchronizes callback userdata.
        check("error callback", unsafe {
            sys::OH_AVPlayer_SetOnErrorCallback(raw, callback, userdata)
        })?;
        self.error_registered = callback.is_some();
        Ok(())
    }
    /// Clears both callbacks. Success waits for prior native info/error delivery
    /// to leave the shared callback mutex. On failure retain userdata and owner.
    /// Call from outside those callbacks (API 12).
    pub fn clear_callbacks(&mut self) -> Result<(), Error> {
        // SAFETY: clearing passes no userdata and retains each existing raw
        // callback owner until that setter successfully drains native delivery.
        unsafe { self.set_info_callback(None, std::ptr::null_mut()) }?;
        // SAFETY: the same explicit drain contract applies to the error callback.
        unsafe { self.set_error_callback(None, std::ptr::null_mut()) }
    }
    /// Starts asynchronous preparation (API 11).
    pub fn prepare(&mut self) -> Result<(), Error> {
        let raw = self.raw()?; // SAFETY: exclusive live handle.

        check("prepare", unsafe { sys::OH_AVPlayer_Prepare(raw) })
    }
    /// Starts/resumes a prepared player (API 11).
    pub fn play(&mut self) -> Result<(), Error> {
        let raw = self.raw()?; // SAFETY: exclusive live handle.

        check("play", unsafe { sys::OH_AVPlayer_Play(raw) })
    }
    /// Pauses playback (API 11).
    pub fn pause(&mut self) -> Result<(), Error> {
        let raw = self.raw()?; // SAFETY: exclusive live handle.

        check("pause", unsafe { sys::OH_AVPlayer_Pause(raw) })
    }
    /// Stops playback (API 11).
    pub fn stop(&mut self) -> Result<(), Error> {
        let raw = self.raw()?; // SAFETY: exclusive live handle.

        check("stop", unsafe { sys::OH_AVPlayer_Stop(raw) })
    }
    /// Resets a player and releases its retained source only on success (API 11).
    pub fn reset(&mut self) -> Result<(), Error> {
        let raw = self.raw()?; // SAFETY: exclusive live handle.

        check("reset", unsafe { sys::OH_AVPlayer_Reset(raw) })?;
        self.source = None;
        Ok(())
    }
    /// Sets finite left/right volume in the SDK range 0..=1 (API 11).
    pub fn volume(&mut self, left: f32, right: f32) -> Result<(), Error> {
        if !left.is_finite()
            || !right.is_finite()
            || !(0.0..=1.0).contains(&left)
            || !(0.0..=1.0).contains(&right)
        {
            return Err(invalid("volume"));
        }
        // SAFETY: live serialized owner and validated scalars.
        check("volume", unsafe {
            sys::OH_AVPlayer_SetVolume(self.raw()?, left, right)
        })
    }
    /// Seeks to a nonnegative position in milliseconds (API 11).
    pub fn seek(&mut self, milliseconds: i32, mode: SeekMode) -> Result<(), Error> {
        if milliseconds < 0 {
            return Err(invalid("seek"));
        }
        // SAFETY: live serialized owner and validated scalar.
        check("seek", unsafe {
            sys::OH_AVPlayer_Seek(self.raw()?, milliseconds, mode.raw())
        })
    }
    /// Sets finite playback rate in the SDK range 0.125..=4 (API 20–24),
    /// or 0.125..=8 when the API 25+ feature is selected.
    #[cfg(feature = "api-20")]
    pub fn playback_rate(&mut self, rate: f32) -> Result<(), Error> {
        let maximum = if cfg!(feature = "api-25") { 8.0 } else { 4.0 };
        if !rate.is_finite() || !(0.125..=maximum).contains(&rate) {
            return Err(invalid("playback rate"));
        }
        // SAFETY: live serialized owner and validated scalar.
        check("playback rate", unsafe {
            sys::OH_AVPlayer_SetPlaybackRate(self.raw()?, rate)
        })
    }
    /// Sets native playback repetition (API 11).
    pub fn looping(&mut self, enabled: bool) -> Result<(), Error> {
        let raw = self.raw()?; // SAFETY: exclusive live handle.

        check("looping", unsafe {
            sys::OH_AVPlayer_SetLooping(raw, enabled)
        })
    }
    /// Reads playback position in milliseconds (API 11).
    pub fn current_time(&mut self) -> Result<i32, Error> {
        let mut value = 0; // SAFETY: live handle and writable output.

        check("current time", unsafe {
            sys::OH_AVPlayer_GetCurrentTime(self.raw()?, &mut value)
        })?;
        Ok(value)
    }
    /// Reads duration in milliseconds (API 11).
    pub fn duration(&mut self) -> Result<i32, Error> {
        let mut value = 0; // SAFETY: live handle and writable output.

        check("duration", unsafe {
            sys::OH_AVPlayer_GetDuration(self.raw()?, &mut value)
        })?;
        Ok(value)
    }
    /// Reads native video dimensions (API 11).
    pub fn video_size(&mut self) -> Result<(i32, i32), Error> {
        let (mut width, mut height) = (0, 0); // SAFETY: live handle and writable outputs.

        check("video width", unsafe {
            sys::OH_AVPlayer_GetVideoWidth(self.raw()?, &mut width)
        })?; // SAFETY: live handle and writable output.

        check("video height", unsafe {
            sys::OH_AVPlayer_GetVideoHeight(self.raw()?, &mut height)
        })?;
        Ok((width, height))
    }
    /// Reads the native state, including future SDK values (API 11).
    pub fn state(&mut self) -> Result<State, Error> {
        let mut value = 0; // SAFETY: live handle and writable output.

        check("state", unsafe {
            sys::OH_AVPlayer_GetState(self.raw()?, &mut value)
        })?;
        Ok(value)
    }
    /// Clears callbacks and synchronously releases native playback/buffers (API 11).
    /// This can block. On failure retain this owner, borrowed surface and userdata;
    /// a native failed release may already alter state, so it is never retried
    /// as if an idempotent success proved buffer retirement.
    pub fn close_sync(&mut self) -> Result<(), Error> {
        if let Some(error) = self.close_failed {
            return Err(error);
        }
        if self.raw.is_none() {
            return Ok(());
        }
        self.clear_callbacks()?;
        // SAFETY: callbacks have drained and this owner serializes native release.
        let result = check("synchronous release", unsafe {
            sys::OH_AVPlayer_ReleaseSync(self.raw()?)
        });
        if let Err(error) = result {
            self.close_failed = Some(error);
            return Err(error);
        }
        self.raw = None;
        self.source = None;
        Ok(())
    }
}
impl Drop for AvPlayer {
    fn drop(&mut self) {
        if let Err(error) = self.close_sync() {
            log::error!("{error}; retaining native source after failed close");
            if let Some(source) = self.source.take() {
                std::mem::forget(source);
            }
        }
    }
}

/// Native metadata keys with SDK-defined storage and values.
#[derive(Clone, Copy, Debug)]
pub enum InfoField {
    /// Native playback state.
    State,
    /// Current position in milliseconds.
    CurrentPosition,
    /// Duration in milliseconds.
    Duration,
    /// Buffering event type.
    BufferingType,
    /// Buffering event value.
    BufferingValue,
    /// Decoded video width.
    VideoWidth,
    /// Decoded video height.
    VideoHeight,
    /// Completed seek position in milliseconds.
    SeekPosition,
}
impl InfoField {
    fn key(self) -> *const std::ffi::c_char {
        // SAFETY: SDK-exported key pointers have process lifetime and are read only.
        unsafe {
            match self {
                Self::State => sys::OH_PLAYER_STATE,
                Self::CurrentPosition => sys::OH_PLAYER_CURRENT_POSITION,
                Self::Duration => sys::OH_PLAYER_DURATION,
                Self::BufferingType => sys::OH_PLAYER_BUFFERING_TYPE,
                Self::BufferingValue => sys::OH_PLAYER_BUFFERING_VALUE,
                Self::VideoWidth => sys::OH_PLAYER_VIDEO_WIDTH,
                Self::VideoHeight => sys::OH_PLAYER_VIDEO_HEIGHT,
                Self::SeekPosition => sys::OH_PLAYER_SEEK_POSITION,
            }
        }
    }
}
/// Metadata borrowed for one native info callback; never an owning format handle.
pub struct InfoRef<'callback> {
    raw: NonNull<RawFormat>,
    _lifetime: PhantomData<&'callback RawFormat>,
}
impl<'callback> InfoRef<'callback> {
    /// Borrows a callback's metadata.
    ///
    /// # Safety
    /// The format must remain live and unmodified for `'callback`, which must
    /// end before the native callback returns. Do not release the native format.
    pub unsafe fn from_raw(raw: *mut RawFormat) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self {
            raw,
            _lifetime: PhantomData,
        })
    }
    /// Copies an SDK integer value, or returns None when the field is absent.
    pub fn int(&self, field: InfoField) -> Option<i32> {
        let key = field.key();
        if key.is_null() {
            return None;
        }
        let mut value = 0;
        // SAFETY: metadata/key remain live for this synchronous scalar copy.
        unsafe { sys::OH_AVFormat_GetIntValue(self.raw.as_ptr(), key, &mut value) }.then_some(value)
    }
}
