use std::{io, thread, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    widgets::{Block, Borders},
    layout::{Constraint, Direction, Layout},
    Terminal,
    Frame
};
use crossterm::{
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, DisableMouseCapture, EnableMouseCapture, Event}
};

fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10)
            ].as_ref()
        )
        .split(f.size());

    let block = Block::default()
        .title("Block")
        .borders(Borders::ALL);

    f.render_widget(block, chunks[0]);

    let block = Block::default()
        .title("Block 2")
        .borders(Borders::ALL);

    f.render_widget(block, chunks[1]);

    let block = Block::default()
        .title("Block 3")
        .borders(Borders::ALL);

    f.render_widget(block, chunks[2]);
}

fn main() -> Result<(), io::Error>{
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // draw
    terminal.draw(|f| {
        ui(f);
    })?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal (get input)
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;

    terminal.show_cursor()?;
    Ok(())
}
