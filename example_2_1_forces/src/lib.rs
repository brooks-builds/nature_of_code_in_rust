use ggez::event::EventHandler;
use ggez::graphics::{self, Color, BLACK};
use ggez::{Context, GameResult};
use mover::Mover;
use utilities::vector2::Vector2;

mod mover;
pub mod utilities;

pub struct MainState {
    background_color: Color,
    mover: Mover,
    wind_force: Vector2,
    gravity_force: Vector2,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = BLACK;
        let (width, height) = graphics::drawable_size(context);
        let mover = Mover::new(width / 2.0, height / 2.0, 10.0, context)?;
        let wind_force = Vector2::new(0.1, 0.0);
        let gravity_force = Vector2::new(0.0, 1.0);

        Ok(Self {
            background_color,
            mover,
            wind_force,
            gravity_force,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        self.mover.apply_force(self.wind_force);
        self.mover.apply_force(self.gravity_force);
        self.mover.update();
        self.mover.check_edges(context);

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);
        self.mover.display(context)?;
        graphics::present(context)
    }
}
