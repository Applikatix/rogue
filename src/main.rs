use std::io::Result;

use crossterm::{
    execute,
    terminal,
    cursor,
    event::{self, Event, KeyEvent, KeyCode, KeyEventKind},
};
use rogue;

fn main() -> Result<()> {
    let mut out = rogue::Out::new();
    let (room, mut player) = rogue::world(
        5, 3,
        10, 5,
    );

    terminal::enable_raw_mode()?;
    execute!(
        out.screen,
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    )?;

    loop {
        out.draw(&room, &player)?;
        
        match input()? {
            KeyCode::Left => if player.pos.x > room.pos.x + 1 {
                player.pos.x -= 1;
            },
            KeyCode::Right => if player.pos.x < room.pos.x + room.size.x {
                player.pos.x += 1;
            },
            KeyCode::Up => if player.pos.y > room.pos.y + 1 {
                player.pos.y -= 1;
            },
            KeyCode::Down => if player.pos.y < room.pos.y + room.size.y {
                player.pos.y += 1;
            },
            KeyCode::Esc => break,
            _ => {},
        }
    }

    terminal::disable_raw_mode()?;
    execute!(
        out.screen,
        cursor::Show,
        terminal::Clear(terminal::ClearType::All)
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
