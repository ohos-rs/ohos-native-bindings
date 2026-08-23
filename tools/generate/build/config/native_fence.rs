use once_cell::sync::Lazy;

use crate::SysConfig;

pub const NATIVE_FENCE: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-native-fence-sys",
    headers: vec!["native_fence/native_fence.h"],
    // The whole surface is four `OH_NativeFence_*` functions; there are no types or constants.
    // `OH_NativeFence_Close` is the release call and matches the same prefix.
    white_list: vec!["OH_NativeFence_.*"],
    block_list: vec![],
    dynamic_library: vec!["native_fence"],
    extra: "",
});
