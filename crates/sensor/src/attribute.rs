use std::ptr::NonNull;

use ohos_sensor_sys::{
    OH_SensorSubscriptionAttribute_GetSamplingInterval,
    OH_SensorSubscriptionAttribute_SetSamplingInterval, OH_Sensor_CreateSubscriptionAttribute,
    Sensor_SubscriptionAttribute,
};

use crate::SensorError;

pub struct SensorAttribute {
    attribute: NonNull<Sensor_SubscriptionAttribute>,
}

impl SensorAttribute {
    pub fn new() -> Self {
        let attribute = unsafe { OH_Sensor_CreateSubscriptionAttribute() };
        #[cfg(debug_assertions)]
        assert!(!attribute.is_null(), "Failed to create sensor attribute");

        Self {
            attribute: unsafe { NonNull::new_unchecked(attribute) },
        }
    }

    pub fn raw(&self) -> *const Sensor_SubscriptionAttribute {
        self.attribute.as_ptr()
    }

    pub fn set_sampling_interval(&self, sampling_interval: i64) -> Result<(), SensorError> {
        let ret = unsafe {
            OH_SensorSubscriptionAttribute_SetSamplingInterval(
                self.attribute.as_ptr(),
                sampling_interval,
            )
        };
        if ret != 0 {
            return Err(SensorError::InternalError(ret as _));
        }
        Ok(())
    }

    pub fn get_sampling_interval(&self) -> Result<i64, SensorError> {
        let mut sampling_interval = 0;
        let ret = unsafe {
            OH_SensorSubscriptionAttribute_GetSamplingInterval(
                self.attribute.as_ptr(),
                &mut sampling_interval,
            )
        };
        if ret != 0 {
            return Err(SensorError::InternalError(ret as _));
        }
        Ok(sampling_interval)
    }
}
