use once_cell::sync::Lazy;

use crate::SysConfig;

pub const NATIVE_EFFECT: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-native-effect-sys",
    headers: vec![
        "native_effect/effect_types.h",
        "native_effect/effect_filter.h",
    ],
    white_list: vec!["OH_Filter.*", "Effect.*"],
    // `OH_PixelmapNative` is owned by `ohos-image-native-sys`; regenerating it here would
    // produce a second, incompatible type and make the pixelmap in/out parameters unusable.
    block_list: vec!["OH_PixelmapNative"],
    dynamic_library: vec!["native_effect"],
    extra: "\n\n#[allow(unused_imports)]\nuse ohos_image_native_sys::OH_PixelmapNative;",
});
