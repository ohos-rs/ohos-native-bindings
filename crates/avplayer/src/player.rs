use std::ffi::{CStr, CString};
use std::os::fd::RawFd;
use std::os::raw::{c_char, c_void};
use std::ptr::NonNull;
use std::sync::mpsc::Sender;
use std::time::Duration;

use ohos_avplayer_sys::*;
use ohos_native_window_binding::NativeWindow;

use crate::{
    AvPlayerBuffering, AvPlayerError, AvPlayerEvent, AvPlayerResult, AvPlayerSeekMode,
    AvPlayerState, VideoSize,
};
#[cfg(feature = "api-23")]
use crate::{AvPlayerTrack, AvPlayerTrackType};

struct CallbackContext {
    sender: Sender<AvPlayerEvent>,
}

#[cfg(feature = "api-23")]
struct OwnedMediaSource {
    raw: NonNull<OH_AVMediaSource>,
    headers: Option<NonNull<OH_AVHttpHeader>>,
}

#[cfg(feature = "api-23")]
impl Drop for OwnedMediaSource {
    fn drop(&mut self) {
        // SAFETY: both pointers were created by their matching constructors
        // and remain exclusively owned by this wrapper.
        unsafe {
            let _ = OH_AVMediaSource_Destroy(self.raw.as_ptr());
            if let Some(headers) = self.headers.take() {
                let _ = OH_AVHttpHeader_Destroy(headers.as_ptr());
            }
        }
    }
}

/// Exclusive owner of one native AVPlayer instance.
pub struct AvPlayer {
    raw: NonNull<OH_AVPlayer>,
    callbacks: Box<CallbackContext>,
    window: Option<NativeWindow>,
    #[cfg(feature = "api-23")]
    media_source: Option<OwnedMediaSource>,
    released: bool,
}

impl AvPlayer {
    pub fn new(sender: Sender<AvPlayerEvent>) -> AvPlayerResult<Self> {
        // SAFETY: constructor has no preconditions.
        let raw = NonNull::new(unsafe { OH_AVPlayer_Create() }).ok_or_else(|| {
            AvPlayerError::unavailable("OH_AVPlayer_Create", "AVPlayer service is unavailable")
        })?;
        let callbacks = Box::new(CallbackContext { sender });
        let user_data = (&*callbacks as *const CallbackContext)
            .cast_mut()
            .cast::<c_void>();

        // SAFETY: `raw` is live and `user_data` points into a stable Box kept
        // by the returned owner until both callbacks are unregistered.
        let info = unsafe { OH_AVPlayer_SetOnInfoCallback(raw.as_ptr(), Some(on_info), user_data) };
        if info != OH_AVErrCode_AV_ERR_OK {
            // SAFETY: creation succeeded and no callback was installed.
            unsafe {
                let _ = OH_AVPlayer_ReleaseSync(raw.as_ptr());
            }
            return Err(AvPlayerError::native("OH_AVPlayer_SetOnInfoCallback", info));
        }
        // SAFETY: same lifetime argument as the information callback.
        let error =
            unsafe { OH_AVPlayer_SetOnErrorCallback(raw.as_ptr(), Some(on_error), user_data) };
        if error != OH_AVErrCode_AV_ERR_OK {
            // SAFETY: unregister before destroying the callback context.
            unsafe {
                let _ = OH_AVPlayer_SetOnInfoCallback(raw.as_ptr(), None, std::ptr::null_mut());
                let _ = OH_AVPlayer_ReleaseSync(raw.as_ptr());
            }
            return Err(AvPlayerError::native(
                "OH_AVPlayer_SetOnErrorCallback",
                error,
            ));
        }

        Ok(Self {
            raw,
            callbacks,
            window: None,
            #[cfg(feature = "api-23")]
            media_source: None,
            released: false,
        })
    }

    pub fn set_url_source(&mut self, url: &str) -> AvPlayerResult<()> {
        let url = c_string("OH_AVPlayer_SetURLSource", "url", url)?;
        self.drop_media_source();
        // SAFETY: AVPlayer copies/retains the URL during this call.
        self.call("OH_AVPlayer_SetURLSource", |player| unsafe {
            OH_AVPlayer_SetURLSource(player, url.as_ptr())
        })
    }

