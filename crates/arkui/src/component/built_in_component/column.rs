// BEGIN_GENERATED_COMPONENT_METHODS_Column
impl super::Column {
    pub fn set_column_align_items<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ColumnAlignItems,
            value.into(),
        )
    }

    pub fn get_column_align_items(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ColumnAlignItems,
        )
    }

    pub fn set_column_justify_content<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ColumnJustifyContent,
            value.into(),
        )
    }

    pub fn get_column_justify_content(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ColumnJustifyContent,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_Column
