use file_browser::{AsEntry, Directory, Entry};
use tracing::info;

const DIR: &'static str = ".";

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> DynResult<()> {
    tracing_subscriber::fmt::init();

    let mut directory = Directory::from(DIR);
    directory.populate()?;
    info!("{directory:?}");

    let mut child = directory.get_entry("./src")?;
    match child.get_mut() {
        Entry::Directory(directory) => {
            let mut child = directory.get_entry("./src/error.rs")?;

            match child.get_mut() {
                Entry::File(file) => {
                    let content = String::from_utf8(file.content()?.to_owned())?;
                    info!("{content}");
                }
                _ => info!("Not a file"),
            }
        }
        _ => info!("Not a directory"),
    }

    Ok(())
}
