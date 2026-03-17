use crate::ArkUICommonFontAttribute;

impl ArkUICommonFontAttribute for super::Button {}

// BEGIN_GENERATED_COMPONENT_METHODS_Button
impl super::Button {
    pub fn set_button_label<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ButtonLabel,
            value.into(),
        )
    }

    pub fn get_button_label(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ButtonLabel,
        )
    }

    pub fn set_button_type<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ButtonType,
            value.into(),
        )
    }

    pub fn get_button_type(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ButtonType,
        )
    }

    #[cfg(feature = "api-18")]
    pub fn set_button_max_font_scale<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ButtonMaxFontScale,
            value.into(),
        )
    }

    #[cfg(feature = "api-18")]
    pub fn get_button_max_font_scale(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ButtonMaxFontScale,
        )
    }

    #[cfg(feature = "api-18")]
    pub fn set_button_min_font_scale<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ButtonMinFontScale,
            value.into(),
        )
    }

    #[cfg(feature = "api-18")]
    pub fn get_button_min_font_scale(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ButtonMinFontScale,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_Button
