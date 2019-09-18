use ggez::nalgebra::{Point2, Vector2};
use rand::prelude::*;
use ggez::graphics::{MeshBuilder, DrawMode, Color, Mesh};
use ggez::{GameResult, Context};

#[derive(Clone)]
pub struct Food {
	pub location: Vector2<f32>,
	pub calories: f32,
	size: f32,
	color: Color,
	pub eaten: bool
}

impl Food {
	pub fn new(width: f32, height: f32, mut rng: ThreadRng) -> Food {
		let x: f32 = rng.gen_range(0.0, width);
		let y: f32 = rng.gen_range(0.0, height);
		let color = if rand::random() {
			Color::from_rgb(255, 0, 0)
		} else {
			Color::from_rgb(0, 255, 0)
		};
		let calories = rng.gen_range(0.1, 5.0);

		Food {
			calories,
			size: calories * 2.0,
			location: Vector2::new(x, y),
			color,
			eaten: false
		}
	}

	pub fn draw(&mut self, context: &mut Context) -> GameResult<Mesh> {
		MeshBuilder::new()
			.circle(DrawMode::fill(), Point2::from(self.location), self.size, 0.5, self.color)
			.build(context)
	}
}