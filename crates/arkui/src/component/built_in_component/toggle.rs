//! Module component::built_in_component::toggle wrappers and related types.

// BEGIN_GENERATED_COMPONENT_METHODS_Toggle
impl super::Toggle {
    pub fn set_toggle_selected_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ToggleSelectedColor,
            value.into(),
        )
    }

    pub fn get_toggle_selected_color(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ToggleSelectedColor,
        )
    }

    pub fn set_toggle_switch_point_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ToggleSwitchPointColor,
            value.into(),
        )
    }

    pub fn get_toggle_switch_point_color(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ToggleSwitchPointColor,
        )
    }

    pub fn set_toggle_value<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ToggleValue,
            value.into(),
        )
    }

    pub fn get_toggle_value(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ToggleValue,
        )
    }

    pub fn set_toggle_unselected_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ToggleUnselectedColor,
            value.into(),
        )
    }

    pub fn get_toggle_unselected_color(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ToggleUnselectedColor,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_Toggle

impl super::Toggle {
    pub fn on_toggle_change<T: Fn(bool) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(self, crate::NodeEventType::ToggleOnChange, move |event| {
            cb(event.i32_value(0).unwrap_or_default() != 0);
        });
    }
}
