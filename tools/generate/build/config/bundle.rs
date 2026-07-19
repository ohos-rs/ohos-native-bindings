use once_cell::sync::Lazy;

use super::SysConfig;

pub const BUNDLE: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-bundle-sys",
    headers: vec!["bundle/native_interface_bundle.h"],
    white_list: vec![
        "BundleManager_.*",
        "OH_AbilityResourceInfo_.*",
        "OH_NativeBundle_.*",
    ],
    block_list: vec![],
    dynamic_library: vec!["bundle_ndk.z"],
    extra: "",
});
