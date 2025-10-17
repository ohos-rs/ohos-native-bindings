use once_cell::sync::Lazy;

use super::SysConfig;

pub const ARK_WEB: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-web-sys",
    headers: vec![
        "web/arkweb_error_code.h",
        "web/arkweb_interface.h",
        "web/arkweb_net_error_list.h",
        "web/arkweb_scheme_handler.h",
        "web/arkweb_type.h",
        "web/native_interface_arkweb.h",
    ],
    white_list: vec!["OH_.*", "ArkWeb_.*"],
    block_list: vec![],
    extra: "",
});
