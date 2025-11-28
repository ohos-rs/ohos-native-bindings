#![allow(non_upper_case_globals)]
use ohos_sensor_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
}

impl From<Sensor_Type> for SensorType {
    fn from(value: Sensor_Type) -> Self {
        match value {
            Sensor_Type_SENSOR_TYPE_ACCELEROMETER => Self::Accelerometer,
            Sensor_Type_SENSOR_TYPE_GYROSCOPE => Self::Gyroscope,
            Sensor_Type_SENSOR_TYPE_AMBIENT_LIGHT => Self::AmbientLight,
            Sensor_Type_SENSOR_TYPE_MAGNETIC_FIELD => Self::MagneticField,
            Sensor_Type_SENSOR_TYPE_BAROMETER => Self::Barometer,
            Sensor_Type_SENSOR_TYPE_HALL => Self::Hall,
            Sensor_Type_SENSOR_TYPE_PROXIMITY => Self::Proximity,
            Sensor_Type_SENSOR_TYPE_ORIENTATION => Self::Orientation,
            Sensor_Type_SENSOR_TYPE_GRAVITY => Self::Gravity,
            #[cfg(feature = "api-13")]
            Sensor_Type_SENSOR_TYPE_LINEAR_ACCELERATION => Self::LinearAcceleration,
            Sensor_Type_SENSOR_TYPE_ROTATION_VECTOR => Self::RotationVector,
            #[cfg(feature = "api-13")]
            Sensor_Type_SENSOR_TYPE_GAME_ROTATION_VECTOR => Self::GameRotationVector,
            Sensor_Type_SENSOR_TYPE_PEDOMETER_DETECTION => Self::PedometerDetection,
            Sensor_Type_SENSOR_TYPE_PEDOMETER => Self::Pedometer,
            Sensor_Type_SENSOR_TYPE_HEART_RATE => Self::HeartRate,
            _ => unreachable!("Invalid sensor type: {}", value),
        }
    }
}

impl From<SensorType> for Sensor_Type {
    fn from(value: SensorType) -> Self {
        match value {
            SensorType::Accelerometer => Sensor_Type_SENSOR_TYPE_ACCELEROMETER,
            SensorType::Gyroscope => Sensor_Type_SENSOR_TYPE_GYROSCOPE,
            SensorType::AmbientLight => Sensor_Type_SENSOR_TYPE_AMBIENT_LIGHT,
            SensorType::MagneticField => Sensor_Type_SENSOR_TYPE_MAGNETIC_FIELD,
            SensorType::Barometer => Sensor_Type_SENSOR_TYPE_BAROMETER,
            SensorType::Hall => Sensor_Type_SENSOR_TYPE_HALL,
            SensorType::Proximity => Sensor_Type_SENSOR_TYPE_PROXIMITY,
            SensorType::Orientation => Sensor_Type_SENSOR_TYPE_ORIENTATION,
            SensorType::Gravity => Sensor_Type_SENSOR_TYPE_GRAVITY,
            #[cfg(feature = "api-13")]
            SensorType::LinearAcceleration => Sensor_Type_SENSOR_TYPE_LINEAR_ACCELERATION,
            SensorType::RotationVector => Sensor_Type_SENSOR_TYPE_ROTATION_VECTOR,
            #[cfg(feature = "api-13")]
            SensorType::GameRotationVector => Sensor_Type_SENSOR_TYPE_GAME_ROTATION_VECTOR,
            SensorType::PedometerDetection => Sensor_Type_SENSOR_TYPE_PEDOMETER_DETECTION,
            SensorType::Pedometer => Sensor_Type_SENSOR_TYPE_PEDOMETER,
            SensorType::HeartRate => Sensor_Type_SENSOR_TYPE_HEART_RATE,
        }
    }
}
