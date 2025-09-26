use std::{cell::RefCell, collections::HashMap, ptr, ptr::NonNull};

use ohos_sensor_sys::{
    OH_SensorEvent_GetAccuracy, OH_SensorEvent_GetData, OH_SensorEvent_GetTimestamp,
    OH_SensorEvent_GetType, OH_SensorSubscriber_SetCallback, OH_SensorSubscriptionId_SetType,
    OH_Sensor_CreateSubscriber, OH_Sensor_CreateSubscriptionId, OH_Sensor_Subscribe, Sensor_Event,
    Sensor_Subscriber, Sensor_SubscriptionId,
};

use crate::{Accuracy, SensorAttribute, SensorError, SensorType};

#[derive(Debug, Clone)]
pub struct SensorCallbackEvent {
    pub timestamp: i64,
    pub accuracy: Accuracy,
    pub data: Vec<f32>,
    pub sensor_type: SensorType,
}

impl SensorCallbackEvent {
    pub fn new(event: *mut Sensor_Event) -> Result<Self, SensorError> {
        let mut timestamp = 0;
        let ret = unsafe { OH_SensorEvent_GetTimestamp(event, &mut timestamp) };
        if ret != 0 {
            return Err(SensorError::InternalError(ret as _));
        }
        let mut accuracy = 0;
        let ret = unsafe { OH_SensorEvent_GetAccuracy(event, &mut accuracy) };
        if ret != 0 {
            return Err(SensorError::InternalError(ret as _));
        }
        let accuracy = unsafe { OH_SensorEvent_GetAccuracy(event, &mut accuracy) };
        let mut data = ptr::null_mut();
        let mut data_len = 0;
        let ret = unsafe { OH_SensorEvent_GetData(event, &mut data, &mut data_len) };
        if ret != 0 {
            return Err(SensorError::InternalError(ret as _));
        }
        let mut sensor_type = 0;
        let ret = unsafe { OH_SensorEvent_GetType(event, &mut sensor_type) };
        if ret != 0 {
            return Err(SensorError::InternalError(ret as _));
        }
        let sensor_type = unsafe { OH_SensorEvent_GetType(event, &mut sensor_type) };
        Ok(Self {
            timestamp,
            accuracy: Accuracy::from(accuracy as u32),
            data: unsafe { Vec::from_raw_parts(data, data_len as usize, data_len as usize) },
            sensor_type: SensorType::from(sensor_type as u32),
        })
    }
}

/// Sensor subscriber
/// Because of the limitation of the OS-API and FFI, we only support one callback for each sensor type with per-thread.
pub struct SensorSubscriber {
    subscriber: RefCell<Option<NonNull<Sensor_Subscriber>>>,
    id: NonNull<Sensor_SubscriptionId>,
    attribute: SensorAttribute,
    sensor_type: SensorType,
}

thread_local! {
    static SENSOR_SUBSCRIBE_CALLBACK: RefCell<HashMap<SensorType, Box<dyn Fn(SensorCallbackEvent) + 'static>>> = RefCell::new(HashMap::new());
}

impl SensorSubscriber {
    pub fn new(sensor_type: SensorType, sampling_interval: i64) -> Self {
        let id = unsafe { OH_Sensor_CreateSubscriptionId() };
        #[cfg(debug_assertions)]
        assert!(!id.is_null(), "Failed to create sensor subscription id");

        unsafe { OH_SensorSubscriptionId_SetType(id, sensor_type.into()) };

        let attribute = SensorAttribute::new();
        attribute
            .set_sampling_interval(sampling_interval)
            .expect("Failed to set sampling interval");

        Self {
            subscriber: RefCell::new(None),
            id: unsafe { NonNull::new_unchecked(id) },
            attribute,
            sensor_type,
        }
    }

    pub fn subscribe<F>(&self, callback: F) -> Result<(), SensorError>
    where
        F: Fn(SensorCallbackEvent) + 'static,
    {
        SENSOR_SUBSCRIBE_CALLBACK
            .with_borrow_mut(|cb| cb.insert(self.sensor_type, Box::new(callback)));

        let subscriber = unsafe { OH_Sensor_CreateSubscriber() };
        #[cfg(debug_assertions)]
        assert!(!subscriber.is_null(), "Failed to create sensor subscriber");
        let ret = unsafe { OH_SensorSubscriber_SetCallback(subscriber, Some(sensor_callback)) };
        if ret != 0 {
            return Err(SensorError::InternalError(ret as _));
        }
        *self.subscriber.borrow_mut() = Some(unsafe { NonNull::new_unchecked(subscriber) });

        let ret =
            unsafe { OH_Sensor_Subscribe(self.id.as_ptr(), self.attribute.raw(), subscriber) };
        if ret != 0 {
            return Err(SensorError::InternalError(ret as _));
        }
        Ok(())
    }
}

extern "C" fn sensor_callback(event: *mut Sensor_Event) {
    SENSOR_SUBSCRIBE_CALLBACK.with_borrow(|cb| {
        let sensor_event = SensorCallbackEvent::new(event).unwrap();

        let types = sensor_event.sensor_type;
        if let Some(callback) = cb.get(&types) {
            callback(sensor_event);
        }
    });
}
