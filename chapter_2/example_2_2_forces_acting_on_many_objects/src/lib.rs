mod ball;

use ggez::{Context, GameResult, graphics, timer};
use ggez::event::EventHandler;
use ggez::nalgebra::{Point2, Vector2};
use rand::prelude::*;
use ball::Ball;

pub struct Game {
		arena_size: (f32, f32),
		rng: ThreadRng,
		balls: Vec<Ball>,
		gravity: Vector2<f32>,
		wind: Vector2<f32>
	}

impl Game {
	pub fn new(context: &mut Context) -> Game {
		let arena_size = graphics::drawable_size(context);
		let mut rng = rand::thread_rng();
		let mut balls = vec![];

		for _ in 0..10 {
			balls.push(Ball::new(Vector2::new(50.0, 50.0), rng.gen_range(10.0, 100.0), &mut rng));
		}

		Game {
			arena_size,
			rng,
			balls,
			gravity: Vector2::new(0.0, 5.0),
			wind: Vector2::new(1.0, 0.0)
	}
}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		let delta_time = timer::delta(context).as_secs_f32(); // 0.000234

		for ball in &mut self.balls {
			ball.apply_force(self.gravity, delta_time);
			ball.apply_force(self.wind, delta_time);
			ball.update(self.arena_size);
		}
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