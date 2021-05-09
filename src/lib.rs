mod walker;

use ggez::event::EventHandler;
use ggez::{graphics, Context, GameResult};
use walker::Walker;

pub struct MainState {
    walker: Walker,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let (width, height) = graphics::drawable_size(context);
        let walker = Walker::new(width / 2.0, height / 2.0, context)?;
        Ok(Self { walker })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _context: &mut ggez::Context) -> ggez::GameResult {
        self.walker.step();
        Ok(())
    }

    fn draw(&mut self, context: &mut ggez::Context) -> ggez::GameResult {
        self.walker.display(context)?;
        graphics::present(context)
    }
}
