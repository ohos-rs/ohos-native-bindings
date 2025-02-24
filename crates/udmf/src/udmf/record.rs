use std::ptr::NonNull;

use ohos_udmf_sys::{
    OH_UdmfRecord, OH_UdmfRecord_AddPlainText, OH_UdmfRecord_Create, OH_UdmfRecord_Destroy,
};

use crate::{UdmfError, Uds};

pub struct UdmfRecord {
    pub(crate) raw: NonNull<OH_UdmfRecord>,
}

impl UdmfRecord {
    pub fn new() -> Self {
        let ret = unsafe { OH_UdmfRecord_Create() };
        Self {
            raw: NonNull::new(ret).expect("OH_UdmfRecord_Create failed"),
        }
    }

    pub fn add(&self, value: Uds) -> Result<(), UdmfError> {
        match value {
            Uds::PlainText(text) => {
                let ret =
                    unsafe { OH_UdmfRecord_AddPlainText(self.raw.as_ptr(), text.raw.as_ptr()) };
                if ret != 0 {
                    return Err(UdmfError::IntervalError(ret));
                }
            }
        }
        Ok(())
    }
}

impl Drop for UdmfRecord {
    fn drop(&mut self) {
        unsafe { OH_UdmfRecord_Destroy(self.raw.as_ptr()) }
    }
}
