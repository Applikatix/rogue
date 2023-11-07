mod arithmatic;

use std::{ops::Add, iter};

use crate::util::average;

pub type Space = Rect<u16>;
pub type Straight = Long<u16>;
pub type Position = Point<u16>;
pub type Coordinate = Coord<u16>;

#[derive(Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Point<N> { pub x: N, pub y: N }

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Coord<N> { X(N), Y(N) }

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Rect<N>{ pub p1: Point<N>, pub p2: Point<N> }

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Long<N>{ pub p1: Point<N>, pub c2: Coord<N> }

//Convenience functions for creating rooms and paths.
impl<N: Add<Output = N> + Copy> Rect<N> {
    pub fn new(x: N, y: N, width: N, height: N) -> Rect<N> {
        Self {
            p1: Point { x, y },
            p2: Point { x: x + width, y: y + height },
        }
    }
}

impl<N: Add<Output = N> + Copy> Long<N> {
    pub fn new(x: N, y: N, length: Coord<N>) -> Long<N> {
        match length {
            Coord::X(cx) => Self { p1: Point { x, y }, c2: Coord::X(x + cx) },
            Coord::Y(cy) => Self { p1: Point { x, y }, c2: Coord::Y(y + cy) },
        }
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
        let Rect { p1, p2 } = self;

        p1.x <= p.x && p.x < p2.x &&
        p1.y <= p.y && p.y < p2.y
    }

    fn middle(&self) -> Position {
        let Rect { p1, p2 } = self;

        Point {
            x: average(p1.x, p2.x),
            y: average(p1.y, p2.y),
        }
    }
}

impl Area for Straight {
    fn contains(&self, p: Position) -> bool {
        let Long { p1, c2 } = *self;

        match c2 {
            Coord::X(cx) => p1.y == p.y && p1.x <= p.x && p.x < cx,
            Coord::Y(cy) => p1.x == p.x && p1.y <= p.y && p.y < cy,
        }
    }

    fn middle(&self) -> Position {
        let Long { p1, c2 } = *self;
        
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
}
impl Iterator for Strip {
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
}
impl<T: Copy> IntoIterator for &Point<T> {
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
        RectPoints { next: self.p1, left: self.p1.x, end: self.p2 }
    }
}
impl IntoIterator for &Space {
    type Item = Position;
    type IntoIter = RectPoints;

    fn into_iter(self) -> Self::IntoIter {
        RectPoints { next: self.p1, left: self.p1.x, end: self.p2 }
    }
}

impl Points for Space {
    type StripsIter = RectStrips;

    fn strips(self) -> Self::StripsIter {
        RectStrips { next: self.p1, end: self.p2 }
    }
}

pub struct RectPoints {
    next: Position,
    left: u16,
    end: Position,
}
impl Iterator for RectPoints {
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

pub struct RectStrips {
    next: Position,
    end: Position,
}
impl Iterator for RectStrips {
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
        LongPoints { next: self.p1, end: self.c2 }
    }
}
impl IntoIterator for &Straight {
    type Item = Position;
    type IntoIter = LongPoints;

    fn into_iter(self) -> Self::IntoIter {
        LongPoints { next: self.p1, end: self.c2 }
    }
}

impl Points for Straight {
    type StripsIter = LongStrips;

    fn strips(self) -> Self::StripsIter {
        LongStrips { next: self.p1, end: Some(self.c2) }
    }
}

pub struct LongPoints {
    next: Position,
    end: Coordinate,
}
impl Iterator for LongPoints {
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

pub struct LongStrips {
    next: Position,
    end: Option<Coordinate>,
}
impl Iterator for LongStrips {
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
