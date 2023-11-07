use std::ops::{Add, Sub, Mul, Div};

use super::{Point, Coord};

//Point
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

impl<N: Mul<Output = N> + Copy> Mul<N> for Point<N> {
    type Output = Self;

    fn mul(self, n: N) -> Self::Output {
        Point { x: self.x * n, y: self.y * n }
    }
}

impl<N: Div<Output = N> + Copy> Div<N> for Point<N> {
    type Output = Self;

    fn div(self, n: N) -> Self::Output {
        Point { x: self.x / n, y: self.y / n }
    }
}

//Coord
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

impl<N: Mul<Output = N>> Mul<N> for Coord<N> {
    type Output = Self;

    fn mul(self, n: N) -> Self::Output {
        match self {
            Coord::X(x) => Coord::X(x * n),
            Coord::Y(y) => Coord::Y(y * n),
        }
    }
}

impl<N: Div<Output = N>> Div<N> for Coord<N> {
    type Output = Self;

    fn div(self, n: N) -> Self::Output {
        match self {
            Coord::X(x) => Coord::X(x / n),
            Coord::Y(y) => Coord::Y(y / n),
        }
    }
}
