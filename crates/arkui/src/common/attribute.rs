//! Module common::attribute wrappers and related types.

use std::ffi::{CStr, CString};
use std::os::raw::c_void;

use ohos_arkui_sys::{ArkUI_AttributeItem, ArkUI_NumberValue};

#[derive(Debug, Clone, Copy, PartialEq)]
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
    /// Composite payload for attributes that accept numbers, string, and object together.
    Composite(ArkUINodeCompositeAttributeItem),
}

#[derive(Debug, Clone, Default)]
/// Composite ArkUI attribute payload.
pub struct ArkUINodeCompositeAttributeItem {
    pub number_values: Vec<ArkUINodeAttributeNumber>,
    pub string: Option<String>,
    pub object: Option<*mut c_void>,
}

impl ArkUINodeCompositeAttributeItem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_number_values(mut self, value: Vec<ArkUINodeAttributeNumber>) -> Self {
        self.number_values = value;
        self
    }

    pub fn with_string<T: Into<String>>(mut self, value: T) -> Self {
        self.string = Some(value.into());
        self
    }

    pub fn with_object(mut self, value: *mut c_void) -> Self {
        self.object = Some(value);
        self
    }
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

impl From<ArkUINodeCompositeAttributeItem> for ArkUINodeAttributeItem {
    fn from(value: ArkUINodeCompositeAttributeItem) -> Self {
        Self::Composite(value)
    }
}

impl From<&ArkUINodeCompositeAttributeItem> for ArkUINodeAttributeItem {
    fn from(value: &ArkUINodeCompositeAttributeItem) -> Self {
        Self::Composite(value.clone())
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
#[cfg(feature = "image")]
impl_from_object_wrapper!(crate::DrawableDescriptor);
impl_from_object_wrapper!(crate::SwiperIndicator);
impl_from_object_wrapper!(crate::ImageAnimatorFrameInfo);
impl_from_object_wrapper!(crate::ListItemSwipeActionItem);
impl_from_object_wrapper!(crate::ListItemSwipeActionOption);
impl_from_object_wrapper!(crate::ListChildrenMainSize);
impl_from_object_wrapper!(crate::AccessibilityState);
impl_from_object_wrapper!(crate::GuidelineOption);
impl_from_object_wrapper!(crate::BarrierOption);
#[cfg(feature = "drawing")]
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
#[cfg(all(feature = "api-22", feature = "drawing"))]
impl_from_object_wrapper!(crate::TextLayoutManager);

impl From<ArkUINodeAttributeItem> for ArkUI_AttributeItem {
    fn from(value: ArkUINodeAttributeItem) -> Self {
        match value {
            ArkUINodeAttributeItem::NumberValue(value) => {
                composite_to_raw(ArkUINodeCompositeAttributeItem::new().with_number_values(value))
            }
            ArkUINodeAttributeItem::Object(obj) => {
                composite_to_raw(ArkUINodeCompositeAttributeItem::new().with_object(obj))
            }
            ArkUINodeAttributeItem::String(s) => {
                composite_to_raw(ArkUINodeCompositeAttributeItem::new().with_string(s))
            }
            ArkUINodeAttributeItem::Composite(value) => composite_to_raw(value),
        }
    }
}

impl TryFrom<ArkUI_AttributeItem> for ArkUINodeAttributeItem {
    type Error = &'static str;

    fn try_from(item: ArkUI_AttributeItem) -> Result<Self, Self::Error> {
        unsafe {
            let has_string = !item.string.is_null();
            let has_object = !item.object.is_null();
            let has_number_values = !item.value.is_null() && item.size > 0;

            if !(has_string || has_object || has_number_values) {
                return Err("Invalid ArkUI_AttributeItem - all fields are null");
            }

            let string = has_string.then(|| {
                let c_str = CStr::from_ptr(item.string);
                c_str.to_string_lossy().into_owned()
            });
            let object = has_object.then_some(item.object);
            let number_values = if has_number_values {
                let slice = std::slice::from_raw_parts(item.value, item.size as usize);
                slice.iter().map(number_value_from_raw).collect()
            } else {
                Vec::new()
            };

            match (has_number_values, has_string, has_object) {
                (true, false, false) => Ok(ArkUINodeAttributeItem::NumberValue(number_values)),
                (false, true, false) => Ok(ArkUINodeAttributeItem::String(
                    string.expect("string should exist"),
                )),
                (false, false, true) => Ok(ArkUINodeAttributeItem::Object(
                    object.expect("object should exist"),
                )),
                _ => Ok(ArkUINodeAttributeItem::Composite(
                    ArkUINodeCompositeAttributeItem {
                        number_values,
                        string,
                        object,
                    },
                )),
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

fn composite_to_raw(value: ArkUINodeCompositeAttributeItem) -> ArkUI_AttributeItem {
    let mut number_values: Vec<ArkUI_NumberValue> =
        value.number_values.iter().map(raw_number_value).collect();
    let value_ptr = if number_values.is_empty() {
        std::ptr::null_mut()
    } else {
        let ptr = number_values.as_mut_ptr();
        std::mem::forget(number_values);
        ptr
    };
    let string_ptr = value
        .string
        .map(|string| CString::new(string).unwrap().into_raw())
        .unwrap_or(std::ptr::null_mut());

    ArkUI_AttributeItem {
        value: value_ptr,
        size: value.number_values.len() as i32,
        string: string_ptr,
        object: value.object.unwrap_or(std::ptr::null_mut()),
    }
}

fn raw_number_value(value: &ArkUINodeAttributeNumber) -> ArkUI_NumberValue {
    match value {
        ArkUINodeAttributeNumber::Float(f) => ArkUI_NumberValue { f32_: *f },
        ArkUINodeAttributeNumber::Int(i) => ArkUI_NumberValue { i32_: *i },
        ArkUINodeAttributeNumber::Uint(u) => ArkUI_NumberValue { u32_: *u },
    }
}

fn number_value_from_raw(value: &ArkUI_NumberValue) -> ArkUINodeAttributeNumber {
    unsafe {
        if value.f32_ != 0.0 {
            ArkUINodeAttributeNumber::Float(value.f32_)
        } else if value.i32_ != 0 {
            ArkUINodeAttributeNumber::Int(value.i32_)
        } else {
            ArkUINodeAttributeNumber::Uint(value.u32_)
        }
    }
}
