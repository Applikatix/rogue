use std::io::Result;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn input() -> Result<KeyCode> {
    loop {
        if let Event::Key(key) = event::read()? {
            if let KeyEventKind::Press = key.kind {
                return Ok(key.code);
            }
        }
    }
}

pub fn average(a: u16, b: u16) -> u16 {
    (a + b)/ 2
}
