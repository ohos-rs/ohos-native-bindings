// BEGIN_GENERATED_COMPONENT_METHODS_CheckboxGroup
#[cfg(feature = "api-15")]
impl super::CheckboxGroup {
    #[cfg(feature = "api-15")]
    pub fn set_checkbox_group<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::CheckboxGroup,
            value.into(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn get_checkbox_group(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::CheckboxGroup,
        )
    }

    #[cfg(feature = "api-15")]
    pub fn set_checkbox_group_mark<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::CheckboxGroupMark,
            value.into(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn get_checkbox_group_mark(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::CheckboxGroupMark,
        )
    }

    #[cfg(feature = "api-15")]
    pub fn set_checkbox_group_name<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::CheckboxGroupName,
            value.into(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn get_checkbox_group_name(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::CheckboxGroupName,
        )
    }

    #[cfg(feature = "api-15")]
    pub fn set_checkbox_group_selected_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::CheckboxGroupSelectedColor,
            value.into(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn get_checkbox_group_selected_color(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::CheckboxGroupSelectedColor,
        )
    }

    #[cfg(feature = "api-15")]
    pub fn set_checkbox_group_select_all<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::CheckboxGroupSelectAll,
            value.into(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn get_checkbox_group_select_all(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::CheckboxGroupSelectAll,
        )
    }

    #[cfg(feature = "api-15")]
    pub fn set_checkbox_group_shape<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::CheckboxGroupShape,
            value.into(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn get_checkbox_group_shape(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::CheckboxGroupShape,
        )
    }

    #[cfg(feature = "api-15")]
    pub fn set_checkbox_group_unselected_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::CheckboxGroupUnselectedColor,
            value.into(),
        )
    }

    #[cfg(feature = "api-15")]
    pub fn get_checkbox_group_unselected_color(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::CheckboxGroupUnselectedColor,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_CheckboxGroup

#[cfg(feature = "api-15")]
impl super::CheckboxGroup {
    pub fn on_checkbox_group_change<T: Fn(String) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::CheckboxGroupEventOnChange,
            move |event| {
                cb(event.async_string().unwrap_or_default());
            },
        );
    }
}
