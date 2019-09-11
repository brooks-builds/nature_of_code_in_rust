use ggez::nalgebra::{Point2};
use ggez::graphics;

pub struct Ball {
	pub location: Point2<f32>,
	pub color: graphics::Color,
	pub radius: f32,
	velocity: Point2<f32>,
	acceleration: Point2<f32>,
	speed: f32
}

impl Ball {
	pub fn new(location: Point2<f32>) -> Ball {
		let color = graphics::WHITE;
		let radius = 15.0;
		let velocity = Point2::new(0.0, 0.0);
		let acceleration = Point2::new(0.0001, 0.0);
		let speed = 0.00001;

		Ball {
			location,
			color,
			radius,
			velocity,
			acceleration,
			speed
		}
	}

	pub fn update(&mut self, mouse_location: Point2<f32>) {
		let direction = mouse_location - self.location;
		let direction: Point2<f32> = direction.normalize().into();

		self.acceleration = direction * self.speed;

		self.velocity.x = self.velocity.x + self.acceleration.x;
		self.velocity.y = self.velocity.y  + self.acceleration.y;

		self.location.x = self.location.x + self.velocity.x;
		self.location.y = self.location.y + self.velocity.y;
	}
}