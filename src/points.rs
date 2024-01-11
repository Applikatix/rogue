pub mod arithmatic;

use std::{ops::{Add, Sub}, iter};

use crate::util::average;

pub type Space = Rect<u16>;
pub type Straight = Line<u16>;
pub type Position = Point<u16>;
pub type Coordinate = Coord<u16>;

#[derive(Clone, Copy, PartialEq, Eq, Default, Hash, Debug)]
pub struct Point<N> { pub x: N, pub y: N }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Coord<N> { X(N), Y(N) }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rect<N>{ pub pos: Point<N>, pub end: Point<N> }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Line<N>{ pub pos: Point<N>, pub end: Coord<N> }

//Convenience functions

// #Properties
impl<N: Sub<Output = N> + Copy> Rect<N> {
    pub fn size(&self) -> Point<N> {
        Point { x: self.end.x - self.pos.x, y: self.end.y - self.pos.y }
    }
    pub fn width(&self) -> N { self.end.x - self.pos.x }
    pub fn height(&self) -> N { self.end.y - self.pos.y }
}

impl<N: Sub<Output = N> + Copy> Line<N> {
    pub fn len(&self) -> N {
        match self.end {
            Coord::X(end) => end - self.pos.x,
            Coord::Y(end) => end - self.pos.y,
        }
    }
}

// #Creating new instances from existing instances
impl<N> Point<N> {
    pub fn x(self, x: N) -> Self { Point { x, ..self } }
    pub fn y(self, y: N) -> Self { Point { y, ..self } }
}

impl<N> Rect<N> {
    pub fn pos(self, pos: Point<N>) -> Self { Rect { pos, ..self } }
    pub fn end(self, end: Point<N>) -> Self { Rect { end, ..self } }
    pub fn pos_x(self, x: N) -> Self { Rect { pos: self.pos.x(x), ..self } }
    pub fn pos_y(self, y: N) -> Self { Rect { pos: self.pos.y(y), ..self } }
    pub fn end_x(self, x: N) -> Self { Rect { end: self.end.x(x), ..self } }
    pub fn end_y(self, y: N) -> Self { Rect { end: self.end.y(y), ..self } }
}

// #Creating rooms and paths.
impl<N: Add<Output = N> + Copy> Rect<N> {
    pub fn new(x: N, y: N, width: N, height: N) -> Self {
        Self {
            pos: Point { x, y },
            end: Point { x: x + width, y: y + height },
        }
    }
    pub fn new_exact(x: N, y: N, endx: N, endy: N) -> Self {
        Self { pos: Point { x, y }, end: Point { x: endx, y: endy } }
    }
}

impl<N: Add<Output = N> + Copy> Line<N> {
    pub fn new(x: N, y: N, length: Coord<N>) -> Self { match length {
        Coord::X(cx) => Self { pos: Point { x, y }, end: Coord::X(x + cx) },
        Coord::Y(cy) => Self { pos: Point { x, y }, end: Coord::Y(y + cy) },
    } }
    pub fn new_exact(x: N, y: N, end: Coord<N>) -> Self {
        Self { pos: Point { x, y }, end }
    }
}

//Conversion to and from tuples
impl<N> From<(N, N)> for Point<N> {
    fn from((x, y): (N, N)) -> Self {
        Point { x, y }
    }
} impl<N> From<Point<N>> for (N, N) {
    fn from(p: Point<N>) -> Self {
        (p.x, p.y)
    }
}

//Moving position
use {strum::IntoEnumIterator, strum_macros::EnumIter};
use Move::*;
#[derive(Clone, Copy, EnumIter)]
pub enum Move { Up, Down, Left, Right, LU, LD, RU, RD } impl Move {
    fn r#move(self, p: Position) -> Position {
        match self {
            Up => Point { y: p.y - 1, ..p },
            Down => Point { y: p.y + 1, ..p },
            Left => Point { x: p.x - 1, ..p },
            Right => Point { x: p.x + 1, ..p },
            LU => Point { x: p.x - 1, y: p.y - 1 },
            LD => Point { x: p.x - 1, y: p.y + 1 },
            RU => Point { x: p.x + 1, y: p.y - 1 },
            RD => Point { x: p.x + 1, y: p.y + 1 },
        }
    }

    pub fn mover(self) -> impl Fn(Position) -> Position {
        move |p| self.r#move(p)
    }

    pub fn movers() -> impl Iterator<Item = impl Fn(Position) -> Position> {
        Self::iter().map(|m| m.mover())
    }
}

/// Trait representing an area.
pub trait Area {
    fn contains(&self, p: Position) -> bool;
    fn middle(&self) -> Position;
}

impl Area for Position {
    fn contains(&self, p: Position) -> bool {
        *self == p
    }

    fn middle(&self) -> Position {
        *self
    }
}

