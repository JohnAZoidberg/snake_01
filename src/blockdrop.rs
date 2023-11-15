extern crate rand;

use rand::Rng;
use std::fmt;

use crate::constants::*;

pub const OFF: usize = 3;
pub const OFF_U8: u8 = 3;
pub const OFF_I8: i8 = 3;

const WIDTH: usize = BOARD_WIDTH as usize;
const HEIGHT: usize = BOARD_HEIGHT as usize;

#[derive(Clone, Debug)]
pub struct ExtendedGrid(pub [[u8; HEIGHT + 3]; WIDTH + 2 * 3]);
impl Default for ExtendedGrid {
    fn default() -> Self {
        let mut grid = ExtendedGrid([[0; HEIGHT + 3]; WIDTH + 2 * 3]);
        for x in 0..WIDTH + 2 * 3 {
            for y in 0..HEIGHT + 3 {
                if x < 3 || x >= 3 + 9 || y > 33 {
                    grid.0[x][y] = 0xFF;
                }
            }
        }
        //println!("Grid: {:?}", grid);
        grid
    }
}
impl ExtendedGrid {
    fn remove_row(&mut self, row: usize) {
        for row in (1..row + 1).rev() {
            for x in 0..WIDTH {
                self.0[x + 3][row] = self.0[x + 3][row - 1];
            }
        }
    }
    fn compact_rows(&mut self) -> u32 {
        let mut score = 0;

        // Check every row
        for row in (0..HEIGHT).rev() {
            let mut full_row = true;
            for x in 0..WIDTH {
                if self.0[x + 3][row] != 0xFF {
                    full_row = false;
                }
            }
            if full_row {
                //println!("Row: {} is full", row);
                score += 1;
                self.remove_row(row);
            }
        }

        score
    }
    pub fn blocks(&self) -> [Block; (HEIGHT + 3) * (WIDTH + 2 * 3)] {
        let mut blocks = [Block {
            position: Position::new(0, 0),
            colour: YELLOW,
        }; (HEIGHT + 3) * (WIDTH + 2 * 3)];
        for x in 0..WIDTH + 6 {
            for y in 0..HEIGHT {
                blocks[x + y * (WIDTH + 2 * 3)].position = Position::new(x as i8, y as i8);
                // TODO: Why subtract by 2 not 3?
                if self.0[x][y] == 0xFF && x > 2 && x < WIDTH + 6 && y < HEIGHT {
                    blocks[x + y * (WIDTH + 2 * 3)].colour = GREEN;
                }
            }
        }
        blocks
        //ExtendedGridIterator {
        //    grid: self,
        //    x: 0,
        //    y: 0,
        //}
    }
}
pub struct ExtendedGridIterator<'a> {
    grid: &'a ExtendedGrid,
    x: usize,
    y: usize,
}
impl<'a> Iterator for ExtendedGridIterator<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        let x_start = self.x;
        let y_start = self.y;
        for x in x_start..WIDTH {
            for y in y_start..HEIGHT {
                //println!("x: {:?}, y: {:?}", x, y);
                self.x = x;
                self.y = y;
                if self.grid.0[x][y] == 0xFF && x > 3 && x < WIDTH + 6 && y < HEIGHT {
                    self.y += 1;
                    //println!("Woah");
                    return Some(Block {
                        position: Position::new(self.x as i8, self.y as i8),
                        colour: GREEN,
                    });
                }
            }
        }
        None
    }
}

