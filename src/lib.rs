use std::io::{Result, Write};

use crossterm::{ queue,
    cursor::MoveTo,
    style::{PrintStyledContent, Stylize, StyledContent, Print},
    event::{self, Event, KeyEvent, KeyCode, KeyEventKind},
};

pub fn run(screen: &mut impl Write) -> Result<()> {
    let (room, mut player) = Room::with_player(
        10, 5,
        5, 3,
    );

    screen.frame(&room)?;

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

#[derive(Clone, Copy)]
pub struct Point { x: u16, y: u16 }

trait Area {
    fn contains(&self, p: &Point) -> bool;
}

struct Room {
    tile: StyledContent<char>,
    p1: Point,
    p2: Point,
}

impl Room {
    fn new(x: u16, y: u16, width: u16, height: u16) -> Room {
        Room {
            tile: '.'.dark_grey(),
            p1: Point { x, y },
            p2: Point { x: x + width - 1, y: y + height - 1 },
        }
    }

    fn with_player(
        x: u16,
        y: u16,
        width: u16,
        height: u16
    ) -> (Room, Object) {
        (Room::new(x, y, width, height), Object::player(x, y))
    }
}

impl Area for Room {
    fn contains(&self, p: &Point) -> bool {
        let Room { tile: _,
            p1: Point { x: left, y: top },
            p2: Point { x: right, y: bottom }
        } = *self;

        p.x >= left && p.x <= right && p.y >= top && p.y <= bottom
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

enum Dir {
    X(u16),
    Y(u16),
}

struct Path {
    tile: StyledContent<char>,
    start: Point,
    end: Dir,
}

impl Path {
    fn new(x: u16, y: u16, length: Dir) -> Path {
        Path {
            tile: '#'.dark_grey(),
            start: Point { x, y },
            end: match length {
                Dir::X(len) => Dir::X(x + len - 1),
                Dir::Y(len) => Dir::Y(y + len - 1),
            },
        }
    }

    fn with_player(x: u16, y: u16, length: Dir) -> (Path, Object) {
        (Path::new(x, y, length), Object::player(x, y))
    }
}

impl Area for Path {
    fn contains(&self, p: &Point) -> bool {
        match self.end {
            Dir::X(end) =>
                p.y == self.start.y &&
                p.x >= self.start.x &&
                p.x <= end,
            Dir::Y(end) => 
                p.x == self.start.x &&
                p.y >= self.start.y &&
                p.y <= end,
        }
    }
}

impl Drawable for Path {
    fn draw(&self, screen: &mut impl Write) -> Result<()> {
        match self.end {
            Dir::X(end) => for col in self.start.x..=end {
                queue!(screen,
                    MoveTo(col, self.start.y),
                    PrintStyledContent(self.tile)
                )?;
            },
            Dir::Y(end) => for row in self.start.y..=end {
                queue!(screen,
                    MoveTo(self.start.x, row),
                    PrintStyledContent(self.tile)
                )?;
            },
        }
        Ok(())
    }
}

struct Object {
    character: StyledContent<char>,
    pos: Point,
}

impl Object {
    fn player(x: u16, y: u16) -> Object {
        Object { character: '@'.yellow(), pos: Point { x, y } }
    }

    fn update(&mut self, area: &impl Area, key: KeyCode) -> KeyCode {
        let point = match key {
            KeyCode::Left => Point { x: self.pos.x - 1, ..self.pos },
            KeyCode::Right => Point { x: self.pos.x + 1, ..self.pos },
            KeyCode::Up => Point { y: self.pos.y - 1, ..self.pos },
            KeyCode::Down => Point { y: self.pos.y + 1, ..self.pos },
            _ => return key,
        };
        if area.contains(&point) {
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
