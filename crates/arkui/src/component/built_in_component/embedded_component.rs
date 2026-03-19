//! Module component::built_in_component::embedded_component wrappers and related types.

// BEGIN_GENERATED_COMPONENT_METHODS_EmbeddedComponent
#[cfg(feature = "api-20")]
impl super::EmbeddedComponent {
    #[cfg(feature = "api-20")]
    pub fn set_embedded_component_option<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::EmbeddedComponentOption,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_embedded_component_option(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::EmbeddedComponentOption,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_embedded_component_want<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::EmbeddedComponentWant,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_embedded_component_want(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::EmbeddedComponentWant,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_EmbeddedComponent

#[cfg(feature = "api-20")]
impl super::EmbeddedComponent {
    pub fn set_embedded_component_option_object(
        &self,
        option: &crate::EmbeddedComponentOption,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::EmbeddedComponentOption,
            crate::ArkUINodeAttributeItem::Object(option.raw().cast()),
        )
    }

    pub fn get_embedded_component_option_object(
        &self,
    ) -> crate::ArkUIResult<Option<crate::EmbeddedComponentOption>> {
        match <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::EmbeddedComponentOption,
        )? {
            crate::ArkUINodeAttributeItem::Object(ptr) => {
                Ok(Some(crate::EmbeddedComponentOption::from_raw(ptr.cast())))
            }
            _ => Ok(None),
        }
    }
}
