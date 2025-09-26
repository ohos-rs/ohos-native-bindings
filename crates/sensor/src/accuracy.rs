use ohos_sensor_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(Sensor_Accuracy, "Sensor_Accuracy_SENSOR_ACCURACY_")]
pub enum Accuracy {
    Unreliable,
    Low,
    Medium,
    High,
}