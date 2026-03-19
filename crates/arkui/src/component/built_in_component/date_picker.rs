//! Module component::built_in_component::date_picker wrappers and related types.

#[derive(Clone, Debug, PartialEq)]
/// Text style object used by date-picker style attributes.
pub struct DatePickerTextStyleObject {
    pub font_color: String,
    pub font_size: f32,
    pub font_weight: String,
    pub font_family: String,
    pub font_style: String,
}

impl DatePickerTextStyleObject {
    pub fn new<C, W, F, S>(
        font_color: C,
        font_size: f32,
        font_weight: W,
        font_family: F,
        font_style: S,
    ) -> Self
    where
        C: Into<String>,
        W: Into<String>,
        F: Into<String>,
        S: Into<String>,
    {
        Self {
            font_color: font_color.into(),
            font_size,
            font_weight: font_weight.into(),
            font_family: font_family.into(),
            font_style: font_style.into(),
        }
    }

    pub fn to_attribute_string(&self) -> String {
        format!(
            "{};{};{};{};{}",
            self.font_color, self.font_size, self.font_weight, self.font_family, self.font_style
        )
    }

    pub fn from_attribute_string(raw: &str) -> Option<Self> {
        let mut parts = raw.splitn(5, ';');
        let font_color = parts.next()?.to_string();
        let font_size = parts.next()?.parse::<f32>().ok()?;
        let font_weight = parts.next()?.to_string();
        let font_family = parts.next()?.to_string();
        let font_style = parts.next()?.to_string();
        Some(Self {
            font_color,
            font_size,
            font_weight,
            font_family,
            font_style,
        })
    }
}

impl super::DatePicker {
    pub fn set_date_picker_disappear_text_style_object(
        &self,
        style: &DatePickerTextStyleObject,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerDisappearTextStyle,
            crate::ArkUINodeAttributeItem::String(style.to_attribute_string()),
        )
    }

    pub fn get_date_picker_disappear_text_style_object(
        &self,
    ) -> crate::ArkUIResult<Option<DatePickerTextStyleObject>> {
        match <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerDisappearTextStyle,
        )? {
            crate::ArkUINodeAttributeItem::String(raw) => {
                Ok(DatePickerTextStyleObject::from_attribute_string(&raw))
            }
            _ => Ok(None),
        }
    }

    pub fn set_date_picker_text_style_object(
        &self,
        style: &DatePickerTextStyleObject,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerTextStyle,
            crate::ArkUINodeAttributeItem::String(style.to_attribute_string()),
        )
    }

    pub fn get_date_picker_text_style_object(
        &self,
    ) -> crate::ArkUIResult<Option<DatePickerTextStyleObject>> {
        match <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerTextStyle,
        )? {
            crate::ArkUINodeAttributeItem::String(raw) => {
                Ok(DatePickerTextStyleObject::from_attribute_string(&raw))
            }
            _ => Ok(None),
        }
    }

    pub fn set_date_picker_selected_text_style_object(
        &self,
        style: &DatePickerTextStyleObject,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerSelectedTextStyle,
            crate::ArkUINodeAttributeItem::String(style.to_attribute_string()),
        )
    }

    pub fn get_date_picker_selected_text_style_object(
        &self,
    ) -> crate::ArkUIResult<Option<DatePickerTextStyleObject>> {
        match <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerSelectedTextStyle,
        )? {
            crate::ArkUINodeAttributeItem::String(raw) => {
                Ok(DatePickerTextStyleObject::from_attribute_string(&raw))
            }
            _ => Ok(None),
        }
    }
}

// BEGIN_GENERATED_COMPONENT_METHODS_DatePicker
impl super::DatePicker {
    pub fn set_date_picker_lunar<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerLunar,
            value.into(),
        )
    }

    pub fn get_date_picker_lunar(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerLunar,
        )
    }

    pub fn set_date_picker_start<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerStart,
            value.into(),
        )
    }

    pub fn get_date_picker_start(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerStart,
        )
    }

    pub fn set_date_picker_end<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerEnd,
            value.into(),
        )
    }

    pub fn get_date_picker_end(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerEnd,
        )
    }

    pub fn set_date_picker_selected<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerSelected,
            value.into(),
        )
    }

    pub fn get_date_picker_selected(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerSelected,
        )
    }

    pub fn set_date_picker_disappear_text_style<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerDisappearTextStyle,
            value.into(),
        )
    }

    pub fn get_date_picker_disappear_text_style(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerDisappearTextStyle,
        )
    }

    pub fn set_date_picker_text_style<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerTextStyle,
            value.into(),
        )
    }

    pub fn get_date_picker_text_style(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerTextStyle,
        )
    }

    pub fn set_date_picker_selected_text_style<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerSelectedTextStyle,
            value.into(),
        )
    }

    pub fn get_date_picker_selected_text_style(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerSelectedTextStyle,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_date_picker_can_loop<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerCanLoop,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_date_picker_can_loop(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerCanLoop,
        )
    }

    #[cfg(feature = "api-18")]
    pub fn set_date_picker_enable_haptic_feedback<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerEnableHapticFeedback,
            value.into(),
        )
    }

    #[cfg(feature = "api-18")]
    pub fn get_date_picker_enable_haptic_feedback(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerEnableHapticFeedback,
        )
    }

    #[cfg(feature = "api-18")]
    pub fn set_date_picker_mode<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerMode,
            value.into(),
        )
    }

    #[cfg(feature = "api-18")]
    pub fn get_date_picker_mode(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::DatePickerMode,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_DatePicker

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Event payload emitted when date-picker value changes.
pub struct DatePickerChangeEvent {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

impl super::DatePicker {
    pub fn on_date_picker_change<T: Fn(DatePickerChangeEvent) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::DatePickerEventOnDateChange,
            move |event| {
                cb(DatePickerChangeEvent {
                    year: event.i32_value(0).unwrap_or_default(),
                    month: event.i32_value(1).unwrap_or_default(),
                    day: event.i32_value(2).unwrap_or_default(),
                });
            },
        );
    }
}
