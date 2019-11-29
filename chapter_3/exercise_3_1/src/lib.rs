use bbggez::ggez::{event::EventHandler, graphics, Context, GameResult};

pub struct Game {
	background_color: graphics::Color,
}

impl Game {
	pub fn new() -> Game {
		let background_color = graphics::Color::from_rgb(128, 207, 216);

		Game { background_color }
	}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, self.background_color);

		graphics::present(context)
	}
}
