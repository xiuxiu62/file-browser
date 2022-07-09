use std::fs::FileType;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("incorrect file type: {path}\nexpected: {expected}, found: {found}")]
    IncorrectFileType {
        path: String,
        expected: String,
        found: String,
    },
    #[error("unrecognized file type: {0:?}")]
    UnrecognizedFileType(FileType),
    #[error("file not found: {0}")]
    FileNotFound(String),
    #[error(transparent)]
    Ignore(#[from] ignore::Error),
    #[error(transparent)]
    Format(#[from] std::fmt::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Unknown(#[from] Box<dyn std::error::Error>),
}
