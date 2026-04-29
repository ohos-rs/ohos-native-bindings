//! Module component::built_in_component::stack wrappers and related types.

// BEGIN_GENERATED_COMPONENT_METHODS_Stack
impl super::Stack {
    pub fn set_stack_align_content<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::StackAlignContent,
            value.into(),
        )
    }

    pub fn get_stack_align_content(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::StackAlignContent,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_Stack
