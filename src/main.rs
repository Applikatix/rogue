use std::io::{self, Result, Write};

use crossterm::{
    execute,
    terminal,
    cursor,
    event::{self, Event, KeyEvent, KeyCode, KeyEventKind},
};
use rogue::{self, Screen};

fn main() -> Result<()> {
    let mut screen = io::stdout();
    let (room, mut player) = rogue::Room::with_player(
        10, 5,
        5, 3
    );

    terminal::enable_raw_mode()?;
    execute!(
        screen,
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    )?;

    screen.print(&rogue::Rect::from(&room))?;

    loop {
        screen.print(&room)?.print(&player)?.flush()?;
        
        match player.update(&room, input()?) {
            KeyCode::Esc => break,
            _ => {}
        }
    }

    terminal::disable_raw_mode()?;
    execute!(
        screen,
        terminal::Clear(terminal::ClearType::All),
        cursor::Show,
    )?;
    println!("Game ended");
    
    Ok(())
}

fn input() -> Result<KeyCode> {
    loop {
        if let Event::Key(key) = event::read()? {
            let KeyEvent {
                code,
                modifiers: _,
                kind,
                state: _,
            } = key;

            if let KeyEventKind::Press = kind {
                return Ok(code);
            }
        }
    }
}
