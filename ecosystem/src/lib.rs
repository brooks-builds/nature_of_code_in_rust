mod food;
mod random_walker;

use ggez::{Context, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::nalgebra::Point2;
use rand::prelude::*;
use food::*;
use random_walker::*;
use std::collections::HashMap;

enum Walker {
	BasicWalker(RandomWalker)
}

pub struct Game {
	window_width: f32,
	window_height: f32,
	rng: ThreadRng,
	foods: Vec<Food>,
	walkers: HashMap<u8, Walker>,
	ticks_per_energy_lost: usize
}

impl Game {
	pub fn new(context: &mut Context) -> Game {
		let (window_width, window_height) = graphics::drawable_size(context);
		let mut foods = vec![];
		let rng = rand::thread_rng();
		let mut walkers = HashMap::new();

		for _ in 0..100 {
			foods.push(Food::new(window_width, window_height, rng))
		}

		for id in 0..5 {
			walkers.insert(id, Walker::BasicWalker(RandomWalker::new(window_width, window_height, rng, id)));
		}

		Game {
			window_width,
			window_height,
			rng,
			foods,
			walkers,
			ticks_per_energy_lost: 100
		}
	}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		let mut dead_ids = vec![];

		for (id, walker) in self.walkers.iter_mut() {
			if let Walker::BasicWalker(basic_walker) = walker {
				basic_walker.walk(self.rng, self.window_width, self.window_height);
				basic_walker.eat(&mut self.foods);

				if ggez::timer::ticks(context) % self.ticks_per_energy_lost == 0 {
					basic_walker.spend_energy();

					if basic_walker.is_dead() {
						dead_ids.push(id);
					}
				}
			}
		}

		for dead_id in &dead_ids {
			self.walkers.remove(dead_id);
		}

		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);

		for food in &mut self.foods {
			let food_mesh = food.draw(context)?;

			graphics::draw(context, &food_mesh, (Point2::new(0.0, 0.0),))?;
		}

		for (id, walker) in self.walkers.iter_mut() {
			if let Walker::BasicWalker(basic_walker) = walker {
				let random_walker = basic_walker.draw(context)?;
				
				graphics::draw(context, &random_walker, (Point2::new(0.0, 0.0),))?;
			}
		}

		graphics::present(context)
	}
}
