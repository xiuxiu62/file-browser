use crate::{
    error::{Error, Result},
    AsEntry, Directory,
};
use std::{cell::RefCell, fmt::Display, fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct File {
    relative_path: PathBuf,
    full_path: PathBuf,
    content: Option<Vec<u8>>,
}

impl File {
    pub fn content(&mut self) -> Result<&Vec<u8>> {
        if self.content.is_none() {
            self.populate()?;
        }

        Ok(self.content.as_ref().unwrap())
    }
}

impl AsEntry for File {
    fn relative_path(&self) -> &PathBuf {
        &self.relative_path
    }

    fn full_path(&self) -> &PathBuf {
        &self.full_path
    }

    fn populate(&mut self) -> Result<()> {
        self.content = Some(fs::read(self.full_path())?);

        Ok(())
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
            content: None,
        })
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.relative_path().display())
    }
}
