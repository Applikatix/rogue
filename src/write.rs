use std::io::{Write, Result};

use crossterm::{queue,
    cursor::MoveTo,
    style::{Print, SetForegroundColor, Color, ResetColor}};

use super::{GameWorld, Next, Dir, Tile, TileMap};

const FADE_COLOUR: SetForegroundColor = SetForegroundColor(Color::DarkGrey);

impl From<Dir> for char {
    fn from(dir: Dir) -> Self { use super::Dir::*;
        match dir { //None => '■',
            //Up => '╴', Down => '╶', Left => '╵', Right => '╷',
            Hor => '─', Ver => '│', //UL => '┌', UR => '┐', DL => '└', DR => '┘',
            //UHor => '┴', DHor => '┬', VerL => '┤', VerR => '├',
            //All => '┼',
        }
    }
}

//Tiles
impl GameWorld {
    pub fn print_alt(&self,
        out: &mut impl Write,
        Next(change): Next,
    ) -> Result<()> { use super::Change::*;
        let Self { player, map } = self;

        match change {
            Nothing => return Ok(()),
            Init => {
                map.visible_tiles(player.pos).draw(out)?;
            }
            Pos(p) | Area(p, _) => {
                let new_vis = map.visible_tiles(p);
                let old_vis = map.visible_tiles(player.pos);
                
                old_vis.clear_diff(out, &new_vis)?;
                new_vis.draw_diff(out, &old_vis)?;
            }
        }
        out.flush()
    }
}

impl TileMap {
    fn draw(&self, out: &mut impl Write) -> Result<()> {
        for tile in self.tiles() {
            tile.draw(out)?;
        }
        Ok(())
    }

    fn draw_diff(&self, out: &mut impl Write, old: &TileMap) -> Result<()> {
        for tile in self.difference_player(old) {
            tile.draw(out)?;
        }
        Ok(())
    }

    fn clear_diff(&self, out: &mut impl Write, new: &TileMap) -> Result<()> {
        queue!(out, FADE_COLOUR)?;
        for tile in self.difference(new) {
            tile.clear(out)?;
        }
        queue!(out, ResetColor)
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
