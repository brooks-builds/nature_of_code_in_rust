use bbggez::{
	ggez::{
		nalgebra::{Vector2, Point2, clamp},
		graphics::{Color, Mesh, MeshBuilder, DrawMode},
		Context
	}
};
use crate::Satelite;

pub struct Planet {
	pub location: Vector2<f32>,
	mass: f32,
	pub mesh: Mesh
}

impl Planet {
	pub fn new(location: Vector2<f32>, mass: f32, color: Color, context: &mut Context) -> Planet {
		let mesh = MeshBuilder::new()
			.circle(DrawMode::fill(), Point2::new(0.0, 0.0), mass, 0.1, color)
			.build(context).unwrap();

		Planet {
			location,
			mass,
			mesh
		}
	}

	pub fn attract(&self, gravitational_constant: f32, satelite: &mut Satelite, delta_time: f32) {
		let distance = (self.location - satelite.location).magnitude();
		let distance = clamp(distance, 5.0, 25.0);
		let force = (gravitational_constant * self.mass * satelite.mass) / (distance * distance);
		let force = force * (self.location - satelite.location).normalize();

		satelite.apply_force(force, delta_time)
	}
}