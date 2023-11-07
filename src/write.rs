use std::io::{Write, Result};

use crossterm::{queue,
    cursor::MoveTo,
    style::{PrintStyledContent, Print,
        SetForegroundColor, Color, ResetColor,
        StyledContent, Stylize}};
use petgraph::graph::NodeIndex;

use crate::{GameWorld, Map, MapElem, GameObj, ObjKind,
    Next, Change, Tile, TileKind,
    points::{Point, Position, Coord, Rect, Space, Straight, Points}};

const SET_AREA_COLOUR: SetForegroundColor = SetForegroundColor(Color::DarkGrey);

impl GameWorld {
    pub fn print(&self,
        out: &mut impl Write,
        Next(change): Next,
    ) -> Result<()> {
        let Self { player, map, .. } = self;

        match change {
            Change::Init => {
                map.draw_init(map.vis_from(player.pos).iter(), out)?;
                player.draw(out)?;
            }
            Change::Pos(p) | Change::Area(p, _) => {
                let prev_vis = map.vis_from(player.pos);
                let curr_vis = map.vis_from(p);

                map.draw_init(curr_vis.difference(&prev_vis), out)?;
                map.draw_area(self.current_area, out)?;
                map.clear_area(prev_vis.difference(&curr_vis), out)?;
                player.update_pos(p).draw(out)?;
            }
            Change::None => return Ok(()),
        }
        out.flush()
    }

    pub fn print_all(&self,
        out: &mut impl Write,
        Next(change): Next,
    ) -> Result<()> {
        let Self { player, map, current_area } = self;

        match change {
            Change::Init => {
                map.draw_all(out)?;
                player.draw(out)?
            }
            Change::Pos(p) | Change::Area(p, _) => {
                map.draw_area(*current_area, out)?;
                player.update_pos(p).draw(out)?;
            }
            Change::None => return Ok(()),
        }
        out.flush()
    }

    pub fn print_exp(&self,
        out: &mut impl Write,
        Next(change): Next,
    ) -> Result<()> {
        let Self { player, map, .. } = self;

        match change {
            Change::Init => {
                map.print_vis_tiles(player.pos, out)?;
                player.draw(out)?;
            }
            Change::Pos(p) | Change::Area(p, _) => {
                let pc = player.update_pos(p);
                map.clear_vis_tiles(player.pos, out)?;
                map.print_vis_tiles(pc.pos, out)?;
                pc.draw(out)?;
            }
            Change::None => return Ok(()),
        }
        out.flush()
    }
}

impl Map {
    //Line of sight
    fn print_vis_tiles(&self, p: Position, out: &mut impl Write) -> Result<()> {
        queue!(out, SET_AREA_COLOUR)?;
        for Tile { pos, kind } in self.visible_tiles(p) {
            match kind {
                TileKind::Room => pos.draw_tile(out, '.')?,
                TileKind::Hall => pos.draw_tile(out, '░')?,
                TileKind::Door => {
                    queue!(out, ResetColor)?;
                    pos.draw_tile(out, '∏')?;
                    queue!(out, SET_AREA_COLOUR)?
                }
                _ => {}
            }
        }
        queue!(out, ResetColor)
    }

    fn clear_vis_tiles(&self, p: Position, out: &mut impl Write) -> Result<()> {
        queue!(out, SET_AREA_COLOUR)?;
        for Tile { pos, kind } in self.visible_tiles(p) {
            match kind {
                TileKind::Room => pos.draw_tile(out, ' ')?,
                TileKind::Hall => pos.draw_tile(out, '/')?,
                TileKind::Door => {
                    queue!(out, ResetColor)?;
                    pos.draw_tile(out, '∏')?;
                    queue!(out, SET_AREA_COLOUR)?
                }
                _ => {}
            }
        }
        queue!(out, ResetColor)
    }
    
