//! Module common::attribute wrappers and related types.

use std::ffi::{CStr, CString};
use std::os::raw::c_void;

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::{ArkUI_AttributeItem, ArkUI_NumberValue};

use super::error::{ArkUIError, ArkUIResult};

const INLINE_ATTRIBUTE_NUMBERS: usize = 16;
const INLINE_ATTRIBUTE_STRING_BYTES: usize = 256;

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
    /// Stack-allocated numeric array payload for common attribute arities (up to 16 values).
    InlineNumbers {
        len: u8,
        values: [ArkUINodeAttributeNumber; INLINE_ATTRIBUTE_NUMBERS],
    },
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

impl ArkUINodeAttributeItem {
    /// Builds a numeric attribute item without heap allocation when `values.len() <= 16`.
    pub fn from_numbers(values: &[ArkUINodeAttributeNumber]) -> Self {
        if values.len() <= INLINE_ATTRIBUTE_NUMBERS {
            let mut inline = [ArkUINodeAttributeNumber::Uint(0); INLINE_ATTRIBUTE_NUMBERS];
            inline[..values.len()].copy_from_slice(values);
            Self::InlineNumbers {
                len: values.len() as u8,
                values: inline,
            }
        } else {
            Self::NumberValue(values.to_vec())
        }
    }
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
        Self::from_numbers(&[value])
    }
}

impl<const N: usize> From<[ArkUINodeAttributeNumber; N]> for ArkUINodeAttributeItem {
    fn from(value: [ArkUINodeAttributeNumber; N]) -> Self {
        Self::from_numbers(&value)
    }
}

impl From<&[ArkUINodeAttributeNumber]> for ArkUINodeAttributeItem {
    fn from(value: &[ArkUINodeAttributeNumber]) -> Self {
        Self::from_numbers(value)
    }
}

impl From<Vec<ArkUINodeAttributeNumber>> for ArkUINodeAttributeItem {
    fn from(value: Vec<ArkUINodeAttributeNumber>) -> Self {
        Self::NumberValue(value)
    }
}

impl From<f32> for ArkUINodeAttributeItem {
    fn from(value: f32) -> Self {
        Self::from_numbers(&[ArkUINodeAttributeNumber::Float(value)])
    }
}

impl From<i32> for ArkUINodeAttributeItem {
    fn from(value: i32) -> Self {
        Self::from_numbers(&[ArkUINodeAttributeNumber::Int(value)])
    }
}

impl From<u32> for ArkUINodeAttributeItem {
    fn from(value: u32) -> Self {
        Self::from_numbers(&[ArkUINodeAttributeNumber::Uint(value)])
    }
}

impl From<bool> for ArkUINodeAttributeItem {
    fn from(value: bool) -> Self {
        Self::from_numbers(&[ArkUINodeAttributeNumber::Int(if value { 1 } else { 0 })])
    }
}

impl<const N: usize> From<[f32; N]> for ArkUINodeAttributeItem {
    fn from(value: [f32; N]) -> Self {
        if N <= INLINE_ATTRIBUTE_NUMBERS {
            let mut inline = [ArkUINodeAttributeNumber::Uint(0); INLINE_ATTRIBUTE_NUMBERS];
            for (dst, src) in inline.iter_mut().zip(value) {
                *dst = ArkUINodeAttributeNumber::Float(src);
            }
            Self::InlineNumbers {
                len: N as u8,
                values: inline,
            }
        } else {
            Self::NumberValue(
                value
                    .into_iter()
                    .map(ArkUINodeAttributeNumber::Float)
                    .collect(),
            )
        }
    }
}

impl From<&[f32]> for ArkUINodeAttributeItem {
    fn from(value: &[f32]) -> Self {
        if value.len() <= INLINE_ATTRIBUTE_NUMBERS {
            let mut inline = [ArkUINodeAttributeNumber::Uint(0); INLINE_ATTRIBUTE_NUMBERS];
            for (dst, &src) in inline.iter_mut().zip(value) {
                *dst = ArkUINodeAttributeNumber::Float(src);
            }
            Self::InlineNumbers {
                len: value.len() as u8,
                values: inline,
            }
        } else {
            Self::NumberValue(
                value
                    .iter()
                    .copied()
                    .map(ArkUINodeAttributeNumber::Float)
                    .collect(),
            )
        }
    }
}

impl<const N: usize> From<[i32; N]> for ArkUINodeAttributeItem {
    fn from(value: [i32; N]) -> Self {
        if N <= INLINE_ATTRIBUTE_NUMBERS {
            let mut inline = [ArkUINodeAttributeNumber::Uint(0); INLINE_ATTRIBUTE_NUMBERS];
            for (dst, src) in inline.iter_mut().zip(value) {
                *dst = ArkUINodeAttributeNumber::Int(src);
            }
            Self::InlineNumbers {
                len: N as u8,
                values: inline,
            }
        } else {
            Self::NumberValue(
                value
                    .into_iter()
                    .map(ArkUINodeAttributeNumber::Int)
                    .collect(),
            )
        }
    }
}

