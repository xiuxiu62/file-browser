use file_browser::{AsEntry, Directory, EntryValue};
use tracing::info;

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> DynResult<()> {
    tracing_subscriber::fmt::init();

    run()
}

fn run() -> DynResult<()> {
    let path = "./src";
    let mut directory = Directory::try_from(path)?;
    directory.populate()?;
    info!("{directory:?}");

    let parent = directory.parent();
    info!("{parent:?}");

    let path = "./src/bin";
    let mut child = directory.get_entry(path)?;
    let directory = match child.get_mut().value_mut() {
        EntryValue::Directory(directory) => directory,
        EntryValue::File(_) => {
            return Err(Box::new(file_browser::error::Error::IncorrectFileType {
                path: path.to_owned(),
                expected: "Directory".to_owned(),
                found: "File".to_owned(),
            }))
        }
        EntryValue::SymLink(_) => {
            return Err(Box::new(file_browser::error::Error::IncorrectFileType {
                path: path.to_owned(),
                expected: "Directory".to_owned(),
                found: "SymLink".to_owned(),
            }))
        }
    };

    let path = "./src/bin/main.rs";
    let mut child = directory.get_entry(path)?;
    let file = match child.get_mut().value_mut() {
        EntryValue::File(file) => file,
        EntryValue::Directory(_) => {
            return Err(Box::new(file_browser::error::Error::IncorrectFileType {
                path: path.to_owned(),
                expected: "File".to_owned(),
                found: "Directory".to_owned(),
            }))
        }
        EntryValue::SymLink(_) => {
            return Err(Box::new(file_browser::error::Error::IncorrectFileType {
                path: path.to_owned(),
                expected: "File".to_owned(),
                found: "SymLink".to_owned(),
            }))
        }
    };

    let content = String::from_utf8(file.content()?.to_owned())?;
    info!("{content}");

    Ok(())
}
