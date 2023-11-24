pub mod write;
mod fov;
mod points;
pub mod util;

use std::collections::HashMap;

use crossterm::event::KeyCode;
use petgraph::{Graph, Undirected, graph::NodeIndex};

use points::{ Point, Position, Coord::{X, Y, self}, Coordinate,
    Rect, Space, Line, Straight, Area,
};
use strum_macros::EnumIs;

pub struct GameWorld {
    map: Map,
    player: Player,
} impl GameWorld {
    pub fn next(&self, key: KeyCode) -> Next {
        let p = match key {
            KeyCode::Left => self.player.pos - X(1),
            KeyCode::Right => self.player.pos + X(1),
            KeyCode::Up => self.player.pos - Y(1),
            KeyCode::Down => self.player.pos + Y(1),
            _ => return Next(Change::Nothing)
        };

        if self.map[self.player.area].contains(p) {
            return Next(Change::Pos(p));
        }
        for i in self.map.neighbors(self.player.area) {
            if self.map[i].contains(p) {
                return Next(Change::Area(p, i));
            }
        }
        Next(Change::Nothing)
    }

    pub fn update(&mut self, Next(change): Next) {
        match change {
            Change::Pos(pos) => {
                self.player.pos = pos;
            }
            Change::Area(pos, area) => {
                self.player.pos = pos;
                self.player.area = area;
            }
            _ => {}
        }
    }
}

struct Player {
    pos: Position,
    area: NodeIndex,
    //Health, inventory, state, etc.
}

#[derive(Clone, Copy, Default)]
pub struct Next(Change);
#[derive(Clone, Copy, Default)]
enum Change {
    Nothing,
    #[default]
    Init,
    Pos(Position),
    Area(Position, NodeIndex),
}

struct Map(Graph<MapElem, (), Undirected>); impl Map {
    fn visible_tiles(&self, p: Position) -> TileMap {
        let mut tiles = HashMap::new();

        let is_floor = |pos| {
            if let Some(tile) = self.contains_tile(pos) {
                if tile.is_clear() {
                    return true;
                }
            }
            false
        };
        let add_visible = |pos| {
            if let Some(tile) = self.get_tile(pos) {
                tiles.insert(pos, tile);
            }
        };

        fov::compute(p, is_floor, add_visible);
        tiles.insert(p, TileKind::Obj(ObjKind::Player));

        tiles.into()
    }

    fn get_tile(&self, p: Position) -> Option<TileKind> {
        for area in self.node_weights() {
            if area.contains(p) {
                return Some(area.into());
            }
        }
        for wall in self.walls() {
            if wall.contains(p) {
                return Some(TileKind::Wall(wall.c2.into()));
            }
        }
        None
    }

    fn contains_tile(&self, p: Position) -> Option<TileKind> {
        for area in self.node_weights() {
            if area.contains(p) {
                return Some(area.into());
            }
        }
        None
    }

    fn walls(&self) -> Vec<Straight> {
        let mut walls = Vec::new();

        for room in self.rooms() {
            let Rect {
                p1: Point { x, y },
                p2: Point { x: right, y: bottom },
            } = *room;
            let (left, top) = (x - 1, y - 1);

            walls.push(Line { p1: Point { x, y: top }, c2: X(right) });
            walls.push(Line { p1: Point { x, y: bottom }, c2: X(right) });
            walls.push(Line {
                p1: Point { x: left, y: top },
                c2: Y(bottom + 1)
            });
            walls.push(Line {
                p1: Point { x: right, y: top },
                c2: Y(bottom + 1)
            });
        }

        walls
    }
    
    fn rooms(&self) -> impl Iterator<Item = &Space> {
        self.node_weights().filter_map(|area| match area {
            MapElem::Room(room) => Some(room),
            _ => None,
        })
    }
}

struct TileMap(HashMap<Position, TileKind>); impl TileMap {
    fn tiles(&self) -> impl Iterator<Item = Tile> + '_ {
        self.iter().map(|t| t.into())
    }

    fn difference<'a>(&'a self, other: &'a TileMap) -> Difference {
        Difference { iter: self.iter(), other  }
    }

    fn difference_player<'a>(&'a self, other: &'a TileMap) -> DifferenceKind {
        DifferenceKind { iter: self.iter(), other  }
    }
}
struct Tile {
    pos: Position,
    kind: TileKind,
}
#[derive(Clone, Copy, PartialEq, EnumIs)]
enum TileKind {
    Void,
    Obj(ObjKind),
    Door,
    Room,
    Hall(Dir),
    Wall(Dir),
} impl TileKind {
    fn is_clear(&self) -> bool {
        self.is_room() || self.is_hall() || self.is_obj()
    }
}
#[derive(Clone, Copy, PartialEq)]
enum ObjKind {
    Player,
}
#[derive(Clone, Copy, PartialEq)]
enum Dir { //None,
    //Up, Down, Left, Right,
    Hor, Ver, //UL, UR, DL, DR,
    //UHor, DHor, VerL, VerR,
    //All,
}