    #[cfg(feature = "api-23")]
    pub fn set_url_source_with_headers(
        &mut self,
        url: &str,
        headers: &[(String, String)],
    ) -> AvPlayerResult<()> {
        let url = c_string("OH_AVMediaSource_CreateWithUrl", "url", url)?;
        // SAFETY: constructor has no preconditions.
        let header_ptr = NonNull::new(unsafe { OH_AVHttpHeader_Create() }).ok_or_else(|| {
            AvPlayerError::unavailable("OH_AVHttpHeader_Create", "failed to allocate HTTP headers")
        })?;

        for (name, value) in headers {
            let name = match c_string("OH_AVHttpHeader_AddRecord", "header name", name) {
                Ok(value) => value,
                Err(error) => {
                    // SAFETY: constructor returned this owned handle.
                    unsafe {
                        let _ = OH_AVHttpHeader_Destroy(header_ptr.as_ptr());
                    }
                    return Err(error);
                }
            };
            let value = match c_string("OH_AVHttpHeader_AddRecord", "header value", value) {
                Ok(value) => value,
                Err(error) => {
                    // SAFETY: constructor returned this owned handle.
                    unsafe {
                        let _ = OH_AVHttpHeader_Destroy(header_ptr.as_ptr());
                    }
                    return Err(error);
                }
            };
            // SAFETY: strings and header are live for the call.
            let code = unsafe {
                OH_AVHttpHeader_AddRecord(header_ptr.as_ptr(), name.as_ptr(), value.as_ptr())
            };
            if code != OH_AVErrCode_AV_ERR_OK {
                // SAFETY: constructor returned this owned handle.
                unsafe {
                    let _ = OH_AVHttpHeader_Destroy(header_ptr.as_ptr());
                }
                return Err(AvPlayerError::native("OH_AVHttpHeader_AddRecord", code));
            }
        }

        // SAFETY: URL and header handle are valid for construction.
        let source = NonNull::new(unsafe {
            OH_AVMediaSource_CreateWithUrl(url.as_ptr(), header_ptr.as_ptr())
        })
        .ok_or_else(|| {
            // SAFETY: source construction failed, so the header is still ours.
            unsafe {
                let _ = OH_AVHttpHeader_Destroy(header_ptr.as_ptr());
            }
            AvPlayerError::unavailable(
                "OH_AVMediaSource_CreateWithUrl",
                "failed to create URL media source",
            )
        })?;
        let owned = OwnedMediaSource {
            raw: source,
            headers: Some(header_ptr),
        };
        // SAFETY: source stays owned by `self` until reset/release.
        let code = unsafe { OH_AVPlayer_SetMediaSource(self.raw.as_ptr(), source.as_ptr()) };
        if code != OH_AVErrCode_AV_ERR_OK {
            return Err(AvPlayerError::native("OH_AVPlayer_SetMediaSource", code));
        }
        self.media_source = Some(owned);
        Ok(())
    }

    pub fn set_fd_source(&mut self, fd: RawFd, offset: u64, size: u64) -> AvPlayerResult<()> {
        let offset = i64::try_from(offset).map_err(|_| {
            AvPlayerError::invalid_configuration(
                "OH_AVPlayer_SetFDSource",
                "file offset exceeds i64::MAX",
            )
        })?;
        let size = i64::try_from(size).map_err(|_| {
            AvPlayerError::invalid_configuration(
                "OH_AVPlayer_SetFDSource",
                "file size exceeds i64::MAX",
            )
        })?;
        self.drop_media_source();
        // SAFETY: AVPlayer validates the descriptor and numeric range.
        self.call("OH_AVPlayer_SetFDSource", |player| unsafe {
            OH_AVPlayer_SetFDSource(player, fd, offset, size)
        })
    }

    /// Attach and retain a native output window.
    pub fn set_video_surface(&mut self, window: NativeWindow) -> AvPlayerResult<()> {
        // SAFETY: `window` owns a live reference and is retained in `self`
        // after the AVPlayer accepts it.
        self.call("OH_AVPlayer_SetVideoSurface", |player| unsafe {
            OH_AVPlayer_SetVideoSurface(player, window.as_ptr())
        })?;
        self.window = Some(window);
        Ok(())
    }

    pub fn prepare(&mut self) -> AvPlayerResult<()> {
        self.call("OH_AVPlayer_Prepare", |player| unsafe {
            OH_AVPlayer_Prepare(player)
        })
    }

