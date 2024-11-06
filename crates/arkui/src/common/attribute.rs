use std::ffi::CString;

use napi_ohos::bindgen_prelude::Either3;
use ohos_arkui_sys::{ArkUI_AttributeItem, ArkUI_NumberValue};

pub struct ArkUINodeAttributeNumber(pub(crate) Either3<f32, i32, u32>);

pub enum ArkUINodeAttributeItem {
    /// Accept number array
    NumberValue(Vec<ArkUINodeAttributeNumber>),
    /// Accept string   
    /// value and size will be set to null and 0
    String(String),
    /// Accept object   
    /// value and size and string will be set to null and 0 and null
    /// TODO: we need to check the type of object
    Object(*mut ::std::os::raw::c_void),
}

impl From<ArkUINodeAttributeItem> for ArkUI_AttributeItem {
    fn from(value: ArkUINodeAttributeItem) -> Self {
        match value {
            ArkUINodeAttributeItem::NumberValue(value) => {
                let v: Vec<ArkUI_NumberValue> = value
                    .iter()
                    .map(|v| match v.0 {
                        Either3::A(f) => ArkUI_NumberValue { f32_: f },
                        Either3::B(i) => ArkUI_NumberValue { i32_: i },
                        Either3::C(u) => ArkUI_NumberValue { u32_: u },
                    })
                    .collect();
                ArkUI_AttributeItem {
                    value: v.as_ptr(),
                    size: v.len() as i32,
                    string: std::ptr::null(),
                    object: std::ptr::null_mut(),
                }
            }
            ArkUINodeAttributeItem::Object(obj) => ArkUI_AttributeItem {
                value: std::ptr::null(),
                size: 0,
                string: std::ptr::null(),
                object: obj,
            },
            ArkUINodeAttributeItem::String(s) => {
                let c_string = CString::new(s).unwrap();
                ArkUI_AttributeItem {
                    value: std::ptr::null(),
                    size: 0,
                    string: c_string.as_ptr(),
                    object: std::ptr::null_mut(),
                }
            }
        }
    }
}
