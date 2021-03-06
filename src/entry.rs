use crate::{
    error::{Error, Result},
    Directory, File, SymLink,
};
use std::{
    cell::RefCell,
    fmt::{self, Display},
    fs::Metadata,
    path::PathBuf,
    time::SystemTime,
};

pub trait AsEntry {
    fn relative_path(&self) -> &PathBuf;

    fn full_path(&self) -> &PathBuf;

    fn parent(&self) -> Result<Option<RefCell<Directory>>>;

    #[inline]
    fn metadata(&self) -> Result<Metadata> {
        Ok(self.full_path().metadata()?)
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Entry {
    value: EntryValue,
    last_modified: SystemTime,
}

impl AsRef<EntryValue> for Entry {
    fn as_ref(&self) -> &EntryValue {
        &self.value
    }
}

impl AsMut<EntryValue> for Entry {
    fn as_mut(&mut self) -> &mut EntryValue {
        &mut self.value
    }
}

impl AsEntry for Entry {
    fn relative_path(&self) -> &PathBuf {
        self.value.relative_path()
    }

    fn full_path(&self) -> &PathBuf {
        self.value.full_path()
    }

    fn parent(&self) -> Result<Option<RefCell<Directory>>> {
        self.value.parent()
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
        let last_modified = value.metadata()?.modified()?;

        Ok(Self {
            value,
            last_modified,
        })
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Directory(directory) => {
                write!(f, "{}", directory.relative_path().to_string_lossy())
            }
            Self::File(file) => write!(f, "{file}"),
            Self::SymLink(symlink) => write!(f, "{symlink}"),
        }
    }
}
