use once_cell::sync::Lazy;

use crate::SysConfig;

// BasicServicesKit ships six independent libraries; each gets its own sys crate.
// No C type is shared across the headers, so splitting is safe.

pub const COMMON_EVENT: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-common-event-sys",
    headers: vec![
        "BasicServicesKit/oh_commonevent.h",
        "BasicServicesKit/oh_commonevent_support.h",
    ],
    white_list: vec![
        "OH_CommonEvent_.*",
        "CommonEvent_.*",
        "COMMON_EVENT_.*",
        "COMMONEVENT_ERR_.*",
    ],
    block_list: vec![],
    dynamic_library: vec!["ohcommonevent"],
    extra: "",
});

pub const BATTERY_INFO: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-battery-info-sys",
    headers: vec!["BasicServicesKit/ohbattery_info.h"],
    white_list: vec![
        "OH_BatteryInfo_.*",
        "BatteryInfo_.*",
        "PLUGGED_TYPE_.*",
        "COMMON_EVENT_KEY_.*",
    ],
    block_list: vec![],
    dynamic_library: vec!["ohbattery_info"],
    extra: "",
});

pub const PRINT: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-print-sys",
    headers: vec!["BasicServicesKit/ohprint.h"],
    white_list: vec![
        "OH_Print_.*",
        "OH_PRINT_.*",
        "Print_.*",
        "PRINT_.*",
        "PRINTER_.*",
        "COLOR_MODE_.*",
        "DUPLEX_MODE_.*",
        "ORIENTATION_MODE_.*",
        "DOCUMENT_FORMAT_.*",
    ],
    block_list: vec![],
    dynamic_library: vec!["ohprint"],
    extra: "",
});

pub const SCAN: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-scan-sys",
    headers: vec!["BasicServicesKit/ohscan.h"],
    white_list: vec!["OH_Scan_.*", "Scan_.*", "SCAN_ERROR_.*"],
    block_list: vec![],
    dynamic_library: vec!["ohscan"],
    extra: "",
});

pub const OS_ACCOUNT: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-os-account-sys",
    headers: vec![
        "BasicServicesKit/os_account.h",
        "BasicServicesKit/os_account_common.h",
    ],
    white_list: vec!["OH_OsAccount_.*", "OsAccount_.*", "OS_ACCOUNT_.*"],
    block_list: vec![],
    dynamic_library: vec!["os_account_ndk"],
    extra: "",
});

pub const TIME_SERVICE: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-time-service-sys",
    headers: vec!["BasicServicesKit/time_service.h"],
    white_list: vec!["OH_TimeService_.*", "TimeService_.*", "TIMESERVICE_ERR_.*"],
    block_list: vec![],
    dynamic_library: vec!["time_service_ndk"],
    extra: "",
});
