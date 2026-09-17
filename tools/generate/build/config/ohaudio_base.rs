use once_cell::sync::Lazy;

use super::SysConfig;

/// Types shared by the OHAudio and OHAudioSuite native APIs.
pub const OHAUDIO_BASE: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-audio-base-sys",
    headers: vec!["multimedia/native_audio_channel_layout.h"],
    white_list: vec!["OH_.*"],
    block_list: vec![],
    dynamic_library: vec![],
    extra: "",
});
