mod random_walker;

use ggez::graphics::{Mesh};
use ggez::{GameResult, Context};
use random_walker::RandomWalker;
use rand::prelude::*;

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
	pub fn new(arena_size: (f32, f32), rng: ThreadRng) -> Walkers {
		Walkers {
			walkers: vec![
				Walker::RandomWalker(RandomWalker::new(arena_size.0, arena_size.1, rng)),
				Walker::RandomWalker(RandomWalker::new(arena_size.0, arena_size.1, rng)),
				Walker::RandomWalker(RandomWalker::new(arena_size.0, arena_size.1, rng)),
				Walker::RandomWalker(RandomWalker::new(arena_size.0, arena_size.1, rng)),
				Walker::RandomWalker(RandomWalker::new(arena_size.0, arena_size.1, rng))
			]
		}
	}

	pub fn update(&mut self, rng: ThreadRng, arena_size: (f32, f32), foods: &mut Vec<Food>, ticks: usize, delta_time: f32) {
		self.walkers
			.iter_mut()
			.for_each(|walker| {
				match walker {
					Walker::RandomWalker(walker) => {
						walker.walk(rng, arena_size.0, arena_size.1, delta_time);
						walker.eat(foods);

						if ticks % 100 == 0 {
							walker.spend_energy();
						}
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
}