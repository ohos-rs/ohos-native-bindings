use ohos_sensor_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(Sensor_Accuracy, "Sensor_Accuracy_SENSOR_ACCURACY_")]
pub enum Accuracy {
    Unreliable,
    Low,
    Medium,
    High,
}
