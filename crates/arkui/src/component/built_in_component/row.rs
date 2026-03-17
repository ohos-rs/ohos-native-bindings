// BEGIN_GENERATED_COMPONENT_METHODS_Row
impl super::Row {
    pub fn set_row_align_items<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::RowAlignItems,
            value.into(),
        )
    }

    pub fn get_row_align_items(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::RowAlignItems,
        )
    }

    pub fn set_row_justify_content<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::RowJustifyContent,
            value.into(),
        )
    }

    pub fn get_row_justify_content(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::RowJustifyContent,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_Row
