use std::ptr::NonNull;

use ohos_udmf_sys::{
    OH_UdmfGetDataParams, OH_UdmfGetDataParams_Create, OH_UdmfGetDataParams_Destroy,
};

pub struct UdmfGetDataParams {
    raw: NonNull<OH_UdmfGetDataParams>,
}

impl UdmfGetDataParams {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { OH_UdmfGetDataParams_Create() })
            .expect("OH_UdmfGetDataParams_Create failed");
        Self { raw }
    }

    pub fn from_raw(raw: *mut OH_UdmfGetDataParams) -> Self {
        let raw = NonNull::new(raw).expect("Create UdmfGetDataParams from raw failed");
        Self { raw }
    }

    pub fn raw(&self) -> NonNull<OH_UdmfGetDataParams> {
        self.raw
    }

    pub fn as_raw(&self) -> *mut OH_UdmfGetDataParams {
        self.raw.as_ptr()
    }
}

impl Default for UdmfGetDataParams {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for UdmfGetDataParams {
    fn drop(&mut self) {
        unsafe { OH_UdmfGetDataParams_Destroy(self.raw.as_ptr()) }
    }
}
