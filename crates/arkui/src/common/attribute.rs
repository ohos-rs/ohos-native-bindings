use std::ffi::{CStr, CString};

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


impl TryFrom<ArkUI_AttributeItem> for ArkUINodeAttributeItem {
    type Error = &'static str;

    fn try_from(item: ArkUI_AttributeItem) -> Result<Self, Self::Error> {
        unsafe {
            if !item.string.is_null() {
                // String case
                let c_str = CStr::from_ptr(item.string);
                Ok(ArkUINodeAttributeItem::String(c_str.to_string_lossy().into_owned()))
            } else if !item.object.is_null() {
                // Object case
                Ok(ArkUINodeAttributeItem::Object(item.object))
            } else if !item.value.is_null() && item.size > 0 {
                // Number array case
                let slice = std::slice::from_raw_parts(item.value, item.size as usize);
                let numbers = slice.iter().map(|num| {
                    // This is a bit tricky since ArkUI_NumberValue is a union in C
                    // You'll need to know which field is actually valid
                    // Here we assume it's based on some external knowledge
                    if num.f32_ != 0.0 {
                        ArkUINodeAttributeNumber::Float(num.f32_)
                    } else if num.i32_ != 0 {
                        ArkUINodeAttributeNumber::Int(num.i32_)
                    } else {
                        ArkUINodeAttributeNumber::Uint(num.u32_)
                    }
                }).collect();
                Ok(ArkUINodeAttributeItem::NumberValue(numbers))
            } else {
                Err("Invalid ArkUI_AttributeItem - all fields are null")
            }
        }
    }
}

impl TryFrom<*const ArkUI_AttributeItem> for ArkUINodeAttributeItem {
    type Error = &'static str;

    fn try_from(ptr: *const ArkUI_AttributeItem) -> Result<Self, Self::Error> {
        unsafe {
            if ptr.is_null() {
                Err("Null pointer provided")
            } else {
                // Dereference the pointer and convert
                ArkUINodeAttributeItem::try_from(*ptr)
            }
        }
    }
}