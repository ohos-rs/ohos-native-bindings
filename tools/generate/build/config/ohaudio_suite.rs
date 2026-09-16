use once_cell::sync::Lazy;

use super::SysConfig;

/// OHAudioSuite audio creation and processing APIs.
pub const OHAUDIO_SUITE: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-audiosuite-sys",
    headers: vec![
        "ohaudiosuite/native_audio_suite_base.h",
        "ohaudiosuite/native_audio_suite_engine.h",
    ],
    white_list: vec!["OH_.*"],
    block_list: vec![],
    dynamic_library: vec!["ohaudiosuite"],
    extra: "",
});
