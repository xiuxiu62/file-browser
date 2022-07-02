use crate::{
    error::{Error, Result},
    AsEntry, Directory, Entry,
};
use std::{cell::RefCell, fmt::Display, fs, path::PathBuf, rc::Rc};

#[derive(Debug, Clone)]
pub struct SymLink {
    relative_path: PathBuf,
    full_path: PathBuf,
    link: Option<Rc<Entry>>,
}

impl SymLink {
    pub fn link(&mut self) -> Result<&Rc<Entry>> {
        if self.link.is_none() {
            self.populate()?;
        }

        Ok(self.link.as_ref().unwrap())
    }
}

impl AsEntry for SymLink {
    fn relative_path(&self) -> &PathBuf {
        &self.relative_path
    }

    fn full_path(&self) -> &PathBuf {
        &self.full_path
    }

    fn populate(&mut self) -> Result<()> {
        let link = fs::read_link(self.full_path())?;
        self.link = Some(Rc::new(Entry::try_from(link)?));

        Ok(())
    }

    fn parent(&self) -> Result<Option<RefCell<Directory>>> {
        Ok(match self.full_path().parent() {
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
            link: None,
        })
    }
}

impl Display for SymLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.relative_path().display())
    }
}
