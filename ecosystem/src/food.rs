use ggez::nalgebra::Point2;
use rand::prelude::*;
use ggez::graphics::{MeshBuilder, DrawMode, Color, Mesh};
use ggez::{GameResult, Context};

pub struct Food {
	location: Point2<f32>,
	calories: f32,
	size: f32,
	color: Color
}

impl Food {
	pub fn new(width: f32, height: f32, mut rng: ThreadRng) -> Food {
		let x: f32 = rng.gen_range(0.0, width);
		let y: f32 = rng.gen_range(0.0, height);
		let color = Color::from_rgb(0, 255, 0);

		Food {
			calories: 10.0,
			size: 3.0,
			location: Point2::new(x, y),
			color
		}
	}

	pub fn draw(&mut self, context: &mut Context) -> GameResult<Mesh> {
		MeshBuilder::new()
			.circle(DrawMode::fill(), self.location, self.size, 0.5, self.color)
			.build(context)
	}
}