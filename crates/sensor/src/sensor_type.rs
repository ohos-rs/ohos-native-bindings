#![allow(non_upper_case_globals)]
use ohos_enum_derive::EnumFrom;
use ohos_sensor_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumFrom)]
#[config(Sensor_Type, "Sensor_Type_SENSOR_TYPE_", i32)]
#[repr(u32)]
pub enum SensorType {
    Accelerometer = 1,
    Gyroscope = 2,
    AmbientLight = 5,
    MagneticField = 6,
    Barometer = 8,
    Hall = 10,
    Proximity = 12,
    Orientation = 256,
    Gravity = 257,
    #[cfg(feature = "api-13")]
    LinearAcceleration = 258,
    RotationVector = 259,
    #[cfg(feature = "api-13")]
    GameRotationVector = 262,
    PedometerDetection = 265,
    Pedometer = 266,
    HeartRate = 278,
    #[fallback]
    Other(Sensor_Type),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Accuracy;

    #[test]
    fn preserves_unmapped_sensor_types_and_falls_back_accuracy() {
        assert_eq!(SensorType::from(9999), SensorType::Other(9999));
        assert_eq!(Sensor_Type::from(SensorType::Other(9999)), 9999);
        assert_eq!(SensorType::try_from_raw(9999), None);
        assert_eq!(Accuracy::from(99), Accuracy::Unreliable);
    }
}
