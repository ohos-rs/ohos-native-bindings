use once_cell::sync::Lazy;

use super::SysConfig;

pub const SENSORS: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-sensor-sys",
    headers: vec!["sensors/oh_sensor.h", "sensors/oh_sensor_type.h"],
    white_list: vec!["OH_.*"],
    block_list: vec![],
    dynamic_library: vec!["ohsensor"],
    extra: "",
});

pub const VIBRATOR: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-vibrator-sys",
    headers: vec!["sensors/vibrator.h", "sensors/vibrator_type.h"],
    white_list: vec!["OH_.*"],
    block_list: vec![],
    dynamic_library: vec!["ohvibrator.z"],
    extra: "",
});
