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
    entries: Option<Vec<RefCell<Entry>>>,
}

impl Directory {
    pub fn get_entry(&mut self, path: &str) -> Result<RefCell<Entry>> {
        match &self.entries {
            Some(entries) => {
                let full_path = fs::canonicalize(path)?;

                match entries
                    .iter()
                    .filter(|entry| entry.borrow().full_path() == &full_path)
                    .collect::<Vec<&RefCell<Entry>>>()
                    .first()
                    .map(|entry| entry.to_owned().to_owned())
                {
                    Some(entry) => Ok(entry),
                    None => Err(Error::FileNotFound(path.to_owned())),
                }
            }
            None => {
                self.populate()?;
                self.get_entry(path)
            }
        }
    }

    pub fn entries(&mut self) -> Result<&Vec<RefCell<Entry>>> {
        if self.entries.is_none() {
            self.populate()?;
        }

        Ok(self.entries.as_ref().unwrap())
    }
}

impl AsEntry for Directory {
    fn relative_path(&self) -> &PathBuf {
        &self.relative_path
    }

    fn full_path(&self) -> &PathBuf {
        &self.full_path
    }

    fn populate(&mut self) -> Result<()> {
        self.entries = Some(
            WalkBuilder::new(self.full_path())
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
                .collect(),
        );

        Ok(())
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
            entries: None,
        })
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self
            .entries
            .as_ref()
            // .unwrap()
            .iter()
            .fold(String::new(), |acc, entry| {
                // format!("{acc}{:?}\n", entry.borrow().clone())
                format!("{acc}{:?}\n", entry)
            });

        write!(f, "{}", message.trim_end())
    }
}
