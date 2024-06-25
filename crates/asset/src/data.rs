use std::mem::ManuallyDrop;

use crate::AssetTag;

pub type AssetBlob = Vec<u8>;

pub enum AssetValue {
    Boolean(bool),
    U32IntT(u32),
    Blob(ManuallyDrop<AssetBlob>),
}

pub struct AssetAttr {
    pub tag: AssetTag,
    pub value: AssetValue,
}
