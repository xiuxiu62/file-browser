use std::fs::FileType;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unrecognized file type: {0:?}")]
    UnrecognizedFileType(FileType),
    #[error("file not found: {0}")]
    FileNotFound(String),
    #[error(transparent)]
    LibIgnore(#[from] ignore::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Unknown(#[from] Box<dyn std::error::Error>),
}
