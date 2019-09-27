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
			color: Color::from_rgba(rng.gen_range(0, 255), rng.gen_range(0, 255), rng.gen_range(0, 255), 100)
		}
	}

	pub fn create_mesh(&mut self, context: &mut Context) -> GameResult<Mesh> {
		MeshBuilder::new()
			.circle(DrawMode::fill(), Point2::from(self.location), self.mass, 0.1, self.color)
			.build(context)
	}
}