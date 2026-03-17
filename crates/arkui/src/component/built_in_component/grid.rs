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

#[cfg(feature = "api-22")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridScrollIndexEvent {
    pub first_index: i32,
    pub last_index: i32,
}

#[cfg(feature = "api-22")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridWillScrollEvent {
    pub offset: f32,
    pub state: i32,
    pub source: i32,
}

#[cfg(feature = "api-22")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridDidScrollEvent {
    pub offset: f32,
    pub state: i32,
}

#[cfg(feature = "api-22")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridScrollBarUpdateEvent {
    pub first_visible_item_index: i32,
    pub first_visible_item_offset: f32,
}

#[cfg(feature = "api-22")]
impl super::Grid {
    pub fn on_grid_scroll_index<T: Fn(GridScrollIndexEvent) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::GridOnScrollIndex,
            move |event| {
                cb(GridScrollIndexEvent {
                    first_index: event.i32_value(0).unwrap_or_default(),
                    last_index: event.i32_value(1).unwrap_or_default(),
                });
            },
        );
    }

    pub fn on_grid_will_scroll<T: Fn(GridWillScrollEvent) -> Option<f32> + 'static>(
        &mut self,
        cb: T,
    ) {
        crate::ArkUIEvent::on_event(self, crate::NodeEventType::GridOnWillScroll, move |event| {
            let data = GridWillScrollEvent {
                offset: event.f32_value(0).unwrap_or_default(),
                state: event.i32_value(1).unwrap_or_default(),
                source: event.i32_value(2).unwrap_or_default(),
            };
            if let Some(value) = cb(data) {
                let _ = event.set_return_f32(value);
            }
        });
    }

    pub fn on_grid_did_scroll<T: Fn(GridDidScrollEvent) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(self, crate::NodeEventType::GridOnDidScroll, move |event| {
            cb(GridDidScrollEvent {
                offset: event.f32_value(0).unwrap_or_default(),
                state: event.i32_value(1).unwrap_or_default(),
            });
        });
    }

    pub fn on_grid_scroll_bar_update<
        T: Fn(GridScrollBarUpdateEvent) -> Option<(f32, f32)> + 'static,
    >(
        &mut self,
        cb: T,
    ) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::GridOnScrollBarUpdate,
            move |event| {
                let data = GridScrollBarUpdateEvent {
                    first_visible_item_index: event.i32_value(0).unwrap_or_default(),
                    first_visible_item_offset: event.f32_value(1).unwrap_or_default(),
                };
                if let Some((total_offset, total_length)) = cb(data) {
                    let mut values = [
                        ohos_arkui_sys::ArkUI_NumberValue { f32_: total_offset },
                        ohos_arkui_sys::ArkUI_NumberValue { f32_: total_length },
                    ];
                    let _ = event.set_return_values(&mut values);
                }
            },
        );
    }
}
