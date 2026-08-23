use once_cell::sync::Lazy;

use crate::SysConfig;

pub const LOCATION: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-location-sys",
    headers: vec![
        "LocationKit/oh_location.h",
        "LocationKit/oh_location_type.h",
    ],
    white_list: vec!["OH_Location.*", "Location_.*", "LOCATION_.*"],
    block_list: vec![],
    dynamic_library: vec!["location_ndk"],
    extra: "",
});
