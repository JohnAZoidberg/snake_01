use std::fmt;

use crate::constants::*;

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    pub fn new(x: i8, y: i8) -> Position {
        Position { x: x as u8, y: y as u8 }
    }

    pub fn new_center() -> Position {
        Position {
            x: BOARD_WIDTH / 2,
            y: BOARD_HEIGHT / 2,
        }
    }

    #[cfg(feature = "snake")]
    pub fn new_offset(x: i8, y: i8) -> Position {
        let mut pos = Position::new_center();
        pos.offset(x, y);
        pos
    }

    #[cfg(feature = "snake")]
    pub fn offset(&mut self, x: i8, y: i8) {
        self.x = Position::calc_offset(self.x, x, BOARD_WIDTH);
        self.y = Position::calc_offset(self.y, y, BOARD_HEIGHT);
    }
    #[cfg(feature = "blockdrop")]
    pub fn offset(&mut self, x: i8, y: i8) {
        self.x = Position::calc_offset(self.x, x, BOARD_WIDTH + 3 * 2);
        self.y = Position::calc_offset(self.y, y, BOARD_HEIGHT + 3);
    }

    pub fn calc_offset(val: u8, offset: i8, max_val: u8) -> u8 {
        if (val == 0 && offset < 0) || (val >= max_val - 1 && offset > 0) {
            val
        } else {
            let off_max = offset as i16 % max_val as i16;
            if off_max < 0 {
                let x1 = off_max as u8;
                let x2 = x1 - std::u8::MAX / 2 - 1 + max_val;
                let x3 = x2 - std::u8::MAX / 2 - 1;
                (val + x3) % max_val
            } else {
                (val + off_max as u8) % max_val
            }
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point").field("x", &self.x).field("y", &self.y).finish()
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Block {
    pub position: Position,
    pub colour: Colour,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn next_cw(&self) -> Self {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }

    pub fn opposite(&mut self) -> Direction {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
        }
    }
}

pub trait GameT {
    fn new() -> Self;
    fn init(&mut self);
    fn update(&mut self, dir: Direction);
    fn next_tick(&mut self, dt: f64);
}
