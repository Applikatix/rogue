use std::ops::{Add, Sub};

pub type Position = Point<u16>;
pub type Coordinate = Coord<u16>;

#[derive(Clone, Copy)]
pub struct Point<N> { pub x: N, pub y: N }

#[derive(Clone, Copy)]
pub enum Coord<N> { X(N), Y(N) }

impl<N: Add<Output = N>> Add for Point<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl<N: Sub<Output = N>> Sub for Point<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point { x: self.x - other.x, y: self.y - other.y }
    }
}

impl<N: Add<Output = N>> Add<Coord<N>> for Point<N> {
    type Output = Self;

    fn add(self, other: Coord<N>) -> Self {
        match other {
            Coord::X(x) => Point { x: self.x + x, ..self },
            Coord::Y(y) => Point { y: self.y + y, ..self },
        }
    }
}

impl<N: Sub<Output = N>> Sub<Coord<N>> for Point<N> {
    type Output = Self;

    fn sub(self, other: Coord<N>) -> Self {
        match other {
            Coord::X(x) => Point { x: self.x - x, ..self },
            Coord::Y(y) => Point { y: self.y - y, ..self },
        }
    }
}

impl<N: Add<Output = N>> Add<N> for Coord<N> {
    type Output = Self;

    fn add(self, n: N) -> Self {
        match self {
            Coord::X(x) => Coord::X(x + n),
            Coord::Y(y) => Coord::Y(y + n),
        }
    }
}

impl<N: Sub<Output = N>> Sub<N> for Coord<N> {
    type Output = Self;

    fn sub(self, n: N) -> Self {
        match self {
            Coord::X(x) => Coord::X(x - n),
            Coord::Y(y) => Coord::Y(y - n),
        }
    }
}
