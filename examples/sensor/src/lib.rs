use napi_derive_ohos::napi;

use ohos_sensor_binding::{SensorSubscriber, SensorType};

#[napi]
pub fn sensor_test() {
    let subscriber = SensorSubscriber::new(SensorType::Accelerometer, 1000);
    subscriber.subscribe(|event| {
        println!("sensor_test: {:?}", event);
    });
}
