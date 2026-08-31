//! Module component::built_in_component::xcomponent wrappers and related types.

use crate::{
    ArkUIGesture, ArkUIResult, Gesture, GestureDirection, GestureEventAction, GestureEventData,
};

#[cfg(feature = "xcomponent")]
use crate::{
    ArkUIAttributeBasic, ArkUINodeAttributeItem, ArkUINodeAttributeType, ARK_UI_NATIVE_NODE_API_1,
};

#[cfg(feature = "xcomponent")]
use ohos_xcomponent_binding::NativeXComponent as XC;
#[cfg(feature = "xcomponent")]
use ohos_xcomponent_sys::OH_NativeXComponent_GetNativeXComponent;

impl Clone for super::XComponent {
    fn clone(&self) -> Self {
        Self(self.0.borrowed_clone())
    }
}

impl super::XComponent {
    #[cfg(feature = "xcomponent")]
    pub fn native_xcomponent(&self) -> XC {
        use ohos_xcomponent_binding::XComponentRaw;

        let handle = unsafe { OH_NativeXComponent_GetNativeXComponent(self.raw().raw()) };
        let id = ARK_UI_NATIVE_NODE_API_1
            .with(|api| api.get_attribute(self.raw(), ArkUINodeAttributeType::XComponentId))
            .ok()
            .and_then(|attr| {
                if let ArkUINodeAttributeItem::String(xcomponent_id) = attr {
                    Some(xcomponent_id)
                } else {
                    None
                }
            });
        match id {
            Some(id) => XC::with_id(XComponentRaw(handle), id),
            None => XC::new(XComponentRaw(handle)),
        }
    }

    /// Create and attach a one- or multi-tap recognizer to this XComponent.
    ///
    /// The returned handle can later be passed to [`ArkUIGesture::remove_gesture`]
    /// and explicitly disposed with [`Gesture::dispose`]. The gesture is added
    /// in parallel so native XComponent touch delivery remains available.
    pub fn on_tap_gesture<T: Fn(GestureEventData) + 'static>(
        &self,
        finger: i32,
        count: i32,
        callback: T,
    ) -> ArkUIResult<Gesture> {
        let gesture = Gesture::create_tap_gesture(finger, count)?;
        self.bind_gesture(gesture, GestureEventAction::Accept, callback)
    }

    /// Create and attach a pan recognizer to this XComponent.
    ///
    /// Use pan for continuous press-and-drag interaction. The callback receives
    /// accept, update, end, and cancel phases.
    pub fn on_pan_gesture<T: Fn(GestureEventData) + 'static>(
        &self,
        finger: i32,
        direction: GestureDirection,
        distance: f64,
        callback: T,
    ) -> ArkUIResult<Gesture> {
        let gesture = Gesture::create_pan_gesture(finger, direction, distance)?;
        self.bind_gesture(
            gesture,
            GestureEventAction::Accept
                | GestureEventAction::Update
                | GestureEventAction::End
                | GestureEventAction::Cancel,
            callback,
        )
    }

    /// Create and attach a fast-swipe recognizer to this XComponent.
    pub fn on_swipe_gesture<T: Fn(GestureEventData) + 'static>(
        &self,
        finger: i32,
        direction: GestureDirection,
        speed: f64,
        callback: T,
    ) -> ArkUIResult<Gesture> {
        let gesture = Gesture::create_swipe_gesture(finger, direction, speed)?;
        self.bind_gesture(
            gesture,
            GestureEventAction::Accept
                | GestureEventAction::Update
                | GestureEventAction::End
                | GestureEventAction::Cancel,
            callback,
        )
    }

    fn bind_gesture<T: Fn(GestureEventData) + 'static>(
        &self,
        gesture: Gesture,
        actions: GestureEventAction,
        callback: T,
    ) -> ArkUIResult<Gesture> {
        if let Err(error) = gesture.on_gesture(actions, callback) {
            let _ = gesture.dispose();
            return Err(error);
        }
        if let Err(error) = self.add_gesture_ref(&gesture, None, None) {
            let _ = gesture.dispose();
            return Err(error);
        }
        Ok(gesture)
    }
}

// BEGIN_GENERATED_COMPONENT_METHODS_XComponent
impl super::XComponent {
    pub fn set_x_component_id<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::XComponentId,
            value.into(),
        )
    }

    pub fn get_x_component_id(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::XComponentId,
        )
    }

    pub fn set_x_component_type<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::XComponentType,
            value.into(),
        )
    }

    pub fn get_x_component_type(&self) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::XComponentType,
        )
    }

    pub fn set_x_component_surface_size<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::XComponentSurfaceSize,
            value.into(),
        )
    }

    pub fn get_x_component_surface_size(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::XComponentSurfaceSize,
        )
    }

    #[cfg(feature = "api-18")]
    pub fn set_x_component_enable_analyzer<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::XComponentEnableAnalyzer,
            value.into(),
        )
    }

    #[cfg(feature = "api-18")]
    pub fn get_x_component_enable_analyzer(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::XComponentEnableAnalyzer,
        )
    }

    #[cfg(feature = "api-18")]
    pub fn set_x_component_surface_rect<T: Into<crate::ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> crate::ArkUIResult<()> {
        <Self as crate::ArkUICommonAttribute>::set_attribute(
            self,
            crate::ArkUINodeAttributeType::XComponentSurfaceRect,
            value.into(),
        )
    }

    #[cfg(feature = "api-18")]
    pub fn get_x_component_surface_rect(
        &self,
    ) -> crate::ArkUIResult<crate::ArkUINodeAttributeItem> {
        <Self as crate::ArkUICommonAttribute>::get_attribute(
            self,
            crate::ArkUINodeAttributeType::XComponentSurfaceRect,
        )
    }
}
// END_GENERATED_COMPONENT_METHODS_XComponent
