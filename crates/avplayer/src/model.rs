use std::time::Duration;

use ohos_avplayer_sys::{
    AVPlayerSeekMode, AVPlayerSeekMode_AV_SEEK_CLOSEST, AVPlayerSeekMode_AV_SEEK_NEXT_SYNC,
    AVPlayerSeekMode_AV_SEEK_PREVIOUS_SYNC,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct VideoSize {
    pub width: u32,
    pub height: u32,
}

impl VideoSize {
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub const fn is_empty(self) -> bool {
        self.width == 0 || self.height == 0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AvPlayerSeekMode {
    NextSync,
    PreviousSync,
    #[default]
    Closest,
    #[cfg(feature = "api-23")]
    Continuous,
}

impl AvPlayerSeekMode {
    pub(crate) fn raw(self) -> AVPlayerSeekMode {
        match self {
            Self::NextSync => AVPlayerSeekMode_AV_SEEK_NEXT_SYNC,
            Self::PreviousSync => AVPlayerSeekMode_AV_SEEK_PREVIOUS_SYNC,
            Self::Closest => AVPlayerSeekMode_AV_SEEK_CLOSEST,
            #[cfg(feature = "api-23")]
            Self::Continuous => ohos_avplayer_sys::AVPlayerSeekMode_AV_SEEK_CONTINUOUS,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AvPlayerState {
    #[default]
    Idle,
    Initialized,
    Prepared,
    Playing,
    Paused,
    Stopped,
    Completed,
    Released,
    Error,
    Unknown(u32),
}

impl AvPlayerState {
    #[allow(non_upper_case_globals)]
    pub(crate) fn from_raw(value: u32) -> Self {
        use ohos_avplayer_sys::*;
        match value {
            AVPlayerState_AV_IDLE => Self::Idle,
            AVPlayerState_AV_INITIALIZED => Self::Initialized,
            AVPlayerState_AV_PREPARED => Self::Prepared,
            AVPlayerState_AV_PLAYING => Self::Playing,
            AVPlayerState_AV_PAUSED => Self::Paused,
            AVPlayerState_AV_STOPPED => Self::Stopped,
            AVPlayerState_AV_COMPLETED => Self::Completed,
            AVPlayerState_AV_RELEASED => Self::Released,
            AVPlayerState_AV_ERROR => Self::Error,
            other => Self::Unknown(other),
        }
    }

    pub const fn is_ready(self) -> bool {
        matches!(
            self,
            Self::Prepared | Self::Playing | Self::Paused | Self::Completed
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AvPlayerBuffering {
    Started,
    Ended,
    Percent(u8),
    CachedDuration(Duration),
    Unknown { kind: i32, value: i32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AvPlayerTrackType {
    Audio,
    Video,
    Subtitle,
    TimedMetadata,
    Auxiliary,
    Unknown(u32),
}

#[cfg(feature = "api-23")]
impl AvPlayerTrackType {
    #[allow(non_upper_case_globals)]
    pub(crate) fn from_raw(value: u32) -> Self {
        use ohos_avplayer_sys::*;
        match value {
            OH_MediaType_MEDIA_TYPE_AUD => Self::Audio,
            OH_MediaType_MEDIA_TYPE_VID => Self::Video,
            OH_MediaType_MEDIA_TYPE_SUBTITLE => Self::Subtitle,
            #[cfg(feature = "api-23")]
            OH_MediaType_MEDIA_TYPE_TIMED_METADATA => Self::TimedMetadata,
            #[cfg(feature = "api-23")]
            OH_MediaType_MEDIA_TYPE_AUXILIARY => Self::Auxiliary,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AvPlayerTrack {
    pub index: u32,
    pub track_type: AvPlayerTrackType,
    pub title: Option<String>,
    pub language: Option<String>,
    pub mime_type: Option<String>,
    pub bitrate: Option<u64>,
    pub video_size: Option<VideoSize>,
    pub frame_rate: Option<f64>,
    pub channel_count: Option<u32>,
    pub sample_rate: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum AvPlayerEvent {
    StateChanged(AvPlayerState),
    Position(Duration),
    Duration(Duration),
    Resolution(VideoSize),
    Buffering(AvPlayerBuffering),
    SeekCompleted(Duration),
    PlaybackRateChanged(f32),
    VolumeChanged(f32),
    BitrateChanged(u32),
    AvailableBitrates(Vec<u32>),
    LiveChanged(bool),
    TrackChanged {
        index: i32,
        selected: bool,
    },
    TracksChanged,
    Subtitle {
        text: String,
        start: Duration,
        duration: Duration,
    },
    Ended,
    AudioInterrupted,
    Error(crate::AvPlayerError),
}
