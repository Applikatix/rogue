use std::io::{self, Result};

use crossterm::{execute, terminal, cursor};

use rogue::run;

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
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
        cursor::Show,
    )?;
    println!("Goodbye");
    
    Ok(())
}
