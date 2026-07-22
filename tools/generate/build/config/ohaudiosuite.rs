use once_cell::sync::Lazy;

use super::SysConfig;

pub const AUDIO_SUITE: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-audio-suite-sys",
    headers: vec![
        "ohaudiosuite/native_audio_suite_base.h",
        "ohaudiosuite/native_audio_suite_engine.h",
    ],
    // `OH_.*` also covers the destructors, which live under `OH_AudioSuiteEngine_*` rather than
    // each handle's own prefix (`OH_AudioSuitePipeline`/`OH_AudioNode` have no functions of their
    // own). `EQUALIZER_BAND_NUM` is a macro, so it is not prefixed with its enum type name.
    white_list: vec!["OH_.*", "EQUALIZER_BAND_NUM"],
    block_list: vec![],
    dynamic_library: vec!["ohaudiosuite"],
    extra: "",
});
