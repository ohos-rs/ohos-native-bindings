//! Module component::built_in_component::text wrappers and related types.

use crate::{
    ArkUIAttributeBasic, ArkUICommonFontAttribute, ArkUINodeAttributeItem,
    ArkUINodeAttributeNumber, ArkUIResult, TextAlignment, ARK_UI_NATIVE_NODE_API_1,
};

impl super::Text {
    pub fn content<T: Into<String>>(&self, content: T) -> ArkUIResult<()> {
        let content_property = ArkUINodeAttributeItem::String(content.into());
        ARK_UI_NATIVE_NODE_API_1.with(|api| {
            api.set_attribute(
                self.raw(),
                crate::ArkUINodeAttributeType::TextContent,
                content_property,
            )
        })?;
        Ok(())
    }

    pub fn alignment(&self, alignment: TextAlignment) -> ArkUIResult<()> {
        let alignment_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Int(
                alignment.into(),
            )]);
        ARK_UI_NATIVE_NODE_API_1.with(|api| {
            api.set_attribute(
                self.raw(),
                crate::ArkUINodeAttributeType::TextAlign,
                alignment_property,
            )
        })?;
        Ok(())
    }
}

impl ArkUICommonFontAttribute for super::Text {}

// BEGIN_GENERATED_COMPONENT_METHODS_Text
impl super::Text {
    pub fn set_text_content<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextContent,
            value.into(),
        )
    }

    pub fn get_text_content(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextContent,
        )
    }

    pub fn set_text_line_height<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextLineHeight,
            value.into(),
        )
    }

    pub fn get_text_line_height(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextLineHeight,
        )
    }

    pub fn set_text_decoration<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextDecoration,
            value.into(),
        )
    }

    pub fn get_text_decoration(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextDecoration,
        )
    }

    pub fn set_text_case<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextCase,
            value.into(),
        )
    }

    pub fn get_text_case(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextCase,
        )
    }

    pub fn set_text_letter_spacing<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextLetterSpacing,
            value.into(),
        )
    }

    pub fn get_text_letter_spacing(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextLetterSpacing,
        )
    }

    pub fn set_text_max_lines<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextMaxLines,
            value.into(),
        )
    }

    pub fn get_text_max_lines(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextMaxLines,
        )
    }

    pub fn set_text_align<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAlign,
            value.into(),
        )
    }

    pub fn get_text_align(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextAlign,
        )
    }

    pub fn set_text_overflow<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextOverflow,
            value.into(),
        )
    }

    pub fn get_text_overflow(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextOverflow,
        )
    }

    pub fn set_text_copy_option<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextCopyOption,
            value.into(),
        )
    }

    pub fn get_text_copy_option(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextCopyOption,
        )
    }

    pub fn set_text_baseline_offset<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextBaselineOffset,
            value.into(),
        )
    }

    pub fn get_text_baseline_offset(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextBaselineOffset,
        )
    }

    pub fn set_text_text_shadow<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextTextShadow,
            value.into(),
        )
    }

    pub fn get_text_text_shadow(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextTextShadow,
        )
    }

    pub fn set_text_min_font_size<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextMinFontSize,
            value.into(),
        )
    }

    pub fn get_text_min_font_size(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextMinFontSize,
        )
    }

    pub fn set_text_max_font_size<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextMaxFontSize,
            value.into(),
        )
    }

    pub fn get_text_max_font_size(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextMaxFontSize,
        )
    }

    pub fn set_text_font<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextFont,
            value.into(),
        )
    }

    pub fn get_text_font(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextFont,
        )
    }

    pub fn set_text_height_adaptive_policy<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextHeightAdaptivePolicy,
            value.into(),
        )
    }

    pub fn get_text_height_adaptive_policy(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextHeightAdaptivePolicy,
        )
    }

    pub fn set_text_indent<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextIndent,
            value.into(),
        )
    }

    pub fn get_text_indent(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextIndent,
        )
    }

    pub fn set_text_word_break<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextWordBreak,
            value.into(),
        )
    }

    pub fn get_text_word_break(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextWordBreak,
        )
    }

    pub fn set_text_ellipsis_mode<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextEllipsisMode,
            value.into(),
        )
    }

    pub fn get_text_ellipsis_mode(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextEllipsisMode,
        )
    }

    pub fn set_text_line_spacing<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextLineSpacing,
            value.into(),
        )
    }

    pub fn get_text_line_spacing(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextLineSpacing,
        )
    }

    pub fn set_text_enable_data_detector<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextEnableDataDetector,
            value.into(),
        )
    }

    pub fn get_text_enable_data_detector(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextEnableDataDetector,
        )
    }

    pub fn set_text_enable_data_detector_config<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextEnableDataDetectorConfig,
            value.into(),
        )
    }

    pub fn get_text_enable_data_detector_config(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextEnableDataDetectorConfig,
        )
    }

    pub fn set_text_selected_background_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextSelectedBackgroundColor,
            value.into(),
        )
    }

    pub fn get_text_selected_background_color(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextSelectedBackgroundColor,
        )
    }

    pub fn set_text_content_with_styled_string<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextContentWithStyledString,
            value.into(),
        )
    }

    pub fn get_text_content_with_styled_string(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextContentWithStyledString,
        )
    }

    pub fn set_text_half_leading<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextHalfLeading,
            value.into(),
        )
    }

    pub fn get_text_half_leading(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextHalfLeading,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_text_bind_selection_menu<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextBindSelectionMenu,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_text_bind_selection_menu(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextBindSelectionMenu,
        )
    }

    #[cfg(feature = "api-21")]
    pub fn set_text_content_align<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextContentAlign,
            value.into(),
        )
    }

    #[cfg(feature = "api-21")]
    pub fn get_text_content_align(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextContentAlign,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_text_edit_menu_options<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextEditMenuOptions,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_text_edit_menu_options(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextEditMenuOptions,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_text_enable_selected_data_detector<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextEnableSelectedDataDetector,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_text_enable_selected_data_detector(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextEnableSelectedDataDetector,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_text_layout_manager<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextLayoutManager,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_text_layout_manager(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextLayoutManager,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_text_linear_gradient<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextLinearGradient,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_text_linear_gradient(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextLinearGradient,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_text_line_count<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextLineCount,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_text_line_count(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextLineCount,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_text_line_height_multiple<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextLineHeightMultiple,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_text_line_height_multiple(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextLineHeightMultiple,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_text_max_line_height<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextMaxLineHeight,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_text_max_line_height(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextMaxLineHeight,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_text_min_lines<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextMinLines,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_text_min_lines(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextMinLines,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_text_min_line_height<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextMinLineHeight,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_text_min_line_height(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextMinLineHeight,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_text_optimize_trailing_space<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextOptimizeTrailingSpace,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_text_optimize_trailing_space(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextOptimizeTrailingSpace,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_text_radial_gradient<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextRadialGradient,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_text_radial_gradient(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextRadialGradient,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_text_vertical_align<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextVerticalAlign,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_text_vertical_align(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextVerticalAlign,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_Text

impl super::Text {
    #[cfg(feature = "drawing")]
    pub fn set_text_content_with_styled_string_object(
        &self,
        styled_string: &crate::StyledString,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextContentWithStyledString,
            crate::ArkUINodeAttributeItem::Object(styled_string.raw().cast()),
        )
    }

    #[cfg(feature = "drawing")]
    pub fn get_text_content_with_styled_string_object(
        &self,
    ) -> crate::ArkUIResult<Option<crate::StyledString>> {
        match <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextContentWithStyledString,
        )? {
            crate::ArkUINodeAttributeItem::Object(ptr) => {
                Ok(Some(crate::StyledString::from_raw(ptr.cast())))
            }
            _ => Ok(None),
        }
    }

    pub fn on_text_detect_result_update<T: Fn(Option<String>) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::TextOnDetectResultUpdate,
            move |event| {
                cb(event.async_string());
            },
        );
    }
}

#[cfg(feature = "api-22")]
impl super::Text {
    pub fn set_text_bind_selection_menu_object(
        &self,
        options: &crate::TextSelectionMenuOptions,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextBindSelectionMenu,
            crate::ArkUINodeAttributeItem::Object(options.raw().cast()),
        )
    }

    pub fn get_text_bind_selection_menu_object(
        &self,
    ) -> crate::ArkUIResult<Option<crate::TextSelectionMenuOptions>> {
        match <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextBindSelectionMenu,
        )? {
            crate::ArkUINodeAttributeItem::Object(ptr) => {
                Ok(Some(crate::TextSelectionMenuOptions::from_raw(ptr.cast())))
            }
            _ => Ok(None),
        }
    }

    pub fn set_text_edit_menu_options_object(
        &self,
        options: &crate::TextEditMenuOptions,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextEditMenuOptions,
            crate::ArkUINodeAttributeItem::Object(options.raw().cast()),
        )
    }

    pub fn get_text_edit_menu_options_object(
        &self,
    ) -> crate::ArkUIResult<Option<crate::TextEditMenuOptions>> {
        match <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextEditMenuOptions,
        )? {
            crate::ArkUINodeAttributeItem::Object(ptr) => {
                Ok(Some(crate::TextEditMenuOptions::from_raw(ptr.cast())))
            }
            _ => Ok(None),
        }
    }

    #[cfg(feature = "drawing")]
    pub fn set_text_layout_manager_object(
        &self,
        manager: &crate::TextLayoutManager,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::TextLayoutManager,
            crate::ArkUINodeAttributeItem::Object(manager.raw().cast()),
        )
    }

    #[cfg(feature = "drawing")]
    pub fn get_text_layout_manager_object(
        &self,
    ) -> crate::ArkUIResult<Option<crate::TextLayoutManager>> {
        match <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::TextLayoutManager,
        )? {
            crate::ArkUINodeAttributeItem::Object(ptr) => {
                Ok(Some(crate::TextLayoutManager::from_raw(ptr.cast())))
            }
            _ => Ok(None),
        }
    }
}