    //Visibility
    fn draw_vis<'a>(&self,
        areas: impl Iterator<Item = &'a NodeIndex>,
        out: &mut impl Write,
    ) -> Result<()> {
        for i in areas.cloned() {
            self[i].draw(out)?;
        }
        Ok(())
    }
    
    fn draw_init<'a>(&self,
        areas: impl Iterator<Item = &'a NodeIndex> + Clone,
        out: &mut impl Write,
    ) -> Result<()> {
        for i in areas.clone().cloned() {
            self[i].perimeter(out)?;
        }
        self.draw_vis(areas, out)
    }

    fn clear_area<'a>(&self,
        areas: impl Iterator<Item = &'a NodeIndex>,
        out: &mut impl Write,
    ) -> Result<()> {
        for i in areas.cloned() {
            self[i].clear(out)?;
        }
        Ok(())
    }

    //Simple
    fn draw_all(&self, out: &mut impl Write) -> Result<()> {
        for room in self.rooms() {
            room.perimeter(out)?;
        }
        for area in self.node_weights() {
            area.draw(out)?;
        }
        Ok(())
    }

    fn draw_area(&self, i: NodeIndex, out: &mut impl Write) -> Result<()> {
        self[i].draw(out)
    }
}

impl MapElem {
    fn draw(&self, out: &mut impl Write) -> Result<()> {
        match self {
            MapElem::Room(room) => {
                queue!(out, SET_AREA_COLOUR)?;
                room.draw_tile(out, '.')?;
                queue!(out, ResetColor)
            }
            MapElem::Hall(hall) => {
                queue!(out, SET_AREA_COLOUR)?;
                hall.draw_tile(out, '░')?;
                queue!(out, ResetColor)
            }
            MapElem::Door(door) => door.draw_tile(out, '∏'),
            _ => Ok(()),
        }
    }
    
    fn clear(&self, out: &mut impl Write) -> Result<()> {
        match self {
            MapElem::Room(room) => room.draw_tile(out, ' '),
            MapElem::Hall(hall) => {
                queue!(out, SET_AREA_COLOUR)?;
                hall.draw_tiles(out, ('─', '│'))?;
                queue!(out, ResetColor)
            }
            _ => Ok(()),
        }
    }

    fn perimeter(&self, out: &mut impl Write) -> Result<()> {
        match self {
            MapElem::Room(room) => room.perimeter(out),
            _ => Ok(()),
        }
    }
}

impl Position {
    fn draw_tile(&self, out: &mut impl Write, tile: char) -> Result<()> {
        queue!(out, MoveTo(self.x, self.y), Print(tile))
    }
}

impl Space {
    fn draw_tile(&self, out: &mut impl Write, tile: char) -> Result<()> {
        for strip in self.strips() {
            queue!(out, MoveTo(strip.next.x, strip.next.y))?;
            for _ in strip {
                queue!(out, Print(tile))?;
            }
        }
        Ok(())
    }

    fn perimeter(&self, out: &mut impl Write) -> Result<()>{
        let Rect {
            p1: Point { x, y },
            p2: Point { x: right, y: bottom }
        } = *self;
        let left = x - 1;
        let top = y - 1;

        queue!(out, MoveTo(left, top), Print('┌'))?;
        for _ in x..right { queue!(out, Print('─'))?; }
        queue!(out, Print('┐'))?;

        queue!(out, MoveTo(left, bottom), Print('└'))?;
        for _ in x..right { queue!(out, Print('─'))?; }
        queue!(out, Print('┘'))?;

        for row in y..bottom {
            queue!(out, MoveTo(left, row), Print('│'))?;
            queue!(out, MoveTo(right, row), Print('│'))?;
        }
        Ok(())
    }
}

impl Straight {
    fn draw_tile(&self, out: &mut impl Write, tile: char) -> Result<()> {
        for p in self { p.draw_tile(out, tile)?; }
        Ok(())
    }

    fn draw_tiles(
        &self, out: &mut impl Write, (tx, ty): (char, char),
    ) -> Result<()> {
        let tile = match self.c2 {
            Coord::X(_) => tx,
            Coord::Y(_) => ty,
        };
        self.draw_tile(out, tile)
    }
}

impl GameObj {
    fn draw(&self, out: &mut impl Write) -> Result<()> {
        queue!(out,
            MoveTo(self.pos.x, self.pos.y),
            PrintStyledContent(self.kind.into()),
        )
    }
}

impl From<ObjKind> for StyledContent<char> {
    fn from(kind: ObjKind) -> Self {
        match kind {
            ObjKind::Player => '@'.yellow(),
        }
    }
}