impl Area for Space {
    fn contains(&self, p: Position) -> bool {
        let Rect { pos: p1, end: p2 } = self;

        p1.x <= p.x && p.x < p2.x &&
        p1.y <= p.y && p.y < p2.y
    }

    fn middle(&self) -> Position {
        let Rect { pos: p1, end: p2 } = self;

        Point {
            x: average(p1.x, p2.x),
            y: average(p1.y, p2.y),
        }
    }
}

impl Area for Straight {
    fn contains(&self, p: Position) -> bool {
        let Line { pos: p1, end: c2 } = *self;

        match c2 {
            Coord::X(cx) => p1.y == p.y && p1.x <= p.x && p.x < cx,
            Coord::Y(cy) => p1.x == p.x && p1.y <= p.y && p.y < cy,
        }
    }

    fn middle(&self) -> Position {
        let Line { pos: p1, end: c2 } = *self;
        
        match c2 {
            Coord::X(cx) => Point { x: average(p1.x, cx), ..p1 },
            Coord::Y(cy) => Point { y: average(p1.y, cy), ..p1 },
        }
    }
}

// # Point Iterators
pub trait Points: Area + IntoIterator<Item = Position> {
    type StripsIter: Iterator<Item = Strip>;

    fn strips(self) -> Self::StripsIter;
}

pub struct Strip {
    pub next: Position,
    end: u16,
} impl Iterator for Strip {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.x < self.end {
            let res = Some(self.next);
            self.next.x += 1;
            return res;
        }
        None
    }
}

//Point
impl<T> IntoIterator for Point<T> {
    type Item = Self;
    type IntoIter = iter::Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
} impl<T: Copy> IntoIterator for &Point<T> {
    type Item = Point<T>;
    type IntoIter = iter::Once<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        iter::once(*self)
    }
}

impl Points for Position {
    type StripsIter = iter::Once<Strip>;

    fn strips(self) -> Self::StripsIter {
        iter::once(Strip { next: self, end: self.x + 1 })
    }
}

//Space
impl IntoIterator for Space {
    type Item = Position;
    type IntoIter = RectPoints;

    fn into_iter(self) -> Self::IntoIter {
        RectPoints { next: self.pos, left: self.pos.x, end: self.end }
    }
} impl IntoIterator for &Space {
    type Item = Position;
    type IntoIter = RectPoints;

    fn into_iter(self) -> Self::IntoIter {
        RectPoints { next: self.pos, left: self.pos.x, end: self.end }
    }
}
pub struct RectPoints {
    next: Position,
    left: u16,
    end: Position,
} impl Iterator for RectPoints {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let RectPoints { next: current,
            left, end,
        } = *self;

        if current.x < end.x {
            self.next.x += 1;
            return Some(current);
        }
        let current = Point { x: left, y: current.y + 1 };
        if current.y < end.y {
            self.next = Point { x: current.x + 1, ..current };
            return Some(current);
        }
        None
    }
}

impl Points for Space {
    type StripsIter = RectStrips;

    fn strips(self) -> Self::StripsIter {
        RectStrips { next: self.pos, end: self.end }
    }
} 
pub struct RectStrips {
    next: Position,
    end: Position,
} impl Iterator for RectStrips {
    type Item = Strip;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.y < self.end.y {
            let strip = Strip { next: self.next, end: self.end.x };
            self.next.y += 1;
            return Some(strip);
        }
        None
    }
}

//Straight
impl IntoIterator for Straight {
    type Item = Position;
    type IntoIter = LongPoints;

    fn into_iter(self) -> Self::IntoIter {
        LongPoints { next: self.pos, end: self.end }
    }
} impl IntoIterator for &Straight {
    type Item = Position;
    type IntoIter = LongPoints;

    fn into_iter(self) -> Self::IntoIter {
        LongPoints { next: self.pos, end: self.end }
    }
}
pub struct LongPoints {
    next: Position,
    end: Coordinate,
} impl Iterator for LongPoints {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next;
        match self.end {
            Coord::X(cx) => if current.x < cx {
                self.next.x += 1;
                return Some(current);
            },
            Coord::Y(cy) => if current.y < cy {
                self.next.y += 1;
                return Some(current);
            },
        }
        None
    }
}

impl Points for Straight {
    type StripsIter = LongStrips;

    fn strips(self) -> Self::StripsIter {
        LongStrips { next: self.pos, end: Some(self.end) }
    }
}
pub struct LongStrips {
    next: Position,
    end: Option<Coordinate>,
} impl Iterator for LongStrips {
    type Item = Strip;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next;
        match self.end {
            Some(Coord::X(cx)) => {
                self.end = None;
                Some(Strip { next: current, end: cx })
            }
            Some(Coord::Y(cy)) if current.y < cy => {
                self.next.y += 1;
                Some(Strip { next: current, end: current.x + 1 })
            }
            _ => None,
        }
    }
}
