use crate::{
    ArkUIAttributeBasic, ArkUINodeAttributeItem, ArkUINodeAttributeNumber, ArkUIResult,
    ScrollBarDisplayMode, ARK_UI_NATIVE_NODE_API_1,
};

impl super::List {
    pub fn scroll_bar_state(&mut self, mode: ScrollBarDisplayMode) -> ArkUIResult<()> {
        let scroll_bar_display_mode_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Int(mode.into())]);
        ARK_UI_NATIVE_NODE_API_1.with(|api| {
            api.set_attribute(
                self.raw(),
                crate::ArkUINodeAttributeType::ScrollBarDisplayMode,
                scroll_bar_display_mode_property,
            )
        })?;
        Ok(())
    }
}

// BEGIN_GENERATED_COMPONENT_METHODS_List
impl super::List {
    pub fn set_list_direction<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListDirection,
            value.into(),
        )
    }

    pub fn get_list_direction(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListDirection,
        )
    }

    pub fn set_list_sticky<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListSticky,
            value.into(),
        )
    }

    pub fn get_list_sticky(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListSticky,
        )
    }

    pub fn set_list_space<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListSpace,
            value.into(),
        )
    }

    pub fn get_list_space(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListSpace,
        )
    }

    pub fn set_list_cached_count<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListCachedCount,
            value.into(),
        )
    }

    pub fn get_list_cached_count(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListCachedCount,
        )
    }

    pub fn set_list_scroll_to_index<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListScrollToIndex,
            value.into(),
        )
    }

    pub fn get_list_scroll_to_index(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListScrollToIndex,
        )
    }

    pub fn set_list_align_list_item<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListAlignListItem,
            value.into(),
        )
    }

    pub fn get_list_align_list_item(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListAlignListItem,
        )
    }

    pub fn set_list_children_main_size<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListChildrenMainSize,
            value.into(),
        )
    }

    pub fn get_list_children_main_size(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListChildrenMainSize,
        )
    }

    pub fn set_list_initial_index<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListInitialIndex,
            value.into(),
        )
    }

    pub fn get_list_initial_index(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListInitialIndex,
        )
    }

    pub fn set_list_divider<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListDivider,
            value.into(),
        )
    }

    pub fn get_list_divider(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListDivider,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_list_focus_wrap_mode<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListFocusWrapMode,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_list_focus_wrap_mode(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListFocusWrapMode,
        )
    }

    #[cfg(feature = "api-15")]
    pub fn set_list_lanes<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListLanes,
            value.into(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn get_list_lanes(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListLanes,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_list_lanes_itemfillpolicy<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListLanesItemfillpolicy,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_list_lanes_itemfillpolicy(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListLanesItemfillpolicy,
        )
    }

    #[cfg(feature = "api-15")]
    pub fn set_list_maintain_visible_content_position<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListMaintainVisibleContentPosition,
            value.into(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn get_list_maintain_visible_content_position(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListMaintainVisibleContentPosition,
        )
    }

    pub fn set_list_node_adapter<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListNodeAdapter,
            value.into(),
        )
    }

    pub fn get_list_node_adapter(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListNodeAdapter,
        )
    }

    #[cfg(feature = "api-15")]
    pub fn set_list_scroll_snap_align<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListScrollSnapAlign,
            value.into(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn get_list_scroll_snap_align(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListScrollSnapAlign,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_list_scroll_snap_animation_speed<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListScrollSnapAnimationSpeed,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_list_scroll_snap_animation_speed(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListScrollSnapAnimationSpeed,
        )
    }

    #[cfg(feature = "api-15")]
    pub fn set_list_scroll_to_index_in_group<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListScrollToIndexInGroup,
            value.into(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn get_list_scroll_to_index_in_group(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListScrollToIndexInGroup,
        )
    }

    #[cfg(feature = "api-19")]
    pub fn set_list_stack_from_end<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListStackFromEnd,
            value.into(),
        )
    }

    #[cfg(feature = "api-19")]
    pub fn get_list_stack_from_end(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListStackFromEnd,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_list_sync_load<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ListSyncLoad,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_list_sync_load(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ListSyncLoad,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_List
