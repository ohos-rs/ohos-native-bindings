//! Module animate::animator::option wrappers and related types.

use ohos_arkui_input_binding::ArkUIErrorCode;

use crate::animate::curve::CurveHandle;
use crate::animate::options::AnimationFrameRateRange;
use crate::{AnimationDirection, AnimationFillMode, ArkUIError, ArkUIResult};

use super::native::AnimatorOptions;
use super::{AnimatorEvent, AnimatorFrameEvent};

/// Builder-style option wrapper for animator configuration.
pub struct AnimatorOption {
    inner: Option<AnimatorOptions>,
}

impl AnimatorOption {
    pub fn new(keyframe_size: i32) -> ArkUIResult<Self> {
        Ok(Self {
            inner: Some(AnimatorOptions::new(keyframe_size)?),
        })
    }

    pub fn duration(&mut self, value: i32) -> ArkUIResult<&mut Self> {
        self.inner()?.set_duration(value)?;
        Ok(self)
    }

    pub fn delay(&mut self, value: i32) -> ArkUIResult<&mut Self> {
        self.inner()?.set_delay(value)?;
        Ok(self)
    }

    pub fn iterations(&mut self, value: i32) -> ArkUIResult<&mut Self> {
        self.inner()?.set_iterations(value)?;
        Ok(self)
    }

    pub fn fill_mode(&mut self, value: AnimationFillMode) -> ArkUIResult<&mut Self> {
        self.inner()?.set_fill(value)?;
        Ok(self)
    }

    pub fn direction(&mut self, value: AnimationDirection) -> ArkUIResult<&mut Self> {
        self.inner()?.set_direction(value)?;
        Ok(self)
    }

    pub fn curve(&mut self, value: &CurveHandle) -> ArkUIResult<&mut Self> {
        self.inner()?.set_curve(value.as_raw())?;
        Ok(self)
    }

    pub fn begin(&mut self, value: f32) -> ArkUIResult<&mut Self> {
        self.inner()?.set_begin(value)?;
        Ok(self)
    }

    pub fn end(&mut self, value: f32) -> ArkUIResult<&mut Self> {
        self.inner()?.set_end(value)?;
        Ok(self)
    }

    pub fn expected_frame_rate_range(
        &mut self,
        value: AnimationFrameRateRange,
    ) -> ArkUIResult<&mut Self> {
        let mut raw = value.raw();
        self.inner()?.set_expected_frame_rate_range(&mut raw)?;
        Ok(self)
    }

    pub fn keyframe(&mut self, time: f32, value: f32, index: i32) -> ArkUIResult<&mut Self> {
        self.inner()?.set_keyframe(time, value, index)?;
        Ok(self)
    }

    pub fn keyframe_curve(&mut self, value: &CurveHandle, index: i32) -> ArkUIResult<&mut Self> {
        self.inner()?.set_keyframe_curve(value.as_raw(), index)?;
        Ok(self)
    }

    pub fn on_frame<T: Fn(AnimatorFrameEvent) + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<&mut Self> {
        self.inner_mut()?
            .register_on_frame_callback(move |value| callback(AnimatorFrameEvent::new(value)))?;
        Ok(self)
    }

    pub fn clear_on_frame(&mut self) -> ArkUIResult<&mut Self> {
        self.inner_mut()?.clear_on_frame_callback()?;
        Ok(self)
    }

    pub fn on_finish<T: Fn(AnimatorEvent) + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<&mut Self> {
        self.inner_mut()?
            .register_on_finish_callback(move || callback(AnimatorEvent::new()))?;
        Ok(self)
    }

    pub fn clear_on_finish(&mut self) -> ArkUIResult<&mut Self> {
        self.inner_mut()?.clear_on_finish_callback()?;
        Ok(self)
    }

    pub fn on_cancel<T: Fn(AnimatorEvent) + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<&mut Self> {
        self.inner_mut()?
            .register_on_cancel_callback(move || callback(AnimatorEvent::new()))?;
        Ok(self)
    }

    pub fn clear_on_cancel(&mut self) -> ArkUIResult<&mut Self> {
        self.inner_mut()?.clear_on_cancel_callback()?;
        Ok(self)
    }

    pub fn on_repeat<T: Fn(AnimatorEvent) + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<&mut Self> {
        self.inner_mut()?
            .register_on_repeat_callback(move || callback(AnimatorEvent::new()))?;
        Ok(self)
    }

    pub fn clear_on_repeat(&mut self) -> ArkUIResult<&mut Self> {
        self.inner_mut()?.clear_on_repeat_callback()?;
        Ok(self)
    }

    pub fn get_duration(&self) -> ArkUIResult<i32> {
        Ok(self.inner()?.get_duration())
    }

    pub fn get_delay(&self) -> ArkUIResult<i32> {
        Ok(self.inner()?.get_delay())
    }

    pub fn get_iterations(&self) -> ArkUIResult<i32> {
        Ok(self.inner()?.get_iterations())
    }

    pub fn get_fill_mode(&self) -> ArkUIResult<AnimationFillMode> {
        Ok(self.inner()?.get_fill())
    }

    pub fn get_direction(&self) -> ArkUIResult<AnimationDirection> {
        Ok(self.inner()?.get_direction())
    }

    pub fn get_curve(&self) -> ArkUIResult<Option<CurveHandle>> {
        Ok(CurveHandle::from_raw_borrowed(self.inner()?.get_curve()))
    }

    pub fn get_begin(&self) -> ArkUIResult<f32> {
        Ok(self.inner()?.get_begin())
    }

    pub fn get_end(&self) -> ArkUIResult<f32> {
        Ok(self.inner()?.get_end())
    }

    pub fn get_expected_frame_rate_range(&self) -> ArkUIResult<Option<AnimationFrameRateRange>> {
        Ok(self
            .inner()?
            .get_expected_frame_rate_range()
            .map(AnimationFrameRateRange::from_raw))
    }

    pub fn get_keyframe_time(&self, index: i32) -> ArkUIResult<f32> {
        Ok(self.inner()?.get_keyframe_time(index))
    }

    pub fn get_keyframe_value(&self, index: i32) -> ArkUIResult<f32> {
        Ok(self.inner()?.get_keyframe_value(index))
    }

    pub fn get_keyframe_curve(&self, index: i32) -> ArkUIResult<Option<CurveHandle>> {
        Ok(CurveHandle::from_raw_borrowed(
            self.inner()?.get_keyframe_curve(index),
        ))
    }

    pub fn dispose(mut self) {
        if let Some(inner) = self.inner.take() {
            inner.dispose();
        }
    }

    pub(crate) fn inner(&self) -> ArkUIResult<&AnimatorOptions> {
        self.inner.as_ref().ok_or_else(disposed_error)
    }

    fn inner_mut(&mut self) -> ArkUIResult<&mut AnimatorOptions> {
        self.inner.as_mut().ok_or_else(disposed_error)
    }
}

impl Drop for AnimatorOption {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            inner.dispose();
        }
    }
}

fn disposed_error() -> ArkUIError {
    ArkUIError::new(
        ArkUIErrorCode::ParamInvalid,
        "AnimatorOption has been disposed",
    )
}
