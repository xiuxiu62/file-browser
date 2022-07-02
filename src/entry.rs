use crate::{
    error::{Error, Result},
    Directory, File, SymLink,
};
use std::{cell::RefCell, fmt::Display, path::PathBuf};

pub trait AsEntry {
    fn relative_path(&self) -> &PathBuf;

    fn full_path(&self) -> &PathBuf;

    fn populate(&mut self) -> Result<()>;

    fn parent(&self) -> Result<Option<RefCell<Directory>>>;
}

#[derive(Debug, Clone)]
pub struct Entry {
    value: EntryValue,
    parent: Option<RefCell<Directory>>,
}

impl Entry {
    pub fn new(value: EntryValue) -> Result<Self> {
        let parent = value.parent()?;

        Ok(Self { value, parent })
    }

    pub fn value(&self) -> &EntryValue {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut EntryValue {
        &mut self.value
    }

    pub fn relative_path(&self) -> &PathBuf {
        self.value.relative_path()
    }

    pub fn full_path(&self) -> &PathBuf {
        self.value.full_path()
    }

    pub fn populate(&mut self) -> Result<()> {
        self.value.populate()
    }

    pub fn update_parent(&mut self) -> Result<()> {
        self.parent = self.value.parent()?;

        Ok(())
    }

    pub fn parent(&self) -> &Option<RefCell<Directory>> {
        &self.parent
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
        let value = EntryValue::try_from(path)?;
        let parent = value.parent()?;

        Ok(Self { value, parent })
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub enum EntryValue {
    Directory(Directory),
    File(File),
    SymLink(SymLink),
}

impl EntryValue {
    fn get_parent(entry: &impl AsEntry) -> Result<Option<RefCell<Directory>>> {
        Ok(match entry.full_path().parent() {
            Some(parent) => Some(RefCell::new(Directory::try_from(parent.to_path_buf())?)),
            None => None,
        })
    }
}

impl AsEntry for EntryValue {
    fn relative_path(&self) -> &PathBuf {
        match self {
            Self::Directory(directory) => directory.relative_path(),
            Self::File(file) => file.relative_path(),
            Self::SymLink(symlink) => symlink.relative_path(),
        }
    }

    fn full_path(&self) -> &PathBuf {
        match self {
            Self::Directory(directory) => directory.full_path(),
            Self::File(file) => file.full_path(),
            Self::SymLink(symlink) => symlink.full_path(),
        }
    }

    fn populate(&mut self) -> Result<()> {
        match self {
            Self::Directory(directory) => directory.populate(),
            Self::File(file) => file.populate(),
            Self::SymLink(symlink) => symlink.populate(),
        }
    }

    fn parent(&self) -> Result<Option<RefCell<Directory>>> {
        match self {
            Self::Directory(directory) => Self::get_parent(directory),
            Self::File(file) => Self::get_parent(file),
            Self::SymLink(symlink) => Self::get_parent(symlink),
        }
    }
}

impl TryFrom<&str> for EntryValue {
    type Error = Error;

    fn try_from(path: &str) -> Result<Self> {
        Self::try_from(PathBuf::from(path))
    }
}

impl TryFrom<PathBuf> for EntryValue {
    type Error = Error;

    fn try_from(path: PathBuf) -> Result<Self> {
        let file_type = std::fs::metadata(path.clone())?.file_type();
        if file_type.is_dir() {
            return Ok(Self::Directory(Directory::try_from(path)?));
        }

        if file_type.is_file() {
            return Ok(Self::File(File::try_from(path)?));
        }

        if file_type.is_symlink() {
            return Ok(Self::SymLink(SymLink::try_from(path)?));
        }

        Err(Error::UnrecognizedFileType(file_type))
    }
}

impl Display for EntryValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Directory(directory) => write!(f, "{}", directory),
            Self::File(file) => write!(f, "{}", file),
            Self::SymLink(symlink) => write!(f, "{}", symlink),
        }
    }
}
