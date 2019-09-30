mod ball;

use ggez::{Context, GameResult, graphics, timer};
use ggez::event::EventHandler;
use ggez::nalgebra::{Point2, Vector2};
use rand::prelude::*;
use ball::Ball;
use bbggez::Utility;

pub struct Game {
		rng: ThreadRng,
		balls: Vec<Ball>,
		utility: Utility,
		gravity: Vector2<f32>,
		wind: Vector2<f32>,
		friction_coeficient: f32
	}

impl Game {
	pub fn new(context: &mut Context) -> Game {
		let mut rng = rand::thread_rng();
		let mut balls = vec![];
		let mut utility = Utility::new();

		for _ in 0..25 {
			balls.push(Ball::new(Vector2::new(100.0, 100.0), rng.gen_range(10.0, 75.0), utility.random_bright_color(), context));
		}

		Game {
			rng,
			balls,
			utility,
			gravity: Vector2::new(0.0, 10.0),
			wind: Vector2::new(0.5, 0.0),
			friction_coeficient: 20.0
		}
	}

	fn handle_window_size_change(&self, context: &mut Context, (width, height): (f32, f32)) -> GameResult<()> {
		graphics::set_screen_coordinates(context, graphics::Rect::new(0.0, 0.0, width, height))
	}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		let delta_time = timer::delta(context).as_secs_f32();
		let arena_size = graphics::drawable_size(context);
		self.handle_window_size_change(context, arena_size)?;

		for ball in &mut self.balls {
			let mut friction = ball.velocity * -1.0;

			friction.normalize();
			friction = friction * self.friction_coeficient;

			ball.apply_force(friction, delta_time);
			ball.apply_force(self.gravity, delta_time);
			ball.apply_force(self.wind, delta_time);

			ball.update();
			ball.bounce(arena_size);
		}

		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);

		for ball in &self.balls {
			graphics::draw(context, &ball.mesh, (Point2::from(ball.location),))?;
		}

		graphics::present(context)
	}
}