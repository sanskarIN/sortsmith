use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, SortSmithError>;

#[derive(Debug, Error)]
pub enum SortSmithError {
    #[error("I/O error at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("invalid rule: {0}")]
    InvalidRule(String),
    #[error("unsafe destination path: {0}")]
    UnsafeDestination(PathBuf),
    #[error("journal error: {0}")]
    Journal(String),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub(crate) fn io(path: impl Into<PathBuf>, source: std::io::Error) -> SortSmithError {
    SortSmithError::Io { path: path.into(), source }
}
