use bbggez::{
	ggez::{
		nalgebra::Vector2,
		graphics::{
			Color,
			Mesh,
		},
		Context,
		timer::{duration_to_f64, delta},
	},
	Utility,
	rand,
	rand::prelude::*,
};

use crate::Foods;
use crate::foods::food::Food;

#[derive(Clone, Debug)]
pub struct AttractionWalker {
	pub location: Vector2<f32>,
	acceleration: Vector2<f32>,
	velocity: Vector2<f32>,
	speed: f32,
	pub health: f32,
	initial_health: f32,
	pub mesh: Mesh,
	pub name: String,
	energy_spend_rate: f32,
	friction: f32,
	timer: f64,
	lose_energy_every_seconds: f64,
	cached_target: Option<usize>,
	rng: ThreadRng,
}

impl AttractionWalker {
	pub fn new(location: Vector2<f32>, utility: &mut Utility, context: &mut Context) -> AttractionWalker {
		let initial_health = 10.0;
		let lose_energy_every_seconds = 0.1;
		let mut rng = rand::thread_rng();

		AttractionWalker {
			location,
			acceleration: Vector2::new(0.0, 0.0),
			velocity: Vector2::new(0.0, 0.0),
			speed: 2.3,
			initial_health,
			health: initial_health,
			mesh: utility.create_circle(0.0, 0.0, 1.0, Color::new(1.0, 1.0, 0.0, 1.0), context),
			name: String::from("attraction walker"),
			energy_spend_rate: 0.05,
			friction: 0.0005,
			timer: lose_energy_every_seconds,
			lose_energy_every_seconds,
			cached_target: None,
			rng,
		}
	}

	pub fn update(&mut self, foods: &mut Foods, delta_time: f32, context: &mut Context) {
		let target = self.choose_target(foods);

		if let Some(target) = target.as_ref() {
			self.move_towards(target, delta_time);
		}
		
		self.slow_down();
		self.cap_velocity();
		self.location = self.location + self.velocity;
		// stop moving if no target
		// eat food
		// wrap around screen
		self.acceleration = self.acceleration * 0.0;

		self.timer = self.timer - duration_to_f64(delta(context));
		if self.timer < 0.0 {
			self.spend_energy();
			self.timer = self.lose_energy_every_seconds;
		}
	}

	pub fn is_alive(&self) -> bool {
		self.health >= 0.0
	}

	fn choose_target<'a>(&mut self, food_controller: &'a mut Foods) -> Option<&'a Food> {
		// if let Some(id) = self.cached_target {
		// 	for food in foods {
		// 		if food.id == id {
		// 			 return Some(&food);
		// 		}
		// 	}

		// 	self.cached_target = None;
		// 	return None;
		// } else if foods.len() == 0 {
		// 	return None;
		// } else {
		// 	return Some(&foods[self.rng.gen_range(0, foods.len())]);
		// }

		if let Some(food_id) = self.cached_target {
			if food_controller.id_exists(food_id) {
				return food_controller.get_food_by_id(food_id);
			} else {
				self.cached_target = None;
				return None;
			}
		} else {
			self.cached_target = food_controller.random_id();
			return None;
		}
	}

	fn move_towards(&mut self, food: &Food, delta_time: f32) {
		let mut direction = food.location - self.location;
		
		if direction.x != 0.0 && direction.y != 0.0 {
			direction = direction.normalize();
		}

		let force = direction * (self.speed * delta_time);

		self.apply_force(force);
	}

	fn apply_force(&mut self, force: Vector2<f32>) {
		self.acceleration = self.acceleration + force;
		self.velocity = self.velocity + self.acceleration;
	}

	fn spend_energy(&mut self) {
		self.health = self.health - self.energy_spend_rate;
	}

	fn slow_down(&mut self) {
		if !self.is_standing_still() {
			let mut opposite_direction = self.velocity * -1.0;

			opposite_direction = opposite_direction.normalize();
			opposite_direction = opposite_direction * self.friction;

			self.apply_force(opposite_direction);
		}
	}

	fn is_standing_still(&self) -> bool {
		self.velocity.x.abs() < 0.00000000001 && self.velocity.y.abs() < 0.00000000001
	}

	fn cap_velocity(&mut self) {
		if self.velocity.x < -1.0 {
			self.velocity.x = -1.0;
		} else if self.velocity.x > 1.0 {
			self.velocity.x = 1.0;
		}

		if self.velocity.y < -1.0 {
			self.velocity.y = -1.0;
		} else if self.velocity.y > 1.0 {
			self.velocity.y = 1.0;
		}
	}

	pub fn eat(&mut self, foods: &mut Vec<Food>) {
		for (index, food) in foods.clone().iter().enumerate().rev() {
			let direction = food.location - self.location;
			let distance = direction.magnitude();

			if distance <= self.health {
				self.health = self.health + food.calories;
				foods.remove(index);
			}

		}
	}
}

