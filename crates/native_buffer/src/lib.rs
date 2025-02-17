use std::ptr::NonNull;

use ohos_native_buffer_sys::{OH_NativeBuffer, OH_NativeBuffer_Config};

pub type NativeBufferConfig = OH_NativeBuffer_Config;

pub struct NativeBuffer {
    config: NativeBufferConfig,
    buffer: NonNull<OH_NativeBuffer>,
}
