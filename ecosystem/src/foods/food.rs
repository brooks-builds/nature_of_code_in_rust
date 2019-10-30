use bbggez::{
	ggez::nalgebra::{Vector2},
	ggez::graphics::{Color, Mesh},
	ggez::{
		Context,
		timer::{duration_to_f64, delta},
	},
	Utility,
};

#[derive(Clone, Debug)]
pub struct Food {
	pub location: Vector2<f32>,
	pub calories: f32,
	size: f32,
	pub eaten: bool,
	calorie_loss_rate: f64,
	timer: f64,
	calorie_loss_amount: f32,
	minimum_calories: f32,
	healthy_mesh: Mesh,
	poison_mesh: Mesh,
	pub id: usize,
}

impl Food {
	pub fn new((width, height): (f32, f32), utility: &mut Utility, context: &mut Context, id: usize) -> Food {
		let location = utility.random_location(width, height);
		let size = 15.0;
		let healthy_color = Color::new(0.0, 1.0, 0.0, 1.0);
		let poison_color = Color::new(0.0, 0.0, 1.0, 1.0);
		let calorie_loss_rate = 0.3;

		Food {
			calories: 10.0,
			size,
			location,
			eaten: false,
			calorie_loss_rate,
			timer: calorie_loss_rate,
			calorie_loss_amount: 0.1,
			minimum_calories: -10.0,
			healthy_mesh: utility.create_equilateral_triangle(location.x, location.y, size, healthy_color, context),
			poison_mesh: utility.create_square(location.x, location.y, size, poison_color, context),
			id
		}
	}

	pub fn draw(&mut self, context: &mut Context) -> &Mesh {
		if self.calories > 0.0 {
			&self.healthy_mesh
		} else {
			&self.poison_mesh
		}
	}

	pub fn update(&mut self, context: &mut Context) {
		self.timer = self.timer - duration_to_f64(delta(context));

		if self.timer <= 0.0 {
			self.calories = self.calories - self.calorie_loss_amount;
			self.timer = self.calorie_loss_rate;
		}
	}

	pub fn is_rotton(&self) -> bool {
		self.calories <= self.minimum_calories
	}
}