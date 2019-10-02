mod ball;
mod ocean;

use ggez::{Context, GameResult, graphics, timer};
use ggez::event::EventHandler;
use ggez::nalgebra::{Point2, Vector2};
use ggez::graphics::Rect;
use ball::Ball;
use bbggez::Utility;
use rand::prelude::*;
use ocean::Ocean;

pub struct Game {
		balls: Vec<Ball>,
		utility: Utility,
		gravity: f32,
		ocean: Ocean
	}

impl Game {
	pub fn new(context: &mut Context) -> Game {
		let mut balls = vec![];
		let (arena_width, arena_height) = graphics::drawable_size(context);
		let ball_count = 10.0;
		let distance_between = arena_width / ball_count;
		let mut utility = Utility::new();
		let mut rng = rand::thread_rng();

		for count in 0..10 {
			let mass = rng.gen_range(10.0, 40.0);
			balls.push(Ball::new(Vector2::new(count as f32 * distance_between + mass, 50.0), mass, utility.random_bright_color(), context));
		}

		Game {
			balls,
			utility,
			gravity: 5.0,
			ocean: Ocean::new(Rect::new(0.0, arena_height / 2.0, arena_width, arena_height / 2.0), context)
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
			let gravity = Vector2::new(0.0, self.gravity * ball.mass);
			ball.apply_force(gravity, delta_time);

			ball.update();
			ball.bounce(arena_size.1);

			if self.ocean.is_inside(ball) {
				self.ocean.resist_mover(ball, delta_time);
			}
		}

		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);

		for ball in &mut self.balls {
			graphics::draw(context, &ball.mesh, (Point2::from(ball.location),))?;
		}

		graphics::draw(context, &self.ocean.mesh, (Point2::new(0.0, 0.0),))?;

		graphics::present(context)
	}
}