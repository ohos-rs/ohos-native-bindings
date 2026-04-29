//! Module component::built_in_component::span wrappers and related types.

use crate::ArkUICommonFontAttribute;

impl ArkUICommonFontAttribute for super::Span {}

// BEGIN_GENERATED_COMPONENT_METHODS_Span
impl super::Span {
    pub fn set_span_content<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SpanContent,
            value.into(),
        )
    }

    pub fn get_span_content(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SpanContent,
        )
    }

    pub fn set_span_text_background_style<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SpanTextBackgroundStyle,
            value.into(),
        )
    }

    pub fn get_span_text_background_style(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SpanTextBackgroundStyle,
        )
    }

    pub fn set_span_baseline_offset<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SpanBaselineOffset,
            value.into(),
        )
    }

    pub fn get_span_baseline_offset(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SpanBaselineOffset,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_Span

impl super::Span {
    #[cfg(feature = "api-20")]
    pub fn on_text_span_long_press<T: Fn(&crate::Event) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(self, crate::NodeEventType::TextSpanOnLongPress, cb);
    }
}
