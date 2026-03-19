//! Module component::built_in_component::slider wrappers and related types.

// BEGIN_GENERATED_COMPONENT_METHODS_Slider
impl super::Slider {
    pub fn set_slider_block_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderBlockColor,
            value.into(),
        )
    }

    pub fn get_slider_block_color(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderBlockColor,
        )
    }

    pub fn set_slider_track_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderTrackColor,
            value.into(),
        )
    }

    pub fn get_slider_track_color(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderTrackColor,
        )
    }

    pub fn set_slider_selected_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderSelectedColor,
            value.into(),
        )
    }

    pub fn get_slider_selected_color(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderSelectedColor,
        )
    }

    pub fn set_slider_show_steps<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderShowSteps,
            value.into(),
        )
    }

    pub fn get_slider_show_steps(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderShowSteps,
        )
    }

    pub fn set_slider_block_style<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderBlockStyle,
            value.into(),
        )
    }

    pub fn get_slider_block_style(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderBlockStyle,
        )
    }

    pub fn set_slider_value<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderValue,
            value.into(),
        )
    }

    pub fn get_slider_value(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderValue,
        )
    }

    pub fn set_slider_min_value<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderMinValue,
            value.into(),
        )
    }

    pub fn get_slider_min_value(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderMinValue,
        )
    }

    pub fn set_slider_max_value<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderMaxValue,
            value.into(),
        )
    }

    pub fn get_slider_max_value(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderMaxValue,
        )
    }

    pub fn set_slider_step<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderStep,
            value.into(),
        )
    }

    pub fn get_slider_step(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderStep,
        )
    }

    pub fn set_slider_direction<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderDirection,
            value.into(),
        )
    }

    pub fn get_slider_direction(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderDirection,
        )
    }

    pub fn set_slider_reverse<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderReverse,
            value.into(),
        )
    }

    pub fn get_slider_reverse(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderReverse,
        )
    }

    pub fn set_slider_style<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderStyle,
            value.into(),
        )
    }

    pub fn get_slider_style(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderStyle,
        )
    }

    pub fn set_slider_track_thickness<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderTrackThickness,
            value.into(),
        )
    }

    pub fn get_slider_track_thickness(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderTrackThickness,
        )
    }

    #[cfg(feature = "api-21")]
    pub fn set_slider_block_linear_gradient_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderBlockLinearGradientColor,
            value.into(),
        )
    }

    #[cfg(feature = "api-21")]
    pub fn get_slider_block_linear_gradient_color(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderBlockLinearGradientColor,
        )
    }

    #[cfg(feature = "api-18")]
    pub fn set_slider_enable_haptic_feedback<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderEnableHapticFeedback,
            value.into(),
        )
    }

    #[cfg(feature = "api-18")]
    pub fn get_slider_enable_haptic_feedback(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderEnableHapticFeedback,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_slider_prefix<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderPrefix,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_slider_prefix(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderPrefix,
        )
    }

    #[cfg(feature = "api-21")]
    pub fn set_slider_selected_linear_gradient_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderSelectedLinearGradientColor,
            value.into(),
        )
    }

    #[cfg(feature = "api-21")]
    pub fn get_slider_selected_linear_gradient_color(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderSelectedLinearGradientColor,
        )
    }

    #[cfg(feature = "api-20")]
    pub fn set_slider_suffix<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderSuffix,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    pub fn get_slider_suffix(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderSuffix,
        )
    }

    #[cfg(feature = "api-21")]
    pub fn set_slider_track_linear_gradient_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderTrackLinearGradientColor,
            value.into(),
        )
    }

    #[cfg(feature = "api-21")]
    pub fn get_slider_track_linear_gradient_color(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::SliderTrackLinearGradientColor,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_Slider

#[derive(Clone, Copy, Debug, PartialEq)]
/// Event payload emitted when slider value changes.
pub struct SliderChangeEvent {
    pub value: f32,
    pub mode: i32,
}

impl super::Slider {
    pub fn on_slider_change<T: Fn(SliderChangeEvent) + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event(
            self,
            crate::NodeEventType::SliderEventOnChange,
            move |event| {
                cb(SliderChangeEvent {
                    value: event.f32_value(0).unwrap_or_default(),
                    mode: event.i32_value(1).unwrap_or_default(),
                });
            },
        );
    }
}
