use std::ptr;

use ohos_sensor_sys::{
    OH_SensorInfo_GetMaxSamplingInterval, OH_SensorInfo_GetMinSamplingInterval,
    OH_SensorInfo_GetName, OH_SensorInfo_GetResolution, OH_SensorInfo_GetType,
    OH_SensorInfo_GetVendorName, OH_Sensor_CreateInfos, OH_Sensor_DestroyInfos, OH_Sensor_GetInfos,
    Sensor_Info, Sensor_Result_SENSOR_PARAMETER_ERROR, Sensor_Result_SENSOR_SUCCESS,
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

/// Size of the buffers used for the sensor name / vendor name strings.
const NAME_BUF_LEN: u32 = 128;

/// Get the list of sensors
pub fn get_sensor_list() -> Result<Vec<SensorInfo>, SensorError> {
    let mut count: u32 = 0;
    // First call with a null array: the sensor count is written to `count`.
    let ret = unsafe { OH_Sensor_GetInfos(ptr::null_mut(), &mut count) };
    if ret != Sensor_Result_SENSOR_SUCCESS {
        return Err(SensorError::InternalError(ret as _));
    }
    // `OH_Sensor_CreateInfos` returns an array of `Sensor_Info` pointers
    // (double pointer) owned by the sensor SDK.
    let origin_infos = unsafe { OH_Sensor_CreateInfos(count) };
    if origin_infos.is_null() {
        return Err(SensorError::InternalError(
            Sensor_Result_SENSOR_PARAMETER_ERROR as _,
        ));
    }
    let ret = unsafe { OH_Sensor_GetInfos(origin_infos, &mut count) };
    if ret != Sensor_Result_SENSOR_SUCCESS {
        unsafe { OH_Sensor_DestroyInfos(origin_infos, count) };
        return Err(SensorError::InternalError(ret as _));
    }

    let mut human_infos = Vec::with_capacity(count as usize);
    let mut result = Ok(());
    for i in 0..count as usize {
        // SAFETY: `origin_infos` is a valid array of `count` pointers filled
        // by the previous `OH_Sensor_GetInfos` call.
        let info = unsafe { *origin_infos.add(i) };
        match unsafe { read_sensor_info(info) } {
            Ok(info) => human_infos.push(info),
            Err(e) => {
                result = Err(e);
                break;
            }
        }
    }

    // The array belongs to the sensor SDK: destroy it exactly once. Never
    // wrap it in a `Vec` (that would hand the pointer to the Rust allocator
    // and double-free it here).
    unsafe { OH_Sensor_DestroyInfos(origin_infos, count) };
    result?;

    Ok(human_infos)
}

/// # Safety
///
/// `info` must be a valid `Sensor_Info` pointer produced by
/// `OH_Sensor_CreateInfos`.
unsafe fn read_sensor_info(info: *mut Sensor_Info) -> Result<SensorInfo, SensorError> {
    let name = read_string_attr(info, OH_SensorInfo_GetName)?;
    let vendor_name = read_string_attr(info, OH_SensorInfo_GetVendorName)?;

    let mut resolution = 0.0;
    let ret = unsafe { OH_SensorInfo_GetResolution(info, &mut resolution) };
    if ret != 0 {
        return Err(SensorError::InternalError(ret as _));
    }

    let mut min_sampling_interval = 0;
    let ret = unsafe { OH_SensorInfo_GetMinSamplingInterval(info, &mut min_sampling_interval) };
    if ret != 0 {
        return Err(SensorError::InternalError(ret as _));
    }

    let mut max_sampling_interval = 0;
    let ret = unsafe { OH_SensorInfo_GetMaxSamplingInterval(info, &mut max_sampling_interval) };
    if ret != 0 {
        return Err(SensorError::InternalError(ret as _));
    }

    let mut sensor_type = 0;
    let ret = unsafe { OH_SensorInfo_GetType(info, &mut sensor_type) };
    if ret != 0 {
        return Err(SensorError::InternalError(ret as _));
    }

    Ok(SensorInfo {
        sensor_type: SensorType::from(sensor_type),
        sensor_name: name,
        sensor_vendor_name: vendor_name,
        sensor_resolution: resolution,
        sensor_min_sampling_interval: min_sampling_interval,
        sensor_max_sampling_interval: max_sampling_interval,
    })
}

/// Read one of the string attributes. `length` is in/out: the buffer size
/// goes in, the actual string length comes back.
///
/// # Safety
///
/// `info` must be a valid `Sensor_Info` pointer produced by
/// `OH_Sensor_CreateInfos`.
unsafe fn read_string_attr(
    info: *mut Sensor_Info,
    getter: unsafe extern "C" fn(*mut Sensor_Info, *mut std::os::raw::c_char, *mut u32) -> i32,
) -> Result<String, SensorError> {
    let mut buf = vec![0u8; NAME_BUF_LEN as usize];
    let mut length = NAME_BUF_LEN;
    let ret = unsafe { getter(info, buf.as_mut_ptr().cast(), &mut length) };
    if ret != 0 {
        return Err(SensorError::InternalError(ret as _));
    }
    let written = (length as usize).min(buf.len());
    let end = buf[..written]
        .iter()
        .position(|&b| b == 0)
        .unwrap_or(written);
    Ok(String::from_utf8_lossy(&buf[..end]).into_owned())
}
