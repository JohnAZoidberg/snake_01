use crate::constants::*;
use crate::game::*;

const WIDTH: usize = BOARD_WIDTH as usize;
const HEIGHT: usize = BOARD_HEIGHT as usize;
const WIDTH_U8: u8 = BOARD_WIDTH as u8;
const HEIGHT_U8: u8 = BOARD_HEIGHT as u8;

// --

#[derive(Clone, Debug, Default)]
struct Score {
    upper: usize,
    lower: usize,
}

type Velocity = (i8, i8);

#[derive(Clone)]
struct Ball {
    pos: Position,
    // Not a position, more like a directional vector
    direction: Velocity,
}

#[derive(Clone)]
pub struct PongG {
    score: Score,
    ball: Ball,
    paddles: (usize, usize),
    pub speed: u64,
}

impl GameT for PongG {
    fn new() -> PongG {
        PongG {
            score: Score::default(),
            ball: Ball {
                pos: Position::new(4, 20),
                direction: (0, 1),
            },
            paddles: (PADDLE_WIDTH / 2, PADDLE_WIDTH / 2),
            speed: 0,
        }
    }
    fn init(&mut self) {
        self.score = Score::default();
        self.ball = Ball {
            pos: Position::new(4, 20),
            direction: (0, 1),
        };
    }

    fn update(&mut self, dir: Direction) {
        match dir {
            Direction::RIGHT => {
                if self.paddles.0 + PADDLE_WIDTH < WIDTH {
                    self.paddles.0 += 1;
                }
            }
            Direction::LEFT => {
                if self.paddles.0 >= 1 {
                    self.paddles.0 -= 1;
                }
            }
            Direction::SECOND_RIGHT => {
                if self.paddles.1 + PADDLE_WIDTH < WIDTH {
                    self.paddles.1 += 1;
                }
            }
            Direction::SECOND_LEFT => {
                if self.paddles.1 >= 1 {
                    self.paddles.1 -= 1;
                }
            }
            Direction::UP | Direction::DOWN => (),
            Direction::SECOND_UP | Direction::SECOND_DOWN => (),
        }
    }

    fn next_tick(&mut self, _dt: f64) {
        self.ball.pos = {
            let (vx, vy) = self.ball.direction;
            let new_pos = add_velocity(self.ball.pos, self.ball.direction);
            let (x, y) = (new_pos.x, new_pos.y);
            let x = if x > WIDTH_U8 - 1 { WIDTH_U8 - 1 } else { x };
            if x == 0 || x == WIDTH_U8 - 1 {
                // Hit wall, bounce back
                self.ball.direction = (-vx, vy);
            }

            let y = if y > HEIGHT_U8 - 1 { HEIGHT_U8 - 1 } else { y };
            let (x, y) = if let Some(paddle_hit) = hit_paddle((x, y), self.paddles) {
                // Hit paddle, bounce back
                let (vx, vy) = self.ball.direction;
                self.ball.direction = match (vx, paddle_hit) {
                    // In the middle it bounces vertically
                    (_, 2) => (0, -vy),

                    // On the far side of the paddle, the ball bounces off as in real life
                    (-1, 0) => (vx, -vy),
                    (-1, 1) => (vx, -vy),
                    (1, 3) => (vx, -vy),
                    (1, 4) => (vx, -vy),

                    // On the close side of the paddle, the ball bounces back to where it came from
                    (1, 0) => (-vx, -vy),
                    (1, 1) => (-vx, -vy),
                    (-1, 3) => (-vx, -vy),
                    (-1, 4) => (-vx, -vy),

                    // A vertical ball will bounce towards the side where it hit the paddle
                    (0, 0) => (-1, -vy),
                    (0, 1) => (-1, -vy),
                    (0, 3) => (1, -vy),
                    (0, 4) => (1, -vy),

                    (_, _) => unimplemented!("vx:{vx}, p:{paddle_hit}"),
                };
                // println!("P: {}, Dir: {:?}", paddle_hit, self.ball.direction);
                // TODO: Not sure if I want the speed to change. Speed by angle change is already high enough
                //self.speed += 1;
                (x, y)
            } else if y == 0 || y == HEIGHT_U8 - 1 {
                if y == 0 {
                    self.score.lower += 1;
                } else if y == HEIGHT_U8 - 1 {
                    self.score.upper += 1;
                }
                // println!("Score: {:?}", self.score);
                // Hit top of bottom, missed the paddle
                self.speed = 0;
                self.ball.direction = (1, 1); //random_v(random);
                (WIDTH_U8 / 2, HEIGHT_U8 / 2)
            } else {
                (x, y)
            };
            Position::new(x as i8, y as i8)
        };
    }

    fn ball(&self) -> Option<Block> {
        Some(Block {
            position: self.ball.pos,
            colour: Colour::Green,
        })
    }

    fn paddle_blocks(&self) -> Vec<Block> {
        let mut blocks = vec![
            Block {
                position: Position::new(0, 0),
                colour: Colour::Yellow,
            };
            PADDLE_WIDTH * 2
        ];

        for x in 0..PADDLE_WIDTH {
            blocks[x].position = Position::new(self.paddles.0 as i8 + x as i8, 0);
            blocks[x].colour = Colour::Green;
        }
        for x in 0..PADDLE_WIDTH {
            blocks[x + PADDLE_WIDTH].position = Position::new(self.paddles.1 as i8 + x as i8, (HEIGHT - 1) as i8);
            blocks[x + PADDLE_WIDTH].colour = Colour::Green;
        }

        blocks
    }
}

fn add_velocity(pos: Position, v: Velocity) -> Position {
    let (vx, vy) = v;
    let (x, y) = (pos.x, pos.y);
    Position::new(((x as i8) + vx) as i8, ((y as i8) + vy) as i8)
}

fn hit_paddle(ball: (u8, u8), paddles: (usize, usize)) -> Option<usize> {
    let (x, y) = ball;
    if y <= 1 && paddles.0 <= (x as usize) && (x as usize) < paddles.0 + PADDLE_WIDTH {
        // println!("Hit upper paddle");
        Some(((paddles.0 as i32) - (x as i32)).unsigned_abs() as usize)
    } else if y >= HEIGHT_U8 - 2 && paddles.1 <= (x as usize) && (x as usize) < paddles.1 + PADDLE_WIDTH {
        // println!("Hit lower paddle P:{}, X:{}", paddles.1, x);
        Some(((paddles.1 as i32) - (x as i32)).unsigned_abs() as usize)
    } else {
        None
    }
}

// fn draw_matrix(state: &PongG) -> Grid {
//     let mut grid = Grid::default();
//
//     for x in state.paddles.0..state.paddles.0 + PADDLE_WIDTH {
//         grid.0[x][0] = 0xFF;
//     }
//     for x in state.paddles.1..state.paddles.1 + PADDLE_WIDTH {
//         grid.0[x][HEIGHT - 1] = 0xFF;
//     }
//     grid.0[state.ball.pos.0][state.ball.pos.1] = 0xFF;
//
//     grid
// }
