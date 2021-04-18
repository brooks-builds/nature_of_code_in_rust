use ggez::event::EventHandler;
use ggez::graphics::{self, Color, BLACK};
use ggez::{Context, GameResult};

mod mover;
mod utilities;

pub struct MainState {
    background_color: Color,
}

impl MainState {
    pub fn new(_context: &mut Context) -> GameResult<Self> {
        let background_color = BLACK;

        Ok(Self { background_color })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);
        graphics::present(context)
    }
}
