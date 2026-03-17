#[derive(Clone, Debug, PartialEq)]
pub struct TimePickerTextStyleObject {
    pub font_color: String,
    pub font_size: f32,
    pub font_weight: String,
    pub font_family: String,
    pub font_style: String,
}

impl TimePickerTextStyleObject {
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

impl super::TimePicker {
    pub fn set_time_picker_disappear_text_style_object(
        &self,
        style: &TimePickerTextStyleObject,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerDisappearTextStyle,
            crate::ArkUINodeAttributeItem::String(style.to_attribute_string()),
        )
    }

    pub fn get_time_picker_disappear_text_style_object(
        &self,
    ) -> crate::ArkUIResult<Option<TimePickerTextStyleObject>> {
        match <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerDisappearTextStyle,
        )? {
            crate::ArkUINodeAttributeItem::String(raw) => {
                Ok(TimePickerTextStyleObject::from_attribute_string(&raw))
            }
            _ => Ok(None),
        }
    }

    pub fn set_time_picker_text_style_object(
        &self,
        style: &TimePickerTextStyleObject,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerTextStyle,
            crate::ArkUINodeAttributeItem::String(style.to_attribute_string()),
        )
    }

    pub fn get_time_picker_text_style_object(
        &self,
    ) -> crate::ArkUIResult<Option<TimePickerTextStyleObject>> {
        match <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerTextStyle,
        )? {
            crate::ArkUINodeAttributeItem::String(raw) => {
                Ok(TimePickerTextStyleObject::from_attribute_string(&raw))
            }
            _ => Ok(None),
        }
    }

    pub fn set_time_picker_selected_text_style_object(
        &self,
        style: &TimePickerTextStyleObject,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerSelectedTextStyle,
            crate::ArkUINodeAttributeItem::String(style.to_attribute_string()),
        )
    }

    pub fn get_time_picker_selected_text_style_object(
        &self,
    ) -> crate::ArkUIResult<Option<TimePickerTextStyleObject>> {
        match <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerSelectedTextStyle,
        )? {
            crate::ArkUINodeAttributeItem::String(raw) => {
                Ok(TimePickerTextStyleObject::from_attribute_string(&raw))
            }
            _ => Ok(None),
        }
    }
}

// BEGIN_GENERATED_COMPONENT_METHODS_TimePicker
impl super::TimePicker {
    pub fn set_time_picker_selected<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerSelected,
            value.into(),
        )
    }

    pub fn get_time_picker_selected(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerSelected,
        )
    }

    pub fn set_time_picker_use_military_time<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerUseMilitaryTime,
            value.into(),
        )
    }

    pub fn get_time_picker_use_military_time(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerUseMilitaryTime,
        )
    }

    pub fn set_time_picker_disappear_text_style<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerDisappearTextStyle,
            value.into(),
        )
    }

    pub fn get_time_picker_disappear_text_style(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerDisappearTextStyle,
        )
    }

    pub fn set_time_picker_text_style<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerTextStyle,
            value.into(),
        )
    }

    pub fn get_time_picker_text_style(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerTextStyle,
        )
    }

    pub fn set_time_picker_selected_text_style<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerSelectedTextStyle,
            value.into(),
        )
    }

    pub fn get_time_picker_selected_text_style(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerSelectedTextStyle,
        )
    }

    #[cfg(feature = "api-18")]
    pub fn set_time_picker_enable_cascade<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerEnableCascade,
            value.into(),
        )
    }

    #[cfg(feature = "api-18")]
    pub fn get_time_picker_enable_cascade(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerEnableCascade,
        )
    }

    #[cfg(feature = "api-18")]
    pub fn set_time_picker_end<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerEnd,
            value.into(),
        )
    }

    #[cfg(feature = "api-18")]
    pub fn get_time_picker_end(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerEnd,
        )
    }

    #[cfg(feature = "api-18")]
    pub fn set_time_picker_start<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerStart,
            value.into(),
        )
    }

    #[cfg(feature = "api-18")]
    pub fn get_time_picker_start(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TimePickerStart,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_TimePicker

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimePickerChangeEvent {
    pub hour: i32,
    pub minute: i32,
}

impl super::TimePicker {
    pub fn on_time_picker_change<T: Fn(TimePickerChangeEvent) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::TimePickerEventOnChange,
            move |event| {
                cb(TimePickerChangeEvent {
                    hour: event.i32_value(0).unwrap_or_default(),
                    minute: event.i32_value(1).unwrap_or_default(),
                });
            },
        );
    }
}
