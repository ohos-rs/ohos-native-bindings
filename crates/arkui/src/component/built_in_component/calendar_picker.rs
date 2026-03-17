use crate::ArkUINodeAttributeNumber;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CalendarPickerTextStyleObject {
    pub font_color: u32,
    pub font_size: f32,
    pub font_weight: i32,
}

impl CalendarPickerTextStyleObject {
    pub fn new(font_color: u32, font_size: f32, font_weight: i32) -> Self {
        Self {
            font_color,
            font_size,
            font_weight,
        }
    }
}

impl super::CalendarPicker {
    pub fn set_calendar_picker_text_style_object(
        &self,
        style: CalendarPickerTextStyleObject,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::CalendarPickerTextStyle,
            crate::ArkUINodeAttributeItem::NumberValue(vec![
                ArkUINodeAttributeNumber::Uint(style.font_color),
                ArkUINodeAttributeNumber::Float(style.font_size),
                ArkUINodeAttributeNumber::Int(style.font_weight),
            ]),
        )
    }

    pub fn get_calendar_picker_text_style_object(
        &self,
    ) -> crate::ArkUIResult<Option<CalendarPickerTextStyleObject>> {
        let item = <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::CalendarPickerTextStyle,
        )?;
        let style = if let crate::ArkUINodeAttributeItem::NumberValue(values) = item {
            if values.len() >= 3 {
                match (values[0], values[1], values[2]) {
                    (
                        ArkUINodeAttributeNumber::Uint(font_color),
                        ArkUINodeAttributeNumber::Float(font_size),
                        ArkUINodeAttributeNumber::Int(font_weight),
                    ) => Some(CalendarPickerTextStyleObject::new(
                        font_color,
                        font_size,
                        font_weight,
                    )),
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        };
        Ok(style)
    }
}

// BEGIN_GENERATED_COMPONENT_METHODS_CalendarPicker
impl super::CalendarPicker {
    pub fn set_calendar_picker_hint_radius<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::CalendarPickerHintRadius,
            value.into(),
        )
    }

    pub fn get_calendar_picker_hint_radius(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::CalendarPickerHintRadius,
        )
    }

    pub fn set_calendar_picker_selected_date<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::CalendarPickerSelectedDate,
            value.into(),
        )
    }

    pub fn get_calendar_picker_selected_date(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::CalendarPickerSelectedDate,
        )
    }

    pub fn set_calendar_picker_edge_alignment<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::CalendarPickerEdgeAlignment,
            value.into(),
        )
    }

    pub fn get_calendar_picker_edge_alignment(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::CalendarPickerEdgeAlignment,
        )
    }

    pub fn set_calendar_picker_text_style<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::CalendarPickerTextStyle,
            value.into(),
        )
    }

    pub fn get_calendar_picker_text_style(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::CalendarPickerTextStyle,
        )
    }

    #[cfg(feature = "api-19")]
    pub fn set_calendar_picker_disabled_date_range<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::CalendarPickerDisabledDateRange,
            value.into(),
        )
    }

    #[cfg(feature = "api-19")]
    pub fn get_calendar_picker_disabled_date_range(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::CalendarPickerDisabledDateRange,
        )
    }

    #[cfg(feature = "api-18")]
    pub fn set_calendar_picker_end<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::CalendarPickerEnd,
            value.into(),
        )
    }

    #[cfg(feature = "api-18")]
    pub fn get_calendar_picker_end(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::CalendarPickerEnd,
        )
    }

    #[cfg(feature = "api-19")]
    pub fn set_calendar_picker_mark_today<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::CalendarPickerMarkToday,
            value.into(),
        )
    }

    #[cfg(feature = "api-19")]
    pub fn get_calendar_picker_mark_today(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::CalendarPickerMarkToday,
        )
    }

    #[cfg(feature = "api-18")]
    pub fn set_calendar_picker_start<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::CalendarPickerStart,
            value.into(),
        )
    }

    #[cfg(feature = "api-18")]
    pub fn get_calendar_picker_start(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::CalendarPickerStart,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_CalendarPicker

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CalendarPickerChangeEvent {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

impl super::CalendarPicker {
    pub fn on_calendar_picker_change<T: Fn(CalendarPickerChangeEvent) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::CalendarPickerEventOnChange,
            move |event| {
                cb(CalendarPickerChangeEvent {
                    year: event.u32_value(0).unwrap_or_default(),
                    month: event.u32_value(1).unwrap_or_default(),
                    day: event.u32_value(2).unwrap_or_default(),
                });
            },
        );
    }
}
