//! Module component::built_in_component::image_span wrappers and related types.

// BEGIN_GENERATED_COMPONENT_METHODS_ImageSpan
impl super::ImageSpan {
    pub fn set_image_span_src<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageSpanSrc,
            value.into(),
        )
    }

    pub fn get_image_span_src(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageSpanSrc,
        )
    }

    pub fn set_image_span_vertical_alignment<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageSpanVerticalAlignment,
            value.into(),
        )
    }

    pub fn get_image_span_vertical_alignment(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageSpanVerticalAlignment,
        )
    }

    pub fn set_image_span_alt<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageSpanAlt,
            value.into(),
        )
    }

    pub fn get_image_span_alt(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageSpanAlt,
        )
    }

    #[cfg(feature = "api-13")]
    pub fn set_image_span_baseline_offset<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageSpanBaselineOffset,
            value.into(),
        )
    }

    #[cfg(feature = "api-13")]
    pub fn get_image_span_baseline_offset(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageSpanBaselineOffset,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_image_span_color_filter<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageSpanColorFilter,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_image_span_color_filter(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageSpanColorFilter,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_image_span_support_svg2<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageSpanSupportSvg2,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_image_span_support_svg2(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageSpanSupportSvg2,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_ImageSpan
