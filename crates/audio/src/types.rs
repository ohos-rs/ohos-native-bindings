use ohos_audio_sys as sys;

/// PCM sample format (`@since 10`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SampleFormat {
    U8,
    S16Le,
    S24Le,
    S32Le,
    #[cfg(feature = "api-17")]
    F32Le,
    Other(u32),
}

impl SampleFormat {
    pub const fn to_raw(self) -> sys::OH_AudioStream_SampleFormat {
        match self {
            Self::U8 => sys::OH_AudioStream_SampleFormat_AUDIOSTREAM_SAMPLE_U8,
            Self::S16Le => sys::OH_AudioStream_SampleFormat_AUDIOSTREAM_SAMPLE_S16LE,
            Self::S24Le => sys::OH_AudioStream_SampleFormat_AUDIOSTREAM_SAMPLE_S24LE,
            Self::S32Le => sys::OH_AudioStream_SampleFormat_AUDIOSTREAM_SAMPLE_S32LE,
            #[cfg(feature = "api-17")]
            Self::F32Le => sys::OH_AudioStream_SampleFormat_AUDIOSTREAM_SAMPLE_F32LE,
            Self::Other(raw) => raw,
        }
    }

    #[allow(non_upper_case_globals)]
    pub const fn from_raw(raw: sys::OH_AudioStream_SampleFormat) -> Self {
        match raw {
            sys::OH_AudioStream_SampleFormat_AUDIOSTREAM_SAMPLE_U8 => Self::U8,
            sys::OH_AudioStream_SampleFormat_AUDIOSTREAM_SAMPLE_S16LE => Self::S16Le,
            sys::OH_AudioStream_SampleFormat_AUDIOSTREAM_SAMPLE_S24LE => Self::S24Le,
            sys::OH_AudioStream_SampleFormat_AUDIOSTREAM_SAMPLE_S32LE => Self::S32Le,
            #[cfg(feature = "api-17")]
            sys::OH_AudioStream_SampleFormat_AUDIOSTREAM_SAMPLE_F32LE => Self::F32Le,
            other => Self::Other(other),
        }
    }
}

/// Audio encoding type (`@since 10`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EncodingType {
    Raw,
    AudioVivid,
    #[cfg(feature = "api-19")]
    EAc3,
    Other(u32),
}

impl EncodingType {
    pub const fn to_raw(self) -> sys::OH_AudioStream_EncodingType {
        match self {
            Self::Raw => sys::OH_AudioStream_EncodingType_AUDIOSTREAM_ENCODING_TYPE_RAW,
            Self::AudioVivid => {
                sys::OH_AudioStream_EncodingType_AUDIOSTREAM_ENCODING_TYPE_AUDIOVIVID
            }
            #[cfg(feature = "api-19")]
            Self::EAc3 => sys::OH_AudioStream_EncodingType_AUDIOSTREAM_ENCODING_TYPE_E_AC3,
            Self::Other(raw) => raw,
        }
    }

    #[allow(non_upper_case_globals)]
    pub const fn from_raw(raw: sys::OH_AudioStream_EncodingType) -> Self {
        match raw {
            sys::OH_AudioStream_EncodingType_AUDIOSTREAM_ENCODING_TYPE_RAW => Self::Raw,
            sys::OH_AudioStream_EncodingType_AUDIOSTREAM_ENCODING_TYPE_AUDIOVIVID => {
                Self::AudioVivid
            }
            #[cfg(feature = "api-19")]
            sys::OH_AudioStream_EncodingType_AUDIOSTREAM_ENCODING_TYPE_E_AC3 => Self::EAc3,
            other => Self::Other(other),
        }
    }
}

/// Audio stream usage scenario (`@since 10`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamUsage {
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
    Other(u32),
}

