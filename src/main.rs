use ggez::{Context, event, GameError, GameResult, graphics};
use ggez::event::EventHandler;
use crate::grid::Grid;
use crate::point::Point;

mod grid;
mod cell;
mod point;

/// Config for the start of the game
#[derive(Debug, Clone)]
pub struct Config {
    pub grid_size: (usize, usize),
    pub cell_size: f32,
    pub screen_size: (f32, f32),
    pub fps: u32,
}

struct State {
    grid: Grid,
    config: Config,
}

impl State {
    pub fn new(config: Config) -> Self {
        let mut grid = Grid::new(config.grid_size.0, config.grid_size.1);

        State {
            grid,
            config,
        }
    }
}

impl EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        while _ctx.time.check_update_time(20) {
            self.grid.update()
        }
        Ok(())

    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas =
            graphics::Canvas::from_frame(_ctx, graphics::Color::from([0.0, 1.0, 0.0, 1.0]));

        for (idx, cell) in self.grid.cells.iter().enumerate() {
            if let Some(point) = self.grid.point_from_idx(idx) {
                if cell.is_alive() {
                    canvas.draw(
                        &graphics::Quad,
                        graphics::DrawParam::new()
                            .dest_rect(graphics::Rect::new(
                                point.x as f32 * self.config.cell_size,
                                point.y as f32 * self.config.cell_size,
                                self.config.cell_size,
                                self.config.cell_size,
                            ), )
                            .color([1.0, 0.5, 0.0, 1.0]),
                    );
                }
            }
        }

        canvas.finish(_ctx)?;

        // We yield the current thread until the next update
        ggez::timer::yield_now();

        Ok(())
    }
}


#[warn(unreachable_code)]
fn main() -> GameResult {
    let (ctx, events_loop) = ggez::ContextBuilder::new("Conway", "Sergio")
        .window_setup(ggez::conf::WindowSetup::default().title("Conway!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(2048.0, 2048.0))
        .build()?;

    let conf = Config {
        screen_size: (2048., 2048.),
        fps: 8,
        grid_size: (256, 256),
        cell_size: 2048.0 / 256.0,
    };

    let state = State::new(conf);
    event::run(ctx, events_loop, state)
}