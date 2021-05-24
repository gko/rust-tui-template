pub(crate) mod event;
pub(crate) mod ui;

use std::{error::Error, io};
use termion::{input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{backend::TermionBackend, Terminal};

fn main() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);

    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    ui::AppLayout::draw_loop(&mut terminal)?;

    Ok(())
}
