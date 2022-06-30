use crate::{
    error::{Error, Result},
    Directory, File, SymLink,
};
use std::{fs, path::PathBuf};

pub trait AsEntry {
    fn path(&self) -> &PathBuf;

    fn canonicalize(&self) -> Result<PathBuf> {
        let path = fs::canonicalize(self.path())?;

        Ok(path)
    }

    fn populate(&mut self) -> Result<()>;
}

#[derive(Debug, Clone)]
pub enum Entry {
    Directory(Directory),
    File(File),
    SymLink(SymLink),
}

impl AsEntry for Entry {
    fn path(&self) -> &PathBuf {
        match self {
            Self::Directory(directory) => directory.path(),
            Self::File(file) => file.path(),
            Self::SymLink(symlink) => symlink.path(),
        }
    }

    fn populate(&mut self) -> Result<()> {
        match self {
            Self::Directory(directory) => directory.populate(),
            Self::File(file) => file.populate(),
            Self::SymLink(symlink) => symlink.populate(),
        }
    }
}

impl TryFrom<&str> for Entry {
    type Error = Error;

    fn try_from(path: &str) -> Result<Self> {
        Self::try_from(PathBuf::from(path))
    }
}

impl TryFrom<PathBuf> for Entry {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self> {
        let file_type = std::fs::metadata(path.clone())?.file_type();
        if file_type.is_dir() {
            return Ok(Self::Directory(Directory::new(path)));
        }

        if file_type.is_file() {
            return Ok(Self::File(File::new(path)));
        }

        if file_type.is_symlink() {
            return Ok(Self::SymLink(SymLink::new(path)));
        }

        Err(Error::UnrecognizedFileType(file_type))
    }
}
