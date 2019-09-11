mod ball;

use ggez::{Context, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::nalgebra::Point2;
use ball::*;

pub struct Game {
	window_width: f32,
	window_height: f32,
	ball: Ball
}

impl Game {
	pub fn new(context: &mut Context) -> Game {
		let (window_width, window_height) = graphics::drawable_size(context);
		let location = Point2::new(window_width / 2.0, window_height / 2.0);
		let ball = Ball::new(location);

		Game {
			window_width,
			window_height,
			ball
		}
	}
}

impl EventHandler for Game {
	fn update(&mut self, _context: &mut Context) -> GameResult<()> {
		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);

		let mesh = graphics::MeshBuilder::new()
			.circle(graphics::DrawMode::fill(), self.ball.location, self.ball.radius, 0.01, self.ball.color)
			.build(context)?;

		graphics::draw(context, &mesh, (Point2::new(0.0, 0.0),))?;

		graphics::present(context)
	}
}