mod food;
mod walkers;

use ggez::{Context, GameResult, graphics, timer};
use ggez::event::EventHandler;
use ggez::nalgebra::Point2;
use rand::prelude::*;
use food::*;
use walkers::Walkers;

pub struct Game {
	arena_size: (f32, f32),
	rng: ThreadRng,
	foods: Vec<Food>,
	walkers: Walkers
}

impl Game {
	pub fn new(context: &mut Context) -> Game {
		let arena_size = graphics::drawable_size(context);
		let mut foods = vec![];
		let rng = rand::thread_rng();

		for _ in 0..100 {
			foods.push(Food::new(arena_size.0, arena_size.1, rng))
		}

		Game {
			arena_size,
			rng,
			foods,
			walkers: Walkers::new(arena_size, rng)
		}
	}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		self.walkers.update(self.rng, self.arena_size, &mut self.foods, timer::ticks(context));

		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);

		for food in &mut self.foods {
			let food_mesh = food.draw(context)?;

			graphics::draw(context, &food_mesh, (Point2::new(0.0, 0.0),))?;
		}

		for walker in self.walkers.draw(context) {
			let walker = walker?;

			graphics::draw(context, &walker, (Point2::new(0.0, 0.0),))?;
		}

		graphics::present(context)
	}
}
