use std::{ffi::CStr, ptr};

use ohos_sensor_sys::{
    OH_SensorInfo_GetMaxSamplingInterval, OH_SensorInfo_GetMinSamplingInterval,
    OH_SensorInfo_GetName, OH_SensorInfo_GetResolution, OH_SensorInfo_GetType,
    OH_SensorInfo_GetVendorName, OH_Sensor_CreateInfos, OH_Sensor_DestroyInfos, OH_Sensor_GetInfos,
    Sensor_Result_SENSOR_PARAMETER_ERROR, Sensor_Result_SENSOR_SUCCESS,
};

mod accuracy;
mod attribute;
mod error;
mod info;
mod sensor_type;
mod sub;

pub use accuracy::*;
pub use attribute::*;
pub use error::*;
pub use info::*;
pub use sensor_type::*;
pub use sub::*;

/// Get the list of sensors
pub fn get_sensor_list() -> Result<Vec<SensorInfo>, SensorError> {
    let mut count = 0;
    let ret = unsafe { OH_Sensor_GetInfos(ptr::null_mut(), &mut count) };
    if ret != Sensor_Result_SENSOR_SUCCESS {
        return Err(SensorError::InternalError(ret as _));
    }
    let origin_infos = unsafe { OH_Sensor_CreateInfos(count as _) };
    if origin_infos.is_null() {
        return Err(SensorError::InternalError(
            Sensor_Result_SENSOR_PARAMETER_ERROR as _,
        ));
    }
    let ret = unsafe { OH_Sensor_GetInfos(origin_infos, &mut count) };
    if ret != Sensor_Result_SENSOR_SUCCESS {
        return Err(SensorError::InternalError(ret as _));
    }
    let infos = unsafe { Vec::from_raw_parts(origin_infos, count as usize, count as usize) };

    let human_infos = infos
        .iter()
        .map(|info| {
            let name = ptr::null_mut();

            let ret = unsafe { OH_SensorInfo_GetName(info.clone(), name, 128 as _) };
            if ret != 0 {
                return Err(SensorError::InternalError(ret as _));
            }
            let name = unsafe { CStr::from_ptr(name).to_str().unwrap_or("") };

            let vendor_name = ptr::null_mut();
            let ret = unsafe { OH_SensorInfo_GetVendorName(info.clone(), vendor_name, 128 as _) };
            if ret != 0 {
                return Err(SensorError::InternalError(ret as _));
            }
            let vendor_name = unsafe { CStr::from_ptr(vendor_name).to_str().unwrap_or("") };

            let mut resolution = 0.0;
            let ret = unsafe { OH_SensorInfo_GetResolution(info.clone(), &mut resolution as _) };
            if ret != 0 {
                return Err(SensorError::InternalError(ret as _));
            }
            let mut min_sampling_interval = 0;
            let ret = unsafe {
                OH_SensorInfo_GetMinSamplingInterval(info.clone(), &mut min_sampling_interval as _)
            };
            if ret != 0 {
                return Err(SensorError::InternalError(ret as _));
            }
            let mut max_sampling_interval = 0;
            let ret = unsafe {
                OH_SensorInfo_GetMaxSamplingInterval(info.clone(), &mut max_sampling_interval as _)
            };
            if ret != 0 {
                return Err(SensorError::InternalError(ret as _));
            }

            let mut sensor_type = 0;
            let ret = unsafe { OH_SensorInfo_GetType(info.clone(), &mut sensor_type as _) };
            if ret != 0 {
                return Err(SensorError::InternalError(ret as _));
            }

            Ok(SensorInfo {
                sensor_type: SensorType::from(sensor_type),
                sensor_name: name.to_string(),
                sensor_vendor_name: vendor_name.to_string(),
                sensor_resolution: resolution,
                sensor_min_sampling_interval: min_sampling_interval,
                sensor_max_sampling_interval: max_sampling_interval,
            })
        })
        .collect::<Result<Vec<SensorInfo>, SensorError>>()?;

    let ret = unsafe { OH_Sensor_DestroyInfos(origin_infos, count as _) };
    if ret != 0 {
        return Err(SensorError::InternalError(ret as _));
    }

    Ok(human_infos)
}
