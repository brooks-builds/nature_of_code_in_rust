mod foods;
mod walkers;

use ggez::{Context, GameResult, graphics, timer};
use ggez::event::EventHandler;
use ggez::nalgebra::Point2;
use rand::prelude::*;
use foods::Foods;
use walkers::Walkers;

pub struct Game {
	arena_size: (f32, f32),
	rng: ThreadRng,
	foods: Foods,
	walkers: Walkers
}

impl Game {
	pub fn new(context: &mut Context) -> Game {
		let arena_size = graphics::drawable_size(context);
		let mut rng = rand::thread_rng();

		Game {
			arena_size,
			rng,
			foods: Foods::new(arena_size, &mut rng),
			walkers: Walkers::new(arena_size, rng)
		}
	}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		let ticks = timer::ticks(context);
		self.walkers.update(self.rng, self.arena_size, &mut self.foods.foods, timer::ticks(context));
		self.foods.update(ticks, self.arena_size, &mut self.rng);

		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);

		for food in self.foods.draw(context) {
			let food = food?;

			graphics::draw(context, &food, (Point2::new(0.0, 0.0),))?;
		}

		for walker in self.walkers.draw(context) {
			let walker = walker?;

			graphics::draw(context, &walker, (Point2::new(0.0, 0.0),))?;
		}

		graphics::present(context)
	}
}
