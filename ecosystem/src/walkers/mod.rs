mod random_walker;
mod attraction_walker;

use bbggez::{
	ggez::{
		graphics::Mesh,
		Context,
		nalgebra::Vector2,
	},
	Utility
};
use random_walker::RandomWalker;
use attraction_walker::AttractionWalker;

use crate::foods::food::Food;

#[derive(Clone, Debug)]
enum Walker {
	RandomWalker(RandomWalker),
	AttractionWalker(AttractionWalker),
}

#[derive(Clone)]
pub struct Walkers {
	walkers: Vec<Walker>,
	starting_sizes: f32,
	walker_speed: f32,
	iterator_index: usize,
}

impl Walkers {
	pub fn new() -> Walkers {
		Walkers {
			walkers: vec![],
			starting_sizes: 10.0,
			walker_speed: 5.0,
			iterator_index: 0,
		}
	}

	pub fn update(&mut self, arena_size: (f32, f32), foods: &mut Vec<Food>, ticks: usize, delta_time: f32, context: &mut Context) {
		self.walkers
			.iter_mut()
			.for_each(|walker| {
				match walker {
					Walker::RandomWalker(walker) => {
						walker.update(arena_size, delta_time, foods, context);
					},
					Walker::AttractionWalker(walker) => walker.update(),
				};
			});

		self.walkers = self.walkers
			.clone()
			.into_iter()
			.filter(|walker| match walker {
				Walker::RandomWalker(walker) => {
					walker.is_alive()
				},
				Walker::AttractionWalker(walker) => walker.is_alive(),
			})
			.collect();
	}

	// pub fn draw(&mut self, context: &mut Context) -> Vec<Mesh> {
	// 	self.walkers.iter_mut()
	// 		.map(|walker| {
	// 			// if let Walker::RandomWalker(walker) = walker {
	// 			// 	walker.draw(context)
	// 			// } else {
	// 			// 	unreachable!()
	// 			// }
	// 			match walker {
	// 				Walker::RandomWalker(walker) => walker.draw(context).unwrap(),
	// 				Walker::AttractionWalker(walker) => walker.mesh.clone(),
	// 			}
	// 		})
	// 		.collect()
	// }

	pub fn create_walkers(&mut self, arena_size: (f32, f32), utility: &mut Utility, context: &mut Context) {
		self.walkers.push(Walker::RandomWalker(RandomWalker::new(arena_size, utility, context, self.starting_sizes)));
		self.walkers.push(Walker::AttractionWalker(AttractionWalker::new(utility.random_location(arena_size.0, arena_size.1), self.starting_sizes, self.walker_speed, utility, context)));
	}
}

impl Iterator for Walkers {
	type Item = (Mesh, Vector2<f32>);

	fn next(&mut self) -> Option<(Mesh, Vector2<f32>)> {
		let current_walker = self.walkers[self.iterator_index].clone();

		self.iterator_index = self.iterator_index + 1;

		if self.iterator_index >= self.walkers.len() {
			self.iterator_index = 0;
			None
		} else {
			match current_walker {
				Walker::RandomWalker(walker) => Some((walker.mesh, walker.location)),
				Walker::AttractionWalker(walker) => Some((walker.mesh, walker.location)),
			}
		}

	}
}