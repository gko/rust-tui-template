use super::event::{Event, Events};

use std::io::Stdout;

use termion::{event::Key, input::MouseTerminal, raw::RawTerminal, screen::AlternateScreen};
use tui::{backend::TermionBackend, Frame, Terminal};

type Backend = TermionBackend<AlternateScreen<MouseTerminal<RawTerminal<Stdout>>>>;

pub struct AppLayout {}

impl AppLayout {
    pub fn draw_loop(terminal: &mut Terminal<Backend>) -> Result<(), Box<dyn std::error::Error>> {
        let events = Events::new();

        loop {
            terminal.draw(AppLayout::draw)?;

            if matches!(
                events.next()?,
                Event::Input(Key::Char('q')) | Event::Input(Key::Esc)
            ) {
                break;
            }
        }

        Ok(())
    }

    fn draw(_f: &mut Frame<Backend>) {
        // your layout goes here
    }
}
