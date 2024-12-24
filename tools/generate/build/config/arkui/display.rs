use once_cell::sync::Lazy;

use crate::SysConfig;

pub const DISPLAY: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-display-sys",
    headers: vec![
        "window_manager/oh_display_info.h",
        "window_manager/oh_display_manager.h",
    ],
    white_list: vec!["NativeDisplayManager_.*", "OH_NativeDisplayManager_.*"],
    block_list: vec![],
    extra: "",
});
