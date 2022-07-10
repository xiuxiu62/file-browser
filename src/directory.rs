use crate::{
    error::{Error, Result},
    AsEntry, Entry,
};
use ignore::WalkBuilder;
use std::{cell::RefCell, fmt::Display, fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Directory {
    relative_path: PathBuf,
    full_path: PathBuf,
}

impl Directory {
    pub fn entries(&self) -> Result<Vec<RefCell<Entry>>> {
        Ok(WalkBuilder::new(self.full_path())
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
            .map(RefCell::new)
            .collect())
    }
}

impl AsEntry for Directory {
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

impl TryFrom<&str> for Directory {
    type Error = Error;

    fn try_from(path: &str) -> Result<Self> {
        Self::try_from(PathBuf::from(path))
    }
}

impl TryFrom<PathBuf> for Directory {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self> {
        let full_path = fs::canonicalize(&path)?;

        Ok(Self {
            relative_path: path,
            full_path,
        })
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fold_entry =
            |acc: String, entry: &RefCell<Entry>| format!("{acc}{}\n", entry.borrow().clone());

        let message = self
            .entries()
            .unwrap()
            .iter()
            .fold("".to_owned(), fold_entry);

        write!(f, "{}", message.trim_end())
    }
}
