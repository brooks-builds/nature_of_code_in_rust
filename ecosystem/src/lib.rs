mod foods;
mod walkers;

use bbggez::{
	ggez::{Context, GameResult, graphics, timer},
	ggez::event::EventHandler,
	ggez::nalgebra::Point2,
	Utility,
};
use foods::Foods;
use walkers::Walkers;

pub struct Game {
	foods: Foods,
	walkers: Walkers,
	utility: Utility,
}

impl Game {
	pub fn new() -> Game {

		Game {
			foods: Foods::new(),
			walkers: Walkers::new(),
			utility: Utility::new(),
		}
	}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		let delta_time = timer::delta(context).as_nanos() as f32 / 1e9;
		let ticks = timer::ticks(context);
		let arena_size = graphics::drawable_size(context);
		self.walkers.update(arena_size, &mut self.foods.foods, timer::ticks(context), delta_time);
		self.foods.update(ticks, arena_size, &mut self.utility);

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
