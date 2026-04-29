use std::ptr::NonNull;

use ohos_udmf_sys::{
    OH_UdmfDataLoadParams, OH_UdmfDataLoadParams_Create, OH_UdmfDataLoadParams_Destroy,
};

pub struct UdmfDataLoadParams {
    raw: NonNull<OH_UdmfDataLoadParams>,
}

impl UdmfDataLoadParams {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { OH_UdmfDataLoadParams_Create() })
            .expect("OH_UdmfDataLoadParams_Create failed");
        Self { raw }
    }

    pub fn from_raw(raw: *mut OH_UdmfDataLoadParams) -> Self {
        let raw = NonNull::new(raw).expect("Create UdmfDataLoadParams from raw failed");
        Self { raw }
    }

    pub fn raw(&self) -> NonNull<OH_UdmfDataLoadParams> {
        self.raw
    }

    pub fn as_raw(&self) -> *mut OH_UdmfDataLoadParams {
        self.raw.as_ptr()
    }
}

impl Default for UdmfDataLoadParams {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for UdmfDataLoadParams {
    fn drop(&mut self) {
        unsafe { OH_UdmfDataLoadParams_Destroy(self.raw.as_ptr()) }
    }
}
