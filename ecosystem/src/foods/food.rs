use bbggez::{
	ggez::nalgebra::{Point2, Vector2},
	ggez::graphics::{MeshBuilder, DrawMode, Color, Mesh},
	ggez::{GameResult, Context},
	Utility,
	rand,
	rand::prelude::*,
};

#[derive(Clone)]
pub struct Food {
	pub location: Vector2<f32>,
	pub calories: f32,
	size: f32,
	pub eaten: bool,
	calorie_loss_rate: usize,
	calorie_loss_amount: f32,
	minimum_calories: f32
}

impl Food {
	pub fn new((width, height): (f32, f32), utility: &mut Utility) -> Food {
		Food {
			calories: 10.0,
			size: 5.0,
			location: utility.random_location(width, height),
			eaten: false,
			calorie_loss_rate: 100,
			calorie_loss_amount: 0.1,
			minimum_calories: -10.0
		}
	}

	pub fn draw(&mut self, context: &mut Context) -> GameResult<Mesh> {
		let scaled_calories = self.calories * 25.0;
		let color = if self.calories < 0.0 {
			Color::from_rgb(10, 50, scaled_calories.abs() as u8)
		} else {
			Color::from_rgb(0, scaled_calories as u8, 0)
		};
		let mut rng = rand::thread_rng();

		let points = if self.calories > 0.0 {
			[
				Point2::new(self.location.x, self.location.y - self.size),
				Point2::new(self.location.x + self.size, self.location.y + self.size),
				Point2::new(self.location.x - self.size, self.location.y + self.size)
			]
		} else {
			[
				Point2::new(self.location.x + rng.gen_range(-5.0, 5.0), self.location.y - self.size),
				Point2::new(self.location.x + self.size, self.location.y + rng.gen_range(-5.0, 5.0)),
				Point2::new(self.location.x - self.size, self.location.y + rng.gen_range(-5.0, 5.0))
			]
		};

		MeshBuilder::new()
			.polyline(DrawMode::fill(), &points, color)?
			.build(context)
	}

	pub fn update(&mut self, ticks: usize) {
		if ticks % self.calorie_loss_rate == 0 {
			self.calories = self.calories - self.calorie_loss_amount;
		}
	}

	pub fn is_rotton(&self) -> bool {
		self.calories <= self.minimum_calories
	}
}