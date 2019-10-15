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
};

pub struct Foods {
	pub foods: Vec<Food>,
	add_every_seconds: f64,
	timer: f64,
	max_food_count: usize,
}

impl Foods {
	pub fn new() -> Foods {
		let add_every_seconds = 3.0;

		Foods {
			add_every_seconds,
			foods: vec![],
			max_food_count: 20,
			timer: add_every_seconds,
		}
	}

	pub fn update(&mut self, ticks: usize, arena_size: (f32, f32), utility: &mut Utility, context: &mut Context) {
		self.timer = self.timer - duration_to_f64(delta(context));

		if self.timer <= 0.0 && self.foods.len() < self.max_food_count {
			self.foods.push(Food::new(arena_size, utility, context));
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
}