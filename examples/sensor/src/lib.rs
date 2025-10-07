use napi_derive_ohos::napi;

use ohos_hilog_binding::hilog_info;
use ohos_sensor_binding::{SensorSubscriber, SensorType};

#[napi]
pub fn sensor_test() {
    let subscriber = Box::leak(Box::new(SensorSubscriber::new(
        SensorType::Accelerometer,
        200000000,
    )));
    let _ = subscriber.subscribe(|event| hilog_info!("sensor: {:?}", event));
}
