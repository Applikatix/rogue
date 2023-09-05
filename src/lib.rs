use std::io::{Result, Write};

use crossterm::{ queue,
    cursor::MoveTo,
    style::{PrintStyledContent, Stylize, StyledContent, Color, Print},
    event::KeyCode
};

pub trait Screen {
    fn print(&mut self, item: &impl Drawable) -> Result<&mut Self>;
}

impl<T: Write> Screen for T {
    fn print(&mut self, item: &impl Drawable) -> Result<&mut Self> {
        item.draw(self)?;
        Ok(self)
    }
}

pub trait Drawable {
    fn draw(&self, screen: &mut impl Write) -> Result<()>;
}

pub struct Room {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}
impl Room {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Room {
        Room { x, y, width, height }
    }

    pub fn with_player(
        x: u16,
        y: u16,
        width: u16,
        height: u16
    ) -> (Room, Object) {
        (
            Room { x, y, width, height },
            Object { character: '@'.with(Color::Black).on(Color::Yellow), x, y }
        )
    }
}
impl Drawable for Room {
    fn draw(&self, screen: &mut impl Write) -> Result<()> {
        for row in self.y..(self.y + self.height) {
            queue!(screen, MoveTo(self.x, row))?;
            for _ in 0..self.width {
                queue!(screen, PrintStyledContent('.'.dark_grey()))?;
            }
        }
        Ok(())
    }
}

pub struct Rect {
    left: u16,
    top: u16,
    right: u16,
    bottom: u16,
}
impl Rect {
    pub fn new(left: u16, top: u16,right: u16, bottom: u16) -> Rect {
        Rect { left, top, right, bottom }
    }
    
    pub fn from(room: &Room) -> Rect {
        Rect {
            left: room.x - 1,
            top: room.y - 1,
            right: room.x + room.width,
            bottom: room.y + room.height
        }
    }
}
impl Drawable for Rect {
    fn draw(&self, screen: &mut impl Write) -> Result<()> {
        let Rect { left, top, right, bottom } = *self;
        let x = left + 1;
        let y = top + 1;

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
    x: u16,
    y: u16,
}
impl Object {
    pub fn update(&mut self, room: &Room, key: KeyCode) -> KeyCode {
        match key {
            KeyCode::Left => if self.x > room.x {
                self.x -= 1;
            }
            KeyCode::Right => if self.x < room.x + room.width - 1 {
                self.x += 1;
            }
            KeyCode::Up => if self.y > room.y {
                self.y -= 1;
            }
            KeyCode::Down => if self.y < room.y + room.height - 1 {
                self.y += 1;
            }
            _ => {}
        }
        key
    }
}
impl Drawable for Object {
    fn draw(&self, screen: &mut impl Write) -> Result<()> {
        queue!(screen,
            MoveTo(self.x, self.y),
            PrintStyledContent(self.character)
        )?;
        Ok(())
    }
}
