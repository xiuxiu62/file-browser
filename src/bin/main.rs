use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use file_browser::{AsEntry, Directory, EntryValue};
use std::{io, thread, time::Duration};
use tracing::info;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> DynResult<()> {
    tracing_subscriber::fmt::init();

    tui()?;

    run()
}

macro_rules! percentage {
    ($value:expr) => {
        Constraint::Percentage($value)
    };

    ($($value:expr),*) => {
        [$(percentage!($value)),*].as_ref()
    }
}

fn tui() -> DynResult<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|frame| {
        let size = frame.size();
        let default_widget = Block::default();

        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(percentage![95, 5])
            .split(size);

        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(percentage![30, 40, 30])
            .split(vertical_chunks[0]);

        macro_rules! render_widget {
            ($widget:expr, $rect:expr) => {
                frame.render_widget($widget, $rect);
            };

            ($(($widget:expr, $rect:expr)),*) => {
                $(render_widget!($widget, $rect);)*
            }
        }

        render_widget![
            (
                default_widget
                    .clone()
                    .borders(Borders::ALL)
                    .title(" Parent "),
                horizontal_chunks[0]
            ),
            (
                Paragraph::new("test").block(
                    default_widget
                        .clone()
                        .borders(Borders::ALL)
                        .title(" Current ")
                ),
                horizontal_chunks[1]
            ),
            (
                default_widget
                    .clone()
                    .borders(Borders::ALL)
                    .title(" Preview "),
                horizontal_chunks[2]
            ),
            (
                default_widget.borders(Borders::ALL).title(" Commands "),
                vertical_chunks[1]
            )
        ];
    })?;

    thread::sleep(Duration::from_millis(4000));

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
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
