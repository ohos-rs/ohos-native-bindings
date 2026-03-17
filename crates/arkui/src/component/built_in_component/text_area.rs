use crate::ArkUICommonFontAttribute;

impl ArkUICommonFontAttribute for super::TextArea {}

// BEGIN_GENERATED_COMPONENT_METHODS_TextArea
impl super::TextArea {
    pub fn set_text_area_placeholder<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaPlaceholder,
            value.into(),
        )
    }

    pub fn get_text_area_placeholder(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaPlaceholder,
        )
    }

    pub fn set_text_area_text<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaText,
            value.into(),
        )
    }

    pub fn get_text_area_text(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaText,
        )
    }

    pub fn set_text_area_max_length<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaMaxLength,
            value.into(),
        )
    }

    pub fn get_text_area_max_length(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaMaxLength,
        )
    }

    pub fn set_text_area_placeholder_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaPlaceholderColor,
            value.into(),
        )
    }

    pub fn get_text_area_placeholder_color(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaPlaceholderColor,
        )
    }

    pub fn set_text_area_placeholder_font<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaPlaceholderFont,
            value.into(),
        )
    }

    pub fn get_text_area_placeholder_font(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaPlaceholderFont,
        )
    }

    pub fn set_text_area_caret_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaCaretColor,
            value.into(),
        )
    }

    pub fn get_text_area_caret_color(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaCaretColor,
        )
    }

    pub fn set_text_area_editing<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaEditing,
            value.into(),
        )
    }

    pub fn get_text_area_editing(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaEditing,
        )
    }

    pub fn set_text_area_type<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaType,
            value.into(),
        )
    }

    pub fn get_text_area_type(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaType,
        )
    }

    pub fn set_text_area_show_counter<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaShowCounter,
            value.into(),
        )
    }

    pub fn get_text_area_show_counter(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaShowCounter,
        )
    }

    pub fn set_text_area_selection_menu_hidden<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaSelectionMenuHidden,
            value.into(),
        )
    }

    pub fn get_text_area_selection_menu_hidden(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaSelectionMenuHidden,
        )
    }

    pub fn set_text_area_blur_on_submit<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaBlurOnSubmit,
            value.into(),
        )
    }

    pub fn get_text_area_blur_on_submit(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaBlurOnSubmit,
        )
    }

    pub fn set_text_area_input_filter<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaInputFilter,
            value.into(),
        )
    }

    pub fn get_text_area_input_filter(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaInputFilter,
        )
    }

    pub fn set_text_area_selected_background_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaSelectedBackgroundColor,
            value.into(),
        )
    }

    pub fn get_text_area_selected_background_color(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaSelectedBackgroundColor,
        )
    }

    pub fn set_text_area_enter_key_type<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaEnterKeyType,
            value.into(),
        )
    }

    pub fn get_text_area_enter_key_type(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaEnterKeyType,
        )
    }

    pub fn set_text_area_enable_keyboard_on_focus<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaEnableKeyboardOnFocus,
            value.into(),
        )
    }

    pub fn get_text_area_enable_keyboard_on_focus(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaEnableKeyboardOnFocus,
        )
    }

    pub fn set_text_area_caret_offset<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaCaretOffset,
            value.into(),
        )
    }

    pub fn get_text_area_caret_offset(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaCaretOffset,
        )
    }

    pub fn set_text_area_content_rect<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaContentRect,
            value.into(),
        )
    }

    pub fn get_text_area_content_rect(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaContentRect,
        )
    }

    pub fn set_text_area_content_line_count<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaContentLineCount,
            value.into(),
        )
    }

    pub fn get_text_area_content_line_count(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaContentLineCount,
        )
    }

    pub fn set_text_area_text_selection<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaTextSelection,
            value.into(),
        )
    }

    pub fn get_text_area_text_selection(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaTextSelection,
        )
    }

    pub fn set_text_area_enable_auto_fill<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaEnableAutoFill,
            value.into(),
        )
    }

    pub fn get_text_area_enable_auto_fill(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaEnableAutoFill,
        )
    }

    pub fn set_text_area_content_type<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaContentType,
            value.into(),
        )
    }

    pub fn get_text_area_content_type(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaContentType,
        )
    }

    pub fn set_text_area_show_keyboard_on_focus<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaShowKeyboardOnFocus,
            value.into(),
        )
    }

    pub fn get_text_area_show_keyboard_on_focus(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaShowKeyboardOnFocus,
        )
    }

    pub fn set_text_area_number_of_lines<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaNumberOfLines,
            value.into(),
        )
    }

    pub fn get_text_area_number_of_lines(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaNumberOfLines,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_text_area_bar_state<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaBarState,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_text_area_bar_state(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaBarState,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_text_area_custom_keyboard<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaCustomKeyboard,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_text_area_custom_keyboard(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaCustomKeyboard,
        )
    }

    #[cfg(feature = "api-15")]
    pub fn set_text_area_enable_preview_text<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaEnablePreviewText,
            value.into(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn get_text_area_enable_preview_text(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaEnablePreviewText,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_text_area_enable_selected_data_detector<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaEnableSelectedDataDetector,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_text_area_enable_selected_data_detector(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaEnableSelectedDataDetector,
        )
    }

    #[cfg(feature = "api-18")]
    pub fn set_text_area_half_leading<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaHalfLeading,
            value.into(),
        )
    }

    #[cfg(feature = "api-18")]
    pub fn get_text_area_half_leading(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaHalfLeading,
        )
    }

    #[cfg(feature = "api-15")]
    pub fn set_text_area_keyboard_appearance<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaKeyboardAppearance,
            value.into(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn get_text_area_keyboard_appearance(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaKeyboardAppearance,
        )
    }

    #[cfg(feature = "api-15")]
    pub fn set_text_area_letter_spacing<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaLetterSpacing,
            value.into(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn get_text_area_letter_spacing(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaLetterSpacing,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_text_area_line_height<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaLineHeight,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_text_area_line_height(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaLineHeight,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_text_area_line_spacing<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaLineSpacing,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_text_area_line_spacing(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaLineSpacing,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_text_area_max_lines<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaMaxLines,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_text_area_max_lines(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaMaxLines,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_text_area_max_lines_with_scroll<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaMaxLinesWithScroll,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_text_area_max_lines_with_scroll(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaMaxLinesWithScroll,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_text_area_min_lines<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaMinLines,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_text_area_min_lines(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaMinLines,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_text_area_scroll_bar_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaScrollBarColor,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_text_area_scroll_bar_color(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAreaScrollBarColor,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_TextArea
