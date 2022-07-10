use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use file_browser::{AsEntry, Directory, Entry, EntryValue};
use std::{cell::RefCell, io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
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

    let current_directory = Directory::try_from("./src")?;
    let parent_directory = current_directory.parent()?.unwrap().into_inner();

    let current_entries = current_directory.entries()?;
    let parent_entries = parent_directory.entries()?;

    let file = current_entries[1].clone().into_inner();
    let preview = match file.as_ref() {
        EntryValue::File(file) => Some(file.content()),
        _ => None,
    }
    .unwrap_or_else(|| {
        eprintln!("entry not of type [FILE]");
        std::process::exit(1);
    })?;
    let preview = String::from_utf8(preview)?;

    tui(&current_entries, &parent_entries, &preview)?;

    Ok(())
}

fn tui(
    current_entries: &[RefCell<Entry>],
    parent_entries: &[RefCell<Entry>],
    preview: &str,
) -> DynResult<()> {
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

        fn list_entries<'a>(title: &'a str, entries: &[RefCell<Entry>]) -> List<'a> {
            let entries = entries.iter().fold(vec![], |mut acc, entry| {
                acc.push(ListItem::new(format!("{}", entry.borrow().clone())));

                acc
            });

            List::new(entries).block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White)),
            )
        }

        let preview_widget = Paragraph::new(preview).block(
            default_widget
                .clone()
                .title(" Preview ")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        );

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
                list_entries(" Parent ", parent_entries),
                horizontal_chunks[0]
            ),
            (
                list_entries(" Current ", current_entries),
                horizontal_chunks[1]
            ),
            (preview_widget, horizontal_chunks[2]),
            (
                default_widget.title(" Commands ").borders(Borders::ALL),
                vertical_chunks[1]
            )
        ];
    })?;

    thread::sleep(Duration::from_millis(6000));

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
