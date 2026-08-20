use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_native_buffer_binding::{NativeBuffer, NativeBufferConfig, NativeBufferFormat};
use ohos_native_buffer_sys::{
    OH_NativeBuffer_Usage_NATIVEBUFFER_USAGE_CPU_READ,
    OH_NativeBuffer_Usage_NATIVEBUFFER_USAGE_CPU_WRITE,
};

fn to_err(e: ohos_native_buffer_binding::NativeBufferError) -> Error {
    Error::from_reason(format!("{e:?}"))
}

fn config(width: i32, height: i32) -> NativeBufferConfig {
    NativeBufferConfig {
        width,
        height,
        format: NativeBufferFormat::RGBA_8888 as i32,
        usage: (OH_NativeBuffer_Usage_NATIVEBUFFER_USAGE_CPU_READ
            | OH_NativeBuffer_Usage_NATIVEBUFFER_USAGE_CPU_WRITE) as i32,
        stride: 0,
    }
}

#[napi]
pub fn alloc_and_write() -> Result<String> {
    let buffer = NativeBuffer::new(config(32, 16));
    let info = buffer.config();
    let cloned = buffer.clone();
    let mut mapped = buffer.map_owned().map_err(to_err)?;
    let len = mapped.bytes().len();
    if !mapped.bytes_mut().is_empty() {
        mapped.bytes_mut()[0] = 0xAB;
        mapped.bytes_mut()[1] = 0xCD;
    }
    let first = mapped.bytes()[0];
    drop(mapped);
    drop(cloned);
    Ok(format!(
        "alloc {}x{} format={} stride={} mapped_len={len} first=0x{first:02x}",
        info.width, info.height, info.format, info.stride
    ))
}

#[napi]
pub fn smoke() -> Result<String> {
    alloc_and_write()
}
