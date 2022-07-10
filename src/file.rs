use crate::{
    error::{Error, Result},
    AsEntry, Directory,
};
use std::{
    cell::RefCell,
    fmt::{self, Display},
    fs,
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub struct File {
    relative_path: PathBuf,
    full_path: PathBuf,
}

impl File {
    pub fn content(&self) -> Result<Vec<u8>> {
        Ok(fs::read(self.full_path())?)
    }
}

impl AsEntry for File {
    fn relative_path(&self) -> &PathBuf {
        &self.relative_path
    }

    fn full_path(&self) -> &PathBuf {
        &self.full_path
    }

    fn parent(&self) -> Result<Option<RefCell<Directory>>> {
        Ok(match self.full_path().parent() {
            Some(parent) => Some(RefCell::new(Directory::try_from(parent.to_path_buf())?)),
            None => None,
        })
    }
}

impl TryFrom<&str> for File {
    type Error = Error;

    fn try_from(path: &str) -> Result<Self> {
        Self::try_from(PathBuf::from(path))
    }
}

impl TryFrom<PathBuf> for File {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self> {
        let full_path = fs::canonicalize(&path)?;

        Ok(Self {
            relative_path: path,
            full_path,
        })
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.relative_path().display())
    }
}
