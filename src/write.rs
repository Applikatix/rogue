use std::io::{Write, Result};
use crossterm::{queue,
    cursor::MoveTo,
    style::{Print, PrintStyledContent, Stylize, StyledContent},
};
use crate::{points::{Coord, Point},
    GameWorld, MapElem, Room, Path, GameObj, ObjKind,
    Next, Change, Doors,
};

impl GameWorld {
    pub fn init(&self, screen: &mut impl Write) -> Result<()> {
        for area in self.map.iter() {
            area.draw_walls(screen)?;
        }
        for area in self.map.iter() {
            area.draw(screen)?;
        }
        self.player.draw(screen)
    }

    pub fn print(&self, screen: &mut impl Write, next: &Next) -> Result<()> {
        match &next.change {
            Change::Pos(pc) => {
                self.player.area.draw(screen)?;
                pc.draw(screen)
            },
            Change::None => Ok(()),
        }
    }
}

impl MapElem {
    fn draw(&self, screen: &mut impl Write) -> Result<()> {
        match self {
            MapElem::R(room) => room.draw(screen),
            MapElem::P(path) => path.draw(screen),
        }
    }

    fn draw_walls(&self, screen: &mut impl Write) -> Result<()> {
        if let Self::R(room) = self {
            room.draw_walls(screen)?;
        }
        Ok(())
    }
}

impl Room {
    fn draw(&self, screen: &mut impl Write) -> Result<()> {
        for row in self.p1.y..self.p2.y {
            queue!(screen, MoveTo(self.p1.x, row))?;
            for _ in self.p1.x..self.p2.x {
                queue!(screen, PrintStyledContent('.'.dark_grey()))?;
            }
        }
        Ok(())
    }

    fn draw_walls(&self, screen: &mut impl Write) -> Result<()> {
        let Room {
            p1: Point { x, y },
            p2: Point { x: right, y: bottom },
        } = *self;
        let left = x - 1;
        let top = y - 1;

        queue!(screen, MoveTo(left, top), Print('┌'))?;
        for _ in x..right { queue!(screen, Print('─'))?; }
        queue!(screen, Print('┐'))?;

        queue!(screen, MoveTo(left, bottom), Print('└'))?;
        for _ in x..right { queue!(screen, Print('─'))?; }
        queue!(screen, Print('┘'))?;

        for row in y..bottom {
            queue!(screen, MoveTo(left, row), Print('│'))?;
            queue!(screen, MoveTo(right, row), Print('│'))?;
        }

        Ok(())
    }
}

impl Path {
    fn draw(&self, screen: &mut impl Write) -> Result<()> {
        let (p1, p2) = self.end_points();

        match self.c2 {
            Coord::X(cx) => for col in p1.x..cx {
                queue!(screen,
                    MoveTo(col, p1.y),
                    PrintStyledContent('#'.dark_grey()),
                )?;
            },
            Coord::Y(cy) => for row in p1.y..cy {
                queue!(screen,
                    MoveTo(p1.x, row),
                    PrintStyledContent('#'.dark_grey()),
                )?;
            },
        }

        if let Doors::Start | Doors::Both = self.doors {
            queue!(screen,
                MoveTo(p1.x, p1.y),
                PrintStyledContent('∏'.stylize()),
            )?;
        }
        if let Doors::End | Doors::Both = self.doors {
            queue!(screen,
                MoveTo(p2.x, p2.y),
                PrintStyledContent('∏'.stylize()),
            )?;
        }
        Ok(())
    }
}

impl GameObj {
    fn draw(&self, screen: &mut impl Write) -> Result<()> {
        queue!(screen,
            MoveTo(self.pos.x, self.pos.y),
            PrintStyledContent(self.kind.into()),
        )
    }
}

// Logic for converting verious kinds into styled printable character.
impl From<ObjKind> for StyledContent<char> {
    fn from(value: ObjKind) -> Self {
        match value {
            ObjKind::Player => '@'.yellow(),
        }
    }
}
