use std::io::{self, Result, Write};
use crossterm::{execute, terminal, cursor, event::KeyCode};

const CLEAR_ALL: terminal::Clear = terminal::Clear(terminal::ClearType::All);

fn main() -> Result<()> {
    //Setup
    let mut screen = io::stdout();

    terminal::enable_raw_mode()?;
    execute!(screen, cursor::Hide, CLEAR_ALL)?;
    
    //Run game
    run(&mut screen, FeaturesEnabled::_Exp)?;

    //Cleanup
    terminal::disable_raw_mode()?;
    execute!(screen, CLEAR_ALL, cursor::MoveTo(0, 0), cursor::Show)?;
    
    Ok(())
}

/// Repeatedly takes input from the player and writes to the screen.
/// Function exits when player presses escape or error occurs.
fn run(out: &mut impl Write, features: FeaturesEnabled) -> Result<()> {
    let mut world = rogue::custom_world();

    match features {
        FeaturesEnabled::_None => world.print_all(out, Default::default()),
        FeaturesEnabled::_Vis => world.print(out, Default::default()),
        FeaturesEnabled::_Exp => world.print_exp(out, Default::default()),
    }?;

    loop {
        let key = rogue::util::input()?;
        if let KeyCode::Esc = key {
            break Ok(());
        }

        let next = world.next(key);
        match features {
            FeaturesEnabled::_None => world.print_all(out, next),
            FeaturesEnabled::_Vis => world.print(out, next),
            FeaturesEnabled::_Exp => world.print_exp(out, next),
        }?;
        world.update(next);
    }
}

enum FeaturesEnabled {
    _None,
    _Vis,
    _Exp,
}
