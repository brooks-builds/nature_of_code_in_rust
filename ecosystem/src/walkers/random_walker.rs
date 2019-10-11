use bbggez::{
	ggez::nalgebra::{Vector2, Point2},
	ggez::graphics::{Color, Mesh, MeshBuilder, DrawMode},
	ggez::{Context, GameResult},
	rand::prelude::*,
};
use crate::foods::food::Food;

#[derive(Clone, Debug)]
pub struct RandomWalker {
	location: Vector2<f32>,
	velocity: Vector2<f32>,
	acceleration: Vector2<f32>,
	size: f32,
	color: Color,
	max_size: f32
}

impl RandomWalker {
	pub fn new(arena_width: f32, arena_height: f32, mut rng: ThreadRng) -> RandomWalker {
		let x: f32 = rng.gen_range(0.0, arena_width);
		let y: f32 = rng.gen_range(0.0, arena_height);

		RandomWalker {
			location: Vector2::new(x, y),
			velocity: Vector2::new(0.0, 0.0),
			acceleration: Vector2::new(0.0, 0.0),
			size: 10.0,
			color: Color::from_rgb(255, 255, 255),
			max_size: 25.0
		}
	}

	pub fn draw(&mut self, context: &mut Context) -> GameResult<Mesh> {
		MeshBuilder::new()
			.circle(DrawMode::fill(), Point2::from(self.location), self.size, 0.01, self.color)
			.build(context)
	}

	pub fn walk(&mut self, mut rng: ThreadRng, arena_width: f32, arena_height: f32, delta_time: f32) {
		self.acceleration.x = rng.gen_range(-1.0, 1.0) * delta_time;
		self.acceleration.y = rng.gen_range(-1.0, 1.0) * delta_time;

		self.velocity = self.velocity + self.acceleration;
		self.location = self.location + self.velocity;

		self.location.x = self.location.x % arena_width;
		self.location.y = self.location.y % arena_height;

		self.keep_in_arena((arena_width, arena_height));
	}

	fn keep_in_arena(&mut self, (arena_width, arena_height): (f32, f32)) {
		if self.location.x - self.size > arena_width {
			self.location.x = -self.size;
		}  else if self.location.x + self.size < 0.0 {
			self.location.x = arena_width - self.size;
		} 

		if self.location.y - self.size > arena_height {
			self.location.y = -self.size;
		} else if self.location.y + self.size < 0.0 {
			self.location.y = arena_height - self.size;
		}
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
		self.size = self.size - 0.1;
	}

	pub fn is_alive(&self) -> bool {
		self.size >= 0.0
	}

	pub fn cap_size(&mut self) {
		if self.size > self.max_size {
			self.size = self.max_size;
		}
	}
}