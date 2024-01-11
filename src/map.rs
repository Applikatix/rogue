use crate::{points::{Space, Point, Rect}, Map};
use num_rational::Ratio;
use rand::prelude::*;

/*
pub fn generate_world(width: u16, height: u16, max_partition: u8) -> GameWorld {
    todo!()
} */

fn space_size(width: u16, height: u16) -> Space {
    Rect::new(0, 0, width, height)
}

const MIN_SIZE: u16 = 4;
const MIN_ROOM_SIZE: u16 = 2;
const MIN_ROOM_AREA: u16 = 8;
const VARIANCE: Ratio<u16> = Ratio::new_raw(1, 2);

impl RngWrapper {
    fn new() -> Self { Self(thread_rng()) }

    fn rand_map(&mut self, areas: Areas) -> Map {
        if areas.is_leaf() {
            let room = self.rand_room(areas.val);
        }
        todo!()
    }

    // generating rooms
    fn rand_rooms(&mut self, areas: Areas) -> Vec<Space> {
        let mut rooms = Vec::new();
        for space in areas.leaf_iter() {
            rooms.push(self.rand_room(*space))
        }
        rooms
    }

    fn rand_room(&mut self, space: Space) -> Space {
        let Rect {
            pos: Point { x, y },
            end: Point { x: endx, y: endy }
        } = space;
        loop {
            let (x, endx) = self.rand_wall(x, endx);
            let (y, endy) = self.rand_wall(y, endy);
            let room = Rect::new_exact(x, y, endx, endy);
            let Point { x: width, y: height } = room.size();

            if width >= MIN_ROOM_SIZE &&
            height >= MIN_ROOM_SIZE &&
            width * height >= MIN_ROOM_AREA {
                return room;
            }
        }
    }

    fn rand_wall(&mut self, pos: u16, end: u16) -> (u16, u16) {
        let a = self.gen_range(pos..=end);
        let b = self.gen_range(pos..=end);

        if a < b { (a, b) } else { (b, a) }
    }

    // generating partitions
    fn rand_spaces(&mut self, space: Space, max_depth: u8) -> Areas {
        if max_depth == 0 { return Node::leaf(space); }
        if let Some((l, r)) = self.split_space(space) {
            return Node::new(space,
                self.rand_spaces(l, max_depth - 1),
                self.rand_spaces(r, max_depth - 1),
            );
        }
        Node::leaf(space)
    }

    fn split_space(&mut self, space: Space) -> Option<(Space, Space)> {
        let Point { x: width, y: height } = space.size();

        if self.gen_ratio(
            width.into(), (width + height).into()
        ) {
            //Split on X axis
            let x = self.split_size(width, space.pos.x)?;
            Some((space.end_x(x), space.pos_x(x)))
        } else {
            //Split on Y axis
            let y = self.split_size(height, space.pos.y)?;
            Some((space.end_y(y), space.pos_y(y)))
        }
    }

