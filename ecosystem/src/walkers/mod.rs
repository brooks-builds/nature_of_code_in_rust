mod random_walker;

use bbggez::{
	ggez::graphics::{Mesh},
	ggez::{GameResult, Context},
	Utility
};
use random_walker::RandomWalker;

use crate::foods::food::Food;

#[derive(Clone, Debug)]
enum Walker {
	RandomWalker(RandomWalker)
}

#[derive(Clone)]
pub struct Walkers {
	walkers: Vec<Walker>
}

impl Walkers {
	pub fn new() -> Walkers {
		Walkers {
			walkers: vec![],
		}
	}

	pub fn update(&mut self, arena_size: (f32, f32), foods: &mut Vec<Food>, ticks: usize, delta_time: f32, context: &mut Context) {
		self.walkers
			.iter_mut()
			.for_each(|walker| {
				match walker {
					Walker::RandomWalker(walker) => {
						walker.update(arena_size, delta_time, foods, context);
					}
				};
			});

		self.walkers = self.walkers
			.clone()
			.into_iter()
			.filter(|walker| match walker {
				Walker::RandomWalker(walker) => {
					walker.is_alive()
				}
			})
			.collect();
	}

	pub fn draw(&mut self, context: &mut Context) -> Vec<GameResult<Mesh>> {
		self.walkers.iter_mut()
			.map(|walker| {
				if let Walker::RandomWalker(walker) = walker {
					walker.draw(context)
				} else {
					unreachable!()
				}
			})
			.collect()
	}

	pub fn create_walkers(&mut self, arena_size: (f32, f32), utility: &mut Utility) {
		let walker = Walker::RandomWalker(RandomWalker::new(arena_size, utility));

		self.walkers.push(walker);
	}
}