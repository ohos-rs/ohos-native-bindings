use std::fmt;

/// Result alias for native fence operations.
pub type Result<T> = std::result::Result<T, FenceError>;

/// An error returned by a native fence operation.
///
/// The native API reports failures as a plain `false`, without an error code,
/// so the variants below distinguish the cases this crate can tell apart on its
/// own from the single opaque failure the C layer surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FenceError {
    /// The file descriptor is negative, which the native API treats as "no
    /// fence".
    InvalidFd,
    /// The fence did not signal.
    ///
    /// The native API folds several causes into one `false` return: the wait
    /// timed out, the underlying `poll` call failed, or the file descriptor
    /// could not be duplicated. They are not distinguishable from the caller
    /// side.
    NotSignaled,
    /// The requested timeout is less than one millisecond.
    ///
    /// The native API takes a whole number of milliseconds and documents a
    /// timeout of `0` as a failure case rather than a poll-and-return, so such
    /// a wait is rejected instead of being forwarded.
    ZeroTimeout,
    /// The requested timeout does not fit in the 32-bit millisecond field of
    /// the native API. Use an unbounded wait instead.
    TimeoutTooLong,
}

impl fmt::Display for FenceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FenceError::InvalidFd => write!(f, "invalid fence file descriptor"),
            FenceError::NotSignaled => {
                write!(f, "fence did not signal (timed out or the wait failed)")
            }
            FenceError::ZeroTimeout => write!(f, "timeout is shorter than one millisecond"),
            FenceError::TimeoutTooLong => {
                write!(f, "timeout exceeds the supported millisecond range")
            }
        }
    }
}

impl std::error::Error for FenceError {}