    pub fn play(&mut self) -> AvPlayerResult<()> {
        self.call("OH_AVPlayer_Play", |player| unsafe {
            OH_AVPlayer_Play(player)
        })
    }

    pub fn pause(&mut self) -> AvPlayerResult<()> {
        self.call("OH_AVPlayer_Pause", |player| unsafe {
            OH_AVPlayer_Pause(player)
        })
    }

    pub fn stop(&mut self) -> AvPlayerResult<()> {
        self.call("OH_AVPlayer_Stop", |player| unsafe {
            OH_AVPlayer_Stop(player)
        })
    }

    pub fn reset(&mut self) -> AvPlayerResult<()> {
        self.call("OH_AVPlayer_Reset", |player| unsafe {
            OH_AVPlayer_Reset(player)
        })?;
        self.drop_media_source();
        Ok(())
    }

    pub fn seek(&mut self, position: Duration, mode: AvPlayerSeekMode) -> AvPlayerResult<()> {
        let millis = duration_millis_i32("OH_AVPlayer_Seek", position)?;
        self.call("OH_AVPlayer_Seek", |player| unsafe {
            OH_AVPlayer_Seek(player, millis, mode.raw())
        })
    }

    pub fn set_volume(&mut self, volume: f32) -> AvPlayerResult<()> {
        if !volume.is_finite() || !(0.0..=1.0).contains(&volume) {
            return Err(AvPlayerError::invalid_configuration(
                "OH_AVPlayer_SetVolume",
                "volume must be finite and within 0.0..=1.0",
            ));
        }
        self.call("OH_AVPlayer_SetVolume", |player| unsafe {
            OH_AVPlayer_SetVolume(player, volume, volume)
        })
    }

    pub fn set_looping(&mut self, looping: bool) -> AvPlayerResult<()> {
        self.call("OH_AVPlayer_SetLooping", |player| unsafe {
            OH_AVPlayer_SetLooping(player, looping)
        })
    }

    #[cfg(feature = "api-20")]
    pub fn set_playback_rate(&mut self, rate: f32) -> AvPlayerResult<()> {
        if !rate.is_finite() || !(0.125..=4.0).contains(&rate) {
            return Err(AvPlayerError::invalid_configuration(
                "OH_AVPlayer_SetPlaybackRate",
                "playback rate must be finite and within 0.125..=4.0",
            ));
        }
        self.call("OH_AVPlayer_SetPlaybackRate", |player| unsafe {
            OH_AVPlayer_SetPlaybackRate(player, rate)
        })
    }

    pub fn select_bitrate(&mut self, bitrate: u32) -> AvPlayerResult<()> {
        self.call("OH_AVPlayer_SelectBitRate", |player| unsafe {
            OH_AVPlayer_SelectBitRate(player, bitrate)
        })
    }

    pub fn select_track(&mut self, index: i32) -> AvPlayerResult<()> {
        self.call("OH_AVPlayer_SelectTrack", |player| unsafe {
            OH_AVPlayer_SelectTrack(player, index)
        })
    }

    pub fn deselect_track(&mut self, index: i32) -> AvPlayerResult<()> {
        self.call("OH_AVPlayer_DeselectTrack", |player| unsafe {
            OH_AVPlayer_DeselectTrack(player, index)
        })
    }

    #[cfg(feature = "api-23")]
    pub fn add_url_subtitle(&mut self, url: &str) -> AvPlayerResult<()> {
        let url = c_string("OH_AVPlayer_AddUrlSubtitleSource", "url", url)?;
        self.call("OH_AVPlayer_AddUrlSubtitleSource", |player| unsafe {
            OH_AVPlayer_AddUrlSubtitleSource(player, url.as_ptr())
        })
    }

    pub fn state(&self) -> AvPlayerResult<AvPlayerState> {
        let mut state = AVPlayerState_AV_ERROR;
        self.call_ref("OH_AVPlayer_GetState", |player| unsafe {
            OH_AVPlayer_GetState(player, &mut state)
        })?;
        Ok(AvPlayerState::from_raw(state))
    }

    pub fn current_time(&self) -> AvPlayerResult<Duration> {
        let mut millis = 0;
        self.call_ref("OH_AVPlayer_GetCurrentTime", |player| unsafe {
            OH_AVPlayer_GetCurrentTime(player, &mut millis)
        })?;
        Ok(millis_duration(millis))
    }

