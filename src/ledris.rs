extern crate rand;

use rand::Rng;
use std::collections::VecDeque;
use std::fmt;

use crate::constants::*;

pub const OFF: usize = 3;
pub const OFF_U8: u8 = 3;
pub const OFF_I8: i8 = 3;

const WIDTH: usize = BOARD_WIDTH as usize;
const HEIGHT: usize = BOARD_HEIGHT as usize;

#[derive(Clone, Debug)]
pub struct ExtendedGrid(pub [[u8; HEIGHT+3]; WIDTH+2*3]);
impl Default for ExtendedGrid {
    fn default() -> Self {
        let mut grid = ExtendedGrid([[0; HEIGHT+3]; WIDTH+2*3]);
        for x in 0..WIDTH+2*3 {
            for y in 0..HEIGHT+3 {
                if x < 3 || x >= 3+9 || y > 33 {
                    grid.0[x][y] = 0xFF;
                }
            }
        }
        println!("Grid: {:?}", grid);
        grid
    }
}
impl ExtendedGrid {
    pub fn blocks(&self) -> [Block; (HEIGHT+3) * (WIDTH+2*3)] {
        let mut blocks = [Block {
            position: Position::new_abs(0, 0),
            colour: YELLOW,
        }; (HEIGHT+3) * (WIDTH+2*3)];
        for x in 0..WIDTH+6 {
            for y in 0..HEIGHT {
                blocks[x+y*(WIDTH+2*3)].position = Position::new_abs(x as i8, y as i8);
                // TODO: Why subtract by 2 not 3?
                if self.0[x][y] == 0xFF && x > 2 && x < WIDTH+6 && y < HEIGHT {
                    blocks[x+y*(WIDTH+2*3)].colour = GREEN;
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
impl <'a>Iterator for ExtendedGridIterator<'a> {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        let x_start = self.x;
        let y_start = self.y;
        for x in x_start..WIDTH {
            for y in y_start..HEIGHT {
                //println!("x: {:?}, y: {:?}", x, y);
                self.x = x;
                self.y = y;
                if self.grid.0[x][y] == 0xFF && x > 3 && x < WIDTH+6 && y < HEIGHT {
                    self.y += 1;
                    //println!("Woah");
                    return Some(Block {
                        position: Position::new_abs(self.x as i8, self.y as i8),
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
            pos: Position::new_abs(rng.gen_range(3..(WIDTH) as i8), 0),
        }
    }
    pub fn blocks(&self) -> [Block; 16] {
        let ((off_x, off_y), shape) = PIECES[self.shape][self.rotation as usize];
        let mut blocks = [Block {
            position: Position::new_abs(0, 0),
            colour: YELLOW,
        }; 16];

        for x in 0..4 {
            for y in 0..4 {
                if shape[y][x] == 1 {
                    let res_x = (self.pos.x as i8) + (x as i8);
                    let res_y = (self.pos.y as i8) + (y as i8);
                    blocks[x+y*4].position = Position::new_abs(res_x, res_y);
                    //if res_x >= 3 && res_x < 3 + 9 && res_y < 34+3 {
                        blocks[x+y*4].colour = GREEN;
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
    fn new() -> Position {
        Position {
            x: BOARD_WIDTH / 2,
            y: BOARD_HEIGHT / 2,
        }
    }

    fn new_abs(x: i8, y: i8) -> Position {
        Position { x: x as u8, y: y as u8 }
    }

    fn new_offset(x: i8, y: i8) -> Position {
        let mut pos = Position::new();
        pos.offset(x, y);
        pos
    }

    fn offset(&mut self, x: i8, y: i8) {
        self.x = Position::calc_offset(self.x, x, BOARD_WIDTH+3*2);
        self.y = Position::calc_offset(self.y, y, BOARD_HEIGHT+3);
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
    fn opposite(&self) -> Self {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
        }
    }
    fn next(&self) -> Self {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }
}

pub struct Snake {
    pub body: VecDeque<Block>,
    pub direction: Direction,
    pub alive: bool,
    pub eat: bool,
}

impl Snake {
    fn new() -> Snake {
        Snake {
            body: VecDeque::from(vec![
                Block {
                    position: Position::new(),
                    colour: YELLOW,
                },
                Block {
                    position: Position::new_offset(-1, 0),
                    colour: GREEN,
                },
                Block {
                    position: Position::new_offset(-2, 0),
                    colour: GREEN,
                },
            ]),
            direction: Direction::UP,
            alive: true,
            eat: false,
        }
    }

    fn update(&mut self, mut dir: Direction) {
        if self.direction == dir.opposite() {
            // Do nothing
        } else {
            self.direction = dir;
        }
    }

    fn perform_next(&mut self, food_pos: &mut Position) {
        if self.alive {
            let next_pos = self.next_head_pos();
            if self.check_collide_wall(next_pos) || self.check_collide_body(next_pos) {
                self.alive = false;
            } else if self.check_eat_food(next_pos, *food_pos) {
                self.eat_next(food_pos);
                self.eat = true;
            } else {
                self.move_next();
            }
        }
    }

    fn next_head_pos(&mut self) -> Position {
        let mut current_head = self.body[0].position;
        match self.direction {
            Direction::RIGHT => current_head.offset(1, 0),
            Direction::UP => current_head.offset(0, -1),
            Direction::LEFT => current_head.offset(-1, 0),
            Direction::DOWN => current_head.offset(0, 1),
        }
        current_head
    }

    fn check_collide_wall(&self, next_pos: Position) -> bool {
        self.body[0].position == next_pos
    }

    fn check_collide_body(&self, pos: Position) -> bool {
        self.body.iter().any(|block| block.position == pos)
    }

    fn check_eat_food(&self, next_pos: Position, food_pos: Position) -> bool {
        next_pos == food_pos
    }

    fn move_next(&mut self) {
        for i in (1..self.body.len()).rev() {
            self.body[i].position = self.body[i - 1].position;
        }
        self.body[0].position = self.next_head_pos();
    }

    fn eat_next(&mut self, pos: &mut Position) {
        let head = Block {
            position: *pos,
            colour: YELLOW,
        };
        self.body.push_front(head);
        self.body[1].colour = GREEN;
    }
}

pub struct Game {
    pub snake: Snake,
    pub food: Block,
    pub time: u32,
    pub score: u32,
    pub board: ExtendedGrid,
    pub piece: Piece,
    pub game_over: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            snake: Snake::new(),
            food: Block {
                position: Position::new(),
                colour: RED,
            },
            time: 0,
            score: 0,
            board: ExtendedGrid::default(),
            piece: Piece::random(),
            game_over: false,
        }
    }

    pub fn init(&mut self) {
        self.snake = Snake::new();
        self.food.position = self.get_food_pos();
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
                self.save();
                self.piece = Piece::random();
            }
            println!("Collision");
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
                println!("Test: {:?}", b);
                if self.board.0[b.position.x as usize][b.position.y as usize] == 0xFF {
                    return true;
                }
            }
        }
        false
    }

    pub fn next_tick(&mut self, _dt: f64) {
        if self.game_over {
            return;
        }
        let mut next_piece = self.piece.clone();
        next_piece.pos.offset(0, 1);
        println!("Current: {:?}", self.piece);
        println!("Next: {:?}", next_piece);
        println!("");
        if self.check_collision(&next_piece) {
            self.save();
            self.piece = Piece::random();
            println!("Rock bottom");
            if self.check_collision(&self.piece) {
                // Game Over
                self.game_over = true;
            }
        } else {
            self.piece = next_piece;
        };

        if self.snake.alive {
            self.snake.perform_next(&mut self.food.position);
            self.time += 1;
            if self.snake.eat {
                self.score += 1;
                self.food.position = self.get_food_pos();
                self.snake.eat = false;
            }
        }
    }

    fn get_direction_from_index(&self, index: usize) -> Direction {
        match index {
            0 => Direction::RIGHT,
            1 => Direction::UP,
            2 => Direction::LEFT,
            3 => Direction::DOWN,
            _ => self.snake.direction,
        }
    }

    fn get_food_pos(&mut self) -> Position {
        let mut rng = rand::thread_rng();
        loop {
            let pos = Position {
                x: rng.gen_range(0..BOARD_WIDTH),
                y: rng.gen_range(0..BOARD_HEIGHT),
            };
            if !self.snake.check_collide_body(pos) {
                return pos;
            }
        }
    }

    fn get_food_dist(&self) -> i64 {
        let dist_x = (self.snake.body[0].position.x as i64 - self.food.position.x as i64).abs();
        let dist_y = (self.snake.body[0].position.y as i64 - self.food.position.y as i64).abs();
        dist_x + dist_y
    }

    fn get_pos_dead(&self, pos: Position) -> f64 {
        if self.snake.check_collide_wall(pos) || self.snake.check_collide_body(pos) {
            1f64
        } else {
            0f64
        }
    }
}
