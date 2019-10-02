use ggez::{
	graphics::{Rect, Mesh, MeshBuilder, Color, DrawMode},
	nalgebra::Vector2,
	Context
};
use crate::Ball;

pub struct Ocean {
	location: Rect,
	pub mesh: Mesh,
	resistance_coefficient: f32
}

impl Ocean {
	pub fn new(location: Rect, context: &mut Context) -> Ocean {
		let color = Color::new(0.0, 0.0, 0.8, 0.2);
		let mesh = MeshBuilder::new()
			.rectangle(DrawMode::fill(), location, color)
			.build(context)
			.unwrap();

		Ocean {
			location,
			mesh,
			resistance_coefficient: 1000.0
		}
	}

	pub fn resist_mover(&self, ball: &mut Ball, delta_time: f32) {
		let resistance = (ball.speed() * ball.speed()) * self.resistance_coefficient;
		let mut drag = ball.velocity * -1.0;
		drag = drag.normalize();
		drag = drag * resistance;

		ball.apply_force(drag, delta_time);
	}

	pub fn is_inside(&self, ball: &mut Ball) -> bool {
		ball.location.y + ball.mass > self.location.y
	}
}