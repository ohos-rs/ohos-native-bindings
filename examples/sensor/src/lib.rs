use std::sync::{LazyLock, Mutex};

use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_hilog_binding::hilog_info;
use ohos_sensor_binding::{SensorSubscriber, SensorType};

static SUBSCRIBER: LazyLock<Mutex<Option<SensorSubscriber>>> = LazyLock::new(|| Mutex::new(None));

#[napi]
pub fn sensor_test() {
    let subscriber = Box::leak(Box::new(SensorSubscriber::new(
        SensorType::Accelerometer,
        200000000,
    )));
    let _ = subscriber.subscribe(|event| hilog_info!("sensor: {:?}", event));
}

/// List all sensors available on this device.
#[napi]
pub fn sensor_list() -> Result<String> {
    let list = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(
        ohos_sensor_binding::get_sensor_list,
    )) {
        Ok(Ok(list)) => list,
        Ok(Err(e)) => return Err(Error::from_reason(e.to_string())),
        Err(_) => return Ok("sensor_list panicked".to_string()),
    };
    let mut out = String::new();
    for info in list {
        out.push_str(&format!(
            "{}: {} ({}) resolution={} interval=[{}, {}]ns\n",
            info.sensor_type as i32,
            info.sensor_name,
            info.sensor_vendor_name,
            info.sensor_resolution,
            info.sensor_min_sampling_interval,
            info.sensor_max_sampling_interval,
        ));
    }
    Ok(out)
}

/// Subscribe to a sensor by type code until unsubscribed.
/// type codes: 1=Accelerometer 2=Gyroscope 5=AmbientLight 6=MagneticField
/// 8=Barometer 10=Hall 12=Proximity 256=Orientation 257=Gravity
/// 258=LinearAcceleration 259=RotationVector 262=GameRotationVector
/// 265=PedometerDetection 266=Pedometer 278=HeartRate
#[napi]
pub fn subscribe_sensor(type_code: i32, sampling_interval: i64) -> Result<()> {
    let sensor_type = SensorType::from(type_code as u32);
    let subscriber = SensorSubscriber::new(sensor_type, sampling_interval);
    let logged_type = sensor_type;
    subscriber
        .subscribe(move |event| {
            hilog_info!("sensor[{:?}]: {:?}", logged_type, event);
        })
        .map_err(|e| Error::from_reason(e.to_string()))?;
    let mut guard = SUBSCRIBER.lock().unwrap();
    *guard = Some(subscriber);
    Ok(())
}

/// Unsubscribe the sensor registered by subscribe_sensor.
#[napi]
pub fn unsubscribe_sensor() -> Result<()> {
    let mut guard = SUBSCRIBER.lock().unwrap();
    if let Some(subscriber) = guard.take() {
        subscriber
            .unsubscribe()
            .map_err(|e| Error::from_reason(e.to_string()))?;
    }
    Ok(())
}
