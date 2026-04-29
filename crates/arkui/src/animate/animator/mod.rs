//! Animator abstractions and options.

#[allow(clippy::module_inception)]
mod animator;
mod event;
mod native;
mod option;

pub use animator::{Animator, AnimatorController};
pub use event::{AnimatorEvent, AnimatorFrameEvent};
pub use option::AnimatorOption;
