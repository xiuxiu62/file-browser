use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use file_browser::{AsEntry, Directory, Entry};
use std::{cell::RefCell, io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

macro_rules! percentage {
    ($value:expr) => {
        Constraint::Percentage($value)
    };

    ($($value:expr),*) => {
        [$(percentage!($value)),*].as_ref()
    }
}

fn main() -> DynResult<()> {
    tracing_subscriber::fmt::init();

    let mut directory = Directory::try_from("./src/bin")?;
    directory.populate()?;

    let mut parent_directory = directory.parent()?.unwrap().into_inner();
    parent_directory.populate()?;

    let parent_entries = parent_directory.entries()?;
    parent_entries
        .iter()
        .try_for_each(|entry| entry.borrow_mut().populate())?;

    let current_entries = directory.entries()?;
    current_entries
        .iter()
        .try_for_each(|entry| entry.borrow_mut().populate())?;

    tui(current_entries, parent_entries)

    // run()
}

fn tui(current_entries: &[RefCell<Entry>], parent_entries: &[RefCell<Entry>]) -> DynResult<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // let current_entries = directory.entries()?;
    // current_entries
    //     .iter()
    //     .try_for_each(|entry| entry.borrow_mut().populate())?;

    // let parent_entries = directory.parent()?.unwrap().borrow_mut().entries()?.clone();
    // parent_entries
    //     .iter()
    //     .try_for_each(|entry| entry.borrow_mut().populate())?;

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

        fn list_entries<'a>(
            title: &'a str,
            block: Block<'a>,
            entries: &[RefCell<Entry>],
        ) -> List<'a> {
            let entries = entries.iter().fold(vec![], |mut acc, entry| {
                acc.push(ListItem::new(format!("{}", entry.borrow().clone())));

                acc
            });

            List::new(entries).block(
                block
                    .title(title)
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White)),
            )
        }

        // let current_list = list_entries(current_entries);
        // let previous_entries =

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
                list_entries(" Parent ", default_widget.clone(), parent_entries),
                horizontal_chunks[0]
            ),
            (
                list_entries(" Current ", default_widget.clone(), current_entries),
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

// fn run() -> DynResult<()> {
//     let path = "./src";
//     let mut directory = Directory::try_from(path)?;
//     directory.populate()?;
//     info!("{directory:?}");

//     let parent = directory.parent();
//     info!("{parent:?}");

//     let path = "./src/bin";
//     let mut child = directory.get_entry(path)?;
//     let directory = match child.get_mut().value_mut() {
//         EntryValue::Directory(directory) => directory,
//         EntryValue::File(_) => {
//             return Err(Box::new(file_browser::error::Error::IncorrectFileType {
//                 path: path.to_owned(),
//                 expected: "Directory".to_owned(),
//                 found: "File".to_owned(),
//             }))
//         }
//         EntryValue::SymLink(_) => {
//             return Err(Box::new(file_browser::error::Error::IncorrectFileType {
//                 path: path.to_owned(),
//                 expected: "Directory".to_owned(),
//                 found: "SymLink".to_owned(),
//             }))
//         }
//     };

//     let path = "./src/bin/main.rs";
//     let mut child = directory.get_entry(path)?;
//     let file = match child.get_mut().value_mut() {
//         EntryValue::File(file) => file,
//         EntryValue::Directory(_) => {
//             return Err(Box::new(file_browser::error::Error::IncorrectFileType {
//                 path: path.to_owned(),
//                 expected: "File".to_owned(),
//                 found: "Directory".to_owned(),
//             }))
//         }
//         EntryValue::SymLink(_) => {
//             return Err(Box::new(file_browser::error::Error::IncorrectFileType {
//                 path: path.to_owned(),
//                 expected: "File".to_owned(),
//                 found: "SymLink".to_owned(),
//             }))
//         }
//     };

//     let content = String::from_utf8(file.content()?.to_owned())?;
//     info!("{content}");

//     Ok(())
// }