type PieceShape = [[u8; 4]; 4];
#[rustfmt::skip]
const PIECES: [[((i8, i8), PieceShape); 4]; 2] = [
    [
        (
            (0, -1),
            [
                [0, 0, 0, 0],
                [1, 1, 1, 1],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
        ),
        (
            (-1, 0),
            [
                [0, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 0, 0],
            ],
        ),
        (
            (0, -1),
            [
                [0, 0, 0, 0],
                [1, 1, 1, 1],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
        ),
        (
            (0, -1),
            [
                [0, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 0, 0],
            ],
        ),
    ],
    [
        (
            (-1, 0),
            [
                [0, 0, 0, 0],
                [0, 1, 1, 1],
                [0, 1, 0, 0],
                [0, 0, 0, 0],
            ],
        ),
        (
            (-1, 0),
            [
                [0, 0, 0, 0],
                [0, 1, 1, 0],
                [0, 0, 1, 0],
                [0, 0, 1, 0],
            ],
        ),
        (
            (-1, 0),
            [
                [0, 0, 0, 0],
                [0, 0, 1, 0],
                [1, 1, 1, 0],
                [0, 0, 0, 0],
            ]
        ),
        (
            (-1, 0),
            [
                [0, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0],
            ],
        ),
    ],
];

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    shape: usize,
    rotation: Direction,
    pos: Position,
}
impl Piece {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            shape: rng.gen_range(0..2),
            rotation: [Direction::UP, Direction::DOWN, Direction::LEFT, Direction::RIGHT][rng.gen_range(0..4)],
            pos: Position::new(rng.gen_range(3..(WIDTH) as i8), 0),
        }
    }
    pub fn blocks(&self) -> [Block; 16] {
        let ((off_x, off_y), shape) = PIECES[self.shape][self.rotation as usize];
        let mut blocks = [Block {
            position: Position::new(0, 0),
            colour: YELLOW,
        }; 16];

        for x in 0..4 {
            for y in 0..4 {
                if shape[y][x] == 1 {
                    let res_x = (self.pos.x as i8) + (x as i8);
                    let res_y = (self.pos.y as i8) + (y as i8);
                    blocks[x + y * 4].position = Position::new(res_x, res_y);
                    //if res_x >= 3 && res_x < 3 + 9 && res_y < 34+3 {
                    blocks[x + y * 4].colour = GREEN;
                    //}
                }
            }
        }

        blocks
    }
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    fn new(x: i8, y: i8) -> Position {
        Position { x: x as u8, y: y as u8 }
    }

    fn offset(&mut self, x: i8, y: i8) {
        self.x = Position::calc_offset(self.x, x, BOARD_WIDTH + 3 * 2);
        self.y = Position::calc_offset(self.y, y, BOARD_HEIGHT + 3);
    }

    fn calc_offset(val: u8, offset: i8, max_val: u8) -> u8 {
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

#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub position: Position,
    pub colour: [f32; 4],
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
}

pub struct Game {
    pub time: u32,
    pub score: u32,
    pub board: ExtendedGrid,
    pub piece: Piece,
    pub game_over: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            time: 0,
            score: 0,
            board: ExtendedGrid::default(),
            piece: Piece::random(),
            game_over: false,
        }
    }

    pub fn init(&mut self) {
        self.time = 0;
        self.score = 0;

        self.board = ExtendedGrid::default();
        self.piece = Piece::random();
    }

    pub fn update(&mut self, dir: Direction) {
        if self.game_over {
            return;
        }
        let mut next_piece = self.piece.clone();
        match dir {
            Direction::UP => next_piece.rotation = next_piece.rotation.next(),
            Direction::DOWN => next_piece.pos.offset(0, 1),
            Direction::LEFT => next_piece.pos.offset(-1, 0),
            Direction::RIGHT => next_piece.pos.offset(1, 0),
        }
        if self.check_collision(&next_piece) {
            // Don't collide on rotation or sideways move
            if dir == Direction::DOWN {
                self.fallen_down();
            }
            //println!("Collision");
        } else {
            self.piece = next_piece;
        };
    }

    fn save(&mut self) {
        for b in self.piece.blocks() {
            if b.colour == GREEN {
                self.board.0[b.position.x as usize][b.position.y as usize] = 0xFF;
            }
        }
    }

    fn check_collision(&self, piece: &Piece) -> bool {
        for b in piece.blocks() {
            if b.colour == GREEN {
                //println!("Test: {:?}", b);
                if self.board.0[b.position.x as usize][b.position.y as usize] == 0xFF {
                    return true;
                }
            }
        }
        false
    }

    fn fallen_down(&mut self) {
        self.save();
        //println!("Rock bottom");

        self.score += self.board.compact_rows();
        println!("Score: {}", self.score);

        self.piece = Piece::random();
        // If the newly generated piece already overlaps, it's game over
        if self.check_collision(&self.piece) {
            // Game Over
            self.game_over = true;
        }
    }

    pub fn next_tick(&mut self, _dt: f64) {
        if self.game_over {
            return;
        }
        let mut next_piece = self.piece.clone();
        next_piece.pos.offset(0, 1);
        //println!("Current: {:?}", self.piece);
        //println!("Next: {:?}", next_piece);
        //println!("");
        if self.check_collision(&next_piece) {
            self.fallen_down();
        } else {
            self.piece = next_piece;
        };
    }

    // TODO: Do I need this?
    fn _get_direction_from_index(&self, index: usize) -> Direction {
        match index {
            0 => Direction::RIGHT,
            1 => Direction::UP,
            2 => Direction::LEFT,
            3 => Direction::DOWN,
            _ => unimplemented!(),
        }
    }
}
