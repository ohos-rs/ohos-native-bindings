use std::ffi::CString;

use ohos_arkui_sys::{ArkUI_AttributeItem, ArkUI_NumberValue};

#[derive(Clone, Copy, PartialEq)]
pub enum ArkUINodeAttributeNumber {
    Float(f32),
    Int(i32),
    Uint(u32),
}

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
                let mut v: Vec<ArkUI_NumberValue> = value
                    .iter()
                    .map(|v| match v {
                        ArkUINodeAttributeNumber::Float(f) => ArkUI_NumberValue { f32_: *f },
                        ArkUINodeAttributeNumber::Int(i) => ArkUI_NumberValue { i32_: *i },
                        ArkUINodeAttributeNumber::Uint(u) => ArkUI_NumberValue { u32_: *u },
                    })
                    .collect();
                let value_ptr = v.as_mut_ptr();
                let len = v.len();
                std::mem::forget(v);
                ArkUI_AttributeItem {
                    value: value_ptr,
                    size: len as i32,
                    string: std::ptr::null_mut(),
                    object: std::ptr::null_mut(),
                }
            }
            ArkUINodeAttributeItem::Object(obj) => ArkUI_AttributeItem {
                value: std::ptr::null_mut(),
                size: 0,
                string: std::ptr::null_mut(),
                object: obj,
            },
            ArkUINodeAttributeItem::String(s) => {
                let c_string = CString::new(s).unwrap();
                ArkUI_AttributeItem {
                    value: std::ptr::null_mut(),
                    size: 0,
                    string: c_string.into_raw(),
                    object: std::ptr::null_mut(),
                }
            }
        }
    }
}
