// BEGIN_GENERATED_COMPONENT_METHODS_ListItemGroup
impl super::ListItemGroup {
    pub fn set_list_item_group_set_header<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListItemGroupSetHeader,
            value.into(),
        )
    }

    pub fn get_list_item_group_set_header(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListItemGroupSetHeader,
        )
    }

    pub fn set_list_item_group_set_footer<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListItemGroupSetFooter,
            value.into(),
        )
    }

    pub fn get_list_item_group_set_footer(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListItemGroupSetFooter,
        )
    }

    pub fn set_list_item_group_set_divider<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListItemGroupSetDivider,
            value.into(),
        )
    }

    pub fn get_list_item_group_set_divider(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListItemGroupSetDivider,
        )
    }

    pub fn set_list_item_group_children_main_size<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListItemGroupChildrenMainSize,
            value.into(),
        )
    }

    pub fn get_list_item_group_children_main_size(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListItemGroupChildrenMainSize,
        )
    }

    #[cfg(feature = "api-15")]
    pub fn set_list_item_group_node_adapter<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListItemGroupNodeAdapter,
            value.into(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn get_list_item_group_node_adapter(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListItemGroupNodeAdapter,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_ListItemGroup
