//! Module component::built_in_component::water_flow wrappers and related types.

// BEGIN_GENERATED_COMPONENT_METHODS_WaterFlow
impl super::WaterFlow {
    pub fn set_water_flow_layout_direction<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowLayoutDirection,
            value.into(),
        )
    }

    pub fn get_water_flow_layout_direction(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowLayoutDirection,
        )
    }

    pub fn set_water_flow_column_template<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowColumnTemplate,
            value.into(),
        )
    }

    pub fn get_water_flow_column_template(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowColumnTemplate,
        )
    }

    pub fn set_water_flow_row_template<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowRowTemplate,
            value.into(),
        )
    }

    pub fn get_water_flow_row_template(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowRowTemplate,
        )
    }

    pub fn set_water_flow_column_gap<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowColumnGap,
            value.into(),
        )
    }

    pub fn get_water_flow_column_gap(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowColumnGap,
        )
    }

    pub fn set_water_flow_row_gap<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowRowGap,
            value.into(),
        )
    }

    pub fn get_water_flow_row_gap(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowRowGap,
        )
    }

    pub fn set_water_flow_section_option<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowSectionOption,
            value.into(),
        )
    }

    pub fn get_water_flow_section_option(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowSectionOption,
        )
    }

    pub fn set_water_flow_node_adapter<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowNodeAdapter,
            value.into(),
        )
    }

    pub fn get_water_flow_node_adapter(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowNodeAdapter,
        )
    }

    pub fn set_water_flow_cached_count<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowCachedCount,
            value.into(),
        )
    }

    pub fn get_water_flow_cached_count(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowCachedCount,
        )
    }

    pub fn set_water_flow_footer<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowFooter,
            value.into(),
        )
    }

    pub fn get_water_flow_footer(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowFooter,
        )
    }

    pub fn set_water_flow_scroll_to_index<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowScrollToIndex,
            value.into(),
        )
    }

    pub fn get_water_flow_scroll_to_index(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowScrollToIndex,
        )
    }

    pub fn set_water_flow_item_constraint_size<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowItemConstraintSize,
            value.into(),
        )
    }

    pub fn get_water_flow_item_constraint_size(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowItemConstraintSize,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_water_flow_column_template_itemfillpolicy<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowColumnTemplateItemfillpolicy,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_water_flow_column_template_itemfillpolicy(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowColumnTemplateItemfillpolicy,
        )
    }

    #[cfg(feature = "api-18")]
    pub fn set_water_flow_layout_mode<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowLayoutMode,
            value.into(),
        )
    }

    #[cfg(feature = "api-18")]
    pub fn get_water_flow_layout_mode(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowLayoutMode,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_water_flow_sync_load<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowSyncLoad,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_water_flow_sync_load(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::WaterFlowSyncLoad,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_WaterFlow

#[derive(Clone, Copy, Debug, PartialEq)]
/// Event payload emitted after water-flow scrolling.
pub struct WaterFlowDidScrollEvent {
    pub offset: f32,
    pub state: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Event payload describing current first/last visible index of water-flow.
pub struct WaterFlowScrollIndexEvent {
    pub start_index: i32,
    pub end_index: i32,
}

impl super::WaterFlow {
    pub fn on_water_flow_did_scroll<T: Fn(WaterFlowDidScrollEvent) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::WaterFlowOnDidScroll,
            move |event| {
                cb(WaterFlowDidScrollEvent {
                    offset: event.f32_value(0).unwrap_or_default(),
                    state: event.i32_value(1).unwrap_or_default(),
                });
            },
        );
    }

    pub fn on_water_flow_scroll_index<T: Fn(WaterFlowScrollIndexEvent) + 'static>(
        &mut self,
        cb: T,
    ) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::WaterFlowOnScrollIndex,
            move |event| {
                cb(WaterFlowScrollIndexEvent {
                    start_index: event.i32_value(0).unwrap_or_default(),
                    end_index: event.i32_value(1).unwrap_or_default(),
                });
            },
        );
    }
}
