use num_rational::Ratio;
use {strum::IntoEnumIterator, strum_macros::EnumIter};

use crate::points::{Position, Point};

/// A position relative to the origin (0, 0) where x >= y.
type Relative = Position;
/*
    3 |       X
    2 |     X X
    1 |   X X X
    0 | @ X X X
    y + - - - -
      x 0 1 2 3
*/
type Slope = Ratio<u16>;

/// Shadowcasting FOV function.
pub fn compute(
    origin: Position,
    is_floor: impl Fn(Position) -> bool,
    mut add_visible: impl FnMut(Position),
) {
    add_visible(origin);
    for oct in Octant::iter_from(origin) {
        let mut columns = vec![Col::first()];
        'columns: while let Some(col) = columns.pop() {
            use TileType::*;
            let mut prev = NA;
            for rel in col.points() {
                let abs = oct.absolute(rel);
                let curr = if is_floor(abs) { Floor } else { Wall };

                if curr == Wall || col.is_symmetric(rel) {
                    add_visible(abs);
                }
                if prev == Floor && curr == Wall {
                    columns.push(col.top_slope(rel).next());
                } else if prev == Wall && curr == Floor {
                    columns.push(col.bottom_slope(rel));
                    continue 'columns;
                }
                prev = curr;
            }
            if prev == Floor {
                columns.push(col.next());
            }
        }
    }
    #[derive(PartialEq)]
    enum TileType { Wall, Floor, NA }
}

//Octants
struct Octant {
    origin: Position,
    section: OctSection,
} impl Octant {
    fn iter_from(origin: Position) -> impl Iterator<Item = Octant> {
        OctSection::iter()
            .map(move |section| Octant { origin, section })
    }

    fn absolute(&self, rel: Relative) -> Position {
        let Point { x: ox, y: oy } = self.origin;

        use OctSection::*;
        match self.section {
            LeftUp => Point { x: ox - rel.x, y: oy - rel.y },
            LeftDown => Point { x: ox - rel.x, y: oy + rel.y },
            RightUp => Point { x: ox + rel.x, y: oy - rel.y },
            RightDown => Point { x: ox + rel.x, y: oy + rel.y },
            UpLeft => Point { x: ox - rel.y, y: oy - rel.x },
            UpRight => Point { x: ox + rel.y, y: oy - rel.x },
            DownLeft => Point { x: ox - rel.y, y: oy + rel.x },
            DownRight => Point { x: ox + rel.y, y: oy + rel.x },
        }
    }
}
#[derive(EnumIter)]
enum OctSection {
    LeftUp, LeftDown,
    RightUp, RightDown,
    UpLeft, UpRight,
    DownLeft, DownRight,
}

/// Represents a row in an octant.
struct Col {
    depth: u16,
    top_slope: Slope,
    bottom_slope: Slope,
} impl Col {
    fn first() -> Self {
        Self { depth: 1, top_slope: 1.into(), bottom_slope: 0.into() }
    }

    fn next(&self) -> Self {
        Self { depth: self.depth + 1, ..*self }
    }

    fn top_slope(&self, rel: Relative) -> Self {
        Self { top_slope: slope(rel), ..*self }
    }

    fn bottom_slope(&self, rel: Relative) -> Self {
        Self { bottom_slope: slope(rel), ..*self }
    }

    fn points(&self) -> impl Iterator<Item = Relative> + '_ {
        let upper = round_half_down(self.top_slope * self.depth);
        let lower = round_half_up(self.bottom_slope * self.depth);
        (lower..=upper).map(|y| Point { x: self.depth, y })
    }

    fn is_symmetric(&self, rel: Relative) -> bool {
        let row = Ratio::from(rel.y);
    
        row >= self.bottom_slope * self.depth &&
        row <= self.top_slope * self.depth
    }
}

// Convenience functions
/// Calculate new slope from bottom of a tile.
fn slope(p: Relative) -> Slope {
    Ratio::new(2 * p.y - 1, 2 * p.x)
}

const HALF: Ratio<u16> = Ratio::new_raw(1, 2);

fn round_half_down(n: Ratio<u16>) -> u16 {
    (n - HALF).ceil().to_integer()
}

fn round_half_up(n: Ratio<u16>) -> u16 {
    (n + HALF).floor().to_integer()
}

/// Simple FOV function that only checks tiles one tile from origin.
pub fn _compute_simple(
    origin: Position,
    mut add_visible: impl FnMut(Position),
) {
    add_visible(origin);
    for mov in crate::points::Move::iter() {
        let p = origin + mov;
        add_visible(p)
    }
}

#[cfg(test)]
mod tests;
