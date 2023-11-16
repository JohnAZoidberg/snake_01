extern crate rand;

use rand::Rng;
use std::collections::VecDeque;
use std::fmt;

use crate::constants::*;

const WIDTH: usize = BOARD_WIDTH as usize;
const HEIGHT: usize = BOARD_HEIGHT as usize;
const WIDTH_U32: u32 = BOARD_WIDTH as u32;
const HEIGHT_U32: u32 = BOARD_HEIGHT as u32;
const WIDTH_I8: i8 = BOARD_WIDTH as i8;
const HEIGHT_I8: i8 = BOARD_HEIGHT as i8;

const WALL_HEIGHT: usize = 15;

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    fn new(x: i8, y: i8) -> Position {
        Position { x: x as u8, y: y as u8 }
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

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Debug)]
pub struct Grid(pub [[u8; HEIGHT]; WIDTH]);
impl Default for Grid {
    fn default() -> Self {
        let mut grid = Grid([[0; HEIGHT]; WIDTH]);
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                grid.0[x][y] = 0x00;
            }
        }
        grid
    }
}

const PADDLE_WIDTH: usize = 5;
const PADDLE_WIDTH_U32: u32 = PADDLE_WIDTH as u32;
pub struct Game {
    pub paddle_pos: u32,
    pub ball_pos: Position,
    pub ball_v: (i32, i32),
    pub board: Grid,
    pub time: u32,
    pub score: u32,
    pub game_over: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            paddle_pos: 0,
            ball_pos: Position::new(6, (HEIGHT - 2) as i8),
            ball_v: (1, -1),
            board: Grid::default(),
            time: 0,
            score: 0,
            game_over: false,
        }
    }

    pub fn board_blocks(&self) -> [Block; WIDTH * WALL_HEIGHT] {
        let mut blocks = [Block {
            position: Position::new(0, 0),
            colour: Colour::Yellow,
        }; WIDTH * WALL_HEIGHT];

        for x in 0..WIDTH {
            for y in 0..WALL_HEIGHT {
                blocks[x + y * WIDTH].position = Position::new(x as i8, y as i8);
                blocks[x + y * WIDTH].colour = if self.board.0[x][y] == 0xFF {
                    Colour::Green
                } else {
                    Colour::Yellow
                };
            }
        }

        blocks
    }

    pub fn paddle_blocks(&self) -> [Block; PADDLE_WIDTH] {
        let mut blocks = [Block {
            position: Position::new(0, 0),
            colour: Colour::Yellow,
        }; PADDLE_WIDTH];

        for x in 0..PADDLE_WIDTH {
            blocks[x].position = Position::new(self.paddle_pos as i8 + x as i8, (HEIGHT - 1) as i8);
            blocks[x].colour = Colour::Green;
        }

        blocks
    }

    pub fn ball(&self) -> Block {
        Block {
            position: self.ball_pos,
            colour: Colour::Green,
        }
    }

    pub fn init(&mut self) {
        self.board = Grid::default();
        for x in 0..WIDTH {
            for y in 0..WALL_HEIGHT {
                //if (y < 3 || x > 3) || (x == 3 && y == 3){
                    self.board.0[x][y] = 0xFF;
                //}
            }
        }
        self.time = 0;
        self.score = 0;
        self.game_over = false;
    }

    pub fn update(&mut self, dir: Direction) {
        if self.game_over {
            return;
        }

        let new_ppos = match dir {
            Direction::LEFT if self.paddle_pos > 0 => self.paddle_pos - 1,
            Direction::RIGHT if self.paddle_pos + PADDLE_WIDTH_U32 < WIDTH_U32 => self.paddle_pos + 1,
            _ => self.paddle_pos,
        };
        self.paddle_pos = new_ppos;
    }

    fn hit_field(&mut self) -> Vec<Direction> {
        let mut dirs = vec![];
        //let mut new_x = ((self.ball_pos.x as i32) + self.ball_v.0) as i8;
        //let mut new_y = ((self.ball_pos.y as i32) + self.ball_v.1) as i8;

        // Hit something on the left
        if self.ball_pos.x > 0 {
            let left_pos = &mut self.board.0[self.ball_pos.x as usize-1][self.ball_pos.y as usize];
            if self.ball_v.0 < 0 && *left_pos == 0xFF {
                *left_pos = 0x00;
                self.score += 1;
                dirs.push(Direction::LEFT);
            }
        }

        // Hit something on the right
        if usize::from(self.ball_pos.x+1) < WIDTH {
            let right_pos = &mut self.board.0[self.ball_pos.x as usize+1][self.ball_pos.y as usize];
            if self.ball_v.0 > 0 && *right_pos == 0xFF {
                *right_pos = 0x00;
                self.score += 1;
                dirs.push(Direction::RIGHT);
            }
        }

        // Hit something on the top
        if self.ball_pos.y > 0 {
            let up_pos = &mut self.board.0[self.ball_pos.x as usize][self.ball_pos.y as usize - 1];
            if self.ball_v.1 < 0 && *up_pos == 0xFF {
                *up_pos = 0x00;
                self.score += 1;
                dirs.push(Direction::UP);
            }
        }

        // Hit something on the bottom
        if usize::from(self.ball_pos.y+1) < HEIGHT {
            let down_pos = &mut self.board.0[self.ball_pos.x as usize][self.ball_pos.y as usize + 1];
            if self.ball_v.1 > 0 && *down_pos == 0xFF {
                *down_pos = 0x00;
                self.score += 1;
                dirs.push(Direction::DOWN);
            }
        }

        // Hit up diagonal
        // TODO: Bottom diagonal. Should be rare enough
        // Skip if we already hit something else.
        // Because a corner with side and top might also have a vertical, but we only bounce off the directly connected, if they exist.
        if dirs.is_empty() && self.ball_pos.y > 0 {
            // Up-right
            if usize::from(self.ball_pos.x+1) < WIDTH {
                let up_right_pos = &mut self.board.0[self.ball_pos.x as usize+1][self.ball_pos.y as usize-1];
                if self.ball_v.0 > 0 && self.ball_v.1 < 0 && *up_right_pos == 0xFF {
                    *up_right_pos = 0x00;
                    self.score += 1;
                    dirs.push(Direction::RIGHT);
                    dirs.push(Direction::UP);
                    return dirs;
                }
            }

            if self.ball_pos.x > 0 {
                let up_left_pos = &mut self.board.0[self.ball_pos.x as usize-1][self.ball_pos.y as usize - 1];
                if self.ball_v.0 < 0 && self.ball_v.1 < 0 && *up_left_pos == 0xFF {
                    *up_left_pos = 0x00;
                    self.score += 1;
                    dirs.push(Direction::LEFT);
                    dirs.push(Direction::UP);
                    return dirs;
                }
            }
        }

        dirs
    }

    pub fn next_tick(&mut self, _dt: f64) {
        if self.game_over {
            return;
        }

        // Bounce off the field, if anything was hit
        for dir in self.hit_field() {
            //println!("Flipping direction : {:?}", dir);
            self.ball_v = match dir {
                Direction::UP | Direction::DOWN => (self.ball_v.0, -self.ball_v.1),
                Direction::LEFT | Direction::RIGHT => (-self.ball_v.0, self.ball_v.1),
            };
        };

        // Bounce off the walls
        if (self.ball_v.0 < 0 && self.ball_pos.x == 0) || (self.ball_v.0 > 0 && self.ball_pos.x+1 == WIDTH as u8) {
            self.ball_v = (-self.ball_v.0, self.ball_v.1);
        }

        // Bounce off the top
        if self.ball_v.1 < 0 && self.ball_pos.y == 0 {
            self.ball_v = (self.ball_v.0, -self.ball_v.1);
        }

        // Bounce off the paddle
        // TODO: Change velocity vector based on bounce angle
        let above_padel = self.ball_pos.x as u32 >= self.paddle_pos && self.ball_pos.x as u32 <= self.paddle_pos + PADDLE_WIDTH_U32;
        if self.ball_v.1 > 0 && self.ball_pos.y+2 == HEIGHT as u8 && above_padel  {
            let offset = (self.ball_pos.x as i8) - (self.paddle_pos + PADDLE_WIDTH_U32 / 2)as i8;
            //println!("Offset: {:?}", offset);
            self.ball_v = match offset {
                -3 => (-1, -1),
                -2 => (-1, -1),
                -1 => (-1, -1),
                0 => (0, -1),
                1 => (1, -1),
                2 => (1, -1),
                3 => (1, -1),
                _ => self.ball_v
            };
        }

        let mut new_x = ((self.ball_pos.x as i32) + self.ball_v.0) as i8;
        let mut new_y = ((self.ball_pos.y as i32) + self.ball_v.1) as i8;
        if new_x >= WIDTH_I8-1 {
            new_x = WIDTH_I8-1;
        }
        if new_y >= HEIGHT_I8-1 {
            new_y = HEIGHT_I8-1;
        }
        if new_x < 0 {
            new_x = 0;
        }
        if new_y < 0 {
            new_y = 0;
        }

        // Passed by the paddle to the bottom
        if self.ball_pos.y+1 == HEIGHT as u8 {
            self.game_over = true;
            return;
        }

        self.ball_pos = Position::new(new_x, new_y);
    }
}