    fn split_size(&mut self, size: u16, scale: u16) -> Option<u16> {
        const ONE: Ratio<u16> = Ratio::new_raw(1, 1);
        let pad = (((ONE - VARIANCE) * size) / 2).to_integer();
        let partition = self.gen_range(pad..(size-pad));

        if partition < MIN_SIZE || size - partition < MIN_SIZE { return None; }
        
        Some(scale + partition)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const WIDTH: u16 = 30;
    const HEIGHT: u16 = 30;
    const MAX_DEPTH: u8 = 4;

    #[test]
    fn rooms_generation() {
        let mut out = std::io::stdout();
        let mut rng = RngWrapper::new();
        let space = space_size(WIDTH, HEIGHT);
        loop {
            let areas = rng.rand_spaces(space, MAX_DEPTH);

            queue!(out, CLEAR_ALL).unwrap();
            for (i, space) in areas.leaf_iter().enumerate() {
                let c = char::from_digit(i as u32, 32).unwrap();
                rng.rand_room(*space).draw(&mut out, c).unwrap();
            }
            out.flush().unwrap();

            if let KeyCode::Esc = crate::util::input().unwrap() { break; }
        }
    }

    #[test]
    fn room_generation() {
        let mut out = std::io::stdout();
        let mut rng = RngWrapper::new();
        let space = Space::new(5, 3 , 5, 5);
        loop {
            let room = rng.rand_room(space);

            queue!(out, CLEAR_ALL).unwrap();
            space.draw_edge(&mut out, '+').unwrap();
            room.draw(&mut out, '.').unwrap();
            out.flush().unwrap();

            if let KeyCode::Esc = crate::util::input().unwrap() { break; }
        }
    }

    #[test]
    fn tree_generation() {
        let mut out = std::io::stdout();
        let mut rng = RngWrapper::new();
        let space = space_size(WIDTH, HEIGHT);
        loop {
            let tree = rng.rand_spaces(space, MAX_DEPTH);

            queue!(out, CLEAR_ALL).unwrap();
            for (i, space) in tree.leaf_iter().enumerate() {
                let i = char::from_digit(i as u32, 32).unwrap();
                space.draw_tl(&mut out, '+').unwrap();
                space.pos.downright(&mut out, i).unwrap();
            }
            space.draw_br(&mut out, '+').unwrap();
            out.flush().unwrap();
            
            if let KeyCode::Esc = crate::util::input().unwrap() { break; }
        }
    }

    #[test]
    fn random_split() {
        let mut out = std::io::stdout();
        let mut rng = RngWrapper::new();

        let space = Space::new(0, 0 , 10, 10);
        
        loop {
            queue!(out, CLEAR_ALL).unwrap();
            if let Some((l, r)) = rng.split_space(space) {
                l.draw_edge(&mut out, 'l').unwrap();
                r.draw_edge(&mut out, 'r').unwrap();
            } else {
                print!("Not valid");
            }
            out.flush().unwrap();

            if let KeyCode::Esc = crate::util::input().unwrap() {
                break;
            }
        }
    }

    use std::io::{Write, self};
    use crossterm::{queue,
        terminal, cursor::{MoveTo, MoveToColumn}, style::Print, event::KeyCode};
    const CLEAR_ALL: terminal::Clear = terminal::Clear(terminal::ClearType::All);

    use crate::points::{Position, Move};

    impl Position {
        fn downright(self, out: &mut impl Write, c: char) -> io::Result<()> {
            let p = self + Move::RD;
            queue!(out, MoveTo(p.x, p.y), Print(c))
        }
    }

    impl Space {
        fn draw_edge(self, out: &mut impl Write, c: char) -> io::Result<()> {
            let Self { pos, end } = self;
            queue!(out, MoveTo(pos.x, pos.y))?;
            for _ in pos.x..end.x {
                queue!(out, Print(c))?;
            }
            for row in pos.y..end.y {
                queue!(out,
                    MoveTo(pos.x, row), Print(c),
                    MoveToColumn(end.x - 1), Print(c))?;
            }
            queue!(out, MoveTo(pos.x, end.y - 1))?;
            for _ in pos.x..end.x {
                queue!(out, Print(c))?;
            }
    
            Ok(())
        }

        fn draw_tl(self, out: &mut impl Write, c: char) -> io::Result<()> {
            let Self { pos, end } = self;
            queue!(out, MoveTo(pos.x, pos.y))?;
            for _ in pos.x..end.x {
                queue!(out, Print(c))?;
            }
            for row in pos.y..end.y {
                queue!(out, MoveTo(pos.x, row), Print(c))?;
            }

            Ok(())
        }

        fn draw_br(self, out: &mut impl Write, c: char) -> io::Result<()> {
            let Self { pos, end } = self;
            for row in pos.y..end.y {
                queue!(out, MoveTo(end.x, row), Print(c))?;
            }
            queue!(out, MoveTo(pos.x, end.y))?;
            for _ in pos.x..end.x {
                queue!(out, Print(c))?;
            }

            Ok(())
        }

        fn draw(self, out: &mut impl Write, c: char) -> io::Result<()> {
            for row in self.pos.y..self.end.y {
                queue!(out, MoveTo(self.pos.x, row))?;
                for _ in self.pos.x..self.end.x {
                    queue!(out, Print(c))?;
                }
            }
            Ok(())
        }
    }
}

// RNG wrapper
use std::ops::{Deref, DerefMut};

struct RngWrapper(ThreadRng);
impl Deref for RngWrapper {
    type Target = ThreadRng;
    fn deref(&self) -> &Self::Target { &self.0 }
} impl DerefMut for RngWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

// Tree datastructure

type Areas = Node<Space>;
struct Node<T> {
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
} impl<T> Node<T> {
    fn new(val: T, left: Self, right: Self) -> Self {
        Self { val, left: Some(Box::new(left)), right: Some(Box::new(right)) }
    }
    fn leaf(val: T) -> Self {
        Self { val, left: None, right: None }
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    fn leaf_iter(&self) -> Leafs<T> {
        Leafs(self.leaf_iter_alt())
    }
    fn leaf_iter_alt(&self) -> LeafsAlt<T> {
        LeafsAlt { stack: vec![self] }
    }
}

impl<T> Deref for Node<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

struct Leafs<'a, T>(LeafsAlt<'a, T>);
impl<'a, T> Iterator for Leafs<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|node| &node.val)
    }
}

struct LeafsAlt<'a, T> {
    stack: Vec<&'a Node<T>>,
} impl<'a, T> Iterator for LeafsAlt<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(node) = self.stack.pop() {
                if let (None, None) = (&node.left, &node.right) {
                    return Some(node);
                } else {
                    if let Some(r) = &node.right {
                        self.stack.push(r);
                    }
                    if let Some(l) = &node.left {
                        self.stack.push(l);
                    }
                }
            } else {
                return None;
            }
        }
    }
}
