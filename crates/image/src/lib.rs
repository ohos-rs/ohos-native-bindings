use napi_sys_ohos::{napi_env, napi_value};
use std::ptr;
use sys::{OH_AccessPixels, OH_GetImageInfo, OH_UnAccessPixels, OhosPixelMapInfo, PixelResult};

mod sys;

///  获取 PixelMap 的信息，并记录信息到OhosPixelMapInfo结构中
pub fn get_pixel_map_info(
    napi_env: napi_env,
    pixel_map: napi_value,
) -> Result<OhosPixelMapInfo, String> {
    let ptr = Box::into_raw(Box::new(ptr::null_mut()));
    unsafe {
        let result = OH_AccessPixels(napi_env, pixel_map, ptr);
        if result != PixelResult::OhosImageResultSuccess {
            return Err("Try to get pixel's access failed".to_string());
        }
        let info = OhosPixelMapInfo::default();
        let result = OH_GetImageInfo(napi_env, pixel_map, &info);
        if result != PixelResult::OhosImageResultSuccess {
            return Err("Try to get pixel's info failed".to_string());
        }
        let result = OH_UnAccessPixels(napi_env, pixel_map);
        if result != PixelResult::OhosImageResultSuccess {
            return Err("Try to release pixel's access failed".to_string());
        }
        Ok(info)
    }
}
