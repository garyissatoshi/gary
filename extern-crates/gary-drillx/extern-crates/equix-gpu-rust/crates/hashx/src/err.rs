//! Error types for the `hashx` crate
use thiserror_no_std::Error;

/// Errors that could occur while building a hash function
#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// A whole-program constraint in HashX failed, and this particular
    /// seed should be considered unusable and skipped.
    #[error("HashX program can't be constructed for this specific seed")]
    ProgramConstraints,
}