mod ball;

use ggez::{Context, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::nalgebra::{Point2, Vector2};
use rand::prelude::*;
use ball::Ball;

pub struct Game {
		arena_size: (f32, f32),
		rng: ThreadRng,
		balls: Vec<Ball>
	}

impl Game {
	pub fn new(context: &mut Context) -> Game {
		let arena_size = graphics::drawable_size(context);
		let mut rng = rand::thread_rng();
		let mut balls = vec![];

		for _ in 0..1 {
			balls.push(Ball::new(Vector2::new(50.0, 50.0), rng.gen_range(1.0, 25.0), &mut rng));
		}

		Game {
			arena_size,
			rng,
			balls
	}
}
}

impl EventHandler for Game {
	fn update(&mut self, _context: &mut Context) -> GameResult<()> {
		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);

		for ball in &mut self.balls {
			let mesh = ball.create_mesh(context)?;

			graphics::draw(context, &mesh, (Point2::new(0.0, 0.0),))?;
		}

		graphics::present(context)
	}
}