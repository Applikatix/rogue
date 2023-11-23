use std::collections::HashSet;

use crate::points::{Position, Point};

/// An absolute position on a grid.
type Absolute = Position;
/// A position relative to the origin (0, 0) where x >= y.
type Relative = Position;

pub fn compute(
    origin: Absolute,
    area_contains: impl Fn(Absolute) -> bool,
    mark_visible: &mut impl FnMut(Absolute),
) {
    mark_visible(origin);

    for oct in Octant::iter_from(origin) {
        let is_floor =
            |p: Relative| area_contains(oct.to_absolute(p));
        let reveal =
            |p: Relative| mark_visible(oct.to_absolute(p));
    }
}

//Octants
struct Octant { origin: Absolute, section: OctSection }
impl Octant {
    fn iter_from(origin: Absolute) -> impl Iterator<Item = Octant> {
        OctSection::iter()
            .map(move |section| Octant { origin, section })
    }

    fn to_absolute(&self, p: Relative) -> Absolute {
        let Point { x: ox, y: oy } = self.origin;

        use OctSection::*;
        match self.section {
            LeftUp => Point { x: ox - p.x, y: oy - p.y },
            LeftDown => Point { x: ox - p.x, y: oy + p.y },
            RightUp => Point { x: ox + p.x, y: oy - p.y },
            RightDown => Point { x: ox + p.x, y: oy + p.y },
            UpLeft => Point { x: ox - p.y, y: oy - p.x },
            UpRight => Point { x: ox + p.y, y: oy - p.x },
            DownLeft => Point { x: ox - p.y, y: oy + p.x },
            DownRight => Point { x: ox + p.y, y: oy + p.x },
        }
    }
}
use {strum::IntoEnumIterator, strum_macros::EnumIter};
#[derive(Clone, Copy, EnumIter)]
enum OctSection {
    LeftUp, LeftDown,
    RightUp, RightDown,
    UpLeft, UpRight,
    DownLeft, DownRight,
}

use num_rational::Ratio;

/// Represents a row in an octant.
struct Col {
    depth: u16,
    start_slope: Ratio<u16>,
    end_slope: Ratio<u16>,
}
impl Col {
    fn first() -> Self {
        Self { depth: 1, start_slope: 0.into(), end_slope: 1.into() }
    }

    fn next(&self) -> Self {
        Self { depth: self.depth + 1, ..*self }
    }

    fn tiles(&self) -> impl Iterator<Item = Relative> + '_ {
        let lower = (self.start_slope * self.depth).ceil().to_integer();
        let upper = (self.end_slope * self.depth).floor().to_integer();
        (lower..=upper).map(|y| Point { x: self.depth, y })
    }

    fn is_symmetric(&self, p: Relative) -> bool {
        todo!()
    }
}

/// Simple FOV function that only checks tiles one tile from origin.
pub fn compute_simple(
    origin: Absolute,
    _: impl Fn(Absolute) -> bool,
    visible: &mut impl FnMut(Absolute),
) {
    visible(origin);
    for mov in crate::points::Move::iter() {
        let p = origin + mov;
        visible(p)
    }
}

#[cfg(test)]
mod tests;
