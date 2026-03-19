//! Module component::built_in_component::relative_container wrappers and related types.

// BEGIN_GENERATED_COMPONENT_METHODS_RelativeContainer
impl super::RelativeContainer {
    pub fn set_relative_container_guide_line<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::RelativeContainerGuideLine,
            value.into(),
        )
    }

    pub fn get_relative_container_guide_line(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::RelativeContainerGuideLine,
        )
    }

    pub fn set_relative_container_barrier<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::RelativeContainerBarrier,
            value.into(),
        )
    }

    pub fn get_relative_container_barrier(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::RelativeContainerBarrier,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_RelativeContainer
