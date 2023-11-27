use std::io::{Write, Result};

use crossterm::{queue,
    cursor::{MoveTo, MoveToColumn},
    style::{Print, SetForegroundColor, Color, ResetColor}};

use super::{GameWorld, Next, Dir, Tile, TileMap};

const FADE_COLOUR: SetForegroundColor = SetForegroundColor(Color::DarkGrey);

impl GameWorld {
    pub fn print(&self, out: &mut impl Write, next: Next) -> Result<()> {
        use super::Change::*;
        let Self { player, map } = self;
        
        draw_frame(0, 60, 0, 18, out)?;

        match next.0 {
            Nothing => return Ok(()),
            Init => {
                map.visible_tiles(player.pos).draw(out)?;
            }
            Pos(p) | Area(p, _) => {
                let new_vis = map.visible_tiles(p);
                let old_vis = map.visible_tiles(player.pos);
                
                old_vis.clear_old(out, &new_vis)?;
                new_vis.draw_new(out, &old_vis)?;
            }
        }
        out.flush()
    }
}

impl TileMap {
    fn draw(&self, out: &mut impl Write) -> Result<()> {
        queue!(out, ResetColor)?;
        for tile in self.tiles() {
            tile.draw(out)?;
        }
        Ok(())
    }

    fn draw_new(&self, out: &mut impl Write, old: &TileMap) -> Result<()> {
        queue!(out, ResetColor)?;
        for tile in self.difference_player(old) {
            tile.draw(out)?;
        }
        Ok(())
    }

    fn clear_old(&self, out: &mut impl Write, new: &TileMap) -> Result<()> {
        queue!(out, FADE_COLOUR)?;
        for tile in self.difference(new) {
            tile.clear(out)?;
        }
        Ok(())
    }
}

impl Tile {
    fn draw(&self, out: &mut impl Write) -> Result<()> {
        use super::{TileKind::*, ObjKind::*};
        queue!(out, MoveTo(self.pos.x, self.pos.y))?;
        match self.kind {
            Door => queue!(out, Print('∏')),
            Room => queue!(out, Print('.')),
            Hall(_) => queue!(out, Print('░')),
            Wall(dir) => queue!(out, Print(char::from(dir))),
            Obj(Player) => queue!(out, Print('@')),
        }
    }

    fn clear(&self, out: &mut impl Write) -> Result<()> {
        use super::TileKind::*;
        queue!(out, MoveTo(self.pos.x, self.pos.y))?;
        match self.kind {
            Door => queue!(out, Print('∏')),
            Room => queue!(out, Print(' ')),
            Hall(_) => queue!(out, Print('░')),
            Wall(dir) => queue!(out, Print(char::from(dir))),
            _ => Ok(()),
        }
    }
}

impl From<Dir> for char {
    fn from(dir: Dir) -> Self { use super::Dir::*;
        match dir { //None => '■',
            //Up => '╴', Down => '╶', Left => '╵', Right => '╷',
            Hor => '─', Ver => '│', UL => '┌', UR => '┐', DL => '└', DR => '┘',
            //UHor => '┴', DHor => '┬', VerL => '┤', VerR => '├',
            //All => '┼',
        }
    }
}

fn draw_frame(
    left: u16, right: u16, top: u16, bottom: u16,
    out: &mut impl Write,
) -> Result<()> {
    
    queue!(out, FADE_COLOUR)?;

    queue!(out, MoveTo(left, top), Print('╔'))?;
    for _ in (left + 1)..right {
        queue!(out, Print('═'))?;
    }
    queue!(out, Print('╗'))?;

    for row in (top + 1)..bottom {
        queue!(out, MoveTo(left, row), Print('║'))?;
        queue!(out, MoveToColumn(right), Print('║'))?;
    }

    queue!(out, MoveTo(left, bottom), Print('╚'))?;
    for _ in (left + 1)..right {
        queue!(out, Print('═'))?;
    }
    queue!(out, Print('╝'))?;

    Ok(())
}
