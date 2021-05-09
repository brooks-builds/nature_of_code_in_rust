use eyre::Result;
use ggez::event::EventHandler;
use ggez::graphics::{self, Color, BLACK};
use ggez::{timer, Context, GameResult};
use noise::Perlin;
use walker::Walker;

mod utilities;
mod walker;

pub struct MainState {
    background_color: Color,
    walker: Walker,
    noise: Perlin,
}

impl MainState {
    pub fn new(context: &mut Context) -> Result<Self> {
        let background_color = BLACK;
        let walker = Walker::new(context)?;
        let noise = Perlin::new();

        Ok(Self {
            background_color,
            walker,
            noise,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        while timer::check_update_time(context, 60) {
            let (screen_width, screen_height) = graphics::drawable_size(context);
            self.walker.step(&self.noise, screen_width, screen_height);
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);
        self.walker.draw(context).unwrap();
        graphics::present(context)
    }
}
