use once_cell::sync::Lazy;

use crate::SysConfig;

pub const PASTEBOARD: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-pasteboard-sys",
    headers: vec![
        "database/pasteboard/oh_pasteboard.h",
        "database/pasteboard/oh_pasteboard_err_code.h",
    ],
    white_list: vec!["OH_.*"],
    block_list: vec![],
    extra: "",
});
