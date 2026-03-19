//! Module component::built_in_component::refresh wrappers and related types.

// BEGIN_GENERATED_COMPONENT_METHODS_Refresh
impl super::Refresh {
    pub fn set_refresh_refreshing<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::RefreshRefreshing,
            value.into(),
        )
    }

    pub fn get_refresh_refreshing(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::RefreshRefreshing,
        )
    }

    pub fn set_refresh_content<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::RefreshContent,
            value.into(),
        )
    }

    pub fn get_refresh_content(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::RefreshContent,
        )
    }

    pub fn set_refresh_pull_down_ratio<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::RefreshPullDownRatio,
            value.into(),
        )
    }

    pub fn get_refresh_pull_down_ratio(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::RefreshPullDownRatio,
        )
    }

    pub fn set_refresh_offset<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::RefreshOffset,
            value.into(),
        )
    }

    pub fn get_refresh_offset(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::RefreshOffset,
        )
    }

    pub fn set_refresh_pull_to_refresh<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::RefreshPullToRefresh,
            value.into(),
        )
    }

    pub fn get_refresh_pull_to_refresh(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::RefreshPullToRefresh,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_refresh_max_pull_down_distance<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::RefreshMaxPullDownDistance,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_refresh_max_pull_down_distance(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::RefreshMaxPullDownDistance,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_Refresh

impl super::Refresh {
    pub fn on_refresh<T: Fn() + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event_no_param(self, crate::NodeEventType::RefreshOnRefresh, cb);
    }

    pub fn on_refresh_state_change<T: Fn(i32) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::RefreshStateChange,
            move |event| {
                cb(event.i32_value(0).unwrap_or_default());
            },
        );
    }

    pub fn on_refresh_offset_change<T: Fn(f32) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::RefreshOnOffsetChange,
            move |event| {
                cb(event.f32_value(0).unwrap_or_default());
            },
        );
    }
}
