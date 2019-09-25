pub mod food;

use food::Food;
use ggez::{GameResult, Context};
use ggez::graphics::{Mesh};
use rand::prelude::*;

pub struct Foods {
	pub foods: Vec<Food>,
	add_food_after: usize,
	max_food_count: usize
}

impl Foods {
	pub fn new(arena_size: (f32, f32), rng: &mut ThreadRng) -> Foods {
		let mut foods = vec![];

		for _ in 0..5 {
			foods.push(Food::new(arena_size, rng));
		}

		Foods {
			foods,
			add_food_after: 500,
			max_food_count: 20
		}
	}

	pub fn update(&mut self, ticks: usize, arena_size: (f32, f32), rng: &mut ThreadRng) {
		if ticks % self.add_food_after == 0 && self.foods.len() < self.max_food_count {
			self.foods.push(Food::new(arena_size, rng));
		}

		for food in &mut self.foods {
			food.update(ticks);
		}

		self.foods = self.remove_rotton_food(self.foods.clone());
	}

	pub fn draw(&mut self, context: &mut Context) -> Vec<GameResult<Mesh>> {
		let mut food_meshes = vec![];

		for food in &mut self.foods {
			food_meshes.push(food.draw(context));
		}

		food_meshes
	}

	fn remove_rotton_food(&self, foods: Vec<Food>) -> Vec<Food> {
		foods.into_iter().filter(|food| !food.is_rotton()).collect()
	}
}