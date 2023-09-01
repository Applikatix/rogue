use std::io::{self, Result, Write};

use crossterm::{queue, cursor, style::{self, Stylize}};

pub struct Out { pub screen: io::Stdout }

impl Out {
    pub fn new() -> Out {
        Out { screen: io::stdout() }
    }

    pub fn draw(&mut self, room: &Room, player: &Object) -> Result<()> {
        let Out { screen } = self;
        let Room {
            pos: Point { x: left, y: top },
            size: Point { x: width, y: height },
        } = *room;

        let bottom = top + height + 1;

        //Draw room
        queue!(screen, cursor::MoveTo(left, top), style::Print('┌'))?;
        for _ in 0..width {
            queue!(screen, style::Print('─'))?;
        }
        queue!(screen, style::Print('┐'))?;
    
        for row in (top+1)..bottom {
            queue!(screen, cursor::MoveTo(left, row), style::Print('│'))?;
            for _ in 0..width {
                queue!(screen, style::PrintStyledContent('.'.dark_grey()))?;
            }
            queue!(screen, style::Print('│'))?;
        }
    
        queue!(screen, cursor::MoveTo(left, bottom), style::Print('└'))?;
        for _ in 0..width {
            queue!(screen, style::Print('─'))?;
        }
        queue!(screen, style::Print('┘'))?;

        //Draw player
        queue!(screen,
            cursor::MoveTo(player.pos.x, player.pos.y),
            style::Print(player.character),
        )?;

        screen.flush()?;
        
        Ok(())
    }
}

pub struct Point { pub x: u16, pub y: u16}

pub struct Room {
    pub pos: Point,
    pub size: Point,
}

pub struct Object {
    character: char,
    pub pos: Point,
}

pub fn world(left: u16, top: u16, width: u16, height: u16) -> (Room, Object) {
    let room = Room {
        pos: Point { x: left, y: top },
        size: Point { x: width, y: height },
    };
    let player = Object {
        character: '@',
        pos: Point {
            x: left + 1,
            y: top + 1,
        },
    };

    (room, player)
}
