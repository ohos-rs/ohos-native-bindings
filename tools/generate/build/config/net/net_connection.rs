use once_cell::sync::Lazy;

use super::super::SysConfig;

pub const NET_CONNECTION: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-net-connection-sys",
    headers: vec![
        "network/netmanager/net_connection_type.h",
        "network/netmanager/net_connection.h",
    ],
    white_list: vec!["OH_NetConn_.*", "NetConn_.*", "NETCONN_.*"],
    block_list: vec![],
    dynamic_library: vec!["net_connection"],
    extra: "",
});
