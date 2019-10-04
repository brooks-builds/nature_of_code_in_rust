use bbggez::{
	ggez::{
		nalgebra::{Vector2, Point2},
		graphics::{Color, Mesh, MeshBuilder, DrawMode},
		Context
	}
};
use rand::prelude::*;

pub struct Satelite {
	pub location: Vector2<f32>,
	velocity: Vector2<f32>,
	acceleration: Vector2<f32>,
	pub mass: f32,
	pub mesh: Mesh
}

impl Satelite {
	pub fn new(location: Vector2<f32>, mass: f32, color: Color, context: &mut Context, rng: &mut ThreadRng) -> Satelite {
		let mesh = MeshBuilder::new()
			.circle(DrawMode::fill(), Point2::new(0.0, 0.0), mass, 0.1, color)
			.build(context).unwrap();

		Satelite {
			location,
			velocity: Vector2::new(rng.gen_range(-0.2, 0.2), rng.gen_range(-0.2, 0.2)),
			acceleration: Vector2::new(0.0, 0.0),
			mass,
			mesh
		}
	}

	pub fn apply_force(&mut self, force: Vector2<f32>, delta_time: f32) {
		let scaled_force = (force / self.mass) * delta_time;

		self.acceleration = self.acceleration + scaled_force;
	}

	pub fn update(&mut self) {
		self.velocity = self.velocity + self.acceleration;
		self.location = self.location + self.velocity;
		self.acceleration = self.acceleration * 0.0;
	}
}