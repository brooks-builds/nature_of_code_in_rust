use ggez::event::EventHandler;
use ggez::graphics::{self, Color, BLACK};
use ggez::{Context, GameResult};
use mover::Mover;

mod mover;
mod utilities;

pub struct MainState {
    background_color: Color,
    mover: Mover,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = BLACK;
        let (width, height) = graphics::drawable_size(context);
        let mover = Mover::new(width / 2.0, height / 2.0, context)?;

        Ok(Self {
            background_color,
            mover,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
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
