use file_browser::{AsEntry, Directory, Entry};
use tracing::info;

const DIR: &'static str = "src";

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> DynResult<()> {
    tracing_subscriber::fmt::init();

    let mut directory = Directory::from(DIR);
    directory.populate()?;
    info!("{directory:?}");

    let mut child = directory.get_entry("src/error.rs")?;
    info!("{child:?}");

    if let Entry::File(file) = child.get_mut() {
        let content = String::from_utf8(file.content()?.to_owned())?;
        info!("{content}");
    } else {
        info!("Not a file");
    }

    Ok(())
}
