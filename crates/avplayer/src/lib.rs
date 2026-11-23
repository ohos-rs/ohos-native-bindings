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

#[cfg(feature = "api-20")]
pub use sys::AVPlayerOnInfoType_AV_INFO_TYPE_PLAYBACK_RATE_DONE as INFO_PLAYBACK_RATE_DONE;
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
    AVPlayerOnInfoType_AV_INFO_TYPE_AUDIO_OUTPUT_DEVICE_CHANGE as INFO_AUDIO_OUTPUT_DEVICE_CHANGE,
    AVPlayerOnInfoType_AV_INFO_TYPE_BUFFERING_UPDATE as INFO_BUFFERING,
    AVPlayerOnInfoType_AV_INFO_TYPE_DURATION_UPDATE as INFO_DURATION_UPDATE,
    AVPlayerOnInfoType_AV_INFO_TYPE_EOS as INFO_END,
    AVPlayerOnInfoType_AV_INFO_TYPE_INTERRUPT_EVENT as INFO_INTERRUPT_EVENT,
    AVPlayerOnInfoType_AV_INFO_TYPE_POSITION_UPDATE as INFO_POSITION,
    AVPlayerOnInfoType_AV_INFO_TYPE_RESOLUTION_CHANGE as INFO_RESOLUTION_CHANGE,
    AVPlayerOnInfoType_AV_INFO_TYPE_SEEKDONE as INFO_SEEK_DONE,
    AVPlayerOnInfoType_AV_INFO_TYPE_SPEEDDONE as INFO_SPEED_DONE,
    AVPlayerOnInfoType_AV_INFO_TYPE_STATE_CHANGE as INFO_STATE_CHANGE,
    AVPlayerOnInfoType_AV_INFO_TYPE_VOLUME_CHANGE as INFO_VOLUME_CHANGE,
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

/// Audio stream usage reported to the system audio policy (API 12).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioStreamUsage {
    Unknown,
    Music,
    VoiceCommunication,
    VoiceAssistant,
    Alarm,
    VoiceMessage,
    Ringtone,
    Notification,
    Accessibility,
    Movie,
    Game,
    Audiobook,
    Navigation,
    VideoCommunication,
}

impl AudioStreamUsage {
    fn raw(self) -> sys::OH_AudioStream_Usage {
        match self {
            Self::Unknown => sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_UNKNOWN,
            Self::Music => sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_MUSIC,
            Self::VoiceCommunication => {
                sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_VOICE_COMMUNICATION
            }
            Self::VoiceAssistant => sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_VOICE_ASSISTANT,
            Self::Alarm => sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_ALARM,
            Self::VoiceMessage => sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_VOICE_MESSAGE,
            Self::Ringtone => sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_RINGTONE,
            Self::Notification => sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_NOTIFICATION,
            Self::Accessibility => sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_ACCESSIBILITY,
            Self::Movie => sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_MOVIE,
            Self::Game => sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_GAME,
            Self::Audiobook => sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_AUDIOBOOK,
            Self::Navigation => sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_NAVIGATION,
            Self::VideoCommunication => {
                sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_VIDEO_COMMUNICATION
            }
        }
    }
}

/// Audio focus interruption mode for an AVPlayer (API 12).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioInterruptMode {
    /// Shares the application's audio focus session.
    Share,
    /// Manages independent audio focus for this player.
    Independent,
}

impl AudioInterruptMode {
    fn raw(self) -> sys::OH_AudioInterrupt_Mode {
        match self {
            Self::Share => sys::OH_AudioInterrupt_Mode_AUDIOSTREAM_INTERRUPT_MODE_SHARE,
            Self::Independent => sys::OH_AudioInterrupt_Mode_AUDIOSTREAM_INTERRUPT_MODE_INDEPENDENT,
        }
    }
}

/// Audio effect mode for an AVPlayer (API 12).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioEffectMode {
    None,
    Default,
}

impl AudioEffectMode {
    fn raw(self) -> sys::OH_AudioStream_AudioEffectMode {
        match self {
            Self::None => sys::OH_AudioStream_AudioEffectMode_EFFECT_NONE,
            Self::Default => sys::OH_AudioStream_AudioEffectMode_EFFECT_DEFAULT,
        }
    }
}

/// Discrete playback speed presets supported from API 11.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackSpeed {
    #[cfg(feature = "api-13")]
    Forward0_125x,
    #[cfg(feature = "api-13")]
    Forward0_25x,
    Forward0_50x,
    Forward0_75x,
    Forward1_00x,
    Forward1_25x,
    Forward1_50x,
    Forward1_75x,
    Forward2_00x,
    #[cfg(feature = "api-13")]
    Forward3_00x,
}

