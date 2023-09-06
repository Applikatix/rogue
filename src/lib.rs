use std::io::{Result, Write};

use crossterm::{ queue,
    cursor::MoveTo,
    style::{PrintStyledContent, Stylize, StyledContent, Print},
    event::KeyCode
};

pub trait Screen {
    fn print(&mut self, item: &impl Drawable) -> Result<&mut Self>;

    fn print_border(&mut self, item: &impl Drawable) -> Result<&mut Self>;
}

impl<T: Write> Screen for T {
    fn print(&mut self, item: &impl Drawable) -> Result<&mut Self> {
        item.draw(self)?;
        Ok(self)
    }

    fn print_border(&mut self, item: &impl Drawable) -> Result<&mut Self> {
        item.border(self)?;
        Ok(self)
    }
}

pub trait Drawable {
    fn draw(&self, screen: &mut impl Write) -> Result<()>;

    fn border(&self, _: &mut impl Write) -> Result<()> {
        Ok(())
    }
}

struct Point { x: u16, y: u16 }

impl Point {
    fn contains(&self, p2: &Point) -> bool {
        self.x >= p2.x && self.y >= p2.y
    }
}

pub struct Room {
    tile: StyledContent<char>,
    p1: Point,
    p2: Point,
}

impl Room {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Room {
        Room {
            tile: '.'.dark_grey(),
            p1: Point { x, y },
            p2: Point { x: x + width - 1, y: y + height - 1 },
        }
    }

    pub fn with_player(
        x: u16,
        y: u16,
        width: u16,
        height: u16
    ) -> (Room, Object) {
        (Room::new(x, y, width, height), Object::player(x, y))
    }

    fn contains(&self, p: &Point) -> bool {
        p.contains(&self.p1) && self.p2.contains(p)
    }
}

impl Drawable for Room {
    fn draw(&self, screen: &mut impl Write) -> Result<()> {
        for row in self.p1.y..=self.p2.y {
            queue!(screen, MoveTo(self.p1.x, row))?;
            for _ in self.p1.x..=self.p2.x {
                queue!(screen, PrintStyledContent(self.tile))?;
            }
        }
        Ok(())
    }

    fn border(&self, screen: &mut impl Write) -> Result<()> {
        let Room {
            tile: _,
            p1: Point { x, y },
            p2: Point { x: right, y: bottom }
        } = *self;
        let left = x - 1;
        let top = y - 1;
        let right = right + 1;
        let bottom = bottom + 1;

        queue!(screen, MoveTo(left, top), Print('┌'))?;
        for _ in x..right {
            queue!(screen, Print('─'))?;
        }
        queue!(screen, Print('┐'))?;

        queue!(screen, MoveTo(left, bottom), Print('└'))?;
        for _ in x..right {
            queue!(screen, Print('─'))?;
        }
        queue!(screen, Print('┘'))?;

        for row in y..bottom {
            queue!(screen,
                MoveTo(left, row), Print('│'),
                MoveTo(right, row), Print('│'),
            )?;
        }

        Ok(())
    }
}

pub struct Object {
    character: StyledContent<char>,
    pos: Point,
}

impl Object {
    pub fn player(x: u16, y: u16) -> Object {
        Object { character: '@'.yellow(), pos: Point { x, y } }
    }

    pub fn update(&mut self, room: &Room, key: KeyCode) -> KeyCode {
        let point = match key {
            KeyCode::Left => Point { x: self.pos.x - 1, ..self.pos },
            KeyCode::Right => Point { x: self.pos.x + 1, ..self.pos },
            KeyCode::Up => Point { y: self.pos.y - 1, ..self.pos },
            KeyCode::Down => Point { y: self.pos.y + 1, ..self.pos },
            _ => Point { ..self.pos },
        };
        if room.contains(&point) {
            self.pos = point;
        }
        key
    }
}

impl Drawable for Object {
    fn draw(&self, screen: &mut impl Write) -> Result<()> {
        queue!(screen,
            MoveTo(self.pos.x, self.pos.y),
            PrintStyledContent(self.character)
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
