extern crate rand;

use rand::Rng;
use std::collections::VecDeque;
use std::fmt;

use crate::constants::*;

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    shape: u8,
    rotation: Direction,
    pos: Position,
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

    fn new_offset(x: i8, y: i8) -> Position {
        let mut pos = Position::new();
        pos.offset(x, y);
        pos
    }

    fn offset(&mut self, x: i8, y: i8) {
        self.x = Position::calc_offset(self.x, x, BOARD_WIDTH);
        self.y = Position::calc_offset(self.y, y, BOARD_HEIGHT);
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
    fn opposite(&mut self) -> Direction {
        match self {
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
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
        }
    }

    pub fn init(&mut self) {
        self.snake = Snake::new();
        self.food.position = self.get_food_pos();
        self.time = 0;
        self.score = 0;
    }

    pub fn update(&mut self, dir: Direction) {
        self.snake.update(dir);
    }

    pub fn next_tick(&mut self, _dt: f64) {
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

    pub fn get_nn_inputs(&self) -> Vec<f64> {
        let head_pos = self.snake.body[0].position;
        let food_pos = self.food.position;

        let mut pos_right = head_pos;
        pos_right.offset(1, 0);
        let right_dead = self.get_pos_dead(pos_right);
        let right_food = if food_pos.y == head_pos.y && food_pos.x > head_pos.x {
            1f64
        } else {
            0f64
        };

        let mut pos_up = head_pos;
        pos_up.offset(0, -1);
        let up_dead = self.get_pos_dead(pos_up);
        let up_food = if food_pos.x == head_pos.x && food_pos.y > head_pos.y {
            1f64
        } else {
            0f64
        };

        let mut pos_left = head_pos;
        pos_left.offset(-1, 0);
        let left_dead = self.get_pos_dead(pos_left);
        let left_food = if food_pos.y == head_pos.y && food_pos.x < head_pos.x {
            1f64
        } else {
            0f64
        };

        let mut pos_down = head_pos;
        pos_down.offset(0, 1);
        let down_dead = self.get_pos_dead(pos_down);
        let down_food = if food_pos.x == head_pos.x && food_pos.y < head_pos.y {
            1f64
        } else {
            0f64
        };

        vec![
            right_dead, right_food, up_dead, up_food, left_dead, left_food, down_dead, down_food,
        ]
    }

    fn get_pos_dead(&self, pos: Position) -> f64 {
        if self.snake.check_collide_wall(pos) || self.snake.check_collide_body(pos) {
            1f64
        } else {
            0f64
        }
    }
}
