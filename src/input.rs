use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub fn should_quit() -> std::io::Result<bool> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            return Ok(key.code == KeyCode::Char('q'));
        }
    }
    Ok(false)
}

