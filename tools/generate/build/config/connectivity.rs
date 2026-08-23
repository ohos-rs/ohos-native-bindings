use once_cell::sync::Lazy;

use crate::SysConfig;

pub const BLUETOOTH: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-bluetooth-sys",
    headers: vec!["ConnectivityKit/bluetooth/oh_bluetooth.h"],
    white_list: vec!["OH_Bluetooth.*", "Bluetooth_.*", "BLUETOOTH_.*"],
    block_list: vec![],
    dynamic_library: vec!["bluetooth_ndk"],
    extra: "",
});

pub const WIFI: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-wifi-sys",
    headers: vec!["ConnectivityKit/wifi/oh_wifi.h"],
    white_list: vec!["OH_Wifi.*", "OH_WIFI_.*", "Wifi_.*", "WIFI_.*"],
    block_list: vec![],
    dynamic_library: vec!["wifi_ndk"],
    extra: "",
});