impl PlaybackSpeed {
    fn raw(self) -> sys::AVPlaybackSpeed {
        match self {
            #[cfg(feature = "api-13")]
            Self::Forward0_125x => sys::AVPlaybackSpeed_AV_SPEED_FORWARD_0_125_X,
            #[cfg(feature = "api-13")]
            Self::Forward0_25x => sys::AVPlaybackSpeed_AV_SPEED_FORWARD_0_25_X,
            Self::Forward0_50x => sys::AVPlaybackSpeed_AV_SPEED_FORWARD_0_50_X,
            Self::Forward0_75x => sys::AVPlaybackSpeed_AV_SPEED_FORWARD_0_75_X,
            Self::Forward1_00x => sys::AVPlaybackSpeed_AV_SPEED_FORWARD_1_00_X,
            Self::Forward1_25x => sys::AVPlaybackSpeed_AV_SPEED_FORWARD_1_25_X,
            Self::Forward1_50x => sys::AVPlaybackSpeed_AV_SPEED_FORWARD_1_50_X,
            Self::Forward1_75x => sys::AVPlaybackSpeed_AV_SPEED_FORWARD_1_75_X,
            Self::Forward2_00x => sys::AVPlaybackSpeed_AV_SPEED_FORWARD_2_00_X,
            #[cfg(feature = "api-13")]
            Self::Forward3_00x => sys::AVPlaybackSpeed_AV_SPEED_FORWARD_3_00_X,
        }
    }

    #[allow(non_upper_case_globals)]
    fn from_raw(raw: sys::AVPlaybackSpeed) -> Result<Self, Error> {
        match raw {
            #[cfg(feature = "api-13")]
            sys::AVPlaybackSpeed_AV_SPEED_FORWARD_0_125_X => Ok(Self::Forward0_125x),
            #[cfg(feature = "api-13")]
            sys::AVPlaybackSpeed_AV_SPEED_FORWARD_0_25_X => Ok(Self::Forward0_25x),
            sys::AVPlaybackSpeed_AV_SPEED_FORWARD_0_50_X => Ok(Self::Forward0_50x),
            sys::AVPlaybackSpeed_AV_SPEED_FORWARD_0_75_X => Ok(Self::Forward0_75x),
            sys::AVPlaybackSpeed_AV_SPEED_FORWARD_1_00_X => Ok(Self::Forward1_00x),
            sys::AVPlaybackSpeed_AV_SPEED_FORWARD_1_25_X => Ok(Self::Forward1_25x),
            sys::AVPlaybackSpeed_AV_SPEED_FORWARD_1_50_X => Ok(Self::Forward1_50x),
            sys::AVPlaybackSpeed_AV_SPEED_FORWARD_1_75_X => Ok(Self::Forward1_75x),
            sys::AVPlaybackSpeed_AV_SPEED_FORWARD_2_00_X => Ok(Self::Forward2_00x),
            #[cfg(feature = "api-13")]
            sys::AVPlaybackSpeed_AV_SPEED_FORWARD_3_00_X => Ok(Self::Forward3_00x),
            _ => Err(invalid("playback speed")),
        }
    }
}

/// Media stream category (API 9/23).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaType {
    Audio,
    Video,
    Subtitle,
    #[cfg(feature = "api-20")]
    TimedMetadata,
    #[cfg(feature = "api-20")]
    Auxiliary,
}

