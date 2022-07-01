use crate::{error::Result, AsEntry, Directory, Entry};
use std::{cell::RefCell, fs, path::PathBuf, rc::Rc};

#[derive(Debug, Clone)]
pub struct SymLink {
    path: PathBuf,
    link: Option<Rc<crate::Entry>>,
}

impl SymLink {
    pub fn new(path: PathBuf) -> Self {
        Self { path, link: None }
    }
}

impl AsEntry for SymLink {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn populate(&mut self) -> Result<()> {
        let link = fs::read_link(self.path())?;
        self.link = Some(Rc::new(Entry::try_from(link)?));

        Ok(())
    }

    fn parent(&self) -> Option<std::cell::RefCell<crate::Directory>> {
        self.path
            .parent()
            .map(|parent| RefCell::new(Directory::new(parent.to_path_buf())))
    }
}

impl From<&str> for SymLink {
    fn from(path: &str) -> Self {
        Self::new(PathBuf::from(path))
    }
}
