use once_cell::sync::Lazy;

use crate::SysConfig;

pub const UDMF: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-udmf-sys",
    headers: vec![
        "database/udmf/udmf.h",
        "database/udmf/udmf_err_code.h",
        "database/udmf/udmf_meta.h",
        "database/udmf/uds.h",
        "database/udmf/utd.h",
    ],
    white_list: vec!["OH_.*"],
    block_list: vec![],
    dynamic_library: vec!["udmf"],
    extra: "",
});
