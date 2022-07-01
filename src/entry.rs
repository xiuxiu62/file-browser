use crate::{
    error::{Error, Result},
    Directory, File, SymLink,
};
use std::{cell::RefCell, fs, path::PathBuf};

pub trait AsEntry {
    fn path(&self) -> &PathBuf;

    fn canonicalize(&self) -> Result<PathBuf> {
        let path = fs::canonicalize(self.path())?;

        Ok(path)
    }

    fn populate(&mut self) -> Result<()>;

    fn parent(&self) -> Option<RefCell<Directory>>;
}

#[derive(Debug, Clone)]
pub struct Entry {
    value: EntryValue,
    parent: Option<RefCell<Directory>>,
}

impl Entry {
    pub fn new(value: EntryValue) -> Self {
        let parent = value.parent();

        Self { value, parent }
    }

    pub fn value(&self) -> &EntryValue {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut EntryValue {
        &mut self.value
    }

    pub fn path(&self) -> &PathBuf {
        self.value.path()
    }

    pub fn populate(&mut self) -> Result<()> {
        self.value.populate()
    }

    pub fn update_parent(&mut self) {
        self.parent = self.value.parent();
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
        let parent = value.parent();

        Ok(Self { value, parent })
    }
}

#[derive(Debug, Clone)]
pub enum EntryValue {
    Directory(Directory),
    File(File),
    SymLink(SymLink),
}

impl EntryValue {
    fn get_parent(entry: &impl AsEntry) -> Option<RefCell<Directory>> {
        entry
            .path()
            .parent()
            .map(|parent| RefCell::new(Directory::new(parent.to_path_buf())))
    }
}

impl AsEntry for EntryValue {
    fn path(&self) -> &PathBuf {
        match self {
            Self::Directory(directory) => directory.path(),
            Self::File(file) => file.path(),
            Self::SymLink(symlink) => symlink.path(),
        }
    }

    fn populate(&mut self) -> Result<()> {
        match self {
            Self::Directory(directory) => directory.populate(),
            Self::File(file) => file.populate(),
            Self::SymLink(symlink) => symlink.populate(),
        }
    }

    fn parent(&self) -> Option<RefCell<Directory>> {
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
            return Ok(Self::Directory(Directory::new(path)));
        }

        if file_type.is_file() {
            return Ok(Self::File(File::new(path)));
        }

        if file_type.is_symlink() {
            return Ok(Self::SymLink(SymLink::new(path)));
        }

        Err(Error::UnrecognizedFileType(file_type))
    }
}
