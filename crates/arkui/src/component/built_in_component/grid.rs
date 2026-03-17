// BEGIN_GENERATED_COMPONENT_METHODS_Grid
impl super::Grid {
    pub fn set_grid_column_template<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::GridColumnTemplate,
            value.into(),
        )
    }

    pub fn get_grid_column_template(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::GridColumnTemplate,
        )
    }

    pub fn set_grid_row_template<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::GridRowTemplate,
            value.into(),
        )
    }

    pub fn get_grid_row_template(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::GridRowTemplate,
        )
    }

    pub fn set_grid_column_gap<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::GridColumnGap,
            value.into(),
        )
    }

    pub fn get_grid_column_gap(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::GridColumnGap,
        )
    }

    pub fn set_grid_row_gap<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::GridRowGap,
            value.into(),
        )
    }

    pub fn get_grid_row_gap(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::GridRowGap,
        )
    }

    pub fn set_grid_node_adapter<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::GridNodeAdapter,
            value.into(),
        )
    }

    pub fn get_grid_node_adapter(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::GridNodeAdapter,
        )
    }

    pub fn set_grid_cached_count<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::GridCachedCount,
            value.into(),
        )
    }

    pub fn get_grid_cached_count(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::GridCachedCount,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_grid_align_items<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::GridAlignItems,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_grid_align_items(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::GridAlignItems,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_grid_column_template_itemfillpolicy<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::GridColumnTemplateItemfillpolicy,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_grid_column_template_itemfillpolicy(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::GridColumnTemplateItemfillpolicy,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_grid_focus_wrap_mode<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::GridFocusWrapMode,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_grid_focus_wrap_mode(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::GridFocusWrapMode,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_grid_item_style<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::GridItemStyle,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_grid_item_style(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::GridItemStyle,
        )
    }

    #[cfg(feature = "api-22")]
    pub fn set_grid_layout_options<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::GridLayoutOptions,
            value.into(),
        )
    }

    #[cfg(feature = "api-22")]
    pub fn get_grid_layout_options(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::GridLayoutOptions,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_grid_sync_load<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::GridSyncLoad,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_grid_sync_load(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::GridSyncLoad,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_Grid
