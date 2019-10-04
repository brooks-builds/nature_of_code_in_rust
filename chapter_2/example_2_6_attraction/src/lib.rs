mod planet;
mod satelite;

use bbggez::{
	ggez,
	ggez::{Context, GameResult, graphics, timer},
	ggez::event::EventHandler,
	ggez::nalgebra::{Point2, Vector2},
	Utility
};
use planet::Planet;
use satelite::Satelite;
use rand::prelude::*;

pub struct Game {
	planet: Planet,
	utility: Utility,
	satelites: Vec<Satelite>,
	gravitational_constant: f32,
}

impl Game {
	pub fn new(context: &mut Context) -> Game {
		let (arena_width, arena_height) = graphics::drawable_size(context);
		let mut utility = Utility::new();
		let mut satelites = vec![];
		let mut rng = rand::thread_rng();

		for _ in 0..25 {
			satelites.push(Satelite::new(Vector2::new(rng.gen_range(0.0, arena_width), rng.gen_range(0.0, arena_height)), rng.gen_range(0.5, 30.0), utility.random_bright_color(), context, &mut rng))
		}

		Game {
			planet: Planet::new(Vector2::new(arena_width / 2.0, arena_height / 2.0), 50.0, utility.random_dark_color(), context),
			utility,
			satelites,
			gravitational_constant: 1.0,
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

		for satelite in &mut self.satelites {
			self.planet.attract(self.gravitational_constant, satelite, delta_time);
			satelite.update();
		}

		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);

		graphics::draw(context, &self.planet.mesh, (Point2::from(self.planet.location),))?;

		for satelite in &mut self.satelites {
			graphics::draw(context, &satelite.mesh, (Point2::from(satelite.location),))?;
		}

		graphics::present(context)
	}
}