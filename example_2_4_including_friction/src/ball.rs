use ggez::{
	nalgebra::{Vector2, Point2},
	graphics::{Color, Mesh, MeshBuilder, DrawMode},
	Context
};

pub struct Ball {
	pub location: Vector2<f32>,
	pub velocity: Vector2<f32>,
	acceleration: Vector2<f32>,
	mass: f32,
	pub mesh: Mesh
}

impl Ball {
	pub fn new(location: Vector2<f32>, mass: f32, color: Color, context: &mut Context) -> Ball {
		let mesh = MeshBuilder::new()
			.circle(DrawMode::fill(), Point2::new(0.0, 0.0), mass, 0.1, color)
			.build(context).unwrap();

		Ball {
			location,
			velocity: Vector2::new(0.0, 0.0),
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

	pub fn bounce(&mut self, (arena_width, arena_height): (f32, f32)) {
		if self.location.y + self.mass > arena_height {
			self.location.y = arena_height - self.mass;
			self.velocity.y = -self.velocity.y;
		}

		if self.location.x + self.mass > arena_width {
			self.location.x = arena_width - self.mass;
			self.velocity.x = -self.velocity.x;
		}
	}
}