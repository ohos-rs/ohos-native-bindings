use std::ptr::NonNull;

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::{
    ArkUI_TransitionEffect, OH_ArkUI_CreateAsymmetricTransitionEffect,
    OH_ArkUI_CreateMovementTransitionEffect, OH_ArkUI_CreateOpacityTransitionEffect,
    OH_ArkUI_CreateRotationTransitionEffect, OH_ArkUI_CreateScaleTransitionEffect,
    OH_ArkUI_CreateTranslationTransitionEffect, OH_ArkUI_TransitionEffect_Combine,
    OH_ArkUI_TransitionEffect_Dispose, OH_ArkUI_TransitionEffect_SetAnimation,
};

#[cfg(feature = "api-21")]
use ohos_arkui_sys::{ArkUI_ContentTransitionEffect, OH_ArkUI_ContentTransitionEffect_Create};

use crate::{check_arkui_status, Animation, ArkUIError, ArkUIResult, TransitionEdge};

use super::{RotationOptions, ScaleOptions, TranslationOptions};

pub struct TransitionEffect {
    raw: NonNull<ArkUI_TransitionEffect>,
}

impl TransitionEffect {
    pub fn opacity(opacity: f32) -> ArkUIResult<Self> {
        create_opacity_transition_effect(opacity).map(Self::from_non_null)
    }

    pub fn translation(mut options: TranslationOptions) -> ArkUIResult<Self> {
        create_translation_transition_effect(options.raw_mut()).map(Self::from_non_null)
    }

    pub fn scale(mut options: ScaleOptions) -> ArkUIResult<Self> {
        create_scale_transition_effect(options.raw_mut()).map(Self::from_non_null)
    }

    pub fn rotation(mut options: RotationOptions) -> ArkUIResult<Self> {
        create_rotation_transition_effect(options.raw_mut()).map(Self::from_non_null)
    }

    pub fn movement(edge: TransitionEdge) -> ArkUIResult<Self> {
        create_movement_transition_effect(edge).map(Self::from_non_null)
    }

    pub fn asymmetric(
        appear: &TransitionEffect,
        disappear: &TransitionEffect,
    ) -> ArkUIResult<Self> {
        create_asymmetric_transition_effect(appear.raw(), disappear.raw()).map(Self::from_non_null)
    }

    pub fn combine(&mut self, effect: &TransitionEffect) -> ArkUIResult<&mut Self> {
        unsafe {
            check_arkui_status!(OH_ArkUI_TransitionEffect_Combine(self.raw(), effect.raw()))?
        };
        Ok(self)
    }

    pub fn set_animation(&mut self, animation: &Animation) -> ArkUIResult<&mut Self> {
        unsafe {
            check_arkui_status!(OH_ArkUI_TransitionEffect_SetAnimation(
                self.raw(),
                animation.raw()
            ))?
        };
        Ok(self)
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_TransitionEffect_Dispose(self.raw()) }
    }

    pub(crate) fn from_non_null(raw: NonNull<ArkUI_TransitionEffect>) -> Self {
        Self { raw }
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_TransitionEffect {
        self.raw.as_ptr()
    }
}

#[cfg(feature = "api-21")]
pub struct ContentTransitionEffect {
    raw: NonNull<ArkUI_ContentTransitionEffect>,
}

#[cfg(feature = "api-21")]
impl ContentTransitionEffect {
    pub fn new(type_: i32) -> ArkUIResult<Self> {
        content_transition_effect_create(type_).map(Self::from_non_null)
    }

    pub(crate) fn from_non_null(raw: NonNull<ArkUI_ContentTransitionEffect>) -> Self {
        Self { raw }
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_ContentTransitionEffect {
        self.raw.as_ptr()
    }
}

fn transition_effect_or_error(
    effect: *mut ArkUI_TransitionEffect,
    func: &'static str,
) -> ArkUIResult<NonNull<ArkUI_TransitionEffect>> {
    if let Some(effect) = NonNull::new(effect) {
        Ok(effect)
    } else {
        Err(ArkUIError::new(
            ArkUIErrorCode::ParamInvalid,
            format!("{func} returned null"),
        ))
    }
}

fn create_opacity_transition_effect(opacity: f32) -> ArkUIResult<NonNull<ArkUI_TransitionEffect>> {
    let effect = unsafe { OH_ArkUI_CreateOpacityTransitionEffect(opacity) };
    transition_effect_or_error(effect, "OH_ArkUI_CreateOpacityTransitionEffect")
}

fn create_translation_transition_effect(
    translate: *mut ohos_arkui_sys::ArkUI_TranslationOptions,
) -> ArkUIResult<NonNull<ArkUI_TransitionEffect>> {
    let effect = unsafe { OH_ArkUI_CreateTranslationTransitionEffect(translate) };
    transition_effect_or_error(effect, "OH_ArkUI_CreateTranslationTransitionEffect")
}

fn create_scale_transition_effect(
    scale: *mut ohos_arkui_sys::ArkUI_ScaleOptions,
) -> ArkUIResult<NonNull<ArkUI_TransitionEffect>> {
    let effect = unsafe { OH_ArkUI_CreateScaleTransitionEffect(scale) };
    transition_effect_or_error(effect, "OH_ArkUI_CreateScaleTransitionEffect")
}

fn create_rotation_transition_effect(
    rotate: *mut ohos_arkui_sys::ArkUI_RotationOptions,
) -> ArkUIResult<NonNull<ArkUI_TransitionEffect>> {
    let effect = unsafe { OH_ArkUI_CreateRotationTransitionEffect(rotate) };
    transition_effect_or_error(effect, "OH_ArkUI_CreateRotationTransitionEffect")
}

fn create_movement_transition_effect(
    edge: TransitionEdge,
) -> ArkUIResult<NonNull<ArkUI_TransitionEffect>> {
    let effect = unsafe { OH_ArkUI_CreateMovementTransitionEffect(edge.into()) };
    transition_effect_or_error(effect, "OH_ArkUI_CreateMovementTransitionEffect")
}

fn create_asymmetric_transition_effect(
    appear: *mut ArkUI_TransitionEffect,
    disappear: *mut ArkUI_TransitionEffect,
) -> ArkUIResult<NonNull<ArkUI_TransitionEffect>> {
    let effect = unsafe { OH_ArkUI_CreateAsymmetricTransitionEffect(appear, disappear) };
    transition_effect_or_error(effect, "OH_ArkUI_CreateAsymmetricTransitionEffect")
}

#[cfg(feature = "api-21")]
fn content_transition_effect_create(
    type_: i32,
) -> ArkUIResult<NonNull<ArkUI_ContentTransitionEffect>> {
    let effect = unsafe { OH_ArkUI_ContentTransitionEffect_Create(type_) };
    if let Some(effect) = NonNull::new(effect) {
        Ok(effect)
    } else {
        Err(ArkUIError::new(
            ArkUIErrorCode::ParamInvalid,
            "OH_ArkUI_ContentTransitionEffect_Create returned null",
        ))
    }
}