impl StreamUsage {
    pub const fn to_raw(self) -> sys::OH_AudioStream_Usage {
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
            Self::Other(raw) => raw,
        }
    }

    #[allow(non_upper_case_globals)]
    pub const fn from_raw(raw: sys::OH_AudioStream_Usage) -> Self {
        match raw {
            sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_UNKNOWN => Self::Unknown,
            sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_MUSIC => Self::Music,
            sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_VOICE_COMMUNICATION => {
                Self::VoiceCommunication
            }
            sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_VOICE_ASSISTANT => Self::VoiceAssistant,
            sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_ALARM => Self::Alarm,
            sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_VOICE_MESSAGE => Self::VoiceMessage,
            sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_RINGTONE => Self::Ringtone,
            sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_NOTIFICATION => Self::Notification,
            sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_ACCESSIBILITY => Self::Accessibility,
            sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_MOVIE => Self::Movie,
            sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_GAME => Self::Game,
            sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_AUDIOBOOK => Self::Audiobook,
            sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_NAVIGATION => Self::Navigation,
            sys::OH_AudioStream_Usage_AUDIOSTREAM_USAGE_VIDEO_COMMUNICATION => {
                Self::VideoCommunication
            }
            other => Self::Other(other),
        }
    }
}

/// Audio stream latency mode (`@since 10`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LatencyMode {
    Normal,
    Fast,
    Other(u32),
}

impl LatencyMode {
    pub const fn to_raw(self) -> sys::OH_AudioStream_LatencyMode {
        match self {
            Self::Normal => sys::OH_AudioStream_LatencyMode_AUDIOSTREAM_LATENCY_MODE_NORMAL,
            Self::Fast => sys::OH_AudioStream_LatencyMode_AUDIOSTREAM_LATENCY_MODE_FAST,
            Self::Other(raw) => raw,
        }
    }

    #[allow(non_upper_case_globals)]
    pub const fn from_raw(raw: sys::OH_AudioStream_LatencyMode) -> Self {
        match raw {
            sys::OH_AudioStream_LatencyMode_AUDIOSTREAM_LATENCY_MODE_NORMAL => Self::Normal,
            sys::OH_AudioStream_LatencyMode_AUDIOSTREAM_LATENCY_MODE_FAST => Self::Fast,
            other => Self::Other(other),
        }
    }
}

/// Audio stream lifecycle state (`@since 10`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamState {
    Invalid,
    New,
    Prepared,
    Running,
    Stopped,
    Released,
    Paused,
    Other(i32),
}

impl StreamState {
    #[allow(non_upper_case_globals)]
    pub const fn from_raw(raw: sys::OH_AudioStream_State) -> Self {
        match raw {
            sys::OH_AudioStream_State_AUDIOSTREAM_STATE_INVALID => Self::Invalid,
            sys::OH_AudioStream_State_AUDIOSTREAM_STATE_NEW => Self::New,
            sys::OH_AudioStream_State_AUDIOSTREAM_STATE_PREPARED => Self::Prepared,
            sys::OH_AudioStream_State_AUDIOSTREAM_STATE_RUNNING => Self::Running,
            sys::OH_AudioStream_State_AUDIOSTREAM_STATE_STOPPED => Self::Stopped,
            sys::OH_AudioStream_State_AUDIOSTREAM_STATE_RELEASED => Self::Released,
            sys::OH_AudioStream_State_AUDIOSTREAM_STATE_PAUSED => Self::Paused,
            other => Self::Other(other),
        }
    }
}

/// Audio interruption focus mode (`@since 12`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterruptMode {
    Share,
    Independent,
}

impl InterruptMode {
    pub const fn to_raw(self) -> sys::OH_AudioInterrupt_Mode {
        match self {
            Self::Share => sys::OH_AudioInterrupt_Mode_AUDIOSTREAM_INTERRUPT_MODE_SHARE,
            Self::Independent => sys::OH_AudioInterrupt_Mode_AUDIOSTREAM_INTERRUPT_MODE_INDEPENDENT,
        }
    }
}

/// Interrupt force type delivered in audio interrupt callbacks (`@since 10`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterruptForceType {
    Force,
    Share,
    Other(u32),
}

impl InterruptForceType {
    #[allow(non_upper_case_globals)]
    pub const fn from_raw(raw: sys::OH_AudioInterrupt_ForceType) -> Self {
        match raw {
            sys::OH_AudioInterrupt_ForceType_AUDIOSTREAM_INTERRUPT_FORCE => Self::Force,
            sys::OH_AudioInterrupt_ForceType_AUDIOSTREAM_INTERRUPT_SHARE => Self::Share,
            other => Self::Other(other),
        }
    }
}

