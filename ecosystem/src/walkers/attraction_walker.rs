use bbggez::{
	ggez::{
		nalgebra::Vector2,
		graphics::{
			Color,
			Mesh,
		},
		Context,
	},
	Utility,
};

#[derive(Clone, Debug)]
pub struct AttractionWalker {
	pub location: Vector2<f32>,
	acceleration: Vector2<f32>,
	velocity: Vector2<f32>,
	speed: f32,
	size: f32,
	pub mesh: Mesh
}

impl AttractionWalker {
	pub fn new(location: Vector2<f32>, size: f32, speed: f32, utility: &mut Utility, context: &mut Context) -> AttractionWalker {
		AttractionWalker {
			location,
			acceleration: Vector2::new(0.0, 0.0),
			velocity: Vector2::new(0.0, 0.0),
			speed,
			size,
			mesh: utility.create_circle(0.0, 0.0, size, Color::new(1.0, 1.0, 0.0, 1.0), context),
		}
	}

	pub fn update(&mut self) {

	}

	pub fn is_alive(&self) -> bool {
		self.size >= 0.0
	}
}

