use super::event::{Event, Events};

use std::io::Stdout;

use termion::{event::Key, input::MouseTerminal, raw::RawTerminal, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::Alignment,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

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

    fn draw(frame: &mut Frame<Backend>) {
        // your layout goes here
        let text = Paragraph::new("press q or Esc to exit").block(
            Block::default()
                .borders(Borders::ALL)
                .title("Example block")
                .border_type(BorderType::Rounded),
        ).alignment(Alignment::Center);

        frame.render_widget(text, frame.size());
    }
}
