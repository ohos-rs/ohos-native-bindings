use once_cell::sync::Lazy;

use crate::SysConfig;

// `OH_NativeColorSpaceManager` is an opaque handle that several kits forward-declare, so
// `ohos-image-native-sys`, `ohos-input-method-sys`, `ohos-native-drawing-sys` and
// `ohos-native-pasteboard-sys` each already generate their own. This crate owns the kit that
// actually defines the API; the duplicates stay as they are, matching how the repository
// already handles this handle.
pub const NATIVE_COLOR_SPACE_MANAGER: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-native-color-space-manager-sys",
    headers: vec!["native_color_space_manager/native_color_space_manager.h"],
    // `OH_.*` covers all six functions (incl. the regularly named `_Destroy`).
    // The three public types are named without that prefix, so they are listed
    // explicitly rather than relying on bindgen's recursive allowlist.
    white_list: vec![
        "OH_.*",
        "ColorSpaceName",
        "ColorSpacePrimaries",
        "WhitePointArray",
    ],
    block_list: vec![],
    dynamic_library: vec!["native_color_space_manager"],
    extra: "",
});