#[cfg(feature = "api-23")]
impl MediaType {
    fn raw(self) -> sys::OH_MediaType {
        match self {
            Self::Audio => sys::OH_MediaType_MEDIA_TYPE_AUD,
            Self::Video => sys::OH_MediaType_MEDIA_TYPE_VID,
            Self::Subtitle => sys::OH_MediaType_MEDIA_TYPE_SUBTITLE,
            Self::TimedMetadata => sys::OH_MediaType_MEDIA_TYPE_TIMED_METADATA,
            Self::Auxiliary => sys::OH_MediaType_MEDIA_TYPE_AUXILIARY,
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
    /// Reads the current playback rate (API 23).
    #[cfg(feature = "api-23")]
    pub fn get_playback_rate(&mut self) -> Result<f32, Error> {
        let mut rate = 0.0;
        check("get playback rate", unsafe {
            sys::OH_AVPlayer_GetPlaybackRate(self.raw()?, &mut rate)
        })?;
        Ok(rate)
    }
    /// Sets a discrete playback speed preset (API 11).
    pub fn set_playback_speed(&mut self, speed: PlaybackSpeed) -> Result<(), Error> {
        check("playback speed", unsafe {
            sys::OH_AVPlayer_SetPlaybackSpeed(self.raw()?, speed.raw())
        })
    }
    /// Reads the current discrete playback speed preset (API 11).
    pub fn playback_speed(&mut self) -> Result<PlaybackSpeed, Error> {
        let mut speed = 0;
        check("get playback speed", unsafe {
            sys::OH_AVPlayer_GetPlaybackSpeed(self.raw()?, &mut speed)
        })?;
        PlaybackSpeed::from_raw(speed)
    }
    /// Sets the audio stream usage for this player (API 12).
    pub fn set_audio_renderer_info(&mut self, usage: AudioStreamUsage) -> Result<(), Error> {
        check("audio renderer info", unsafe {
            sys::OH_AVPlayer_SetAudioRendererInfo(self.raw()?, usage.raw())
        })
    }
    /// Sets the audio interruption mode for this player (API 12).
    pub fn set_audio_interrupt_mode(&mut self, mode: AudioInterruptMode) -> Result<(), Error> {
        check("audio interrupt mode", unsafe {
            sys::OH_AVPlayer_SetAudioInterruptMode(self.raw()?, mode.raw())
        })
    }
    /// Sets the audio effect mode for this player (API 12).
    pub fn set_audio_effect_mode(&mut self, mode: AudioEffectMode) -> Result<(), Error> {
        check("audio effect mode", unsafe {
            sys::OH_AVPlayer_SetAudioEffectMode(self.raw()?, mode.raw())
        })
    }
    /// Mutes or unmutes the specified media stream (API 23).
    #[cfg(feature = "api-23")]
    pub fn set_media_muted(&mut self, media_type: MediaType, muted: bool) -> Result<(), Error> {
        check("media muted", unsafe {
            sys::OH_AVPlayer_SetMediaMuted(self.raw()?, media_type.raw(), muted)
        })
    }
    /// Returns whether the player is currently playing (API 11).
    pub fn is_playing(&mut self) -> Result<bool, Error> {
        let raw = self.raw()?;
        Ok(unsafe { sys::OH_AVPlayer_IsPlaying(raw) })
    }
    /// Returns whether loop playback is enabled (API 11).
    pub fn is_looping(&mut self) -> Result<bool, Error> {
        let raw = self.raw()?;
        Ok(unsafe { sys::OH_AVPlayer_IsLooping(raw) })
    }
    /// Reads the current playback position in milliseconds, or returns an error if unavailable (API 23).
    #[cfg(feature = "api-23")]
    pub fn playback_position(&mut self) -> Result<i32, Error> {
        let pos = unsafe { sys::OH_AVPlayer_GetPlaybackPosition(self.raw()?) };
        if pos < 0 {
            Err(invalid("playback position"))
        } else {
            Ok(pos)
        }
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
    /// State change reason (1 = user, 2 = system).
    StateChangeReason,
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
    /// Audio interrupt event type.
    InterruptType,
    /// Audio interrupt force type.
    InterruptForce,
    /// Audio interrupt hint.
    InterruptHint,
    /// Audio output device change reason.
    AudioDeviceChangeReason,
    /// Discrete playback speed value.
    PlaybackSpeed,
    /// Message type (1 = first video frame rendered).
    MessageType,
    /// Live stream indicator (1 = live).
    IsLiveStream,
}
impl InfoField {
    fn key(self) -> *const std::ffi::c_char {
        // SAFETY: SDK-exported key pointers have process lifetime and are read only.
        unsafe {
            match self {
                Self::State => sys::OH_PLAYER_STATE,
                Self::StateChangeReason => sys::OH_PLAYER_STATE_CHANGE_REASON,
                Self::CurrentPosition => sys::OH_PLAYER_CURRENT_POSITION,
                Self::Duration => sys::OH_PLAYER_DURATION,
                Self::BufferingType => sys::OH_PLAYER_BUFFERING_TYPE,
                Self::BufferingValue => sys::OH_PLAYER_BUFFERING_VALUE,
                Self::VideoWidth => sys::OH_PLAYER_VIDEO_WIDTH,
                Self::VideoHeight => sys::OH_PLAYER_VIDEO_HEIGHT,
                Self::SeekPosition => sys::OH_PLAYER_SEEK_POSITION,
                Self::InterruptType => sys::OH_PLAYER_AUDIO_INTERRUPT_TYPE,
                Self::InterruptForce => sys::OH_PLAYER_AUDIO_INTERRUPT_FORCE,
                Self::InterruptHint => sys::OH_PLAYER_AUDIO_INTERRUPT_HINT,
                Self::AudioDeviceChangeReason => sys::OH_PLAYER_AUDIO_DEVICE_CHANGE_REASON,
                Self::PlaybackSpeed => sys::OH_PLAYER_PLAYBACK_SPEED,
                Self::MessageType => sys::OH_PLAYER_MESSAGE_TYPE,
                Self::IsLiveStream => sys::OH_PLAYER_IS_LIVE_STREAM,
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
    /// Copies an SDK 64-bit integer value, or returns None when the field is absent.
    pub fn long(&self, field: InfoField) -> Option<i64> {
        let key = field.key();
        if key.is_null() {
            return None;
        }
        let mut value = 0;
        // SAFETY: metadata/key remain live for this synchronous scalar copy.
        unsafe { sys::OH_AVFormat_GetLongValue(self.raw.as_ptr(), key, &mut value) }
            .then_some(value)
    }
}
