use ggez::nalgebra::Point2;
use ggez::graphics;
use rand::prelude::*;

pub struct Ball {
	pub location: Point2<f32>,
	velocity: Point2<f32>,
	acceleration: Point2<f32>,
	pub radius: f32,
	pub color: graphics::Color,
	rng: ThreadRng
}

impl Ball {
	pub fn new(window_width: f32, window_height: f32) -> Ball {
		let location = Point2::new(window_width / 2.0, window_height / 2.0);
		let velocity = Point2::new(0.0, 0.0);
		let acceleration = Point2::new(-0.0001, 0.00001);
		let radius = 10.0;
		let mut rng = rand::thread_rng();
		let red = rng.gen_range(0, 255);
		let green = rng.gen_range(0, 255);
		let blue = rng.gen_range(0, 255);
		let color = graphics::Color::from_rgb(red, green, blue);

		Ball {
			location,
			velocity,
			acceleration,
			radius,
			color,
			rng
		}
	}

	pub fn update(&mut self) {
		self.acceleration.x = self.rng.gen_range(-0.0001, 0.0001);
		self.acceleration.y = self.rng.gen_range(-0.0001, 0.0001);;

		self.velocity.x = self.velocity.x + self.acceleration.x;
		self.velocity.y = self.velocity.y + self.acceleration.y;

		self.location.x = self.location.x + self.velocity.x;
		self.location.y = self.location.y + self.velocity.y;
	}
}