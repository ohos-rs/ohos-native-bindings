//! Module animate::animator::event wrappers and related types.

/// Marker payload for animator lifecycle callbacks.
#[derive(Clone, Copy, Debug, Default)]
pub struct AnimatorEvent;

impl AnimatorEvent {
    pub(crate) fn new() -> Self {
        Self
    }
}

/// Frame callback payload carrying interpolated value.
#[derive(Clone, Copy, Debug)]
pub struct AnimatorFrameEvent {
    value: f32,
}

impl AnimatorFrameEvent {
    pub(crate) fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn value(&self) -> f32 {
        self.value
    }
}
