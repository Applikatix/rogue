use std::io::{self, Result, Write};
use crossterm::{execute, terminal, cursor, event::KeyCode};
use rogue::util::input;

const CLEAR_TERM: terminal::Clear = terminal::Clear(terminal::ClearType::All);

fn main() -> Result<()> {
    //Setup
    let mut screen = io::stdout();

    terminal::enable_raw_mode()?;
    execute!(screen, cursor::Hide, CLEAR_TERM)?;
    
    //Run game
    run(&mut screen)?;

    //Cleanup
    terminal::disable_raw_mode()?;
    execute!(screen, CLEAR_TERM, cursor::MoveTo(0, 0), cursor::Show)?;
    println!("Goodbye");
    
    Ok(())
}

/// Repeatedly takes input from the player and writes to the screen.
/// Function exits when player presses escape or error occurs.
fn run(screen: &mut impl Write) -> Result<()> {
    let mut world = rogue::create_world();

    world.init(screen)?;
    screen.flush()?;

    loop {
        let key = input()?;

        if let KeyCode::Esc = key {
            break Ok(());
        }

        let next = world.next(key);
        world.print(screen, &next)?;
        world.update(next);
        screen.flush()?;
    }
}