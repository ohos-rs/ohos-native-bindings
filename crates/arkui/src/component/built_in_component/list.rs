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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListScrollIndexEvent {
    pub first_index: i32,
    pub last_index: i32,
    pub center_index: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ListWillScrollEvent {
    pub offset: f32,
    pub state: i32,
    pub source: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ListDidScrollEvent {
    pub offset: f32,
    pub state: i32,
}

#[cfg(feature = "api-15")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListVisibleContentChangeEvent {
    pub first_index: i32,
    pub start_area: i32,
    pub start_item_index: i32,
    pub last_index: i32,
    pub end_area: i32,
    pub end_item_index: i32,
}

impl super::List {
    pub fn on_list_scroll_index<T: Fn(ListScrollIndexEvent) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::ListOnScrollIndex,
            move |event| {
                cb(ListScrollIndexEvent {
                    first_index: event.i32_value(0).unwrap_or_default(),
                    last_index: event.i32_value(1).unwrap_or_default(),
                    center_index: event.i32_value(2).unwrap_or_default(),
                });
            },
        );
    }

    pub fn on_list_will_scroll<T: Fn(ListWillScrollEvent) -> Option<f32> + 'static>(
        &mut self,
        cb: T,
    ) {
        crate::ArkUIEvent::on_event(self, crate::NodeEventType::ListOnWillScroll, move |event| {
            let data = ListWillScrollEvent {
                offset: event.f32_value(0).unwrap_or_default(),
                state: event.i32_value(1).unwrap_or_default(),
                source: event.i32_value(2).unwrap_or_default(),
            };
            if let Some(value) = cb(data) {
                let _ = event.set_return_f32(value);
            }
        });
    }

    pub fn on_list_did_scroll<T: Fn(ListDidScrollEvent) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(self, crate::NodeEventType::ListOnDidScroll, move |event| {
            cb(ListDidScrollEvent {
                offset: event.f32_value(0).unwrap_or_default(),
                state: event.i32_value(1).unwrap_or_default(),
            });
        });
    }

    #[cfg(feature = "api-15")]
    pub fn on_list_scroll_visible_content_change<T: Fn(ListVisibleContentChangeEvent) + 'static>(
        &mut self,
        cb: T,
    ) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::ListOnScrollVisibleContentChange,
            move |event| {
                cb(ListVisibleContentChangeEvent {
                    first_index: event.i32_value(0).unwrap_or_default(),
                    start_area: event.i32_value(1).unwrap_or_default(),
                    start_item_index: event.i32_value(2).unwrap_or_default(),
                    last_index: event.i32_value(3).unwrap_or_default(),
                    end_area: event.i32_value(4).unwrap_or_default(),
                    end_item_index: event.i32_value(5).unwrap_or_default(),
                });
            },
        );
    }
}
