//! Safe Rust bindings for OpenHarmony OHAudio.
//!
//! Provides owned RAII wrappers for low-latency PCM audio rendering
//! ([`AudioStreamBuilder`], [`AudioRenderer`]) and application audio focus
//! sessions ([`AudioSessionManager`]).

pub use ohos_audio_base_sys::{OH_AudioChannelLayout, OH_AudioChannelSet};
pub use ohos_audio_sys as sys;

mod error;
mod renderer;
mod session;
mod types;

pub use error::{AudioError, Result};
pub use renderer::{AudioRenderer, AudioStreamBuilder};
#[cfg(feature = "api-20")]
pub use session::AudioSessionStateSubscription;
pub use session::{AudioSessionDeactivatedSubscription, AudioSessionManager};
#[cfg(feature = "api-23")]
pub use types::LatencyType;
#[cfg(feature = "api-19")]
pub use types::VolumeMode;
pub use types::{
    AudioDataCallbackResult, AudioEffectMode, AudioSessionConcurrencyMode,
    AudioSessionDeactivatedReason, DeviceChangeReason, EncodingType, InterruptForceType,
    InterruptHint, InterruptMode, LatencyMode, PrivacyType, SampleFormat, StreamState, StreamUsage,
};
#[cfg(feature = "api-20")]
pub use types::{AudioSessionScene, AudioSessionStateChangeHint};
