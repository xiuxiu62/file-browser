use crate::{error::Result, AsEntry, Directory};
use std::{cell::RefCell, fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct File {
    path: PathBuf,
    content: Option<Vec<u8>>,
}

impl File {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            content: None,
        }
    }

    pub fn content(&mut self) -> Result<&Vec<u8>> {
        if self.content.is_none() {
            self.populate()?;
        }

        Ok(self.content.as_ref().unwrap())
    }
}

impl AsEntry for File {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn populate(&mut self) -> Result<()> {
        self.content = Some(fs::read(self.path())?);

        Ok(())
    }

    fn parent(&self) -> Option<RefCell<Directory>> {
        self.path
            .parent()
            .map(|parent| RefCell::new(Directory::new(parent.to_path_buf())))
    }
}

impl From<&str> for File {
    fn from(path: &str) -> Self {
        Self::new(PathBuf::from(path))
    }
}
