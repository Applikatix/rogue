use std::io::{Result, Write};

use crossterm::{ queue,
    cursor::MoveTo,
    style::{PrintStyledContent, Stylize, StyledContent, Print},
    event::KeyCode,
};

use super::*;

#[derive(Clone, Copy)]
pub struct Point { x: u16, y: u16 }

#[derive(Clone, Copy)]
pub enum Dir { X(u16), Y(u16) }

pub struct Object {
    character: StyledContent<char>,
    p: Point,
}

impl Object {
    fn player(x: u16, y: u16) -> Object {
        Object { character: '@'.yellow(), p: Point { x, y } }
    }
}

impl Drawable for Object {
    fn draw(&self, screen: &mut impl Write) -> Result<()> {
        queue!(screen,
            MoveTo(self.p.x, self.p.y),
            PrintStyledContent(self.character)
        )?;
        Ok(())
    }
}

pub trait Area {
    fn contains(&self, p: &Point) -> bool;

    fn spawn_player(&self) -> Object;

    fn update(&self, object: &mut Object, key: KeyCode) {
        let point = match key {
            KeyCode::Left => Point { x: object.p.x - 1, ..object.p },
            KeyCode::Right => Point { x: object.p.x + 1, ..object.p },
            KeyCode::Up => Point { y: object.p.y - 1, ..object.p },
            KeyCode::Down => Point { y: object.p.y + 1, ..object.p },
            _ => return,
        };
        if self.contains(&point) {
            object.p = point;
        }
    }
}

pub struct Perimeter {
    left: u16,
    top: u16,
    right: u16,
    bottom: u16,
}

impl Perimeter {
    pub fn _new(left: u16, top: u16, right: u16, bottom: u16) -> Perimeter {
        Perimeter { left, top, right, bottom }
    }
}

impl From<&Room> for Perimeter {
    fn from(value: &Room) -> Self {
        let Room { tile: _, p1, p2 } = value;
        Perimeter {
            left: p1.x - 1,
            top: p1.y - 1,
            right: p2.x + 1,
            bottom: p2.y + 1,
        }
    }
}

impl Drawable for Perimeter {
    fn draw(&self, screen: &mut impl Write) -> Result<()> {
        let Perimeter {
            left,
            top,
            right,
            bottom,
        } = *self;
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
}

impl Area for Room {
    fn contains(&self, p: &Point) -> bool {
        p.x >= self.p1.x && p.x <= self.p2.x &&
        p.y >= self.p1.y && p.y <= self.p2.y
    }

    fn spawn_player(&self) -> Object {
        let Room { tile: _, p1, p2 } = self;
        Object::player(
            util::average(p1.x, p2.x),
            util::average(p1.y, p2.y)
        )
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
        Perimeter::from(self).draw(screen)?;
        Ok(())
    }
}

pub struct Path {
    tile: StyledContent<char>,
    p1: Point,
    end: Dir,
}

impl Path {
    pub fn new(x: u16, y: u16, length: Dir) -> Path {
        Path {
            tile: '#'.dark_grey(),
            p1: Point { x, y },
            end: match length {
                Dir::X(len) => Dir::X(x + len - 1),
                Dir::Y(len) => Dir::Y(y + len - 1),
            },
        }
    }
}

impl Area for Path {
    fn contains(&self, p: &Point) -> bool {
        match self.end {
            Dir::X(end) =>
                p.y == self.p1.y &&
                p.x >= self.p1.x &&
                p.x <= end,
            Dir::Y(end) => 
                p.x == self.p1.x &&
                p.y >= self.p1.y &&
                p.y <= end,
        }
    }
    
    fn spawn_player(&self) -> Object {
        let Path { tile: _, p1, end } = *self;
        match end {
            Dir::X(x) =>
                Object::player(util::average(p1.x, x), p1.y),
            Dir::Y(y) => 
                Object::player(p1.x, util::average(p1.y, y)),
        }
    }
}

impl Drawable for Path {
    fn draw(&self, screen: &mut impl Write) -> Result<()> {
        match self.end {
            Dir::X(end) => for col in self.p1.x..=end {
                queue!(screen,
                    MoveTo(col, self.p1.y),
                    PrintStyledContent(self.tile)
                )?;
            },
            Dir::Y(end) => for row in self.p1.y..=end {
                queue!(screen,
                    MoveTo(self.p1.x, row),
                    PrintStyledContent(self.tile)
                )?;
            },
        }
        Ok(())
    }
}

pub struct Map {
    pub rooms: Vec<Room>,
    pub paths: Vec<Path>,
}

impl Area for Map {
    fn contains(&self, p: &Point) -> bool {
        for room in &self.rooms {
            if room.contains(&p) { return true; }
        }
        for path in &self.paths {
            if path.contains(&p) { return true; }
        }
        false
    }

    fn spawn_player(&self) -> Object {
        self.rooms[0].spawn_player()
    }
}

impl Drawable for Map {
    fn draw(&self, screen: &mut impl Write) -> Result<()> {
        for room in &self.rooms {
            room.draw(screen)?;
        }
        for path in &self.paths {
            path.draw(screen)?;
        }
        Ok(())
    }

    fn border(&self, screen: &mut impl Write) -> Result<()> {
        for room in &self.rooms {
            room.border(screen)?;
        }
        Ok(())
    }
}
