// BEGIN_GENERATED_COMPONENT_METHODS_ImageAnimator
impl super::ImageAnimator {
    pub fn set_image_animator_images<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageAnimatorImages,
            value.into(),
        )
    }

    pub fn get_image_animator_images(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageAnimatorImages,
        )
    }

    pub fn set_image_animator_state<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageAnimatorState,
            value.into(),
        )
    }

    pub fn get_image_animator_state(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageAnimatorState,
        )
    }

    pub fn set_image_animator_duration<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageAnimatorDuration,
            value.into(),
        )
    }

    pub fn get_image_animator_duration(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageAnimatorDuration,
        )
    }

    pub fn set_image_animator_reverse<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageAnimatorReverse,
            value.into(),
        )
    }

    pub fn get_image_animator_reverse(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageAnimatorReverse,
        )
    }

    pub fn set_image_animator_fixed_size<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageAnimatorFixedSize,
            value.into(),
        )
    }

    pub fn get_image_animator_fixed_size(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageAnimatorFixedSize,
        )
    }

    pub fn set_image_animator_fill_mode<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageAnimatorFillMode,
            value.into(),
        )
    }

    pub fn get_image_animator_fill_mode(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageAnimatorFillMode,
        )
    }

    pub fn set_image_animator_iteration<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageAnimatorIteration,
            value.into(),
        )
    }

    pub fn get_image_animator_iteration(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::ImageAnimatorIteration,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_ImageAnimator

impl super::ImageAnimator {
    pub fn on_image_animator_start<T: Fn() + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event_no_param(
            self,
            crate::NodeEventType::ImageAnimatorEventOnStart,
            cb,
        );
    }

    pub fn on_image_animator_pause<T: Fn() + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event_no_param(
            self,
            crate::NodeEventType::ImageAnimatorEventOnPause,
            cb,
        );
    }

    pub fn on_image_animator_repeat<T: Fn() + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event_no_param(
            self,
            crate::NodeEventType::ImageAnimatorEventOnRepeat,
            cb,
        );
    }

    pub fn on_image_animator_cancel<T: Fn() + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event_no_param(
            self,
            crate::NodeEventType::ImageAnimatorEventOnCancel,
            cb,
        );
    }

    pub fn on_image_animator_finish<T: Fn() + 'static>(&mut self, cb: T) {
        crate::ArkUIEvent::on_event_no_param(
            self,
            crate::NodeEventType::ImageAnimatorEventOnFinish,
            cb,
        );
    }
}