impl From<&[i32]> for ArkUINodeAttributeItem {
    fn from(value: &[i32]) -> Self {
        if value.len() <= INLINE_ATTRIBUTE_NUMBERS {
            let mut inline = [ArkUINodeAttributeNumber::Uint(0); INLINE_ATTRIBUTE_NUMBERS];
            for (dst, &src) in inline.iter_mut().zip(value) {
                *dst = ArkUINodeAttributeNumber::Int(src);
            }
            Self::InlineNumbers {
                len: value.len() as u8,
                values: inline,
            }
        } else {
            Self::NumberValue(
                value
                    .iter()
                    .copied()
                    .map(ArkUINodeAttributeNumber::Int)
                    .collect(),
            )
        }
    }
}

impl<const N: usize> From<[u32; N]> for ArkUINodeAttributeItem {
    fn from(value: [u32; N]) -> Self {
        if N <= INLINE_ATTRIBUTE_NUMBERS {
            let mut inline = [ArkUINodeAttributeNumber::Uint(0); INLINE_ATTRIBUTE_NUMBERS];
            for (dst, src) in inline.iter_mut().zip(value) {
                *dst = ArkUINodeAttributeNumber::Uint(src);
            }
            Self::InlineNumbers {
                len: N as u8,
                values: inline,
            }
        } else {
            Self::NumberValue(
                value
                    .into_iter()
                    .map(ArkUINodeAttributeNumber::Uint)
                    .collect(),
            )
        }
    }
}

impl From<&[u32]> for ArkUINodeAttributeItem {
    fn from(value: &[u32]) -> Self {
        if value.len() <= INLINE_ATTRIBUTE_NUMBERS {
            let mut inline = [ArkUINodeAttributeNumber::Uint(0); INLINE_ATTRIBUTE_NUMBERS];
            for (dst, &src) in inline.iter_mut().zip(value) {
                *dst = ArkUINodeAttributeNumber::Uint(src);
            }
            Self::InlineNumbers {
                len: value.len() as u8,
                values: inline,
            }
        } else {
            Self::NumberValue(
                value
                    .iter()
                    .copied()
                    .map(ArkUINodeAttributeNumber::Uint)
                    .collect(),
            )
        }
    }
}

impl From<Vec<f32>> for ArkUINodeAttributeItem {
    fn from(value: Vec<f32>) -> Self {
        Self::from(value.as_slice())
    }
}

impl From<Vec<i32>> for ArkUINodeAttributeItem {
    fn from(value: Vec<i32>) -> Self {
        Self::from(value.as_slice())
    }
}

