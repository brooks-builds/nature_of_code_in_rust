use ggez::nalgebra::{Vector2, Point2};
use ggez::graphics::{Color, Mesh, MeshBuilder, DrawMode};
use ggez::{Context, GameResult};
use rand::prelude::*;

pub struct Ball {
	location: Vector2<f32>,
	velocity: Vector2<f32>,
	acceleration: Vector2<f32>,
	mass: f32,
	color: Color
}

impl Ball {
	pub fn new(location: Vector2<f32>, mass: f32, rng: &mut ThreadRng) -> Ball {
		Ball {
			location,
			velocity: Vector2::new(0.0, 0.0),
			acceleration: Vector2::new(0.0, 0.0),
			mass,
			color: Color::from_rgba(rng.gen_range(200, 255), rng.gen_range(200, 255), rng.gen_range(200, 255), 100)
		}
	}

	pub fn create_mesh(&mut self, context: &mut Context) -> GameResult<Mesh> {
		MeshBuilder::new()
			.circle(DrawMode::fill(), Point2::from(self.location), self.mass, 0.1, self.color)
			.build(context)
	}

	pub fn apply_force(&mut self, force: Vector2<f32>, delta_time: f32) {
		// force / mass = acceleration
		self.acceleration = self.acceleration + (force / self.mass) * delta_time;
	}

	pub fn update(&mut self, arena_size: (f32, f32)) {
		self.velocity = self.velocity + self.acceleration;
		self.location = self.location + self.velocity;
		self.acceleration = self.acceleration * 0.0;

		self.bounce(arena_size);
	}

	fn bounce(&mut self, arena_size: (f32, f32)) {
		if self.velocity.y > 0.0 && self.location.y + self.mass > arena_size.1 {
			self.location.y = arena_size.1 - self.mass;
			self.velocity.y = -self.velocity.y;
		}

		if self.velocity.x > 0.0 && self.location.x + self.mass > arena_size.0 {
			self.location.x = arena_size.0 - self.mass;
			self.velocity.x = -self.velocity.x;
		}
	}
}