/// Interrupt hint delivered in audio interrupt callbacks (`@since 10`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterruptHint {
    None,
    Resume,
    Pause,
    Stop,
    Duck,
    Unduck,
    #[cfg(feature = "api-20")]
    Mute,
    #[cfg(feature = "api-20")]
    Unmute,
    Other(u32),
}

impl InterruptHint {
    #[allow(non_upper_case_globals)]
    pub const fn from_raw(raw: sys::OH_AudioInterrupt_Hint) -> Self {
        match raw {
            sys::OH_AudioInterrupt_Hint_AUDIOSTREAM_INTERRUPT_HINT_NONE => Self::None,
            sys::OH_AudioInterrupt_Hint_AUDIOSTREAM_INTERRUPT_HINT_RESUME => Self::Resume,
            sys::OH_AudioInterrupt_Hint_AUDIOSTREAM_INTERRUPT_HINT_PAUSE => Self::Pause,
            sys::OH_AudioInterrupt_Hint_AUDIOSTREAM_INTERRUPT_HINT_STOP => Self::Stop,
            sys::OH_AudioInterrupt_Hint_AUDIOSTREAM_INTERRUPT_HINT_DUCK => Self::Duck,
            sys::OH_AudioInterrupt_Hint_AUDIOSTREAM_INTERRUPT_HINT_UNDUCK => Self::Unduck,
            #[cfg(feature = "api-20")]
            sys::OH_AudioInterrupt_Hint_AUDIOSTREAM_INTERRUPT_HINT_MUTE => Self::Mute,
            #[cfg(feature = "api-20")]
            sys::OH_AudioInterrupt_Hint_AUDIOSTREAM_INTERRUPT_HINT_UNMUTE => Self::Unmute,
            other => Self::Other(other),
        }
    }
}

/// Audio effect mode (`@since 12`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioEffectMode {
    None,
    Default,
    Other(u32),
}

impl AudioEffectMode {
    pub const fn to_raw(self) -> sys::OH_AudioStream_AudioEffectMode {
        match self {
            Self::None => sys::OH_AudioStream_AudioEffectMode_EFFECT_NONE,
            Self::Default => sys::OH_AudioStream_AudioEffectMode_EFFECT_DEFAULT,
            Self::Other(raw) => raw,
        }
    }

    #[allow(non_upper_case_globals)]
    pub const fn from_raw(raw: sys::OH_AudioStream_AudioEffectMode) -> Self {
        match raw {
            sys::OH_AudioStream_AudioEffectMode_EFFECT_NONE => Self::None,
            sys::OH_AudioStream_AudioEffectMode_EFFECT_DEFAULT => Self::Default,
            other => Self::Other(other),
        }
    }
}

/// Playback capture privacy policy (`@since 12`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrivacyType {
    Public,
    Private,
    #[cfg(feature = "api-21")]
    Shared,
    Other(u32),
}

impl PrivacyType {
    pub const fn to_raw(self) -> sys::OH_AudioStream_PrivacyType {
        match self {
            Self::Public => sys::OH_AudioStream_PrivacyType_AUDIO_STREAM_PRIVACY_TYPE_PUBLIC,
            Self::Private => sys::OH_AudioStream_PrivacyType_AUDIO_STREAM_PRIVACY_TYPE_PRIVATE,
            #[cfg(feature = "api-21")]
            Self::Shared => sys::OH_AudioStream_PrivacyType_AUDIO_STREAM_PRIVACY_TYPE_SHARED,
            Self::Other(raw) => raw,
        }
    }

    #[allow(non_upper_case_globals)]
    pub const fn from_raw(raw: sys::OH_AudioStream_PrivacyType) -> Self {
        match raw {
            sys::OH_AudioStream_PrivacyType_AUDIO_STREAM_PRIVACY_TYPE_PUBLIC => Self::Public,
            sys::OH_AudioStream_PrivacyType_AUDIO_STREAM_PRIVACY_TYPE_PRIVATE => Self::Private,
            #[cfg(feature = "api-21")]
            sys::OH_AudioStream_PrivacyType_AUDIO_STREAM_PRIVACY_TYPE_SHARED => Self::Shared,
            other => Self::Other(other),
        }
    }
}

/// Output device change reason (`@since 11`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceChangeReason {
    Unknown,
    NewDeviceAvailable,
    OldDeviceUnavailable,
    Overrode,
    #[cfg(feature = "api-20")]
    SessionActivated,
    #[cfg(feature = "api-20")]
    StreamPriorityChanged,
    Other(u32),
}

