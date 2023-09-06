use std::io::{self, Result, Write, Stdout};

use crossterm::{
    execute,
    terminal,
    cursor,
    event::{self, Event, KeyEvent, KeyCode, KeyEventKind},
};

use rogue::{self, Screen};

fn main() -> Result<()> {
    let mut screen = io::stdout();

    terminal::enable_raw_mode()?;
    execute!(
        screen,
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    )?;
    
    run(&mut screen)?;

    terminal::disable_raw_mode()?;
    execute!(
        screen,
        cursor::MoveTo(0, 0),
        cursor::Show,
    )?;
    println!("Goodbye");
    
    Ok(())
}

fn run(screen: &mut Stdout) -> Result<()> {
    let (room, mut player) = rogue::Room::with_player(
        10, 5,
        5, 3,
    );

    screen.print_border(&room)?;

    loop {
        screen.print(&room)?.print(&player)?.flush()?;
        
        match player.update(&room, input()?) {
            KeyCode::Esc => break Ok(()),
            _ => {}
        }
    }
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