    pub fn duration(&self) -> AvPlayerResult<Duration> {
        let mut millis = 0;
        self.call_ref("OH_AVPlayer_GetDuration", |player| unsafe {
            OH_AVPlayer_GetDuration(player, &mut millis)
        })?;
        Ok(millis_duration(millis))
    }

    pub fn video_size(&self) -> AvPlayerResult<VideoSize> {
        let mut width = 0;
        let mut height = 0;
        self.call_ref("OH_AVPlayer_GetVideoWidth", |player| unsafe {
            OH_AVPlayer_GetVideoWidth(player, &mut width)
        })?;
        self.call_ref("OH_AVPlayer_GetVideoHeight", |player| unsafe {
            OH_AVPlayer_GetVideoHeight(player, &mut height)
        })?;
        Ok(VideoSize::new(
            nonnegative_u32(width),
            nonnegative_u32(height),
        ))
    }

    #[cfg(feature = "api-23")]
    pub fn tracks(&self) -> Vec<AvPlayerTrack> {
        // SAFETY: `raw` is live for this immutable query.
        let count = unsafe { OH_AVPlayer_GetTrackCount(self.raw.as_ptr()) };
        (0..count)
            .filter_map(|index| {
                // SAFETY: indices come from the player-reported range. The
                // returned format is caller-owned.
                let format =
                    NonNull::new(unsafe { OH_AVPlayer_GetTrackFormat(self.raw.as_ptr(), index) })?;
                let track = unsafe { track_from_format(index, format.as_ptr()) };
                // SAFETY: format was returned owned by AVPlayer.
                unsafe { OH_AVFormat_Destroy(format.as_ptr()) };
                Some(track)
            })
            .collect()
    }

    pub fn release(mut self) -> AvPlayerResult<()> {
        self.release_inner()
    }

    fn call(
        &mut self,
        operation: &'static str,
        function: impl FnOnce(*mut OH_AVPlayer) -> OH_AVErrCode,
    ) -> AvPlayerResult<()> {
        if self.released {
            return Err(AvPlayerError::invalid_state(
                operation,
                "AVPlayer was already released",
            ));
        }
        let code = function(self.raw.as_ptr());
        if code == OH_AVErrCode_AV_ERR_OK {
            Ok(())
        } else {
            Err(AvPlayerError::native(operation, code))
        }
    }

    fn call_ref(
        &self,
        operation: &'static str,
        function: impl FnOnce(*mut OH_AVPlayer) -> OH_AVErrCode,
    ) -> AvPlayerResult<()> {
        if self.released {
            return Err(AvPlayerError::invalid_state(
                operation,
                "AVPlayer was already released",
            ));
        }
        let code = function(self.raw.as_ptr());
        if code == OH_AVErrCode_AV_ERR_OK {
            Ok(())
        } else {
            Err(AvPlayerError::native(operation, code))
        }
    }

    fn release_inner(&mut self) -> AvPlayerResult<()> {
        if self.released {
            return Ok(());
        }
        // SAFETY: unregister synchronously while callback context and native
        // window are still alive, then release the player synchronously.
        unsafe {
            let _ = OH_AVPlayer_SetOnInfoCallback(self.raw.as_ptr(), None, std::ptr::null_mut());
            let _ = OH_AVPlayer_SetOnErrorCallback(self.raw.as_ptr(), None, std::ptr::null_mut());
        }
        // Keep a read of the callback owner here: it documents and enforces
        // that the field remains live through callback unregistration.
        let _ = &self.callbacks;
        // SAFETY: this owner has not released its live raw player yet.
        let code = unsafe { OH_AVPlayer_ReleaseSync(self.raw.as_ptr()) };
        self.released = true;
        self.drop_media_source();
        self.window.take();
        if code == OH_AVErrCode_AV_ERR_OK {
            Ok(())
        } else {
            Err(AvPlayerError::native("OH_AVPlayer_ReleaseSync", code))
        }
    }

    #[cfg(feature = "api-23")]
    fn drop_media_source(&mut self) {
        self.media_source.take();
    }

    #[cfg(not(feature = "api-23"))]
    fn drop_media_source(&mut self) {}
}

impl Drop for AvPlayer {
    fn drop(&mut self) {
        let _ = self.release_inner();
    }
}

