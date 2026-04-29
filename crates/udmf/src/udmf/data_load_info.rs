use std::ptr::NonNull;

use ohos_udmf_sys::{OH_UdmfDataLoadInfo, OH_UdmfDataLoadInfo_Create, OH_UdmfDataLoadInfo_Destroy};

pub struct UdmfDataLoadInfo {
    raw: NonNull<OH_UdmfDataLoadInfo>,
}

impl UdmfDataLoadInfo {
    pub fn new() -> Self {
        let raw = NonNull::new(unsafe { OH_UdmfDataLoadInfo_Create() })
            .expect("OH_UdmfDataLoadInfo_Create failed");
        Self { raw }
    }

    pub fn from_raw(raw: *mut OH_UdmfDataLoadInfo) -> Self {
        let raw = NonNull::new(raw).expect("Create UdmfDataLoadInfo from raw failed");
        Self { raw }
    }

    pub fn raw(&self) -> NonNull<OH_UdmfDataLoadInfo> {
        self.raw
    }

    pub fn as_raw(&self) -> *mut OH_UdmfDataLoadInfo {
        self.raw.as_ptr()
    }
}

impl Default for UdmfDataLoadInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for UdmfDataLoadInfo {
    fn drop(&mut self) {
        unsafe { OH_UdmfDataLoadInfo_Destroy(self.raw.as_ptr()) }
    }
}
