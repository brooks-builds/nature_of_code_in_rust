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
	is_first_tick: bool,
}

impl Game {
	pub fn new() -> Game {

		Game {
			foods: Foods::new(),
			walkers: Walkers::new(),
			utility: Utility::new(),
			is_first_tick: true,
		}
	}

	fn handle_screen_resizing(&mut self, arena_size: (f32, f32), context: &mut Context) -> GameResult<()> {
		graphics::set_screen_coordinates(context, graphics::Rect::new(0.0, 0.0, arena_size.0, arena_size.1))
	}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		let delta_time = timer::delta(context).as_nanos() as f32 / 1e9;
		let ticks = timer::ticks(context);
		let arena_size = graphics::drawable_size(context);
		self.walkers.update(arena_size, &mut self.foods.foods, timer::ticks(context), delta_time, context);
		self.foods.update(ticks, arena_size, &mut self.utility, context);

		if self.is_first_tick {
			self.walkers.create_walkers(arena_size, &mut self.utility, context);
			self.is_first_tick = false;
		}

		self.handle_screen_resizing(arena_size, context)
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);

		for food in self.foods.draw(context) {
			let food = food;

			graphics::draw(context, food, (Point2::new(0.0, 0.0),))?;
		}

		self.walkers.draw(context);

		// for (mesh, location, _name) in self.walkers.clone() {
		// 	// dbg!(name);
		// 	graphics::draw(context, &mesh, (Point2::from(location),))?;
		// }

		graphics::present(context)
	}
}
