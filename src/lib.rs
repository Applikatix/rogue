pub mod write;
mod points;
pub mod util;
#[cfg(test)]
mod tests;

use std::collections::HashSet;

use crossterm::event::KeyCode;
use petgraph::{Graph, Undirected, graph::NodeIndex};

use points::{
    Point, Position, Coord::{X, Y}, Coordinate,
    Rect, Space, Long, Straight, Area,
};

pub struct GameWorld {
    player: GameObj,
    map: Map,
    current_area: NodeIndex,
}
impl GameWorld {
    pub fn next(&self, key: KeyCode) -> Next {
        let p = match key {
            KeyCode::Left => self.player.pos - X(1),
            KeyCode::Right => self.player.pos + X(1),
            KeyCode::Up => self.player.pos - Y(1),
            KeyCode::Down => self.player.pos + Y(1),
            _ => return Next(Change::None)
        };

        if self.map[self.current_area].contains(p) {
            return Next(Change::Pos(p));
        }
        for i in self.map.neighbors(self.current_area) {
            if self.map[i].contains(p) {
                return Next(Change::Area(p, i));
            }
        }
        Next(Change::None)
    }

    pub fn update(&mut self, Next(change): Next) {
        if let Change::Pos(pos) = change {
            self.player = self.player.update_pos(pos);
        } else if let Change::Area(pos, area) = change {
            self.player = self.player.update_pos(pos);
            self.current_area = area;
        }
    }
}

struct Map(Graph<MapElem, (), Undirected>);
impl Map {
    fn vis_from(&self, p: Position) -> Submap {
        let mut set = HashSet::new();
        if let Some(i) = self.contains_at_index(p) {
            self.add_vis_areas(&mut set, i);
        }
        set
    }

    fn add_vis_areas(&self, set: &mut Submap, i: NodeIndex) {
        if !set.insert(i) {
            return;
        }
        for area in self.neighbors(i) {
            if let MapElem::Door(_) = self[area] {
                set.insert(area);
            } else {
                self.add_vis_areas(set, area);
            }
        }
    }

    fn rooms(&self) -> impl Iterator<Item = &Space> {
        self.node_weights().filter_map(|area| match area {
            MapElem::Room(room) => Some(room),
            _ => None,
        })
    }

    fn contains_at_index(&self, p: Position) -> Option<NodeIndex> {
        for i in self.node_indices() {
            if self[i].contains(p) {
                return Some(i);
            }
        }
        None
    }
}

impl std::ops::Deref for Map {
    type Target = Graph<MapElem, (), Undirected>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

type Submap = HashSet<NodeIndex>;

#[derive(Clone, Copy, Default)]
pub struct Next(Change);

#[derive(Clone, Copy, Default)]
enum Change {
    None,
    #[default]
    Init,
    Pos(Position),
    Area(Position, NodeIndex),
}

#[derive(Clone, Default)]
enum MapElem {
    Room(Space),
    Hall(Straight),
    Door(Position),
    #[default]
    Void,
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

#[derive(Clone, Copy)]
struct GameObj {
    kind: ObjKind,
    pos: Position,
}

#[derive(Clone, Copy)]
enum ObjKind {
    Player,
}

impl GameObj {
    fn update_pos(&self, pos: Position) -> Self {
        GameObj { pos, ..*self }
    }
}

pub fn custom_world() -> GameWorld {
    let mut map = Map(Graph::new_undirected());

    let a = map.add_node(room(10, 2, 5, 3));
    let b = map.add_node(room(5, 8, 11, 4));
    let c = map.add_node(room(24, 10, 8, 3));
    let d = map.add_node(room(25, 1, 2, 5));
    //let e = map.add_node(room(28, 1, 4, 3));
    let a1 = map.add_node(door(15, 3));
    let b1 = map.add_node(door(16, 9));
    let c1 = map.add_node(door(23, 10));
    let c2 = map.add_node(door(26, 9));
    let d1 = map.add_node(door(26, 6));
    //let de = map.add_node(door(27, 2));
    let p = map.add_node(hall(18, 3, Y(8)));
    let a1p = map.add_node(hall(16, 3, X(2)));
    let b1p = map.add_node(hall(17, 9, X(1)));
    let pc1 = map.add_node(hall(19, 10, X(4)));
    let d1c2 = map.add_node(hall(26, 7, Y(2)));

    map.extend_with_edges(&[
        (a, a1), (b, b1), (c, c1), (c, c2), (d, d1), //(d, de), (de, e),
        (a1, a1p), (a1p, p),
        (b1, b1p), (b1p, p),
        (p, pc1), (pc1, c1),
        (d1, d1c2), (d1c2, c2),
    ]);

    let player = GameObj {
        kind: ObjKind::Player,
        pos: map[a].middle(),
    };

    GameWorld { player, map, current_area: a }
}

fn room(x: u16, y: u16, w: u16, h: u16) -> MapElem {
    MapElem::Room(Rect::new(x, y, w, h))
}

fn hall(x: u16, y: u16, l: Coordinate) -> MapElem {
    MapElem::Hall(Long::new(x, y, l))
}

fn door(x: u16, y: u16) -> MapElem {
    MapElem::Door(Point { x, y })
}

//Line of sight
impl Map {
    fn visible_tiles(&self, p: Position) -> impl Iterator<Item = Tile> + '_ {
        let mut tiles = vec![self.get_tile(p)];

        for walk in [
            |p: Position| Point { x: p.x + 1, ..p },
            |p: Position| Point { x: p.x - 1, ..p },
            |p: Position| Point { y: p.y + 1, ..p },
            |p: Position| Point { y: p.y - 1, ..p },
            |p: Position| Point { x: p.x + 1, y: p.y + 1 },
            |p: Position| Point { x: p.x + 1, y: p.y - 1 },
            |p: Position| Point { x: p.x - 1, y: p.y + 1 },
            |p: Position| Point { x: p.x - 1, y: p.y - 1 },
        ] {
            let mut pos = walk(p);
            while let Some(kind) = self.contains_kind(pos) {
                tiles.push(Tile { pos, kind });
                pos = walk(pos);
            }
        }
        tiles.into_iter()
    }

    fn get_tile(&self, p: Position) -> Tile {
        for area in self.node_weights() {
            if area.contains(p) {
                return Tile { pos: p, kind: area.into() };
            }
        }
        Tile { pos: p, kind: TileKind::Void }
    }

    fn contains_kind(&self, p: Position) -> Option<TileKind> {
        for area in self.node_weights() {
            if area.contains(p) {
                return Some(area.into());
            }
        }
        None
    }
}

struct Tile {
    pos: Position,
    kind: TileKind,
}

enum TileKind {
    Void,
    Room,
    Hall,
    Door,
    _Wall,
}
impl From<&MapElem> for TileKind {
    fn from(area: &MapElem) -> Self {
        match area {
            MapElem::Room(_) => Self::Room,
            MapElem::Hall(_) => Self::Hall,
            MapElem::Door(_) => Self::Door,
            MapElem::Void => Self::Void,
        }
    }
}
