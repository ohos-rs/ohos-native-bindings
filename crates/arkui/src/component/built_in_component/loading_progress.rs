// BEGIN_GENERATED_COMPONENT_METHODS_LoadingProgress
impl super::LoadingProgress {
    pub fn set_loading_progress_color<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::LoadingProgressColor,
            value.into(),
        )
    }

    pub fn get_loading_progress_color(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::LoadingProgressColor,
        )
    }

    pub fn set_loading_progress_enable_loading<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::LoadingProgressEnableLoading,
            value.into(),
        )
    }

    pub fn get_loading_progress_enable_loading(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::LoadingProgressEnableLoading,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_LoadingProgress
