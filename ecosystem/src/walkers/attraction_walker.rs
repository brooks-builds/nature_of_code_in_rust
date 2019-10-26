use bbggez::{
	ggez::{
		nalgebra::Vector2,
		graphics::{
			Color,
			Mesh,
		},
		Context,
		mint,
	},
	Utility,
};

use crate::foods::food::Food;

#[derive(Clone, Debug)]
pub struct AttractionWalker {
	pub location: Vector2<f32>,
	acceleration: Vector2<f32>,
	velocity: Vector2<f32>,
	speed: f32,
	health: f32,
	initial_health: f32,
	pub mesh: Mesh,
	pub name: String,
	energy_spend_rate: f32,
	friction: f32,
}

impl AttractionWalker {
	pub fn new(location: Vector2<f32>, utility: &mut Utility, context: &mut Context) -> AttractionWalker {
		let initial_health = 10.0;

		AttractionWalker {
			location,
			acceleration: Vector2::new(0.0, 0.0),
			velocity: Vector2::new(0.0, 0.0),
			speed: 0.2,
			initial_health,
			health: initial_health,
			mesh: utility.create_circle(0.0, 0.0, 10.0, Color::new(1.0, 1.0, 0.0, 1.0), context),
			name: String::from("attraction walker"),
			energy_spend_rate: 0.0000001,
			friction: 0.0001,
		}
	}

	pub fn update(&mut self, foods: &Vec<Food>, delta_time: f32) {
		let target = self.choose_target(foods);

		if let Some(target) = target {
			self.move_towards(target, delta_time);
			self.spend_energy();
		}
		
		self.slow_down();
		self.cap_velocity();
		self.location = self.location + self.velocity;
		// stop moving if no target
		// eat food
		// wrap around screen
		self.acceleration = self.acceleration * 0.0;
	}

	pub fn is_alive(&self) -> bool {
		self.health >= 0.0
	}

	fn choose_target<'a>(&self, foods: &'a Vec<Food>) -> Option<&'a Food> {
		for food in foods {
			if !food.is_rotton() {
				return Some(&food);
			}
		}

		None
	}

	fn move_towards(&mut self, food: &Food, delta_time: f32) {
		let mut direction = food.location - self.location;
		
		direction = direction.normalize();

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
		self.velocity.x == 0.0 && self.velocity.y == 0.0
	}

	pub fn get_scale(&self) -> mint::Vector2<f32> {
		mint::Vector2{
			x: self.health / self.initial_health,
			y: self.health / self.initial_health,
		}
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
}

