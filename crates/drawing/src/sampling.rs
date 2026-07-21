use std::ptr::NonNull;

use ohos_native_drawing_sys::{
    OH_Drawing_SamplingOptions, OH_Drawing_SamplingOptionsCreate, OH_Drawing_SamplingOptionsDestroy,
};

use crate::{FilterMode, MipmapMode};

#[derive(Debug)]
pub struct SamplingOptions {
    raw: NonNull<OH_Drawing_SamplingOptions>,
}

impl SamplingOptions {
    pub fn new(filter: FilterMode, mipmap: MipmapMode) -> Self {
        let raw = unsafe { OH_Drawing_SamplingOptionsCreate(filter.into(), mipmap.into()) };
        Self {
            raw: NonNull::new(raw).expect("OH_Drawing_SamplingOptionsCreate returned null"),
        }
    }

    pub(crate) fn as_ptr(&self) -> *const OH_Drawing_SamplingOptions {
        self.raw.as_ptr()
    }
}

impl Default for SamplingOptions {
    fn default() -> Self {
        Self::new(FilterMode::Linear, MipmapMode::None)
    }
}

impl Drop for SamplingOptions {
    fn drop(&mut self) {
        unsafe { OH_Drawing_SamplingOptionsDestroy(self.raw.as_ptr()) };
    }
}
