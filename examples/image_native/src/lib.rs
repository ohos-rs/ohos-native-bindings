use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_image_native_binding::{
    PixelFormat, PixelMap, PixelMapAlphaType, PixelMapInitializationOptions,
};

fn to_err(e: ohos_image_native_binding::ImageNativeError) -> Error {
    Error::from_reason(e.to_string())
}

fn make_pixel_map() -> Result<PixelMap> {
    let mut options = PixelMapInitializationOptions::new().map_err(to_err)?;
    options.set_width(8).map_err(to_err)?;
    options.set_height(8).map_err(to_err)?;
    options
        .set_pixel_format(PixelFormat::Rgba8888)
        .map_err(to_err)?;
    options
        .set_alpha_type(PixelMapAlphaType::Opaque)
        .map_err(to_err)?;
    options.set_editable(true).map_err(to_err)?;
    let mut data = vec![0u8; 8 * 8 * 4];
    for (i, chunk) in data.chunks_exact_mut(4).enumerate() {
        chunk[0] = (i * 3) as u8;
        chunk[1] = 80;
        chunk[2] = 160;
        chunk[3] = 255;
    }
    PixelMap::create(&mut data, &mut options).map_err(to_err)
}

#[napi]
pub fn create_and_info() -> Result<String> {
    let pixel_map = make_pixel_map()?;
    let info = pixel_map.image_info().map_err(to_err)?;
    Ok(format!(
        "w={} h={} stride={} format={:?} alpha={:?} editable_opt_w={}",
        info.width().map_err(to_err)?,
        info.height().map_err(to_err)?,
        info.row_stride().map_err(to_err)?,
        info.pixel_format().map_err(to_err)?,
        info.alpha_type().map_err(to_err)?,
        PixelMapInitializationOptions::new()
            .map_err(to_err)?
            .width()
            .map_err(to_err)?
    ))
}

#[napi]
pub fn transform() -> Result<String> {
    let mut pixel_map = make_pixel_map()?;
    pixel_map.set_opacity(0.5).map_err(to_err)?;
    pixel_map.scale(2.0, 2.0).map_err(to_err)?;
    pixel_map.rotate(90.0).map_err(to_err)?;
    pixel_map.translate(1.0, 1.0).map_err(to_err)?;
    let cloned = pixel_map.clone_pixelmap().map_err(to_err)?;
    let alpha = pixel_map.create_alpha_pixelmap().map_err(to_err)?;
    let info = pixel_map.image_info().map_err(to_err)?;
    let cloned_info = cloned.image_info().map_err(to_err)?;
    let alpha_info = alpha.image_info().map_err(to_err)?;
    Ok(format!(
        "scaled={}x{} cloned={}x{} alpha={}x{}",
        info.width().map_err(to_err)?,
        info.height().map_err(to_err)?,
        cloned_info.width().map_err(to_err)?,
        cloned_info.height().map_err(to_err)?,
        alpha_info.width().map_err(to_err)?,
        alpha_info.height().map_err(to_err)?,
    ))
}

#[napi]
pub fn read_write_pixels() -> Result<String> {
    let mut pixel_map = make_pixel_map()?;
    let info = pixel_map.image_info().map_err(to_err)?;
    let stride = info.row_stride().map_err(to_err)? as usize;
    let height = info.height().map_err(to_err)? as usize;
    let mut buf = vec![0u8; stride * height];
    let n = pixel_map.read_pixels(&mut buf).map_err(to_err)?;
    buf[0] = 1;
    pixel_map.write_pixels(&buf).map_err(to_err)?;
    Ok(format!("read={n} wrote={}", buf.len()))
}

#[napi]
pub fn smoke() -> Result<String> {
    Ok(format!(
        "{}\n{}\n{}",
        create_and_info()?,
        transform()?,
        read_write_pixels()?
    ))
}
