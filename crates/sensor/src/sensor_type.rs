use ohos_sensor_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom, Hash)]
#[enum_from_config(Sensor_Type, "Sensor_Type_SENSOR_TYPE_")]
pub enum SensorType {
    Accelerometer,
    Gyroscope,
    AmbientLight,
    MagneticField,
    Barometer,
    Hall,
    Proximity,
    Orientation,
    Gravity,
    #[cfg(feature = "api-13")]
    LinearAcceleration,
    RotationVector,
    #[cfg(feature = "api-13")]
    GameRotationVector,
    PedometerDetection,
    Pedometer,
    HeartRate,
}
