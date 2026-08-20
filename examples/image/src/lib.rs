use napi_derive_ohos::napi;
use napi_ohos::{bindgen_prelude::Object, Env, Error, JsValue, Result};
use ohos_image_binding::{
    get_pixel_map_info, types::OhosPixelMapCreateOps, ImagePacker, NativePixelMapHandle, PixelMap,
};

fn to_err(e: ohos_image_binding::ImageError) -> Error {
    Error::from_reason(e.to_string())
}

fn create_ops() -> OhosPixelMapCreateOps {
    OhosPixelMapCreateOps {
        width: 8,
        height: 8,
        pixelFormat: 3, // PIXEL_FORMAT_RGBA_8888
        editable: 1,
        alphaType: 0,
        scaleMode: 0,
    }
}

#[napi]
pub fn create_pixel_map(env: Env) -> Result<String> {
    let mut buffer = vec![180u8; 8 * 8 * 4];
    let pixel_map = PixelMap::create(env.raw(), create_ops(), &mut buffer).map_err(to_err)?;
    let native = pixel_map.native(env.raw()).map_err(to_err)?;
    describe_native(native)
}

fn describe_native(native: NativePixelMapHandle) -> Result<String> {
    let info = native.image_info().map_err(to_err)?;
    let _row = native.bytes_number_per_row().map_err(to_err)?;
    let editable = native.is_editable().map_err(to_err)?;
    let alpha = native.supports_alpha().map_err(to_err)?;
    native.set_alpha_able(true).map_err(to_err)?;
    let _addr = native.access_pixels().map_err(to_err)?;
    native.unaccess_pixels().map_err(to_err)?;
    Ok(format!(
        "w={} h={} row={} format={} editable={editable} alpha={alpha}",
        info.width, info.height, info.rowSize, info.pixelFormat
    ))
}

/// Accept a JS PixelMap and print native info.
#[napi]
pub fn inspect_pixel_map(
    env: Env,
    #[napi(ts_arg_type = "image.PixelMap")] pixel_map: Object,
) -> Result<String> {
    let info = get_pixel_map_info(env.raw(), pixel_map.raw()).map_err(to_err)?;
    let native = NativePixelMapHandle::from_napi(env.raw(), pixel_map.raw()).map_err(to_err)?;
    let row = native.bytes_number_per_row().map_err(to_err)?;
    Ok(format!(
        "w={} h={} rowSize={} format={} native_row={row}",
        info.width, info.height, info.rowSize, info.pixelFormat
    ))
}

#[napi]
pub fn create_packer(env: Env) -> Result<String> {
    let packer = ImagePacker::create(env.raw()).map_err(to_err)?;
    let native = packer.native(env.raw()).map_err(to_err)?;
    Ok(format!("packer native ptr={:?}", native.as_raw()))
}

#[napi]
pub fn smoke(env: Env) -> Result<String> {
    Ok(format!(
        "{}\n{}",
        create_pixel_map(env)?,
        create_packer(env)?
    ))
}
