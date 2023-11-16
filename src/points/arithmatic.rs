use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

use super::{Point, Coord::{self, *}, Move, Position};

//Point N

// #Point N
impl<N: Add<Output = N>> Add for Point<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
} impl<N: Add<Output = N> + Copy> AddAssign for Point<N> {
    fn add_assign(&mut self, other: Self) {
        *self = Self { x: self.x + other.x, y: self.y + other.y };
    }
}
impl<N: Sub<Output = N>> Sub for Point<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y }
    }
} impl<N: Sub<Output = N> + Copy> SubAssign for Point<N> {
    fn sub_assign(&mut self, other: Self) {
        *self = Self { x: self.x - other.x, y: self.y - other.y };
    }
}

// #Coord N
impl<N: Add<Output = N>> Add<Coord<N>> for Point<N> {
    type Output = Self;

    fn add(self, coord: Coord<N>) -> Self { match coord {
        X(x) => Self { x: self.x + x, ..self },
        Y(y) => Self { y: self.y + y, ..self },
    }}
} impl<N: Add<Output = N> + Copy> AddAssign<Coord<N>> for Point<N> {
    fn add_assign(&mut self, coord: Coord<N>) {
        *self = *self + coord;
    }
}
impl<N: Sub<Output = N>> Sub<Coord<N>> for Point<N> {
    type Output = Self;

    fn sub(self, coord: Coord<N>) -> Self { match coord {
        X(x) => Self { x: self.x - x, ..self },
        Y(y) => Self { y: self.y - y, ..self },
    }}
} impl<N: Sub<Output = N> + Copy> SubAssign<Coord<N>> for Point<N> {
    fn sub_assign(&mut self, coord: Coord<N>) {
        *self = *self - coord;
    }
}

// #N
impl<N: Mul<Output = N> + Copy> Mul<N> for Point<N> {
    type Output = Self;

    fn mul(self, n: N) -> Self {
        Self { x: self.x * n, y: self.y * n }
    }
} impl<N: Mul<Output = N> + Copy> MulAssign<N> for Point<N> {
    fn mul_assign(&mut self, n: N) {
        *self = Self { x: self.x * n, y: self.y * n };
    }
}
impl<N: Div<Output = N> + Copy> Div<N> for Point<N> {
    type Output = Self;

    fn div(self, n: N) -> Self {
        Self { x: self.x / n, y: self.y / n }
    }
} impl<N: Div<Output = N> + Copy> DivAssign<N> for Point<N> {
    fn div_assign(&mut self, n: N) {
        *self = Self { x: self.x / n, y: self.y / n };
    }
}

//Position

// #Move
impl Add<Move> for Position {
    type Output = Self;

    fn add(self, r#move: Move) -> Self {
        r#move.r#move(self)
    }
} impl AddAssign<Move> for Position {
    fn add_assign(&mut self, r#move: Move) {
        *self = r#move.r#move(*self);
    }
}

//Coord N

// #N
impl<N: Add<Output = N>> Add<N> for Coord<N> {
    type Output = Self;

    fn add(self, n: N) -> Self { match self {
        X(x) => X(x + n),
        Y(y) => Y(y + n),
    }}
} impl<N: Add<Output = N> + Copy> AddAssign<N> for Coord<N> {
    fn add_assign(&mut self, n: N) {
        *self = *self + n;
    }
}
impl<N: Sub<Output = N>> Sub<N> for Coord<N> {
    type Output = Self;

    fn sub(self, n: N) -> Self { match self {
        X(x) => X(x - n),
        Y(y) => Y(y - n),
    }}
} impl<N: Sub<Output = N> + Copy> SubAssign<N> for Coord<N> {
    fn sub_assign(&mut self, n: N) {
        *self = *self - n;
    }
}
impl<N: Mul<Output = N>> Mul<N> for Coord<N> {
    type Output = Self;

    fn mul(self, n: N) -> Self { match self {
        X(x) => X(x * n),
        Y(y) => Y(y * n),
    }}
} impl<N: Mul<Output = N> + Copy> MulAssign<N> for Coord<N> {
    fn mul_assign(&mut self, n: N) {
        *self = *self * n;
    }
}
impl<N: Div<Output = N>> Div<N> for Coord<N> {
    type Output = Self;

    fn div(self, n: N) -> Self { match self {
        X(x) => X(x / n),
        Y(y) => Y(y / n),
    }}
} impl<N: Div<Output = N> + Copy> DivAssign<N> for Coord<N> {
    fn div_assign(&mut self, n: N) {
        *self = *self / n;
    }
}
