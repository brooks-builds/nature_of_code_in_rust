use ggez::event::EventHandler;
use ggez::graphics::{self, Color, BLACK};
use ggez::{timer, Context, GameResult};
use mover::Mover;
use utilities::vector2::Vector2;
use zone::Zone;

mod mover;
mod utilities;
mod zone;

pub struct MainState {
    mover: Mover,
    gravity: Vector2,
    sky: Zone,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let (width, height) = graphics::drawable_size(context);
        let gravity = Vector2::new(0.0, 0.001);
        let sky = Zone::new(
            0.0,
            0.0,
            width,
            height,
            Color::new(0.0, 0.0, 1.0, 0.3),
            0.0025,
        );

        let mover = Mover::new(Vector2::new(50.0, height / 2.0), 1.0, context, 0.001)?;

        Ok(Self {
            mover,
            gravity,
            sky,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let drawable_size = graphics::drawable_size(context);
        while timer::check_update_time(context, 60) {
            self.mover.handle_input(context);
            self.mover.fly();
            let mut gravity = self.gravity;
            gravity *= self.mover.mass;
            self.mover.apply_force(gravity);
            if self.mover.is_inside_zone(&self.sky) {
                self.mover.apply_lift(&self.sky);
            }
            self.mover.update();
            self.mover.check_edges(drawable_size);
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, BLACK);
        self.mover.draw(context)?;
        self.sky.draw(context)?;

        graphics::present(context)
    }
}
