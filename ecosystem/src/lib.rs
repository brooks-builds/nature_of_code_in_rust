mod food;
mod random_walker;

use ggez::{Context, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::nalgebra::Point2;
use rand::prelude::*;
use food::*;
use random_walker::*;

pub struct Game {
	window_width: f32,
	window_height: f32,
	rng: ThreadRng,
	foods: Vec<Food>,
	random_walker: RandomWalker
}

impl Game {
	pub fn new(context: &mut Context) -> Game {
		let (window_width, window_height) = graphics::drawable_size(context);
		let mut foods = vec![];
		let rng = rand::thread_rng();

		for _ in 0..10 {
			foods.push(Food::new(window_width, window_height, rng))
		}

		Game {
			window_width,
			window_height,
			rng,
			foods,
			random_walker: RandomWalker::new(window_width, window_height, rng)
		}
	}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		self.random_walker.update(self.rng, self.window_width, self.window_height);

		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);

		for food in &mut self.foods {
			let food_mesh = food.draw(context)?;

			graphics::draw(context, &food_mesh, (Point2::new(0.0, 0.0),))?;
		}

		let random_walker = self.random_walker.draw(context)?;

		graphics::draw(context, &random_walker, (Point2::new(0.0, 0.0),))?;

		graphics::present(context)
	}
}
