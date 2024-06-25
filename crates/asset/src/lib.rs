mod data;
mod r#type;

pub use data::*;
pub use r#type::*;

/// add
pub fn asset_add(attrs: Vec<AssetAttr>) {
    let count = attrs.len() as u32;
    let real_attrs = attrs
        .iter()
        .map(|a| match &a.value {
            AssetValue::Blob(blob) => {
                let b = ohos_asset_sys::Asset_Blob {
                    size: blob.len() as u32,
                    data: blob.as_ptr() as *mut u8,
                };
                ohos_asset_sys::Asset_Attr {
                    tag: a.tag.into(),
                    value: ohos_asset_sys::Asset_Value { blob: b },
                }
            }
            AssetValue::Boolean(boolean) => ohos_asset_sys::Asset_Attr {
                tag: a.tag.into(),
                value: ohos_asset_sys::Asset_Value {
                    boolean: boolean.to_owned(),
                },
            },
            AssetValue::U32IntT(uint32_t) => ohos_asset_sys::Asset_Attr {
                tag: a.tag.into(),
                value: ohos_asset_sys::Asset_Value {
                    u32_: uint32_t.to_owned(),
                },
            },
        })
        .collect::<Vec<_>>();
    unsafe { ohos_asset_sys::OH_Asset_Add(real_attrs.as_ptr(), count) };
}
