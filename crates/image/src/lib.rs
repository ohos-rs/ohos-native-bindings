//! Safe wrappers for OpenHarmony image APIs.

mod common;
mod error;
mod image;
mod image_packer;
mod image_receiver;
mod image_source;
mod pixel_map;
pub mod types;

pub use ohos_image_sys as sys;

pub use common::{AsNapiValue, NapiValue};
pub use error::{ImageError, ImageResult};
pub use image::{Image, NativeImage};
pub use image_packer::{ImagePacker, NativeImagePacker};
pub use image_receiver::{ImageReceiver, NativeImageReceiver};
pub use image_source::{ImageSource, NativeImageSource};
pub use pixel_map::{NativePixelMapHandle, PixelMap, PixelMapNativeHandle};
pub use types::PixelMapAntiAliasingLevel;

/// Backward-compatible helper for fetching pixel-map info from a N-API value.
pub fn get_pixel_map_info(
    env: napi_sys_ohos::napi_env,
    pixel_map: napi_sys_ohos::napi_value,
) -> ImageResult<types::OhosPixelMapInfos> {
    NativePixelMapHandle::from_napi(env, pixel_map)?.image_info()
}
