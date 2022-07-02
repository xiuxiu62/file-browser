use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use file_browser::{AsEntry, Directory, Entry, EntryValue};
use std::{cell::RefCell, io, thread, time::Duration};
use tracing::info;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, StatefulWidget},
    Terminal,
};

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

// struct DirectoryWidget(Directory);

// impl StatefulWidget for DirectoryWidget {
//     type State;

//     fn render(
//         self,
//         area: tui::layout::Rect,
//         buf: &mut tui::buffer::Buffer,
//         state: &mut Self::State,
//     ) {
//     }
// }

macro_rules! percentage {
    ($value:expr) => {
        Constraint::Percentage($value)
    };

    ($($value:expr),*) => {
        [$(percentage!($value)),*].as_ref()
    }
}

macro_rules! list {
    ($($value:expr),*) => {
        vec![$(ListItem::new($value)),*]
    }
}

fn main() -> DynResult<()> {
    tracing_subscriber::fmt::init();

    let mut directory = Directory::try_from(".")?;
    let entries = directory.entries()?;

    tui(entries)?;

    run()
}

fn tui(entries: &Vec<RefCell<Entry>>) -> DynResult<()> {
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

        let entries_list = {
            let entries = entries.iter().fold(vec![], |mut acc, entry| {
                acc.push(ListItem::new(format!("{}", entry.borrow().clone())));

                acc
            });

            List::new(entries).block(
                default_widget
                    .clone()
                    .title(" Current ")
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White)),
            )
        };

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
            (entries_list, horizontal_chunks[1]),
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
