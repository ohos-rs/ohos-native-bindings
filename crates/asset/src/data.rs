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

impl From<&AssetAttr> for ohos_asset_sys::Asset_Attr {
    fn from(value: &AssetAttr) -> Self {
        match &value.value {
            AssetValue::Blob(blob) => {
                let b = ohos_asset_sys::Asset_Blob {
                    size: blob.len() as u32,
                    data: blob.as_ptr() as *mut u8,
                };
                ohos_asset_sys::Asset_Attr {
                    tag: value.tag.into(),
                    value: ohos_asset_sys::Asset_Value { blob: b },
                }
            }
            AssetValue::Boolean(boolean) => ohos_asset_sys::Asset_Attr {
                tag: value.tag.into(),
                value: ohos_asset_sys::Asset_Value {
                    boolean: boolean.to_owned(),
                },
            },
            AssetValue::U32IntT(uint32_t) => ohos_asset_sys::Asset_Attr {
                tag: value.tag.into(),
                value: ohos_asset_sys::Asset_Value {
                    u32_: uint32_t.to_owned(),
                },
            },
        }
    }
}

pub struct AssetResult {
    pub count: u32,
    pub attrs: Vec<AssetAttr>,
}

impl From<*mut ohos_asset_sys::Asset_Result> for AssetResult {
    fn from(value: *mut ohos_asset_sys::Asset_Result) -> Self {
        unsafe {
            let raw_result = &*value;
            let mut attrs = Vec::with_capacity(raw_result.count as usize);
            for i in 0..raw_result.count as isize {
                let raw_attr = &*raw_result.attrs.offset(i);
                attrs.push(AssetAttr {
                    tag: raw_attr.tag.into(),
                    value: match raw_attr.tag & ohos_asset_sys::ASSET_TAG_TYPE_MASK {
                        ohos_asset_sys::Asset_TagType_ASSET_TYPE_BOOL => {
                            AssetValue::Boolean(raw_attr.value.boolean)
                        }
                        ohos_asset_sys::Asset_TagType_ASSET_TYPE_NUMBER => {
                            AssetValue::U32IntT(raw_attr.value.u32_)
                        }
                        ohos_asset_sys::Asset_TagType_ASSET_TYPE_BYTES => {
                            let blob = raw_attr.value.blob;
                            let data_slice =
                                std::slice::from_raw_parts(blob.data, blob.size as usize);
                            AssetValue::Blob(ManuallyDrop::new(data_slice.to_vec()))
                        }
                        _ => unimplemented!(),
                    },
                });
            }
            AssetResult {
                count: raw_result.count,
                attrs,
            }
        }
    }
}

pub struct AssetResultSet {
    pub count: u32,
    pub result: Vec<AssetResult>,
}

impl From<*mut ohos_asset_sys::Asset_ResultSet> for AssetResultSet {
    fn from(raw: *mut ohos_asset_sys::Asset_ResultSet) -> Self {
        unsafe {
            let mut results = Vec::new();
            if raw.is_null() {
                return AssetResultSet {
                    count: 0,
                    result: results,
                };
            }

            let raw_result_set = &*raw;
            for i in 0..raw_result_set.count as isize {
                let raw_result = &*raw_result_set.results.offset(i);
                let mut attrs = Vec::new();
                for j in 0..raw_result.count as isize {
                    let raw_attr = &*raw_result.attrs.offset(j);
                    let value = match raw_attr.tag & ohos_asset_sys::ASSET_TAG_TYPE_MASK {
                        ohos_asset_sys::Asset_TagType_ASSET_TYPE_BOOL => {
                            AssetValue::Boolean(raw_attr.value.boolean)
                        }
                        ohos_asset_sys::Asset_TagType_ASSET_TYPE_NUMBER => {
                            AssetValue::U32IntT(raw_attr.value.u32_)
                        }
                        ohos_asset_sys::Asset_TagType_ASSET_TYPE_BYTES => {
                            let blob = raw_attr.value.blob;
                            let data_slice =
                                std::slice::from_raw_parts(blob.data, blob.size as usize);
                            AssetValue::Blob(ManuallyDrop::new(data_slice.to_vec()))
                        }
                        _ => unimplemented!(),
                    };
                    attrs.push(AssetAttr {
                        tag: raw_attr.tag.into(),
                        value,
                    });
                }
                results.push(AssetResult {
                    count: raw_result.count,
                    attrs,
                });
            }

            AssetResultSet {
                count: raw_result_set.count,
                result: results,
            }
        }
    }
}
