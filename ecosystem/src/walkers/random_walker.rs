use bbggez::{
	ggez::nalgebra::{Vector2, Point2},
	ggez::graphics::{Color, Mesh, MeshBuilder, DrawMode},
	ggez::{
		Context, 
		GameResult,
		timer::{duration_to_f64, delta},
	},
	rand,
	rand::prelude::*,
	Utility,
};
use crate::foods::food::Food;
use super::walker_traits::WalkerMovement;

#[derive(Clone, Debug)]
pub struct RandomWalker {
	pub location: Vector2<f32>,
	velocity: Vector2<f32>,
	acceleration: Vector2<f32>,
	pub size: f32,
	rng: ThreadRng,
	timer: f64,
	lose_energy_every_seconds: f64,
	spend_energy_rate: f32,
	pub mesh: Mesh,
	pub name: String,
}

impl RandomWalker {
	pub fn new((arena_width, arena_height): (f32, f32), utility: &mut Utility, context: &mut Context) -> RandomWalker {
		let mut rng = rand::thread_rng();
		let lose_energy_every_seconds = 0.3;

		RandomWalker {
			location: utility.random_location(arena_width, arena_height),
			velocity: Vector2::new(0.0, 0.0),
			acceleration: Vector2::new(0.0, 0.0),
			size: 30.0,
			rng: rand::thread_rng(),
			timer: lose_energy_every_seconds,
			lose_energy_every_seconds,
			spend_energy_rate: 0.01,
			mesh: utility.create_circle(0.0, 0.0, 1.0, Color::from_rgb(255, 255, 255), context),
			name: String::from("random walker"),
		}
	}

	pub fn walk(&mut self, (arena_width, arena_height): (f32, f32), delta_time: f32) {
		self.acceleration.x = self.rng.gen_range(-1.0, 1.0) * delta_time;
		self.acceleration.y = self.rng.gen_range(-1.0, 1.0) * delta_time;

		self.velocity = self.velocity + self.acceleration;
		self.location = self.location + self.velocity;

		self.location.x = self.location.x % arena_width;
		self.location.y = self.location.y % arena_height;

		self.keep_in_arena((arena_width, arena_height));
	}

	pub fn eat(&mut self, foods: &mut Vec<Food>) {
		for (index, food) in foods.clone().iter().enumerate().rev() {
			let direction = food.location - self.location;
			let distance = direction.magnitude();

			if distance <= self.size {
				self.size = self.size + food.calories;
				foods.remove(index);
			}

		}
	}

	pub fn spend_energy(&mut self) {
		self.size = self.size - self.spend_energy_rate;
	}

	pub fn is_alive(&self) -> bool {
		self.size >= 0.0
	}

	pub fn update(&mut self, arena_size: (f32, f32), delta_time: f32, foods: &mut Vec<Food>, context: &mut Context) {
		self.walk(arena_size, delta_time);
		self.eat(foods);

		self.timer = self.timer - duration_to_f64(delta(context));
		if self.timer < 0.0 {
			self.spend_energy();
			self.timer = self.lose_energy_every_seconds;
		}
	}
}

impl WalkerMovement for RandomWalker {
	fn set_location(&mut self, location: Vector2<f32>) {
		self.location = location;
	}

	fn get_health(&self) -> f32 {
		self.size
	}

	fn get_location(&self) -> Vector2<f32> {
		self.location.clone()
	}
}