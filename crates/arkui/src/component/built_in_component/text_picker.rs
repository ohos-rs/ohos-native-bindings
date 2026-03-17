#[derive(Clone, Debug, PartialEq)]
pub struct TextPickerTextStyleObject {
    pub font_color: String,
    pub font_size: f32,
    pub font_weight: String,
    pub font_family: String,
    pub font_style: String,
}

impl TextPickerTextStyleObject {
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

impl super::TextPicker {
    pub fn set_text_picker_disappear_text_style_object(
        &self,
        style: &TextPickerTextStyleObject,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerDisappearTextStyle,
            crate::ArkUINodeAttributeItem::String(style.to_attribute_string()),
        )
    }

    pub fn get_text_picker_disappear_text_style_object(
        &self,
    ) -> crate::ArkUIResult<Option<TextPickerTextStyleObject>> {
        match <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerDisappearTextStyle,
        )? {
            crate::ArkUINodeAttributeItem::String(raw) => {
                Ok(TextPickerTextStyleObject::from_attribute_string(&raw))
            }
            _ => Ok(None),
        }
    }

    pub fn set_text_picker_text_style_object(
        &self,
        style: &TextPickerTextStyleObject,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerTextStyle,
            crate::ArkUINodeAttributeItem::String(style.to_attribute_string()),
        )
    }

    pub fn get_text_picker_text_style_object(
        &self,
    ) -> crate::ArkUIResult<Option<TextPickerTextStyleObject>> {
        match <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerTextStyle,
        )? {
            crate::ArkUINodeAttributeItem::String(raw) => {
                Ok(TextPickerTextStyleObject::from_attribute_string(&raw))
            }
            _ => Ok(None),
        }
    }

    pub fn set_text_picker_selected_text_style_object(
        &self,
        style: &TextPickerTextStyleObject,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerSelectedTextStyle,
            crate::ArkUINodeAttributeItem::String(style.to_attribute_string()),
        )
    }

    pub fn get_text_picker_selected_text_style_object(
        &self,
    ) -> crate::ArkUIResult<Option<TextPickerTextStyleObject>> {
        match <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerSelectedTextStyle,
        )? {
            crate::ArkUINodeAttributeItem::String(raw) => {
                Ok(TextPickerTextStyleObject::from_attribute_string(&raw))
            }
            _ => Ok(None),
        }
    }
}

// BEGIN_GENERATED_COMPONENT_METHODS_TextPicker
impl super::TextPicker {
    pub fn set_text_picker_option_range<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerOptionRange,
            value.into(),
        )
    }

    pub fn get_text_picker_option_range(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerOptionRange,
        )
    }

    pub fn set_text_picker_option_selected<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerOptionSelected,
            value.into(),
        )
    }

    pub fn get_text_picker_option_selected(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerOptionSelected,
        )
    }

    pub fn set_text_picker_option_value<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerOptionValue,
            value.into(),
        )
    }

    pub fn get_text_picker_option_value(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerOptionValue,
        )
    }

    pub fn set_text_picker_disappear_text_style<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerDisappearTextStyle,
            value.into(),
        )
    }

    pub fn get_text_picker_disappear_text_style(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerDisappearTextStyle,
        )
    }

    pub fn set_text_picker_text_style<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerTextStyle,
            value.into(),
        )
    }

    pub fn get_text_picker_text_style(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerTextStyle,
        )
    }

    pub fn set_text_picker_selected_text_style<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerSelectedTextStyle,
            value.into(),
        )
    }

    pub fn get_text_picker_selected_text_style(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerSelectedTextStyle,
        )
    }

    pub fn set_text_picker_selected_index<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerSelectedIndex,
            value.into(),
        )
    }

    pub fn get_text_picker_selected_index(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerSelectedIndex,
        )
    }

    pub fn set_text_picker_can_loop<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerCanLoop,
            value.into(),
        )
    }

    pub fn get_text_picker_can_loop(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerCanLoop,
        )
    }

    pub fn set_text_picker_default_picker_item_height<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerDefaultPickerItemHeight,
            value.into(),
        )
    }

    pub fn get_text_picker_default_picker_item_height(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerDefaultPickerItemHeight,
        )
    }

    #[cfg(feature = "api-18")]
    pub fn set_text_picker_column_widths<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerColumnWidths,
            value.into(),
        )
    }

    #[cfg(feature = "api-18")]
    pub fn get_text_picker_column_widths(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerColumnWidths,
        )
    }

    #[cfg(feature = "api-18")]
    pub fn set_text_picker_enable_haptic_feedback<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerEnableHapticFeedback,
            value.into(),
        )
    }

    #[cfg(feature = "api-18")]
    pub fn get_text_picker_enable_haptic_feedback(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerEnableHapticFeedback,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_text_picker_selected_background_style<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerSelectedBackgroundStyle,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_text_picker_selected_background_style(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextPickerSelectedBackgroundStyle,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_TextPicker