unsafe extern "C" fn on_error(
    _player: *mut OH_AVPlayer,
    code: i32,
    message: *const c_char,
    user_data: *mut c_void,
) {
    let Some(context) = (unsafe { callback_context(user_data) }) else {
        return;
    };
    let message = if message.is_null() {
        String::new()
    } else {
        // SAFETY: AVPlayer guarantees a callback-scoped NUL-terminated string.
        unsafe { CStr::from_ptr(message) }
            .to_string_lossy()
            .into_owned()
    };
    let _ = context
        .sender
        .send(AvPlayerEvent::Error(AvPlayerError::callback(code, message)));
}

unsafe extern "C" fn on_info(
    _player: *mut OH_AVPlayer,
    kind: AVPlayerOnInfoType,
    info: *mut OH_AVFormat,
    user_data: *mut c_void,
) {
    let Some(context) = (unsafe { callback_context(user_data) }) else {
        return;
    };
    let event = unsafe { event_from_info(kind, info) };
    if let Some(event) = event {
        let _ = context.sender.send(event);
    }
}

unsafe fn callback_context<'a>(user_data: *mut c_void) -> Option<&'a CallbackContext> {
    // SAFETY: caller validates null; non-null pointers originate from the
    // stable Box owned by AvPlayer and callbacks are unregistered before drop.
    unsafe { user_data.cast::<CallbackContext>().as_ref() }
}

