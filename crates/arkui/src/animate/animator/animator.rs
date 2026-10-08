//! Module animate::animator::animator wrappers and related types.

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::OH_ArkUI_GetContextByNode;

use crate::{ArkUIContext, ArkUIError, ArkUINode, ArkUIResult};

use super::{native, AnimatorOption};

/// Runtime animator instance backed by native ArkUI animator.
pub struct Animator {
    handle: native::AnimatorHandle,
}

impl Animator {
    /// Create an animator from context and options.
    pub fn create(ctx: ArkUIContext, option: &AnimatorOption) -> ArkUIResult<Self> {
        let handle = native::AnimatorHandle::create(ctx.raw(), option.inner()?)?;
        Ok(Self { handle })
    }

    pub fn play(&self) -> ArkUIResult<()> {
        self.handle.play()
    }

    pub fn finish(&self) -> ArkUIResult<()> {
        self.handle.finish()
    }

    pub fn pause(&self) -> ArkUIResult<()> {
        self.handle.pause()
    }

    pub fn cancel(&self) -> ArkUIResult<()> {
        self.handle.cancel()
    }

    pub fn reverse(&self) -> ArkUIResult<()> {
        self.handle.reverse()
    }

    pub fn reset_option(&self, option: &AnimatorOption) -> ArkUIResult<()> {
        self.handle.reset_option(option.inner()?)
    }
}

impl Drop for Animator {
    fn drop(&mut self) {
        let _ = self.handle.dispose();
    }
}

/// Convenience controller that owns both animator and mutable options.
pub struct AnimatorController {
    animator: Animator,
    // Keep the callback-owning option after the animator so Rust drops the
    // native animator before unregistering and releasing its callbacks.
    option: AnimatorOption,
}

impl AnimatorController {
    pub fn new(ctx: ArkUIContext, keyframe_size: i32) -> ArkUIResult<Self> {
        let option = AnimatorOption::new(keyframe_size)?;
        let animator = Animator::create(ctx, &option)?;
        Ok(Self { option, animator })
    }

    /// Creates an animator from a live native node without requiring a N-API
    /// `UIContext` value at the call site.
    pub fn from_node(node: &ArkUINode, keyframe_size: i32) -> ArkUIResult<Self> {
        let context = unsafe { OH_ArkUI_GetContextByNode(node.raw_handle()) };
        if context.is_null() {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_GetContextByNode returned null",
            ));
        }
        let option = AnimatorOption::new(keyframe_size)?;
        let animator = Animator {
            handle: native::AnimatorHandle::create(context, option.inner()?)?,
        };
        Ok(Self { option, animator })
    }

    pub fn option(&self) -> &AnimatorOption {
        &self.option
    }

    pub fn option_mut(&mut self) -> &mut AnimatorOption {
        &mut self.option
    }

    pub fn commit_option(&self) -> ArkUIResult<()> {
        self.animator.reset_option(&self.option)
    }

    pub fn play(&self) -> ArkUIResult<()> {
        self.animator.play()
    }

    pub fn finish(&self) -> ArkUIResult<()> {
        self.animator.finish()
    }

    pub fn pause(&self) -> ArkUIResult<()> {
        self.animator.pause()
    }

    pub fn cancel(&self) -> ArkUIResult<()> {
        self.animator.cancel()
    }

    pub fn reverse(&self) -> ArkUIResult<()> {
        self.animator.reverse()
    }
}
