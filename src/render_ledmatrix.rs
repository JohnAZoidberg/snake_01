use crate::constants::*;
use crate::game::{Block, Brain, Direction, Game};
use crate::ledmatrix::*;

use piston_window::*;
use std::time::Duration;

pub const WIDTH: usize = 9;
pub const HEIGHT: usize = 34;
const SERIAL_TIMEOUT: Duration = Duration::from_millis(20);

#[derive(Clone)]
pub struct Grid(pub [[u8; HEIGHT]; WIDTH]);
impl Default for Grid {
    fn default() -> Self {
        Grid([[0; HEIGHT]; WIDTH])
    }
}

pub struct Render {
    window: PistonWindow,
    events: Events,
    serialdev: String,
    grid: Grid,
}

impl Render {
    pub fn new() -> Render {
        let res = find_serialdevs(true);
        println!("Devs: {:?}", res);

        let (devs, _waited) = res;
        Render {
            window: WindowSettings::new(
                NAME,
                [BOARD_WIDTH as u32 * BLOCK_SIZE, BOARD_HEIGHT as u32 * BLOCK_SIZE],
            )
            .graphics_api(OpenGL::V3_2)
            .vsync(true)
            .exit_on_esc(true)
            .build()
            .unwrap(),
            events: Events::new(EventSettings::new().ups(RENDER_UPS).max_fps(RENDER_FPS_MAX)),
            // ledmatrix
            serialdev: devs[0].to_string(),
            grid: Grid::default(),
        }
    }

    pub fn run(&mut self) {
        let mut game = Game::new();
        game.init();

        while let Some(e) = self.events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render_game(&args, &game);
            }

            if let Some(args) = e.update_args() {
                game.next_tick(args.dt);
            }

            if let Some(button) = e.press_args() {
                self.handle_events(button, &mut game);
            }
        }
    }

    pub fn run_brain<T: Brain>(&mut self, brain: &mut T) {
        let mut game = Game::new();
        game.init();

        while let Some(e) = self.events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render_game(&args, &game);
            }

            if let Some(args) = e.update_args() {
                let dir = game.get_dir_from_brain(brain);
                game.update(dir);
                game.next_tick(args.dt);
            }

            if let Some(button) = e.press_args() {
                self.handle_events(button, &mut game);
            }
        }
    }

    fn handle_events(&mut self, button: Button, game: &mut Game) {
        match button {
            Button::Keyboard(key) => match key {
                Key::Up => game.update(Direction::UP),
                Key::Down => game.update(Direction::DOWN),
                Key::Left => game.update(Direction::LEFT),
                Key::Right => game.update(Direction::RIGHT),
                Key::Space => game.init(),
                _ => {}
            },
            _ => {}
        }
    }

    fn render_game(&mut self, _args: &RenderArgs, game: &Game) {
        self.grid = Grid::default();

        for b in game.snake.body.iter() {
            self.render_block(&b);
        }

        self.render_block(&game.food);

        render_matrix(&self.serialdev, &self.grid.0);
        //let mut port = serialport::new(&self.serialdev, 115_200)
        //.timeout(SERIAL_TIMEOUT)
        //.open()
        //.expect("Failed to open port");

        //for (x, col) in self.grid.0.into_iter().enumerate() {
        //    send_col(&mut port, x as u8, col);
        //}
        //commit_cols(&mut port);
    }

    fn render_block(&mut self, block: &Block) {
        // println!("X: {:?}, Y: {:?}, Color: {:?}", block.position.x, block.position.y, block.colour);
        let x = block.position.x as usize;
        let y = block.position.y as usize;
        self.grid.0[x][y] = match block.colour {
            // Red
            [1.0, 0.0, 0.0, 1.0] => 0xFF,
            // Yellow
            [1.0, 1.0, 0.0, 1.0] => 0xFF,
            // Green
            [0.0, 1.0, 0.0, 1.0] => 0xFF,
            // Other
            _ => 0x00,
        };
    }
}
