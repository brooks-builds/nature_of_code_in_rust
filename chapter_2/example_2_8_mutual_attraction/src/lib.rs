mod planet;

use bbggez::{
	ggez::{
		Context,
		GameResult,
		graphics,
		timer,
		event::EventHandler,
		nalgebra::{Point2, Vector2},
		timer::{delta, duration_to_f64},
	},
	Utility,
};
use planet::Planet;
use rand::prelude::*;

pub struct Game {
		planets: Vec<Planet>,
		countdown: f64,
		add_planet_seconds: f64,
		utility: Utility,
		rng: ThreadRng,
		gravitational_constant: f32,
	}

impl Game {
	pub fn new() -> Game {
		Game {
			planets: vec![],
			countdown: 0.0,
			add_planet_seconds: 5.0,
			utility: Utility::new(),
			rng: rand::thread_rng(),
			gravitational_constant: 0.01,
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
		self.countdown = self.countdown - duration_to_f64(delta(context));

		if self.countdown <= 0.0 {
			let location = Vector2::new(self.rng.gen_range(0.0, arena_size.0), self.rng.gen_range(0.0, arena_size.1));
			self.planets.push(Planet::new(location, context, &mut self.utility, &mut self.rng, self.planets.len()));
			self.countdown = self.add_planet_seconds;
		}

		for planet_clone in self.planets.clone() {
			planet_clone.attract(&mut self.planets, self.gravitational_constant);
		}

		for planet in &mut self.planets {
			planet.update();
		}

		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);

		for planet in &mut self.planets {
			graphics::draw(context, &planet.mesh, (Point2::from(planet.location),))?;
		}

		graphics::present(context)
	}
}