#[allow(non_upper_case_globals)]
unsafe fn event_from_info(
    kind: AVPlayerOnInfoType,
    info: *mut OH_AVFormat,
) -> Option<AvPlayerEvent> {
    match kind {
        AVPlayerOnInfoType_AV_INFO_TYPE_EOS => Some(AvPlayerEvent::Ended),
        AVPlayerOnInfoType_AV_INFO_TYPE_STATE_CHANGE => {
            let value = unsafe { get_i32(info, OH_PLAYER_STATE) }?;
            Some(AvPlayerEvent::StateChanged(AvPlayerState::from_raw(
                nonnegative_u32(value),
            )))
        }
        AVPlayerOnInfoType_AV_INFO_TYPE_POSITION_UPDATE => {
            let value = unsafe { get_i32(info, OH_PLAYER_CURRENT_POSITION) }?;
            Some(AvPlayerEvent::Position(millis_duration(value)))
        }
        AVPlayerOnInfoType_AV_INFO_TYPE_DURATION_UPDATE => {
            let value = unsafe { get_i32(info, OH_PLAYER_DURATION) }?;
            Some(AvPlayerEvent::Duration(millis_duration(value)))
        }
        AVPlayerOnInfoType_AV_INFO_TYPE_RESOLUTION_CHANGE => {
            let width = unsafe { get_i32(info, OH_PLAYER_VIDEO_WIDTH) }?;
            let height = unsafe { get_i32(info, OH_PLAYER_VIDEO_HEIGHT) }?;
            Some(AvPlayerEvent::Resolution(VideoSize::new(
                nonnegative_u32(width),
                nonnegative_u32(height),
            )))
        }
        AVPlayerOnInfoType_AV_INFO_TYPE_BUFFERING_UPDATE => {
            let kind = unsafe { get_i32(info, OH_PLAYER_BUFFERING_TYPE) }?;
            let value = unsafe { get_i32(info, OH_PLAYER_BUFFERING_VALUE) }.unwrap_or_default();
            let buffering = match u32::try_from(kind).unwrap_or_default() {
                AVPlayerBufferingType_AVPLAYER_BUFFERING_START => AvPlayerBuffering::Started,
                AVPlayerBufferingType_AVPLAYER_BUFFERING_END => AvPlayerBuffering::Ended,
                AVPlayerBufferingType_AVPLAYER_BUFFERING_PERCENT => {
                    AvPlayerBuffering::Percent(value.clamp(0, 100) as u8)
                }
                AVPlayerBufferingType_AVPLAYER_BUFFERING_CACHED_DURATION => {
                    AvPlayerBuffering::CachedDuration(millis_duration(value))
                }
                _ => AvPlayerBuffering::Unknown { kind, value },
            };
            Some(AvPlayerEvent::Buffering(buffering))
        }
        AVPlayerOnInfoType_AV_INFO_TYPE_SEEKDONE => {
            let value = unsafe { get_i32(info, OH_PLAYER_SEEK_POSITION) }.unwrap_or_default();
            Some(AvPlayerEvent::SeekCompleted(millis_duration(value)))
        }
        AVPlayerOnInfoType_AV_INFO_TYPE_VOLUME_CHANGE => {
            unsafe { get_f32(info, OH_PLAYER_VOLUME) }.map(AvPlayerEvent::VolumeChanged)
        }
        AVPlayerOnInfoType_AV_INFO_TYPE_BITRATEDONE => {
            let value = unsafe { get_i32(info, OH_PLAYER_BITRATE) }?;
            Some(AvPlayerEvent::BitrateChanged(nonnegative_u32(value)))
        }
        AVPlayerOnInfoType_AV_INFO_TYPE_BITRATE_COLLECT => {
            unsafe { get_u32_buffer(info, OH_PLAYER_BITRATE_ARRAY) }
                .map(AvPlayerEvent::AvailableBitrates)
        }
        AVPlayerOnInfoType_AV_INFO_TYPE_IS_LIVE_STREAM => {
            let value = unsafe { get_i32(info, OH_PLAYER_IS_LIVE_STREAM) }?;
            Some(AvPlayerEvent::LiveChanged(value != 0))
        }
        AVPlayerOnInfoType_AV_INFO_TYPE_TRACK_INFO_UPDATE => Some(AvPlayerEvent::TracksChanged),
        AVPlayerOnInfoType_AV_INFO_TYPE_INTERRUPT_EVENT
        | AVPlayerOnInfoType_AV_INFO_TYPE_AUDIO_OUTPUT_DEVICE_CHANGE => {
            Some(AvPlayerEvent::AudioInterrupted)
        }
        #[cfg(feature = "api-20")]
        AVPlayerOnInfoType_AV_INFO_TYPE_PLAYBACK_RATE_DONE => {
            unsafe { get_f32(info, OH_PLAYER_PLAYBACK_RATE) }
                .map(AvPlayerEvent::PlaybackRateChanged)
        }
        #[cfg(feature = "api-23")]
        AVPlayerOnInfoType_AV_INFO_TYPE_TRACKCHANGE => {
            let index = unsafe { get_i32(info, OH_PLAYER_TRACH_CHANGE_INFO_TRACK_INDEX) }?;
            let selected = unsafe { get_i32(info, OH_PLAYER_TRACH_CHANGE_INFO_TRACK_SELECTED) }
                .unwrap_or_default()
                != 0;
            Some(AvPlayerEvent::TrackChanged { index, selected })
        }
        #[cfg(feature = "api-23")]
        AVPlayerOnInfoType_AV_INFO_TYPE_SUBTITLE_UPDATE => {
            let text = unsafe { get_string(info, OH_PLAYER_SUBTITLE_UPDATE_INFO_TEXT) }
                .unwrap_or_default();
            let start = unsafe { get_i32(info, OH_PLAYER_SUBTITLE_UPDATE_INFO_START_TIME) }
                .map(millis_duration)
                .unwrap_or_default();
            let duration = unsafe { get_i32(info, OH_PLAYER_SUBTITLE_UPDATE_INFO_DURATION) }
                .map(millis_duration)
                .unwrap_or_default();
            Some(AvPlayerEvent::Subtitle {
                text,
                start,
                duration,
            })
        }
        _ => None,
    }
}

unsafe fn get_i32(format: *mut OH_AVFormat, key: *const c_char) -> Option<i32> {
    if format.is_null() || key.is_null() {
        return None;
    }
    let mut value = 0;
    // SAFETY: AVPlayer owns `format` for this callback and key is a native
    // constant with static storage duration.
    unsafe { OH_AVFormat_GetIntValue(format, key, &mut value) }.then_some(value)
}

unsafe fn get_f32(format: *mut OH_AVFormat, key: *const c_char) -> Option<f32> {
    if format.is_null() || key.is_null() {
        return None;
    }
    let mut value = 0.0;
    // SAFETY: same callback-scoped argument as `get_i32`.
    unsafe { OH_AVFormat_GetFloatValue(format, key, &mut value) }.then_some(value)
}

#[cfg(feature = "api-23")]
unsafe fn get_f64(format: *mut OH_AVFormat, key: *const c_char) -> Option<f64> {
    if format.is_null() || key.is_null() {
        return None;
    }
    let mut value = 0.0;
    // SAFETY: same callback/owned-format argument as `get_i32`.
    unsafe { OH_AVFormat_GetDoubleValue(format, key, &mut value) }.then_some(value)
}

