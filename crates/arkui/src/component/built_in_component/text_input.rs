//! Module component::built_in_component::text_input wrappers and related types.

use crate::ArkUICommonFontAttribute;

impl Clone for super::TextInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl ArkUICommonFontAttribute for super::TextInput {}

// BEGIN_GENERATED_COMPONENT_METHODS_TextInput
impl super::TextInput {
    pub fn set_text_input_placeholder<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputPlaceholder,
            value.into(),
        )
    }

    pub fn get_text_input_placeholder(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputPlaceholder,
        )
    }

    pub fn set_text_input_text<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputText,
            value.into(),
        )
    }

    pub fn get_text_input_text(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputText,
        )
    }

    pub fn set_text_input_caret_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputCaretColor,
            value.into(),
        )
    }

    pub fn get_text_input_caret_color(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputCaretColor,
        )
    }

    pub fn set_text_input_caret_style<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputCaretStyle,
            value.into(),
        )
    }

    pub fn get_text_input_caret_style(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputCaretStyle,
        )
    }

    pub fn set_text_input_show_underline<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputShowUnderline,
            value.into(),
        )
    }

    pub fn get_text_input_show_underline(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputShowUnderline,
        )
    }

    pub fn set_text_input_max_length<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputMaxLength,
            value.into(),
        )
    }

    pub fn get_text_input_max_length(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputMaxLength,
        )
    }

    pub fn set_text_input_enter_key_type<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputEnterKeyType,
            value.into(),
        )
    }

    pub fn get_text_input_enter_key_type(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputEnterKeyType,
        )
    }

    pub fn set_text_input_placeholder_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputPlaceholderColor,
            value.into(),
        )
    }

    pub fn get_text_input_placeholder_color(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputPlaceholderColor,
        )
    }

    pub fn set_text_input_placeholder_font<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputPlaceholderFont,
            value.into(),
        )
    }

    pub fn get_text_input_placeholder_font(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputPlaceholderFont,
        )
    }

    pub fn set_text_input_enable_keyboard_on_focus<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputEnableKeyboardOnFocus,
            value.into(),
        )
    }

    pub fn get_text_input_enable_keyboard_on_focus(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputEnableKeyboardOnFocus,
        )
    }

    pub fn set_text_input_type<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputType,
            value.into(),
        )
    }

    pub fn get_text_input_type(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputType,
        )
    }

    pub fn set_text_input_selected_background_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputSelectedBackgroundColor,
            value.into(),
        )
    }

    pub fn get_text_input_selected_background_color(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputSelectedBackgroundColor,
        )
    }

    pub fn set_text_input_show_password_icon<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputShowPasswordIcon,
            value.into(),
        )
    }

    pub fn get_text_input_show_password_icon(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputShowPasswordIcon,
        )
    }

    pub fn set_text_input_editing<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputEditing,
            value.into(),
        )
    }

    pub fn get_text_input_editing(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputEditing,
        )
    }

    pub fn set_text_input_cancel_button<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputCancelButton,
            value.into(),
        )
    }

    pub fn get_text_input_cancel_button(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputCancelButton,
        )
    }

    pub fn set_text_input_text_selection<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputTextSelection,
            value.into(),
        )
    }

    pub fn get_text_input_text_selection(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputTextSelection,
        )
    }

    pub fn set_text_input_underline_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputUnderlineColor,
            value.into(),
        )
    }

    pub fn get_text_input_underline_color(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputUnderlineColor,
        )
    }

    pub fn set_text_input_enable_auto_fill<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputEnableAutoFill,
            value.into(),
        )
    }

    pub fn get_text_input_enable_auto_fill(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputEnableAutoFill,
        )
    }

    pub fn set_text_input_content_type<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputContentType,
            value.into(),
        )
    }

    pub fn get_text_input_content_type(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputContentType,
        )
    }

    pub fn set_text_input_password_rules<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputPasswordRules,
            value.into(),
        )
    }

    pub fn get_text_input_password_rules(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputPasswordRules,
        )
    }

    pub fn set_text_input_select_all<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputSelectAll,
            value.into(),
        )
    }

    pub fn get_text_input_select_all(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputSelectAll,
        )
    }

    pub fn set_text_input_input_filter<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputInputFilter,
            value.into(),
        )
    }

    pub fn get_text_input_input_filter(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputInputFilter,
        )
    }

    pub fn set_text_input_style<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputStyle,
            value.into(),
        )
    }

    pub fn get_text_input_style(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputStyle,
        )
    }

    pub fn set_text_input_caret_offset<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputCaretOffset,
            value.into(),
        )
    }

    pub fn get_text_input_caret_offset(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputCaretOffset,
        )
    }

    pub fn set_text_input_content_rect<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputContentRect,
            value.into(),
        )
    }

    pub fn get_text_input_content_rect(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputContentRect,
        )
    }

    pub fn set_text_input_content_line_count<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputContentLineCount,
            value.into(),
        )
    }

    pub fn get_text_input_content_line_count(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputContentLineCount,
        )
    }

    pub fn set_text_input_selection_menu_hidden<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputSelectionMenuHidden,
            value.into(),
        )
    }

    pub fn get_text_input_selection_menu_hidden(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputSelectionMenuHidden,
        )
    }

    pub fn set_text_input_blur_on_submit<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputBlurOnSubmit,
            value.into(),
        )
    }

    pub fn get_text_input_blur_on_submit(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputBlurOnSubmit,
        )
    }

    pub fn set_text_input_custom_keyboard<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputCustomKeyboard,
            value.into(),
        )
    }

    pub fn get_text_input_custom_keyboard(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputCustomKeyboard,
        )
    }

    pub fn set_text_input_word_break<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputWordBreak,
            value.into(),
        )
    }

    pub fn get_text_input_word_break(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputWordBreak,
        )
    }

    pub fn set_text_input_show_keyboard_on_focus<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputShowKeyboardOnFocus,
            value.into(),
        )
    }

    pub fn get_text_input_show_keyboard_on_focus(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputShowKeyboardOnFocus,
        )
    }

    pub fn set_text_input_number_of_lines<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputNumberOfLines,
            value.into(),
        )
    }

    pub fn get_text_input_number_of_lines(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputNumberOfLines,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_text_input_enable_fill_animation<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputEnableFillAnimation,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_text_input_enable_fill_animation(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputEnableFillAnimation,
        )
    }

    #[cfg(feature = "api-15")]
    pub fn set_text_input_enable_preview_text<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputEnablePreviewText,
            value.into(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn get_text_input_enable_preview_text(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputEnablePreviewText,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_text_input_enable_selected_data_detector<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputEnableSelectedDataDetector,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_text_input_enable_selected_data_detector(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputEnableSelectedDataDetector,
        )
    }

    #[cfg(feature = "api-18")]
    pub fn set_text_input_half_leading<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputHalfLeading,
            value.into(),
        )
    }

    #[cfg(feature = "api-18")]
    pub fn get_text_input_half_leading(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputHalfLeading,
        )
    }

    #[cfg(feature = "api-15")]
    pub fn set_text_input_keyboard_appearance<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputKeyboardAppearance,
            value.into(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn get_text_input_keyboard_appearance(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputKeyboardAppearance,
        )
    }

    #[cfg(feature = "api-15")]
    pub fn set_text_input_letter_spacing<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputLetterSpacing,
            value.into(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn get_text_input_letter_spacing(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputLetterSpacing,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_text_input_line_height<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputLineHeight,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_text_input_line_height(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputLineHeight,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_text_input_show_counter<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputShowCounter,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_text_input_show_counter(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextInputShowCounter,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_TextInput

impl super::TextInput {
    pub fn on_text_input_change<T: Fn(String) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::TextInputOnChange,
            move |event| {
                cb(event.async_string().unwrap_or_default());
            },
        );
    }

    pub fn on_text_input_submit<T: Fn(i32) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::TextInputOnSubmit,
            move |event| {
                cb(event.i32_value(0).unwrap_or_default());
            },
        );
    }

    pub fn on_text_input_cut<T: Fn(String) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(self, crate::NodeEventType::TextInputOnCut, move |event| {
            cb(event.async_string().unwrap_or_default());
        });
    }

    pub fn on_text_input_paste<T: Fn(String) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(self, crate::NodeEventType::TextInputOnPaste, move |event| {
            cb(event.async_string().unwrap_or_default());
        });
    }

    pub fn on_text_input_selection_change<T: Fn(i32, i32) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::TextInputOnTextSelectionChange,
            move |event| {
                cb(
                    event.i32_value(0).unwrap_or_default(),
                    event.i32_value(1).unwrap_or_default(),
                );
            },
        );
    }

    pub fn on_text_input_edit_change<T: Fn(bool) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::TextInputOnEditChange,
            move |event| {
                cb(event.i32_value(0).unwrap_or_default() != 0);
            },
        );
    }

    pub fn on_text_input_content_size_change<T: Fn(f32, f32) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::TextInputOnContentSizeChange,
            move |event| {
                cb(
                    event.f32_value(0).unwrap_or_default(),
                    event.f32_value(1).unwrap_or_default(),
                );
            },
        );
    }

    pub fn on_text_input_input_filter_error<T: Fn(String) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::TextInputOnInputFilterError,
            move |event| {
                cb(event.async_string().unwrap_or_default());
            },
        );
    }

    pub fn on_text_input_content_scroll<T: Fn(i32, i32) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::TextInputOnContentScroll,
            move |event| {
                cb(
                    event.i32_value(0).unwrap_or_default(),
                    event.i32_value(1).unwrap_or_default(),
                );
            },
        );
    }

    pub fn on_text_input_will_insert<T: Fn(f32, String) -> bool + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::TextInputOnWillInsert,
            move |event| {
                let should_insert = cb(
                    event.f32_value(0).unwrap_or_default(),
                    event.string_value(0).unwrap_or_default(),
                );
                let _ = event.set_return_bool(should_insert);
            },
        );
    }

    pub fn on_text_input_did_insert<T: Fn(f32, String) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::TextInputOnDidInsert,
            move |event| {
                cb(
                    event.f32_value(0).unwrap_or_default(),
                    event.string_value(0).unwrap_or_default(),
                );
            },
        );
    }

    pub fn on_text_input_will_delete<T: Fn(f32, i32, String) -> bool + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::TextInputOnWillDelete,
            move |event| {
                let should_delete = cb(
                    event.f32_value(0).unwrap_or_default(),
                    event.i32_value(1).unwrap_or_default(),
                    event.string_value(0).unwrap_or_default(),
                );
                let _ = event.set_return_bool(should_delete);
            },
        );
    }

    pub fn on_text_input_did_delete<T: Fn(f32, i32, String) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::TextInputOnDidDelete,
            move |event| {
                cb(
                    event.f32_value(0).unwrap_or_default(),
                    event.i32_value(1).unwrap_or_default(),
                    event.string_value(0).unwrap_or_default(),
                );
            },
        );
    }

    #[cfg(feature = "api-15")]
    pub fn on_text_input_change_with_preview_text<T: Fn(crate::TextChangeEventData) + 'static>(
        &mut self,
        cb: T,
    ) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::TextInputOnChangeWithPreviewText,
            move |event| {
                if let Some(data) = event.text_change() {
                    cb(data);
                }
            },
        );
    }

    #[cfg(feature = "api-20")]
    pub fn on_text_input_will_change<T: Fn(crate::TextChangeEventData) + 'static>(
        &mut self,
        cb: T,
    ) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::TextInputOnWillChange,
            move |event| {
                if let Some(data) = event.text_change() {
                    cb(data);
                }
            },
        );
    }
}
