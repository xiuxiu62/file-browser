use crate::{
    error::{Error, Result},
    AsEntry, Directory, Entry,
};
use std::{cell::RefCell, fmt::Display, fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct SymLink {
    relative_path: PathBuf,
    full_path: PathBuf,
}

impl SymLink {
    pub fn link(&self) -> Result<Entry> {
        let link = fs::read_link(self.full_path())?;

        Entry::try_from(link)
    }
}

impl AsEntry for SymLink {
    fn relative_path(&self) -> &PathBuf {
        &self.relative_path
    }

    fn full_path(&self) -> &PathBuf {
        &self.full_path
    }

    fn parent(&self) -> Result<Option<RefCell<Directory>>> {
        Ok(match self.relative_path().parent() {
            Some(parent) => Some(RefCell::new(Directory::try_from(parent.to_path_buf())?)),
            None => None,
        })
    }
}

impl TryFrom<&str> for SymLink {
    type Error = Error;

    fn try_from(path: &str) -> Result<Self> {
        Self::try_from(PathBuf::from(path))
    }
}

impl TryFrom<PathBuf> for SymLink {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self> {
        let full_path = fs::canonicalize(&path)?;

        Ok(Self {
            relative_path: path,
            full_path,
        })
    }
}

impl Display for SymLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.relative_path().to_string_lossy())
    }
}
