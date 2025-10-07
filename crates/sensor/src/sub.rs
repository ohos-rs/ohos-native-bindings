use std::{
    cell::RefCell,
    collections::HashMap,
    ptr::{self, NonNull},
};

use ohos_sensor_sys::{
    OH_SensorEvent_GetAccuracy, OH_SensorEvent_GetData, OH_SensorEvent_GetTimestamp,
    OH_SensorEvent_GetType, OH_SensorSubscriber_SetCallback, OH_SensorSubscriptionId_SetType,
    OH_Sensor_CreateSubscriber, OH_Sensor_CreateSubscriptionId, OH_Sensor_DestroySubscriber,
    OH_Sensor_DestroySubscriptionId, OH_Sensor_Subscribe, OH_Sensor_Unsubscribe, Sensor_Event,
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
        match unsafe { OH_SensorEvent_GetTimestamp(event, &mut timestamp) } {
            0 => {}
            ret => {
                return Err(SensorError::InternalError(ret as _));
            }
        }
        let mut accuracy = 0;
        match unsafe { OH_SensorEvent_GetAccuracy(event, &mut accuracy) } {
            0 => {}
            ret => {
                return Err(SensorError::InternalError(ret as _));
            }
        }
        let mut data = ptr::null_mut();
        let mut data_len = 0;
        match unsafe { OH_SensorEvent_GetData(event, &mut data, &mut data_len) } {
            0 => {}
            ret => {
                return Err(SensorError::InternalError(ret as _));
            }
        }
        let human_data = unsafe {
            if data.is_null() || data_len == 0 {
                vec![]
            } else {
                std::slice::from_raw_parts(data, data_len as usize).to_vec()
            }
        };
        let mut sensor_type = 0;
        match unsafe { OH_SensorEvent_GetType(event, &mut sensor_type) } {
            0 => {}
            ret => {
                return Err(SensorError::InternalError(ret as _));
            }
        }
        Ok(Self {
            timestamp,
            accuracy: Accuracy::from(accuracy as u32),
            data: human_data,
            sensor_type: SensorType::from(sensor_type),
        })
    }
}

/// Sensor subscriber
/// Because of the limitation of the OS-API and FFI, we only support one callback for each sensor type with per-thread.
pub struct SensorSubscriber {
    subscriber: RefCell<Option<NonNull<Sensor_Subscriber>>>,
    id: RefCell<Option<NonNull<Sensor_SubscriptionId>>>,
    attribute: RefCell<Option<SensorAttribute>>,
    sensor_type: SensorType,
    interval: i64,
}

thread_local! {
    static SENSOR_SUBSCRIBE_CALLBACK: RefCell<HashMap<SensorType, Box<dyn Fn(SensorCallbackEvent) + 'static>>> = RefCell::new(HashMap::new());
}

impl SensorSubscriber {
    pub fn new(sensor_type: SensorType, sampling_interval: i64) -> Self {
        Self {
            subscriber: RefCell::new(None),
            id: RefCell::new(None),
            attribute: RefCell::new(None),
            sensor_type,
            interval: sampling_interval,
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

        match unsafe { OH_SensorSubscriber_SetCallback(subscriber, Some(sensor_callback)) } {
            0 => {}
            ret => {
                return Err(SensorError::InternalError(ret as _));
            }
        }

        let id = unsafe { OH_Sensor_CreateSubscriptionId() };
        #[cfg(debug_assertions)]
        assert!(!id.is_null(), "Failed to create sensor subscription id");

        unsafe { OH_SensorSubscriptionId_SetType(id, self.sensor_type.into()) };

        let attribute = SensorAttribute::new();
        attribute
            .set_sampling_interval(self.interval)
            .expect("Failed to set sampling interval");

        match unsafe { OH_Sensor_Subscribe(id, attribute.raw(), subscriber) } {
            0 => {}
            ret => {
                return Err(SensorError::InternalError(ret as _));
            }
        }
        *self.id.borrow_mut() = Some(unsafe { NonNull::new_unchecked(id) });
        *self.subscriber.borrow_mut() = Some(unsafe { NonNull::new_unchecked(subscriber) });
        *self.attribute.borrow_mut() = Some(attribute);
        Ok(())
    }

    pub fn unsubscribe(&self) -> Result<(), SensorError> {
        let id = self.id.borrow().unwrap();
        let subscriber = self.subscriber.borrow().unwrap();
        unsafe { OH_Sensor_Unsubscribe(id.as_ptr(), subscriber.as_ptr()) };
        Ok(())
    }
}

impl Drop for SensorSubscriber {
    fn drop(&mut self) {
        if let Ok(id) = self.id.try_borrow() {
            if let Some(id) = id.as_ref() {
                if let Ok(subscriber) = self.subscriber.try_borrow() {
                    if let Some(subscriber) = subscriber.as_ref() {
                        unsafe { OH_Sensor_Unsubscribe(id.as_ptr(), subscriber.as_ptr()) };
                    }
                }
            }
        }
        self.attribute.borrow_mut().take();

        if let Some(id) = self.id.borrow().as_ref() {
            unsafe { OH_Sensor_DestroySubscriptionId(id.as_ptr()) };
        }
        if let Some(subscriber) = self.subscriber.borrow().as_ref() {
            unsafe { OH_Sensor_DestroySubscriber(subscriber.as_ptr()) };
        }
    }
}

unsafe extern "C" fn sensor_callback(event: *mut Sensor_Event) {
    SENSOR_SUBSCRIBE_CALLBACK.with_borrow(|cb| {
        let sensor_event = SensorCallbackEvent::new(event).unwrap();
        cb.get(&sensor_event.sensor_type)
            .map(|callback| callback(sensor_event));
    });
}
