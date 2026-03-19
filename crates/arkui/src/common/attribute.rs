//! Module common::attribute wrappers and related types.

use std::ffi::{CStr, CString};

use ohos_arkui_sys::{ArkUI_AttributeItem, ArkUI_NumberValue};

#[derive(Clone, Copy, PartialEq)]
/// Numeric value used in ArkUI attribute arrays.
pub enum ArkUINodeAttributeNumber {
    /// 32-bit float.
    Float(f32),
    /// 32-bit signed integer.
    Int(i32),
    /// 32-bit unsigned integer.
    Uint(u32),
}

/// Generic attribute payload accepted by ArkUI node attribute APIs.
pub enum ArkUINodeAttributeItem {
    /// Numeric array payload.
    NumberValue(Vec<ArkUINodeAttributeNumber>),
    /// String payload.
    /// `value` and `size` are set to null and zero in native struct.
    String(String),
    /// Opaque object pointer payload.
    /// `value`, `size` and `string` are empty in native struct.
    Object(*mut ::std::os::raw::c_void),
}

impl From<ArkUINodeAttributeNumber> for ArkUINodeAttributeItem {
    fn from(value: ArkUINodeAttributeNumber) -> Self {
        Self::NumberValue(vec![value])
    }
}

impl From<Vec<ArkUINodeAttributeNumber>> for ArkUINodeAttributeItem {
    fn from(value: Vec<ArkUINodeAttributeNumber>) -> Self {
        Self::NumberValue(value)
    }
}

impl From<f32> for ArkUINodeAttributeItem {
    fn from(value: f32) -> Self {
        Self::NumberValue(vec![ArkUINodeAttributeNumber::Float(value)])
    }
}

impl From<i32> for ArkUINodeAttributeItem {
    fn from(value: i32) -> Self {
        Self::NumberValue(vec![ArkUINodeAttributeNumber::Int(value)])
    }
}

impl From<u32> for ArkUINodeAttributeItem {
    fn from(value: u32) -> Self {
        Self::NumberValue(vec![ArkUINodeAttributeNumber::Uint(value)])
    }
}

impl From<bool> for ArkUINodeAttributeItem {
    fn from(value: bool) -> Self {
        Self::NumberValue(vec![ArkUINodeAttributeNumber::Int(if value {
            1
        } else {
            0
        })])
    }
}

impl From<Vec<f32>> for ArkUINodeAttributeItem {
    fn from(value: Vec<f32>) -> Self {
        Self::NumberValue(
            value
                .into_iter()
                .map(ArkUINodeAttributeNumber::Float)
                .collect(),
        )
    }
}

impl From<Vec<i32>> for ArkUINodeAttributeItem {
    fn from(value: Vec<i32>) -> Self {
        Self::NumberValue(
            value
                .into_iter()
                .map(ArkUINodeAttributeNumber::Int)
                .collect(),
        )
    }
}

impl From<Vec<u32>> for ArkUINodeAttributeItem {
    fn from(value: Vec<u32>) -> Self {
        Self::NumberValue(
            value
                .into_iter()
                .map(ArkUINodeAttributeNumber::Uint)
                .collect(),
        )
    }
}

impl From<&str> for ArkUINodeAttributeItem {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for ArkUINodeAttributeItem {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<*mut ::std::os::raw::c_void> for ArkUINodeAttributeItem {
    fn from(value: *mut ::std::os::raw::c_void) -> Self {
        Self::Object(value)
    }
}

macro_rules! impl_from_object_wrapper {
    ($(#[$meta:meta])* $wrapper:ty) => {
        $(#[$meta])*
        impl From<&$wrapper> for ArkUINodeAttributeItem {
            fn from(value: &$wrapper) -> Self {
                Self::Object(value.raw().cast())
            }
        }
    };
}

impl_from_object_wrapper!(crate::LayoutConstraint);
impl_from_object_wrapper!(crate::AlignmentRuleOption);
impl_from_object_wrapper!(crate::AccessibilityValue);
impl_from_object_wrapper!(crate::WaterFlowSectionOption);
impl_from_object_wrapper!(crate::DrawableDescriptor);
impl_from_object_wrapper!(crate::SwiperIndicator);
impl_from_object_wrapper!(crate::ImageAnimatorFrameInfo);
impl_from_object_wrapper!(crate::ListItemSwipeActionItem);
impl_from_object_wrapper!(crate::ListItemSwipeActionOption);
impl_from_object_wrapper!(crate::ListChildrenMainSize);
impl_from_object_wrapper!(crate::AccessibilityState);
impl_from_object_wrapper!(crate::GuidelineOption);
impl_from_object_wrapper!(crate::BarrierOption);
impl_from_object_wrapper!(crate::StyledString);
impl_from_object_wrapper!(crate::NodeAdapter);
impl_from_object_wrapper!(crate::TransitionEffect);

#[cfg(feature = "api-15")]
impl_from_object_wrapper!(crate::ProgressLinearStyleOption);
#[cfg(feature = "api-17")]
impl_from_object_wrapper!(crate::VisibleAreaEventOptions);
#[cfg(feature = "api-19")]
impl_from_object_wrapper!(crate::SwiperDigitIndicator);
#[cfg(feature = "api-19")]
impl_from_object_wrapper!(crate::SwiperArrowStyle);
#[cfg(feature = "api-19")]
impl_from_object_wrapper!(crate::TextPickerRangeContentArray);
#[cfg(feature = "api-19")]
impl_from_object_wrapper!(crate::TextCascadePickerRangeContentArray);
#[cfg(feature = "api-20")]
impl_from_object_wrapper!(crate::EmbeddedComponentOption);
#[cfg(feature = "api-21")]
impl_from_object_wrapper!(crate::PositionEdges);
#[cfg(feature = "api-21")]
impl_from_object_wrapper!(crate::PixelRoundPolicy);
#[cfg(feature = "api-21")]
impl_from_object_wrapper!(crate::ContentTransitionEffect);
#[cfg(feature = "api-22")]
impl_from_object_wrapper!(crate::GridLayoutOptions);
#[cfg(feature = "api-22")]
impl_from_object_wrapper!(crate::ShowCounterConfig);
#[cfg(feature = "api-22")]
impl_from_object_wrapper!(crate::TextEditMenuOptions);
#[cfg(feature = "api-22")]
impl_from_object_wrapper!(crate::TextSelectionMenuOptions);
#[cfg(feature = "api-22")]
impl_from_object_wrapper!(crate::TextLayoutManager);

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
                Ok(ArkUINodeAttributeItem::String(
                    c_str.to_string_lossy().into_owned(),
                ))
            } else if !item.object.is_null() {
                // Object case
                Ok(ArkUINodeAttributeItem::Object(item.object))
            } else if !item.value.is_null() && item.size > 0 {
                // Number array case
                let slice = std::slice::from_raw_parts(item.value, item.size as usize);
                let numbers = slice
                    .iter()
                    .map(|num| {
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
                    })
                    .collect();
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