impl DeviceChangeReason {
    #[allow(non_upper_case_globals)]
    pub const fn from_raw(raw: sys::OH_AudioStream_DeviceChangeReason) -> Self {
        match raw {
            sys::OH_AudioStream_DeviceChangeReason_REASON_UNKNOWN => Self::Unknown,
            sys::OH_AudioStream_DeviceChangeReason_REASON_NEW_DEVICE_AVAILABLE => {
                Self::NewDeviceAvailable
            }
            sys::OH_AudioStream_DeviceChangeReason_REASON_OLD_DEVICE_UNAVAILABLE => {
                Self::OldDeviceUnavailable
            }
            sys::OH_AudioStream_DeviceChangeReason_REASON_OVERRODE => Self::Overrode,
            #[cfg(feature = "api-20")]
            sys::OH_AudioStream_DeviceChangeReason_REASON_SESSION_ACTIVATED => {
                Self::SessionActivated
            }
            #[cfg(feature = "api-20")]
            sys::OH_AudioStream_DeviceChangeReason_REASON_STREAM_PRIORITY_CHANGED => {
                Self::StreamPriorityChanged
            }
            other => Self::Other(other),
        }
    }
}

/// Return value from a renderer write-data callback (`@since 12`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioDataCallbackResult {
    /// Buffer was filled with playable audio data.
    Valid,
    /// Buffer does not contain valid audio data and should not be played.
    Invalid,
}

impl AudioDataCallbackResult {
    pub const fn to_raw(self) -> sys::OH_AudioData_Callback_Result {
        match self {
            Self::Valid => sys::OH_AudioData_Callback_Result_AUDIO_DATA_CALLBACK_RESULT_VALID,
            Self::Invalid => sys::OH_AudioData_Callback_Result_AUDIO_DATA_CALLBACK_RESULT_INVALID,
        }
    }
}

/// Audio volume mode (`@since 19`).
#[cfg(feature = "api-19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VolumeMode {
    SystemGlobal,
    AppIndividual,
}

#[cfg(feature = "api-19")]
impl VolumeMode {
    pub const fn to_raw(self) -> sys::OH_AudioStream_VolumeMode {
        match self {
            Self::SystemGlobal => {
                sys::OH_AudioStream_VolumeMode_AUDIOSTREAM_VOLUMEMODE_SYSTEM_GLOBAL
            }
            Self::AppIndividual => {
                sys::OH_AudioStream_VolumeMode_AUDIOSTREAM_VOLUMEMODE_APP_INDIVIDUAL
            }
        }
    }
}

/// Audio latency component queried via `AudioRenderer::latency` (`@since 23`).
#[cfg(feature = "api-23")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LatencyType {
    All,
    Software,
    Hardware,
}

#[cfg(feature = "api-23")]
impl LatencyType {
    pub const fn to_raw(self) -> sys::OH_AudioStream_LatencyType {
        match self {
            Self::All => sys::OH_AudioStream_LatencyType_AUDIOSTREAM_LATENCY_TYPE_ALL,
            Self::Software => sys::OH_AudioStream_LatencyType_AUDIOSTREAM_LATENCY_TYPE_SOFTWARE,
            Self::Hardware => sys::OH_AudioStream_LatencyType_AUDIOSTREAM_LATENCY_TYPE_HARDWARE,
        }
    }
}

/// Audio session concurrency strategy (`@since 12`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioSessionConcurrencyMode {
    /// System default audio focus behavior.
    Default,
    /// Mix with other apps' audio streams.
    MixWithOthers,
    /// Duck (lower volume of) other apps' audio streams.
    DuckOthers,
    /// Pause other apps' audio streams.
    PauseOthers,
}

impl AudioSessionConcurrencyMode {
    pub const fn to_raw(self) -> sys::OH_AudioSession_ConcurrencyMode {
        match self {
            Self::Default => sys::OH_AudioSession_ConcurrencyMode_CONCURRENCY_DEFAULT,
            Self::MixWithOthers => sys::OH_AudioSession_ConcurrencyMode_CONCURRENCY_MIX_WITH_OTHERS,
            Self::DuckOthers => sys::OH_AudioSession_ConcurrencyMode_CONCURRENCY_DUCK_OTHERS,
            Self::PauseOthers => sys::OH_AudioSession_ConcurrencyMode_CONCURRENCY_PAUSE_OTHERS,
        }
    }
}

