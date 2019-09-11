use ggez::nalgebra::Point2;
use ggez::graphics;

pub struct Ball {
	pub location: Point2<f32>,
	pub color: graphics::Color,
	pub radius: f32
}

impl Ball {
	pub fn new(location: Point2<f32>) -> Ball {
		let color = graphics::WHITE;
		let radius = 15.0;

		Ball {
			location,
			color,
			radius
		}
	}
}