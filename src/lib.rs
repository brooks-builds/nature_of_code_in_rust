use ggez::event::EventHandler;
use ggez::graphics::{self, Color, BLACK};
use ggez::{Context, GameResult};
use walker::Walker;

mod utilities;
mod walker;

pub struct MainState {
    background_color: Color,
    walker: Walker,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = BLACK;
        let (width, height) = graphics::drawable_size(context);
        let walker = Walker::new(width / 2.0, height / 2.0);

        Ok(Self {
            background_color,
            walker,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        self.walker.update(context).unwrap();
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);
        self.walker.draw(context).unwrap();
        graphics::present(context)
    }
}