impl From<Vec<u32>> for ArkUINodeAttributeItem {
    fn from(value: Vec<u32>) -> Self {
        Self::from(value.as_slice())
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

impl ArkUINodeAttributeItem {
    /// Borrow a native string attribute view without heap allocation when `value` fits inline.
    pub(crate) fn with_str_raw<R>(
        value: &str,
        call: impl FnOnce(&ArkUI_AttributeItem) -> ArkUIResult<R>,
    ) -> ArkUIResult<R> {
        with_c_str(value, |string| {
            let raw = ArkUI_AttributeItem {
                value: std::ptr::null(),
                size: 0,
                string,
                object: std::ptr::null_mut(),
            };
            call(&raw)
        })
    }

    /// Borrow a native attribute view for exactly one synchronous ArkUI call.
    ///
    /// `ArkUI_AttributeItem` contains borrowed pointers. Keeping the backing
    /// number array and C string in this scope makes their lifetime explicit
    /// and prevents either leaking them or letting a raw item escape after its
    /// storage has been freed.
    pub(crate) fn with_raw<R>(
        self,
        call: impl FnOnce(&ArkUI_AttributeItem) -> ArkUIResult<R>,
    ) -> ArkUIResult<R> {
        match self {
            Self::InlineNumbers { len, values } => {
                let count = usize::from(len);
                let mut raw_numbers = [ArkUI_NumberValue { u32_: 0 }; INLINE_ATTRIBUTE_NUMBERS];
                for (dst, src) in raw_numbers[..count].iter_mut().zip(&values[..count]) {
                    *dst = raw_number_value(src);
                }
                let raw = ArkUI_AttributeItem {
                    value: if count == 0 {
                        std::ptr::null()
                    } else {
                        raw_numbers.as_ptr()
                    },
                    size: count as i32,
                    string: std::ptr::null(),
                    object: std::ptr::null_mut(),
                };
                call(&raw)
            }
            Self::NumberValue(number_values) => with_raw_numbers(&number_values, |value, size| {
                let raw = ArkUI_AttributeItem {
                    value,
                    size,
                    string: std::ptr::null(),
                    object: std::ptr::null_mut(),
                };
                call(&raw)
            }),
            Self::Object(object) => {
                let raw = ArkUI_AttributeItem {
                    value: std::ptr::null(),
                    size: 0,
                    string: std::ptr::null(),
                    object,
                };
                call(&raw)
            }
            Self::String(string) => Self::with_str_raw(&string, call),
            Self::Composite(value) => with_raw_numbers(&value.number_values, |numbers, size| {
                let object = value.object.unwrap_or(std::ptr::null_mut());
                match value.string {
                    Some(string) => with_c_str(&string, |c_str| {
                        let raw = ArkUI_AttributeItem {
                            value: numbers,
                            size,
                            string: c_str,
                            object,
                        };
                        call(&raw)
                    }),
                    None => {
                        let raw = ArkUI_AttributeItem {
                            value: numbers,
                            size,
                            string: std::ptr::null(),
                            object,
                        };
                        call(&raw)
                    }
                }
            }),
        }
    }
}

fn with_raw_numbers<R>(
    values: &[ArkUINodeAttributeNumber],
    call: impl FnOnce(*const ArkUI_NumberValue, i32) -> ArkUIResult<R>,
) -> ArkUIResult<R> {
    if values.is_empty() {
        return call(std::ptr::null(), 0);
    }
    if values.len() <= INLINE_ATTRIBUTE_NUMBERS {
        let mut inline = [ArkUI_NumberValue { u32_: 0 }; INLINE_ATTRIBUTE_NUMBERS];
        for (dst, src) in inline[..values.len()].iter_mut().zip(values) {
            *dst = raw_number_value(src);
        }
        call(inline.as_ptr(), values.len() as i32)
    } else {
        let heap = values.iter().map(raw_number_value).collect::<Vec<_>>();
        call(heap.as_ptr(), heap.len() as i32)
    }
}

fn with_c_str<R>(
    value: &str,
    call: impl FnOnce(*const std::os::raw::c_char) -> ArkUIResult<R>,
) -> ArkUIResult<R> {
    let bytes = value.as_bytes();
    if bytes.contains(&0) {
        return Err(ArkUIError::new(
            ArkUIErrorCode::ParamInvalid,
            "attribute string contains interior NUL bytes",
        ));
    }
    if bytes.len() < INLINE_ATTRIBUTE_STRING_BYTES {
        let mut inline = [0u8; INLINE_ATTRIBUTE_STRING_BYTES];
        inline[..bytes.len()].copy_from_slice(bytes);
        call(inline.as_ptr().cast())
    } else {
        let c_str = CString::new(bytes).map_err(|_| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "attribute string contains interior NUL bytes",
            )
        })?;
        call(c_str.as_ptr())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_view_keeps_all_backing_storage_scoped_to_the_call() {
        let object = 0x1234usize as *mut c_void;
        let item = ArkUINodeCompositeAttributeItem::new()
            .with_number_values(vec![
                ArkUINodeAttributeNumber::Float(1.5),
                ArkUINodeAttributeNumber::Int(-3),
            ])
            .with_string("value")
            .with_object(object);

        ArkUINodeAttributeItem::Composite(item)
            .with_raw(|raw| {
                assert_eq!(raw.size, 2);
                assert!(!raw.value.is_null());
                assert_eq!(unsafe { (*raw.value).f32_ }, 1.5);
                assert_eq!(unsafe { (*raw.value.add(1)).i32_ }, -3);
                assert_eq!(unsafe { CStr::from_ptr(raw.string) }.to_bytes(), b"value");
                assert_eq!(raw.object, object);
                Ok(())
            })
            .expect("scoped raw conversion");
    }

    #[test]
    fn raw_view_rejects_interior_nul_instead_of_panicking() {
        let result = ArkUINodeAttributeItem::String("bad\0value".into()).with_raw(|_| Ok(()));
        assert!(result.is_err());
        let str_result = ArkUINodeAttributeItem::with_str_raw("bad\0value", |_| Ok(()));
        assert!(str_result.is_err());
    }

    #[test]
    fn inline_numeric_arrays_convert_without_heap_wrapping() {
        ArkUINodeAttributeItem::from([0i32, 0, 120, 48])
            .with_raw(|raw| {
                assert_eq!(raw.size, 4);
                assert!(!raw.value.is_null());
                assert!(raw.string.is_null());
                assert!(raw.object.is_null());
                assert_eq!(unsafe { (*raw.value.add(2)).i32_ }, 120);
                assert_eq!(unsafe { (*raw.value.add(3)).i32_ }, 48);
                Ok(())
            })
            .expect("inline array conversion");
    }
}