#[cfg(feature = "api-23")]
unsafe fn get_i64(format: *mut OH_AVFormat, key: *const c_char) -> Option<i64> {
    if format.is_null() || key.is_null() {
        return None;
    }
    let mut value = 0;
    // SAFETY: same callback/owned-format argument as `get_i32`.
    unsafe { OH_AVFormat_GetLongValue(format, key, &mut value) }.then_some(value)
}

#[cfg(feature = "api-23")]
unsafe fn get_string(format: *mut OH_AVFormat, key: *const c_char) -> Option<String> {
    if format.is_null() || key.is_null() {
        return None;
    }
    let mut value = std::ptr::null();
    // SAFETY: the returned string is copied before the callback/format ends.
    if !unsafe { OH_AVFormat_GetStringValue(format, key, &mut value) } || value.is_null() {
        return None;
    }
    Some(
        unsafe { CStr::from_ptr(value) }
            .to_string_lossy()
            .into_owned(),
    )
}

unsafe fn get_u32_buffer(format: *mut OH_AVFormat, key: *const c_char) -> Option<Vec<u32>> {
    if format.is_null() || key.is_null() {
        return None;
    }
    let mut data = std::ptr::null_mut();
    let mut size = 0;
    // SAFETY: buffer remains native-owned and callback-scoped; copy it now.
    if !unsafe { OH_AVFormat_GetBuffer(format, key, &mut data, &mut size) } || data.is_null() {
        return None;
    }
    let count = size.checked_div(std::mem::size_of::<u32>())?;
    Some(unsafe { std::slice::from_raw_parts(data.cast::<u32>(), count) }.to_vec())
}

#[cfg(feature = "api-23")]
unsafe fn track_from_format(index: u32, format: *mut OH_AVFormat) -> AvPlayerTrack {
    let raw_type = unsafe { get_i32(format, OH_MD_KEY_TRACK_TYPE) }
        .and_then(|value| u32::try_from(value).ok())
        .unwrap_or(u32::MAX);
    let width = unsafe { get_i32(format, OH_MD_KEY_WIDTH) }.and_then(positive_u32);
    let height = unsafe { get_i32(format, OH_MD_KEY_HEIGHT) }.and_then(positive_u32);
    AvPlayerTrack {
        index,
        track_type: AvPlayerTrackType::from_raw(raw_type),
        title: unsafe { get_string(format, OH_MD_KEY_TITLE) },
        language: unsafe { get_string(format, OH_MD_KEY_LANGUAGE) },
        mime_type: unsafe { get_string(format, OH_MD_KEY_CODEC_MIME) },
        bitrate: unsafe { get_i64(format, OH_MD_KEY_BITRATE) }
            .and_then(|value| u64::try_from(value).ok()),
        video_size: width
            .zip(height)
            .map(|(width, height)| VideoSize::new(width, height)),
        frame_rate: unsafe { get_f64(format, OH_MD_KEY_FRAME_RATE) },
        channel_count: unsafe { get_i32(format, OH_MD_KEY_AUD_CHANNEL_COUNT) }
            .and_then(positive_u32),
        sample_rate: unsafe { get_i32(format, OH_MD_KEY_AUD_SAMPLE_RATE) }.and_then(positive_u32),
    }
}

fn c_string(operation: &'static str, field: &'static str, value: &str) -> AvPlayerResult<CString> {
    CString::new(value).map_err(|_| {
        AvPlayerError::invalid_configuration(
            operation,
            format!("{field} must not contain a NUL byte"),
        )
    })
}

fn duration_millis_i32(operation: &'static str, duration: Duration) -> AvPlayerResult<i32> {
    i32::try_from(duration.as_millis()).map_err(|_| {
        AvPlayerError::invalid_configuration(operation, "duration exceeds i32 millisecond range")
    })
}

fn millis_duration(millis: i32) -> Duration {
    Duration::from_millis(u64::try_from(millis).unwrap_or_default())
}

fn nonnegative_u32(value: i32) -> u32 {
    u32::try_from(value).unwrap_or_default()
}

#[cfg(feature = "api-23")]
fn positive_u32(value: i32) -> Option<u32> {
    u32::try_from(value).ok().filter(|value| *value > 0)
}
