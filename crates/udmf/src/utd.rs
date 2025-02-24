use std::ptr::NonNull;

use ohos_udmf_sys::{OH_Utd, OH_Utd_Create, OH_Utd_Destroy, OH_Utd_Equals};

use crate::{UdmfError, UdmfMeta};

pub struct Utd {
    raw: NonNull<OH_Utd>,
    meta_type: UdmfMeta,
}

impl Utd {
    pub fn new(meta: UdmfMeta) -> Result<Self, UdmfError> {
        let ret = unsafe { OH_Utd_Create(meta.to_string().as_ptr().cast()) };
        if ret.is_null() {
            return Err(UdmfError::UtdInitError(meta.to_string()));
        } else {
            let raw = NonNull::new(ret).expect("OH_Utd_Create return null");
            Ok(Self {
                raw,
                meta_type: meta,
            })
        }
    }

    pub fn get_type_id(&self) -> UdmfMeta {
        self.meta_type.clone()
    }
}

impl Drop for Utd {
    fn drop(&mut self) {
        unsafe { OH_Utd_Destroy(self.raw.as_ptr()) }
    }
}

impl PartialEq for Utd {
    fn eq(&self, other: &Self) -> bool {
        let ret = unsafe { OH_Utd_Equals(self.raw.as_ptr(), other.raw.as_ptr()) };
        ret
    }
}

impl Eq for Utd {}
