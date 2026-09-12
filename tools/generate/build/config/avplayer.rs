use once_cell::sync::Lazy;

use crate::SysConfig;

pub const AVPLAYER: Lazy<SysConfig> = Lazy::new(|| {
    SysConfig {
        name: "ohos-avplayer-sys",
        headers: vec!["multimedia/player_framework/avplayer.h"],
        white_list: vec![
            "AV.*",
            "DRM_.*",
            "MediaKeySession",
            "OH_AV.*",
            "OH_MEDIA_EVENT_INFO_.*",
            "OH_MD_KEY_.*",
            "OH_PLAYER_.*",
            "Player_MediaKeySystemInfoCallback",
        ],
        block_list: vec![
            "NativeWindow",
            "OHNativeWindow",
            "OH_AudioInterrupt_Mode",
            "OH_AudioStream_AudioEffectMode",
            "OH_AudioStream_Usage",
            "OH_AudioStream_VolumeMode",
        ],
        dynamic_library: vec![
            "avplayer",
            "native_media_core",
            "native_media_codecbase",
            "avmedia_source",
        ],
        extra: "\n\npub use ohos_audio_sys::{OH_AudioInterrupt_Mode, OH_AudioStream_AudioEffectMode, OH_AudioStream_Usage};\n#[cfg(feature = \"api-19\")]\npub use ohos_audio_sys::OH_AudioStream_VolumeMode;\npub use ohos_native_window_sys::{NativeWindow, OHNativeWindow};",
    }
});
