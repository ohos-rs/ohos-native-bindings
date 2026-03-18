#[cfg(feature = "api-21")]
use crate::check_arkui_status;
#[cfg(feature = "api-21")]
use ohos_arkui_sys::{OH_ArkUI_ListItemSwipeAction_Collapse, OH_ArkUI_ListItemSwipeAction_Expand};

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

#[cfg(feature = "api-21")]
impl super::ListItem {
    pub fn expand_swipe_action(
        &self,
        direction: crate::ListItemSwipeActionDirection,
    ) -> crate::ArkUIResult<()> {
        let node = crate::ArkUIAttributeBasic::raw(self).raw();
        unsafe { check_arkui_status!(OH_ArkUI_ListItemSwipeAction_Expand(node, direction.into())) }
    }

    pub fn collapse_swipe_action(&self) -> crate::ArkUIResult<()> {
        let node = crate::ArkUIAttributeBasic::raw(self).raw();
        unsafe { check_arkui_status!(OH_ArkUI_ListItemSwipeAction_Collapse(node)) }
    }
}
