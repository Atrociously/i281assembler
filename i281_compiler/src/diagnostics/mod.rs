mod error;
mod failure;
mod warning;

pub use error::Error;
pub use failure::Failure;
pub use warning::Warning;

#[derive(Clone, Debug, miette::Diagnostic, thiserror::Error)]
pub enum Diagnostic {
    #[error("ERROR occurred while analyzing the assembly")]
    Error(#[from] Error),
    #[error("WARNING something *could* be wrong with the assembly")]
    Warning(#[from] Warning),
}

pub type Result<T> = core::result::Result<T, Failure>;
