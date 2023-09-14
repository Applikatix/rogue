use std::io::{self, Result};

use crossterm::{execute, terminal, cursor};

fn main() -> Result<()> {
    //Setup
    let mut screen = io::stdout();

    terminal::enable_raw_mode()?;
    execute!(
        screen,
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    )?;
    
    //Run game
    rogue::run(&mut screen)?;

    //Cleanup
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
