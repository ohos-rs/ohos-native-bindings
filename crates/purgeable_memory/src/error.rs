use std::fmt;

/// Result alias for purgeable memory operations.
pub type Result<T> = std::result::Result<T, PurgeableMemoryError>;

/// An error returned by a purgeable memory operation.
///
/// The native API reports failures as `false` / `NULL` rather than through an
/// error code, so each variant records which call failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum PurgeableMemoryError {
    /// `OH_PurgeableMemory_Create` returned `NULL`; the object was not created.
    Create,
    /// A begin-access call reported that the content is purged and could not be
    /// rebuilt, so no access permit was granted.
    ContentPurged,
    /// `OH_PurgeableMemory_AppendModify` reported failure; the modification was
    /// not appended.
    AppendModify,
}

impl fmt::Display for PurgeableMemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PurgeableMemoryError::Create => f.write_str("failed to create purgeable memory"),
            PurgeableMemoryError::ContentPurged => {
                f.write_str("content is purged and could not be rebuilt")
            }
            PurgeableMemoryError::AppendModify => {
                f.write_str("failed to append a modification to purgeable memory")
            }
        }
    }
}

impl std::error::Error for PurgeableMemoryError {}
