use crate::{
    error::{Error, Result},
    AsEntry, Entry,
};
use ignore::WalkBuilder;
use std::{cell::RefCell, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Directory {
    path: PathBuf,
    entries: Option<Vec<RefCell<Entry>>>,
}

impl Directory {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            entries: None,
        }
    }

    pub fn get_entry(&mut self, path: &str) -> Result<RefCell<Entry>> {
        match &self.entries {
            Some(entries) => match entries
                .iter()
                .filter(|entry| entry.borrow().path() == &PathBuf::from(path))
                .collect::<Vec<&RefCell<Entry>>>()
                .first()
                .map(|entry| entry.to_owned().to_owned())
            {
                Some(entry) => Ok(entry),
                None => Err(Error::FileNotFound(path.to_owned())),
            },
            None => {
                self.populate()?;
                self.get_entry(path)
            }
        }
    }
}

impl AsEntry for Directory {
    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn populate(&mut self) -> Result<()> {
        self.entries = Some(
            WalkBuilder::new(&self.path)
                .max_depth(Some(1))
                .build()
                .skip(1)
                .try_fold(vec![], |mut acc, entry| {
                    acc.push(entry?);

                    Ok::<Vec<ignore::DirEntry>, Error>(acc)
                })?
                .into_iter()
                .map(|entry| Entry::try_from(entry.into_path()))
                .collect::<Result<Vec<Entry>>>()?
                .into_iter()
                .map(|entry| RefCell::new(entry))
                .collect(),
        );

        Ok(())
    }

    fn parent(&self) -> Option<RefCell<Directory>> {
        self.path
            .parent()
            .map(|parent| RefCell::new(Directory::new(parent.to_path_buf())))
    }
}

impl From<&str> for Directory {
    fn from(path: &str) -> Self {
        Self::new(PathBuf::from(path))
    }
}
