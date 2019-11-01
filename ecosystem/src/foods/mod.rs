pub mod food;

use food::Food;
use bbggez::{
	ggez::{
		GameResult, 
		Context,
		timer::{duration_to_f64, delta},
	},
	ggez::graphics::{Mesh},
	Utility,
	rand,
	rand::prelude::*,
};

pub struct Foods {
	pub foods: Vec<Food>,
	add_every_seconds: f64,
	timer: f64,
	max_food_count: usize,
	next_id: usize,
	rng: ThreadRng,
}

impl Foods {
	pub fn new() -> Foods {
		let add_every_seconds = 1.0;

		Foods {
			add_every_seconds,
			foods: vec![],
			max_food_count: 20,
			timer: add_every_seconds,
			next_id: 0,
			rng: rand::thread_rng(),
		}
	}

	pub fn update(&mut self, ticks: usize, arena_size: (f32, f32), utility: &mut Utility, context: &mut Context) {
		self.timer = self.timer - duration_to_f64(delta(context));

		if self.timer <= 0.0 && self.foods.len() < self.max_food_count {
			self.foods.push(Food::new(arena_size, utility, context, self.next_id));
			self.next_id = self.next_id + 1;
			self.timer = self.add_every_seconds;
		}

		for food in &mut self.foods {
			food.update(context);
		}

		self.foods = self.remove_rotton_food(self.foods.clone());
	}

	pub fn draw(&mut self, context: &mut Context) -> Vec<&Mesh> {
		let mut food_meshes = vec![];

		for food in &mut self.foods {
			food_meshes.push(food.draw(context));
		}

		food_meshes
	}

	fn remove_rotton_food(&self, foods: Vec<Food>) -> Vec<Food> {
		foods.into_iter().filter(|food| !food.is_rotton()).collect()
	}

	pub fn id_exists(&self, id: usize) -> bool {
		for food in &self.foods {
			if food.id == id {
				return true;
			}
		}

		false
	}

	pub fn get_food_by_id(&self, id: usize) -> Option<&Food> {
		for food in &self.foods {
			if food.id == id {
				return Some(food);
			}
		}

		None
	}

	pub fn random_id(&mut self) -> Option<usize> {
		if self.foods.len() > 0 {
			let random_index = self.rng.gen_range(0, self.foods.len());

			Some(self.foods[random_index].id)
		} else {
			None
		}
	}
}
