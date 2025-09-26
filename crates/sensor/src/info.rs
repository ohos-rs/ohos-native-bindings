use crate::SensorType;

#[derive(Debug, Clone)]
pub struct SensorInfo {
    pub sensor_type: SensorType,
    pub sensor_name: String,
    pub sensor_vendor_name: String,
    pub sensor_resolution: f32,
    pub sensor_min_sampling_interval: i64,
    pub sensor_max_sampling_interval: i64,
}
