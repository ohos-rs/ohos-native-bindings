use once_cell::sync::Lazy;

use super::SysConfig;

pub const QOS: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-qos-sys",
    headers: vec!["qos/qos.h"],
    white_list: vec!["OH_QoS.*", "QoS_Level.*"],
    block_list: vec![],
    dynamic_library: vec!["qos"],
    extra: "",
});
