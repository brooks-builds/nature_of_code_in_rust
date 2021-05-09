use ggez::event::EventHandler;
use ggez::graphics::{self, Color, BLACK};
use ggez::{Context, GameResult};
use mover::Mover;
use noise::{NoiseFn, Perlin};
use utilities::vector2::Vector2;

mod mover;
pub mod utilities;

pub struct MainState {
    background_color: Color,
    mover: Mover,
    up_force: Vector2,
    noise: Perlin,
    x_off: f64,
    y_off: f64,
    noise_increment_by: f64,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = BLACK;
        let (width, height) = graphics::drawable_size(context);
        let mover = Mover::new(width / 2.0, height / 2.0, 10.0, context)?;
        let up_force = Vector2::new(0.0, -0.5);
        let noise = Perlin::new();
        let x_off = 0.0;
        let y_off = 10_000.0;
        let noise_increment_by = 0.01;

        Ok(Self {
            background_color,
            mover,
            up_force,
            noise,
            x_off,
            y_off,
            noise_increment_by,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let wind_force = Vector2::new(
            self.noise.get([self.x_off, self.x_off]) as f32,
            self.noise.get([self.y_off, self.y_off]) as f32,
        );
        self.mover.apply_force(wind_force);
        self.mover.apply_force(self.up_force);
        self.mover.update();
        self.mover.check_edges(context);

        self.x_off += self.noise_increment_by;
        self.y_off += self.noise_increment_by;

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);
        self.mover.display(context)?;
        graphics::present(context)
    }
}