#[derive(Default)]
enum MapElem {
    #[default]
    Void,
    Room(Space),
    Hall(Straight),
    Door(Position),
}
impl Area for MapElem {
    fn contains(&self, p: Position) -> bool {
        match self {
            MapElem::Room(room) => room.contains(p),
            MapElem::Hall(hall) => hall.contains(p),
            MapElem::Door(door) => door.contains(p),
            _ => false,
        }
    }

    fn middle(&self) -> Position {
        match self {
            MapElem::Room(room) => room.middle(),
            MapElem::Hall(hall) => hall.middle(),
            MapElem::Door(door) => door.middle(),
            _ => panic!("out of bounds"),
        }
    }
}

//Mapgeneration

///Generates a premade gameworld. Will implement random level generation later.
pub fn custom_world() -> GameWorld {
    let mut map = Map(Graph::new_undirected());

    let a = map.add_node(room(10, 2, 5, 3));
    let b = map.add_node(room(5, 8, 11, 4));
    let c = map.add_node(room(24, 10, 8, 2));
    let d = map.add_node(room(25, 1, 2, 5));
    let e = map.add_node(room(28, 1, 4, 3));
    let a1 = map.add_node(door(15, 3));
    let b1 = map.add_node(door(16, 9));
    let c1 = map.add_node(door(23, 10));
    let c2 = map.add_node(door(26, 9));
    let d1 = map.add_node(door(26, 6));
    let de = map.add_node(door(27, 2));
    let p = map.add_node(hall(18, 3, Y(8)));
    let a1p = map.add_node(hall(16, 3, X(2)));
    let b1p = map.add_node(hall(17, 9, X(1)));
    let pc1 = map.add_node(hall(19, 10, X(4)));
    let d1c2 = map.add_node(hall(26, 7, Y(2)));

    map.extend_with_edges(&[
        (a, a1), (b, b1), (c, c1), (c, c2), (d, d1), (d, de), (de, e),
        (a1, a1p), (a1p, p),
        (b1, b1p), (b1p, p),
        (p, pc1), (pc1, c1),
        (d1, d1c2), (d1c2, c2),
    ]);

    let player = Player { pos: map[a].middle(), area: a };

    GameWorld { player, map }
}

fn room(x: u16, y: u16, w: u16, h: u16) -> MapElem {
    MapElem::Room(Rect::new(x, y, w, h))
}
fn hall(x: u16, y: u16, l: Coordinate) -> MapElem {
    MapElem::Hall(Line::new(x, y, l))
}
fn door(x: u16, y: u16) -> MapElem {
    MapElem::Door(Point { x, y })
}

// #Diverse implementeringer

//From
impl From<HashMap<Position, TileKind>> for TileMap {
    fn from(hashmap: HashMap<Position, TileKind>) -> Self {
        Self(hashmap)
    }
}

impl From<(&Position, &TileKind)> for Tile {
    fn from((pos, kind): (&Position, &TileKind)) -> Self {
        Self { pos: *pos, kind: *kind }
    }
}

impl From<&MapElem> for TileKind {
    fn from(area: &MapElem) -> Self {
        match area {
            MapElem::Room(_) => Self::Room,
            MapElem::Hall(hall) => Self::Hall(hall.c2.into()),
            MapElem::Door(_) => Self::Door,
            MapElem::Void => Self::Void,
        }
    }
}
impl<N> From<Coord<N>> for Dir {
    fn from(coord: Coord<N>) -> Self {
        match coord {
            X(_) => Dir::Hor,
            Y(_) => Dir::Ver,
        }
    }
}

//Dereferencing
use std::ops::{Deref, DerefMut};

impl Deref for Map {
    type Target = Graph<MapElem, (), Undirected>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
} impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for TileMap {
    type Target = HashMap<Position, TileKind>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
} impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

//Iterators
use std::collections::hash_map::Iter;

struct Difference<'a> {
    iter: Iter<'a, Position, TileKind>,
    other: &'a TileMap,
} impl Iterator for Difference<'_> {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> { loop {
        let kv = self.iter.next()?;
        if self.other.contains_key(kv.0) {
            continue;
        }
        return Some(kv.into());
    }}
}

struct DifferenceKind<'a> {
    iter: Iter<'a, Position, TileKind>,
    other: &'a TileMap,
} impl Iterator for DifferenceKind<'_> {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> { loop {
        let kv = self.iter.next()?;
        if let Some(v) = self.other.get(kv.0) {
            if !kv.1.is_obj() && !v.is_obj() {
                continue;
            }
        }
        return Some(kv.into());
    }}
}
