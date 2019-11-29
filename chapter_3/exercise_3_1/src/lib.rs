use bbggez::ggez::{event::EventHandler, graphics, nalgebra::Point2, timer, Context, GameResult};
// use specs::{Component, DenseVecStorage, World, WorldExt};

pub struct Game {
	background_color: graphics::Color,
	baton_mesh: graphics::Mesh,
	angle: f32,
	angular_velocity: f32,
	angular_acceleration: f32,
	angular_velocity_max: f32,
}

impl Game {
	pub fn new(context: &mut Context) -> GameResult<Game> {
		let background_color = graphics::Color::from_rgb(128, 207, 216);
		let cap_radius = 15.0;
		let pole_length = 300.0;
		let pole_color = graphics::Color::from_rgb(246, 177, 127);
		let pole_width = 3.0;
		let cap_color = graphics::Color::from_rgb(178, 87, 122);
		let baton_mesh = graphics::MeshBuilder::new()
			.circle(
				graphics::DrawMode::fill(),
				Point2::new(0.0 - pole_length / 2.0 - cap_radius, 0.0),
				cap_radius,
				0.0001,
				cap_color,
			)
			.line(
				&[
					Point2::new(0.0 - pole_length / 2.0, 0.0),
					Point2::new(pole_length / 2.0, 0.0),
				],
				pole_width,
				pole_color,
			)?
			.circle(
				graphics::DrawMode::fill(),
				Point2::new(pole_length / 2.0 + cap_radius, 0.0),
				cap_radius,
				0.0001,
				cap_color,
			)
			.build(context)?;
		let angle = 0.0;
		let angular_velocity = 0.0;
		let angular_acceleration = 0.001;
		let angular_velocity_max = 0.02;

		Ok(Game {
			background_color,
			baton_mesh,
			angle,
			angular_velocity,
			angular_acceleration,
			angular_velocity_max,
		})
	}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		let delta_time = timer::delta(context).as_secs_f32();

		self.angular_velocity += self.angular_acceleration * delta_time;
		if self.angular_velocity > self.angular_velocity_max {
			self.angular_velocity = self.angular_velocity_max;
		}
		self.angle += self.angular_velocity;
		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, self.background_color);

		let (width, height) = graphics::drawable_size(context);

		graphics::draw(
			context,
			&self.baton_mesh,
			graphics::DrawParam::new()
				.dest(Point2::new(width / 2.0, height / 2.0))
				.rotation(self.angle),
		)?;

		let angular_velocity_text =
			graphics::Text::new(format!("angular velocity: {}", self.angular_velocity));

		graphics::draw(
			context,
			&angular_velocity_text,
			graphics::DrawParam::new()
				.dest(Point2::new(5.0, 5.0))
				.color(graphics::BLACK),
		)?;

		graphics::present(context)
	}
}
