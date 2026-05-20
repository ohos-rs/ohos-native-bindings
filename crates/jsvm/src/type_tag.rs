use ohos_jsvm_sys as sys;

#[derive(Clone, Copy, Debug)]
pub struct TypeTag {
    raw: sys::JSVM_TypeTag,
}

impl TypeTag {
    pub const fn new(lower: u64, upper: u64) -> Self {
        Self {
            raw: sys::JSVM_TypeTag { lower, upper },
        }
    }

    pub const fn as_raw(&self) -> *const sys::JSVM_TypeTag {
        &self.raw
    }
}
