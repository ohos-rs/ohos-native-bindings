use once_cell::sync::Lazy;

use crate::SysConfig;

// Radio and cellular data ship as two libraries but form one kit and one
// `OH_Telephony_` symbol namespace, so they stay in a single crate.
pub const TELEPHONY: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-telephony-sys",
    headers: vec![
        "telephony/core_service/telephony_radio.h",
        "telephony/core_service/telephony_radio_type.h",
        "telephony/cellular_data/telephony_data.h",
    ],
    white_list: vec![
        "OH_Telephony_.*",
        "Telephony_.*",
        "TEL_.*",
        "TELEPHONY_MAX_.*",
    ],
    block_list: vec![],
    dynamic_library: vec!["telephony_radio", "telephony_data"],
    extra: "",
});
