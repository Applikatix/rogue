pub mod write;
mod points;
pub mod util;
#[cfg(test)]
mod tests;

use std::rc::Rc;
use crossterm::event::KeyCode;

use points::{Position, Coord, Coordinate, Point};
use util::average;

pub fn create_world() -> GameWorld {
    let map = vec![
        Rc::new(Room::new(10, 5, 5, 3)),
        Rc::new(Path::new(15, 6, Coord::X(5), Doors::Start)),
        Rc::new(Path::new(20, 6, Coord::Y(4), Doors::End)),
        Rc::new(Room::new(15, 10, 10, 4)),
    ];

    let player = GameObj {
        kind: ObjKind::Player,
        pos: map[0].middle(),
        area: Rc::clone(&map[0]),
    };

    GameWorld { player, map }
}

pub struct GameWorld {
    player: GameObj,
    map: Vec<Rc<MapElem>>,
}

impl GameWorld {
    pub fn next(&self, key: KeyCode) -> Next {
        let pos = match key {
            KeyCode::Left => self.player.pos - Coord::X(1),
            KeyCode::Right => self.player.pos + Coord::X(1),
            KeyCode::Up => self.player.pos - Coord::Y(1),
            KeyCode::Down => self.player.pos + Coord::Y(1),
            _ => return Next::from(Change::None),
        };

        match self.contains(pos) {
            Some(area) => Next::from(Change::Pos(
                GameObj { pos, area, ..self.player }
            )),
            None => Next::from(Change::None),
        }
    }

    pub fn update(&mut self, next: Next) {
        if let Change::Pos(player) = Change::from(next) {
            self.player = player;
        }
    }

    fn contains(&self, p: Position) -> Option<Rc<MapElem>> {
        for area in self.map.iter() {
            if area.contains(p) {
                return Some(Rc::clone(area));
            }
        }
        None
    }
}

pub struct Next{
    change: Change,
}

enum Change {
    Pos(GameObj),
    None
}

impl From<Change> for Next {
    fn from(change: Change) -> Self {
        Next { change }
    }
}

impl From<Next> for Change {
    fn from(next: Next) -> Self {
        next.change
    }
}

#[derive(Clone)]
struct GameObj {
    kind: ObjKind,
    pos: Position,
    area: Rc<MapElem>,
}

#[derive(Clone, Copy)]
enum ObjKind {
    Player,
}

// Map elements/walkable areas.
enum MapElem {
    R(Room),
    P(Path),
}

struct Room { p1: Position, p2: Position }
struct Path { p1: Position, c2: Coordinate, doors: Doors }

#[derive(Clone, Copy)]
enum Doors { None, Start, End, Both }

impl Room {
    fn new(x: u16, width: u16, y: u16, height: u16) -> MapElem {
        MapElem::R(Self {
            p1: Point { x, y },
            p2: Point { x: x + width, y: y + height },
        })
    }
}

impl Path {
    fn new(x: u16, y: u16, length: Coordinate, doors: Doors) -> MapElem {
        MapElem::P(match length {
            Coord::X(cx) => Self {
                p1: Point { x, y }, c2: Coord::X(x + cx), doors,
            },
            Coord::Y(cy) => Self {
                p1: Point { x, y }, c2: Coord::Y(y + cy), doors,
            },
        })
    }

    fn end_points(&self) -> (Position, Position) {
        match self.c2 {
            Coord::X(cx) => (self.p1, Point { x: cx - 1, ..self.p1 }),
            Coord::Y(cy) => (self.p1, Point { y: cy - 1, ..self.p1 }),
        }
    }
}

impl MapElem {
    fn contains(&self, p: Position) -> bool {
        match self {
            Self::R(Room { p1, p2 }) =>
                p1.x <= p.x && p.x < p2.x &&
                p1.y <= p.y && p.y < p2.y,
            Self::P(Path { p1, c2, doors: _ }) =>
            match c2 {
                Coord::X(cx) => p1.y == p.y && p1.x <= p.x && p.x < *cx,
                Coord::Y(cy) => p1.x == p.x && p1.y <= p.y && p.y < *cy,
            },
        }
    }

    fn middle(&self) -> Position {
        match self {
            Self::R(Room { p1, p2 }) => Point {
                x: average(p1.x, p2.x),
                y: average(p1.y, p2.y),
            },
            Self::P(Path { p1, c2, doors: _ }) =>
            match c2 {
                Coord::X(cx) => Point { x: average(p1.x, *cx), ..*p1 },
                Coord::Y(cy) => Point { y: average(p1.y, *cy), ..*p1 },
            },
        }
    }
}
