//! Transition effects and transition option wrappers.

mod effect;
mod options;

#[cfg(feature = "api-21")]
pub use effect::ContentTransitionEffect;
pub use effect::TransitionEffect;
pub use options::{RotationOptions, ScaleOptions, TranslationOptions};
