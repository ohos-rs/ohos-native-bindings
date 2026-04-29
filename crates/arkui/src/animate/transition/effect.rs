//! Module animate::transition::effect wrappers and related types.

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

/// Transition effect wrapper for enter/exit animations.
pub struct TransitionEffect {
    raw: NonNull<ArkUI_TransitionEffect>,
}

impl TransitionEffect {
    pub fn opacity(opacity: f32) -> ArkUIResult<Self> {
        let effect = unsafe { OH_ArkUI_CreateOpacityTransitionEffect(opacity) };
        let effect = NonNull::new(effect).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_CreateOpacityTransitionEffect returned null",
            )
        })?;
        Ok(Self::from_non_null(effect))
    }

    pub fn translation(mut options: TranslationOptions) -> ArkUIResult<Self> {
        let effect = unsafe { OH_ArkUI_CreateTranslationTransitionEffect(options.raw_mut()) };
        let effect = NonNull::new(effect).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_CreateTranslationTransitionEffect returned null",
            )
        })?;
        Ok(Self::from_non_null(effect))
    }

    pub fn scale(mut options: ScaleOptions) -> ArkUIResult<Self> {
        let effect = unsafe { OH_ArkUI_CreateScaleTransitionEffect(options.raw_mut()) };
        let effect = NonNull::new(effect).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_CreateScaleTransitionEffect returned null",
            )
        })?;
        Ok(Self::from_non_null(effect))
    }

    pub fn rotation(mut options: RotationOptions) -> ArkUIResult<Self> {
        let effect = unsafe { OH_ArkUI_CreateRotationTransitionEffect(options.raw_mut()) };
        let effect = NonNull::new(effect).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_CreateRotationTransitionEffect returned null",
            )
        })?;
        Ok(Self::from_non_null(effect))
    }

    pub fn movement(edge: TransitionEdge) -> ArkUIResult<Self> {
        let effect = unsafe { OH_ArkUI_CreateMovementTransitionEffect(edge.into()) };
        let effect = NonNull::new(effect).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_CreateMovementTransitionEffect returned null",
            )
        })?;
        Ok(Self::from_non_null(effect))
    }

    pub fn asymmetric(
        appear: &TransitionEffect,
        disappear: &TransitionEffect,
    ) -> ArkUIResult<Self> {
        let effect =
            unsafe { OH_ArkUI_CreateAsymmetricTransitionEffect(appear.raw(), disappear.raw()) };
        let effect = NonNull::new(effect).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_CreateAsymmetricTransitionEffect returned null",
            )
        })?;
        Ok(Self::from_non_null(effect))
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
/// Content transition effect wrapper available on `api-21+`.
pub struct ContentTransitionEffect {
    raw: NonNull<ArkUI_ContentTransitionEffect>,
}

#[cfg(feature = "api-21")]
impl ContentTransitionEffect {
    pub fn new(type_: i32) -> ArkUIResult<Self> {
        let effect = unsafe { OH_ArkUI_ContentTransitionEffect_Create(type_) };
        let effect = NonNull::new(effect).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_ContentTransitionEffect_Create returned null",
            )
        })?;
        Ok(Self::from_non_null(effect))
    }

    pub(crate) fn from_non_null(raw: NonNull<ArkUI_ContentTransitionEffect>) -> Self {
        Self { raw }
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_ContentTransitionEffect {
        self.raw.as_ptr()
    }
}
