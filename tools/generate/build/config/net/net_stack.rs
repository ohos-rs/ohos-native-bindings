use once_cell::sync::Lazy;

use super::super::SysConfig;

pub const NET_STACK: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-net-stack-sys",
    headers: vec![
        "stdint.h",
        "stddef.h",
        "network/netstack/net_http_type.h",
        "network/netstack/net_http.h",
        "network/netstack/net_ssl/net_ssl_c_type.h",
        "network/netstack/net_ssl/net_ssl_c.h",
        "network/netstack/net_websocket_type.h",
        "network/netstack/net_websocket.h",
    ],
    white_list: vec![
        "OH_Http_.*",
        "Http_.*",
        "OH_HTTP_.*",
        "HTTP_.*",
        "NET_HTTP.*",
        "OH_NetStack_.*",
        "OH_Netstack_.*",
        "NetStack_.*",
        "NETSTACK_.*",
        "PUBLIC_KEY",
        "SHA_256",
        "OH_WebSocketClient_.*",
        "WebSocket_.*",
        "WEBSOCKET_.*",
        "E_BASE",
    ],
    block_list: vec![],
    dynamic_library: vec!["net_http", "net_ssl", "net_websocket"],
    extra: "",
});
