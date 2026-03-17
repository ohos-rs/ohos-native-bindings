// BEGIN_GENERATED_COMPONENT_METHODS_ListItem
impl super::ListItem {
    pub fn set_list_item_swipe_action<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListItemSwipeAction,
            value.into(),
        )
    }

    pub fn get_list_item_swipe_action(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListItemSwipeAction,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_ListItem
