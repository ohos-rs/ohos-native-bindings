use crate::SysConfig;
use once_cell::sync::Lazy;
pub const AVPLAYER: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-avplayer-sys",
    headers: vec![
        "multimedia/player_framework/avplayer.h",
        "multimedia/player_framework/native_avformat.h",
    ],
    white_list: vec![
        "OH_AVPlayer.*",
        "AVPlayer.*",
        "OH_PLAYER_.*",
        "AVPlaybackSpeed.*",
        "OH_AVErrCode.*",
        "AV_ERR_.*",
        "OH_AVFormat_Get(Int|Long|Double|String)Value",
    ],
    block_list: vec!["OHNativeWindow.*", "NativeWindow.*"],
    dynamic_library: vec!["avplayer", "native_media_core"],
    extra: "\nuse ohos_native_window_sys::OHNativeWindow;",
});
