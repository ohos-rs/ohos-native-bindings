use once_cell::sync::Lazy;

use super::SysConfig;

pub const OHAUDIO: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-audio-sys",
    headers: vec![
        "ohaudio/native_audiocapturer.h",
        "ohaudio/native_audiorenderer.h",
        "ohaudio/native_audiostreambuilder.h",
        "ohaudio/native_audiostream_base.h",
        "ohaudio/native_audio_common.h",
        "ohaudio/native_audio_device_base.h",
        "ohaudio/native_audio_manager.h",
        "ohaudio/native_audio_resource_manager.h",
        "ohaudio/native_audio_routing_manager.h",
        "ohaudio/native_audio_session_manager.h",
        "ohaudio/native_audio_stream_manager.h",
        "ohaudio/native_audio_volume_manager.h",
        "ohaudio/native_audio_accessory_common.h",
        "ohaudio/native_audio_accessory_input_stream_manager.h",
        "ohaudio/native_audio_accessory_manager.h",
        "ohaudio/native_audio_debugging_manager.h",
        "ohaudio/native_audio_device_enhance_manager.h",
        "ohaudio/native_audio_session_base.h",
    ],
    white_list: vec!["OH_.*"],
    block_list: vec!["OH_AudioChannel.*", "OH_AmbAttributeSet.*"],
    dynamic_library: vec!["ohaudio"],
    extra: "\n\npub use ohos_audio_base_sys::*;",
});
