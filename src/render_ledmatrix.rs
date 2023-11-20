use crate::blockdrop::BlockdropG;
use crate::constants::*;
use crate::game::{Block, Direction, GameT};
use crate::ledmatrix::*;
#[cfg(feature = "snake")]
use crate::snake::{Brain, SnakeG};

use piston_window::*;
use serialport::SerialPort;
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
    serialport: Box<dyn SerialPort>,
    grid: Grid,
}

impl Render {
    pub fn new() -> Render {
        let res = find_serialdevs(true);
        println!("Devs: {:?}", res);

        let (devs, _waited) = res;
        let serialdev = devs.iter().last().unwrap().to_string();
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
            serialport: serialport::new(serialdev, 115_200)
                .timeout(SERIAL_TIMEOUT)
                .open()
                .unwrap(),
            grid: Grid::default(),
        }
    }

    pub fn run(&mut self) {
        #[cfg(feature = "blockdrop")]
        let mut game = BlockdropG::new();
        #[cfg(feature = "snake")]
        let mut game = SnakeG::new();
        game.init();

        while let Some(e) = self.events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render_game(&args, &game, &e);
            }

            if let Some(args) = e.update_args() {
                game.next_tick(args.dt);
            }

            if let Some(button) = e.press_args() {
                self.handle_events(button, &mut game);
            }
        }
    }

    #[cfg(feature = "snake")]
    pub fn run_brain<T: Brain>(&mut self, brain: &mut T) {
        let mut game = SnakeG::new();
        game.init();

        while let Some(e) = self.events.next(&mut self.window) {
            if let Some(args) = e.render_args() {
                self.render_game(&args, &game, &e);
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

    fn handle_events(&mut self, button: Button, game: &mut dyn GameT) {
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

    #[cfg(feature = "snake")]
    fn render_game(&mut self, _args: &RenderArgs, game: &SnakeG, e: &Event) {
        // Clear
        self.window.draw_2d(e, |_, g, _| {
            clear([1.0; 4], g);
        });
        self.grid = Grid::default();

        // Draw body
        for b in game.snake.body.iter() {
            self.render_block(&b, e);
        }

        // Draw food
        self.render_block(&game.food, e);

        render_matrix_port(&mut self.serialport, &self.grid.0);
    }

    #[cfg(feature = "blockdrop")]
    fn render_game(&mut self, _args: &RenderArgs, game: &GameT, e: &Event) {
        // Clear
        self.window.draw_2d(e, |_, g, _| {
            clear([1.0; 4], g);
        });
        self.grid = Grid::default();

        // Draw body
        //for b in game.snake.body.iter() {
        //    self.render_block(&b, e);
        //}

        // Draw food
        //self.render_block(&game.food, e);

        for b in game.blocks() {
            if b.colour == Colour::Green {
                self.render_block(&b, e);
            }
        }

        // Flush to matrix
        render_matrix_port(&mut self.serialport, &self.grid.0);
        //let mut port = serialport::new(&self.serialdev, 115_200)
        //.timeout(SERIAL_TIMEOUT)
        //.open()
        //.expect("Failed to open port");

        //for (x, col) in self.grid.0.into_iter().enumerate() {
        //    send_col(&mut port, x as u8, col);
        //}
        //commit_cols(&mut port);
    }

    fn render_block(&mut self, block: &Block, e: &Event) {
        self.render_block_ledmatrix(block);
        self.render_block_piston(block, e);
    }
    fn render_block_ledmatrix(&mut self, block: &Block) {
        #[cfg(feature = "blockdrop")]
        if block.colour != Colour::Green {
            return;
        }
        // println!("X: {:?}, Y: {:?}, Color: {:?}", block.position.x, block.position.y, block.colour);
        let x = block.position.x as usize;
        let y = block.position.y as usize;
        #[cfg(feature = "blockdrop")]
        let x = x - 3;
        if x >= WIDTH || y >= HEIGHT {
            // Avoid crash if out of bounds
            return;
        }
        self.grid.0[x][y] = match block.colour {
            // Red
            Colour::Red => 0xFF,
            // Yellow
            Colour::Yellow => 0xFF,
            // Green
            Colour::Green => 0xFF,
            // Other
            _ => return, //0x00,
        };
    }
    fn render_block_piston(&mut self, block: &Block, e: &Event) {
        #[cfg(feature = "blockdrop")]
        if block.colour != Colour::Green {
            return;
        }
        use graphics::math::Matrix2d;

        let x = block.position.x as usize;
        let y = block.position.y as usize;
        #[cfg(feature = "blockdrop")]
        let x = x - 3;
        // It seems piston already ignores this by itself, if you draw off-screen
        //if x >= WIDTH || y >= HEIGHT {
        //    // Avoid crash if out of bounds
        //    return;
        //}

        // TODO: Transforming after apply the border, stretches the border unequally, which we
        // don't want
        let square_ = graphics::rectangle::Rectangle::new(block.colour.into()).border(graphics::rectangle::Border {
            color: Colour::Black.into(),
            radius: 0.01,
        });
        let dims_ =
            graphics::rectangle::rectangle_by_corners(0.0, 0.0, 2.0 / BOARD_WIDTH as f64, 2.0 / BOARD_HEIGHT as f64);
        let transform_: Matrix2d = graphics::math::identity()
            .trans(
                -((BOARD_WIDTH / 2) as f64) * 2.0 / BOARD_WIDTH as f64,
                (BOARD_HEIGHT / 2 - 1) as f64 * 2.0 / BOARD_HEIGHT as f64,
            )
            .trans(
                (x as f64) * 2.0 / BOARD_WIDTH as f64,
                -(y as f64) * 2.0 / BOARD_HEIGHT as f64,
            );
        self.window.draw_2d(e, |c, g, _| {
            square_.draw(dims_, &c.draw_state, transform_, g);
        });
    }
}
