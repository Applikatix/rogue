use std::io::{Result, Write};

use crossterm::event::KeyCode;

use game_elements::{Dir, Area, Room, Path, Map};

/// Run the game
pub fn run(screen: &mut impl Write) -> Result<()> {
    let map = Map {
        rooms: vec![Room::new(10, 5, 5, 3), Room::new(15, 10, 10, 4)],
        paths: vec![Path::new(15, 6, Dir::X(5)), Path::new(20, 6, Dir::Y(4))],
    };
    let mut player = map.spawn_player();

    screen.frame(&map)?;

    loop {
        screen.print(&map)?.print(&player)?.flush()?;
        
        let key = util::input()?;
        if let KeyCode::Esc = key { return Ok(()); }

        map.update(&mut player, key);
    }
}

trait Screen {
    fn print(&mut self, item: &impl Drawable) -> Result<&mut Self>;
    fn frame(&mut self, item: &impl Drawable) -> Result<&mut Self>;
}

impl<T: Write> Screen for T {
    fn print(&mut self, item: &impl Drawable) -> Result<&mut Self> {
        item.draw(self)?;
        Ok(self)
    }

    fn frame(&mut self, item: &impl Drawable) -> Result<&mut Self> {
        item.border(self)?;
        Ok(self)
    }
}

trait Drawable {
    fn draw(&self, screen: &mut impl Write) -> Result<()>;

    fn border(&self, _: &mut impl Write) -> Result<()> {
        Ok(())
    }
}

mod screen;
mod game_elements;
mod util;
#[cfg(test)]
mod tests;
