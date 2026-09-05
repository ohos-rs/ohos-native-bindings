//! Safe owners and typed events for OpenHarmony AVPlayer.
//!
//! The wrapper keeps callback context, media sources, and native windows alive
//! for exactly as long as AVPlayer can access them. All player mutations are
//! intentionally `&mut self`: callers should serialize them on one worker
//! thread and forward [`AvPlayerEvent`] values to their UI runtime.

mod error;
mod model;
mod player;

pub use error::{AvPlayerError, AvPlayerErrorKind, AvPlayerResult};
pub use model::{
    AvPlayerBuffering, AvPlayerEvent, AvPlayerSeekMode, AvPlayerState, AvPlayerTrack,
    AvPlayerTrackType, VideoSize,
};
pub use player::AvPlayer;

pub use ohos_native_window_binding::{NativeWindow, NativeWindowScalingMode};
