use std::io::{self, Result, Write};
use crossterm::{execute, terminal, cursor, event::KeyCode};

const CLEAR_ALL: terminal::Clear = terminal::Clear(terminal::ClearType::All);

fn main() -> Result<()> {
    //Setup
    let mut screen = io::stdout();

    terminal::enable_raw_mode()?;
    execute!(screen, cursor::Hide, CLEAR_ALL)?;
    
    //Run game
    run(&mut screen)?;

    //Cleanup
    terminal::disable_raw_mode()?;
    execute!(screen, CLEAR_ALL, cursor::MoveTo(0, 0), cursor::Show)?;
    
    Ok(())
}

/// Repeatedly takes input from the player and writes to the screen.
/// Function exits when player presses escape or error occurs.
fn run(out: &mut impl Write) -> Result<()> {
    let mut world = rogue::custom_world();

    world.print_alt(out, Default::default())?;

    loop {
        let key = rogue::util::input()?;
        if let KeyCode::Esc = key {
            break Ok(());
        }

        let next = world.next(key);
        world.print_alt(out, next)?;
        world.update(next);
    }
}
