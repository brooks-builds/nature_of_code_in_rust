use ggez::nalgebra::Point2;
use ggez::graphics::{Color, Mesh, MeshBuilder, DrawMode};
use ggez::{Context, GameResult};
use rand::prelude::*;

pub struct RandomWalker {
	location: Point2<f32>,
	velocity: Point2<f32>,
	acceleration: Point2<f32>,
	size: f32,
	color: Color
}

impl RandomWalker {
	pub fn new(arena_width: f32, arena_height: f32, mut rng: ThreadRng) -> RandomWalker {
		let x: f32 = rng.gen_range(0.0, arena_width);
		let y: f32 = rng.gen_range(0.0, arena_height);

		RandomWalker {
			location: Point2::new(x, y),
			velocity: Point2::new(0.0, 0.0),
			acceleration: Point2::new(0.0, 0.0),
			size: 10.0,
			color: Color::from_rgb(255, 255, 255)
		}
	}

	pub fn draw(&mut self, context: &mut Context) -> GameResult<Mesh> {
		MeshBuilder::new()
			.circle(DrawMode::fill(), self.location, self.size, 0.01, self.color)
			.build(context)
	}

	pub fn update(&mut self, mut rng: ThreadRng, arena_width: f32, arena_height: f32) {
		self.acceleration.x = rng.gen_range(-0.01, 0.01);
		self.acceleration.y = rng.gen_range(-0.01, 0.01);

		self.velocity.x = self.velocity.x + self.acceleration.x;
		self.velocity.y = self.velocity.y + self.acceleration.y;

		self.location.x = self.location.x + self.velocity.x;
		self.location.y = self.location.y + self.velocity.y;

		self.location.x = self.location.x % arena_width;
		self.location.y = self.location.y % arena_height;

		self.keep_in_arena(arena_width, arena_height);
	}

	fn keep_in_arena(&mut self, width: f32, height: f32) {
		if self.location.x + self.size > width {
			self.location.x = width - self.size;
		}  else if self.location.x - self.size < 0.0 {
			self.location.x = self.size;
		} 

		if self.location.y + self.size > height {
			self.location.y = height - self.size;
		} else if self.location.y - self.size < 0.0 {
			self.location.y = self.size;
		}
	}
}