/// Reason an activated audio session was deactivated by the system (`@since 12`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioSessionDeactivatedReason {
    LowerPriority,
    Timeout,
    Other(u32),
}

impl AudioSessionDeactivatedReason {
    #[allow(non_upper_case_globals)]
    pub const fn from_raw(raw: sys::OH_AudioSession_DeactivatedReason) -> Self {
        match raw {
            sys::OH_AudioSession_DeactivatedReason_DEACTIVATED_LOWER_PRIORITY => {
                Self::LowerPriority
            }
            sys::OH_AudioSession_DeactivatedReason_DEACTIVATED_TIMEOUT => Self::Timeout,
            other => Self::Other(other),
        }
    }
}

/// Audio session scene (`@since 20`).
#[cfg(feature = "api-20")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioSessionScene {
    Media,
    Game,
    VoiceCommunication,
}

#[cfg(feature = "api-20")]
impl AudioSessionScene {
    pub const fn to_raw(self) -> sys::OH_AudioSession_Scene {
        match self {
            Self::Media => sys::OH_AudioSession_Scene_AUDIO_SESSION_SCENE_MEDIA,
            Self::Game => sys::OH_AudioSession_Scene_AUDIO_SESSION_SCENE_GAME,
            Self::VoiceCommunication => {
                sys::OH_AudioSession_Scene_AUDIO_SESSION_SCENE_VOICE_COMMUNICATION
            }
        }
    }
}

/// Hint reported when an audio session state changes (`@since 20`).
#[cfg(feature = "api-20")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioSessionStateChangeHint {
    Resume,
    Pause,
    Stop,
    TimeOutStop,
    Duck,
    Unduck,
    #[cfg(feature = "api-23")]
    MuteSuggestion,
    #[cfg(feature = "api-23")]
    UnmuteSuggestion,
    #[cfg(feature = "api-24")]
    Mute,
    #[cfg(feature = "api-24")]
    Unmute,
    Other(u32),
}

#[cfg(feature = "api-20")]
impl AudioSessionStateChangeHint {
    #[allow(non_upper_case_globals)]
    pub const fn from_raw(raw: sys::OH_AudioSession_StateChangeHint) -> Self {
        match raw {
            sys::OH_AudioSession_StateChangeHint_AUDIO_SESSION_STATE_CHANGE_HINT_RESUME => {
                Self::Resume
            }
            sys::OH_AudioSession_StateChangeHint_AUDIO_SESSION_STATE_CHANGE_HINT_PAUSE => {
                Self::Pause
            }
            sys::OH_AudioSession_StateChangeHint_AUDIO_SESSION_STATE_CHANGE_HINT_STOP => Self::Stop,
            sys::OH_AudioSession_StateChangeHint_AUDIO_SESSION_STATE_CHANGE_HINT_TIME_OUT_STOP => {
                Self::TimeOutStop
            }
            sys::OH_AudioSession_StateChangeHint_AUDIO_SESSION_STATE_CHANGE_HINT_DUCK => Self::Duck,
            sys::OH_AudioSession_StateChangeHint_AUDIO_SESSION_STATE_CHANGE_HINT_UNDUCK => {
                Self::Unduck
            }
            #[cfg(feature = "api-23")]
            sys::OH_AudioSession_StateChangeHint_AUDIO_SESSION_STATE_CHANGE_HINT_MUTE_SUGGESTION => {
                Self::MuteSuggestion
            }
            #[cfg(feature = "api-23")]
            sys::OH_AudioSession_StateChangeHint_AUDIO_SESSION_STATE_CHANGE_HINT_UNMUTE_SUGGESTION => {
                Self::UnmuteSuggestion
            }
            #[cfg(feature = "api-24")]
            sys::OH_AudioSession_StateChangeHint_AUDIO_SESSION_STATE_CHANGE_HINT_MUTE => Self::Mute,
            #[cfg(feature = "api-24")]
            sys::OH_AudioSession_StateChangeHint_AUDIO_SESSION_STATE_CHANGE_HINT_UNMUTE => {
                Self::Unmute
            }
            other => Self::Other(other),
        }
    }
}
