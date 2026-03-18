use crate::{ArkUIContext, ArkUIResult};

use super::{native, AnimatorOption};

pub struct Animator {
    handle: native::AnimatorHandle,
}

impl Animator {
    pub fn create(ctx: ArkUIContext, option: &AnimatorOption) -> ArkUIResult<Self> {
        let handle = native::create_animator(ctx.raw(), option.inner()?)?;
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

pub struct AnimatorController {
    option: AnimatorOption,
    animator: Animator,
}

impl AnimatorController {
    pub fn new(ctx: ArkUIContext, keyframe_size: i32) -> ArkUIResult<Self> {
        let option = AnimatorOption::new(keyframe_size)?;
        let animator = Animator::create(ctx, &option)?;
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
