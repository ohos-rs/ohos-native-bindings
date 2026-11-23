use ohos_audio_sys::*;
use ohos_enum_derive::EnumFrom;

/// PCM sample format (`@since 10`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(
    OH_AudioStream_SampleFormat,
    "OH_AudioStream_SampleFormat_AUDIOSTREAM_SAMPLE_"
)]
pub enum SampleFormat {
    #[suffix("U8")]
    U8,
    #[suffix("S16LE")]
    S16Le,
    #[suffix("S24LE")]
    S24Le,
    #[suffix("S32LE")]
    S32Le,
    #[cfg(feature = "api-17")]
    #[suffix("F32LE")]
    F32Le,
}

/// Audio encoding type (`@since 10`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(
    OH_AudioStream_EncodingType,
    "OH_AudioStream_EncodingType_AUDIOSTREAM_ENCODING_TYPE_"
)]
pub enum EncodingType {
    Raw,
    #[suffix("AUDIOVIVID")]
    AudioVivid,
    #[cfg(feature = "api-19")]
    #[suffix("E_AC3")]
    EAc3,
}

/// Audio stream usage scenario (`@since 10`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(OH_AudioStream_Usage, "OH_AudioStream_Usage_AUDIOSTREAM_USAGE_")]
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
}

/// Audio stream latency mode (`@since 10`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(
    OH_AudioStream_LatencyMode,
    "OH_AudioStream_LatencyMode_AUDIOSTREAM_LATENCY_MODE_"
)]
pub enum LatencyMode {
    Normal,
    Fast,
}

/// Audio stream lifecycle state (`@since 10`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(OH_AudioStream_State, "OH_AudioStream_State_AUDIOSTREAM_STATE_")]
pub enum StreamState {
    Invalid,
    New,
    Prepared,
    Running,
    Stopped,
    Released,
    Paused,
}

/// Audio interruption focus mode (`@since 12`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(
    OH_AudioInterrupt_Mode,
    "OH_AudioInterrupt_Mode_AUDIOSTREAM_INTERRUPT_MODE_"
)]
pub enum InterruptMode {
    Share,
    Independent,
}

/// Interrupt force type delivered in audio interrupt callbacks (`@since 10`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(
    OH_AudioInterrupt_ForceType,
    "OH_AudioInterrupt_ForceType_AUDIOSTREAM_INTERRUPT_"
)]
pub enum InterruptForceType {
    Force,
    Share,
}

/// Interrupt hint delivered in audio interrupt callbacks (`@since 10`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(
    OH_AudioInterrupt_Hint,
    "OH_AudioInterrupt_Hint_AUDIOSTREAM_INTERRUPT_HINT_"
)]
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
}

/// Audio effect mode (`@since 12`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(
    OH_AudioStream_AudioEffectMode,
    "OH_AudioStream_AudioEffectMode_EFFECT_"
)]
pub enum AudioEffectMode {
    None,
    Default,
}

/// Playback capture privacy policy (`@since 12`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(
    OH_AudioStream_PrivacyType,
    "OH_AudioStream_PrivacyType_AUDIO_STREAM_PRIVACY_TYPE_"
)]
pub enum PrivacyType {
    Public,
    Private,
    #[cfg(feature = "api-21")]
    Shared,
}

/// Output device change reason (`@since 11`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(
    OH_AudioStream_DeviceChangeReason,
    "OH_AudioStream_DeviceChangeReason_REASON_"
)]
pub enum DeviceChangeReason {
    Unknown,
    NewDeviceAvailable,
    OldDeviceUnavailable,
    Overrode,
    #[cfg(feature = "api-20")]
    SessionActivated,
    #[cfg(feature = "api-20")]
    StreamPriorityChanged,
}

/// Return value from a renderer write-data callback (`@since 12`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(
    OH_AudioData_Callback_Result,
    "OH_AudioData_Callback_Result_AUDIO_DATA_CALLBACK_RESULT_"
)]
pub enum AudioDataCallbackResult {
    /// Buffer was filled with playable audio data.
    Valid,
    /// Buffer does not contain valid audio data and should not be played.
    Invalid,
}

/// Audio volume mode (`@since 19`).
#[cfg(feature = "api-19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(
    OH_AudioStream_VolumeMode,
    "OH_AudioStream_VolumeMode_AUDIOSTREAM_VOLUMEMODE_"
)]
pub enum VolumeMode {
    SystemGlobal,
    AppIndividual,
}

/// Audio latency component queried via `AudioRenderer::latency` (`@since 23`).
#[cfg(feature = "api-23")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(
    OH_AudioStream_LatencyType,
    "OH_AudioStream_LatencyType_AUDIOSTREAM_LATENCY_TYPE_"
)]
pub enum LatencyType {
    All,
    Software,
    Hardware,
}

/// Audio session concurrency strategy (`@since 12`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(
    OH_AudioSession_ConcurrencyMode,
    "OH_AudioSession_ConcurrencyMode_CONCURRENCY_"
)]
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

/// Reason an activated audio session was deactivated by the system (`@since 12`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(
    OH_AudioSession_DeactivatedReason,
    "OH_AudioSession_DeactivatedReason_DEACTIVATED_"
)]
pub enum AudioSessionDeactivatedReason {
    LowerPriority,
    Timeout,
}

/// Audio session scene (`@since 20`).
#[cfg(feature = "api-20")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(OH_AudioSession_Scene, "OH_AudioSession_Scene_AUDIO_SESSION_SCENE_")]
pub enum AudioSessionScene {
    Media,
    Game,
    VoiceCommunication,
}

/// Hint reported when an audio session state changes (`@since 20`).
#[cfg(feature = "api-20")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[config(
    OH_AudioSession_StateChangeHint,
    "OH_AudioSession_StateChangeHint_AUDIO_SESSION_STATE_CHANGE_HINT_"
)]
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
}
