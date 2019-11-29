use bbggez::{
	ggez::{
		event::EventHandler,
		graphics,
		nalgebra::{Point2, Vector2},
		timer, Context, GameResult,
	},
	rand,
	rand::prelude::*,
};
use specs::{
	Builder, Component, DenseVecStorage, ReadStorage, RunNow, System, World, WorldExt, WriteStorage,
};

pub struct Game {
	background_color: graphics::Color,
	baton_mesh: graphics::Mesh,
	world: World,
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
		let mut world = World::new();
		let mut rng = rand::thread_rng();
		let (width, height) = graphics::drawable_size(context);

		world.register::<Location>();
		world.register::<Angle>();
		world.register::<AngularVelocity>();
		world.register::<AngularAcceleration>();
		world.register::<AngularVelocityMax>();
		world.register::<Scale>();

		for _ in 0..20 {
			world
				.create_entity()
				.with(Location::new(
					rng.gen_range(0.0, width),
					rng.gen_range(0.0, height),
				))
				.with(Angle(0.0))
				.with(AngularVelocity(0.0))
				.with(AngularAcceleration(rng.gen_range(0.01, 0.9)))
				.with(AngularVelocityMax(rng.gen_range(3.0, 10.0)))
				.with(Scale(rng.gen_range(-2.0, 0.0)))
				.build();
		}

		Ok(Game {
			background_color,
			baton_mesh,
			world,
		})
	}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		let delta_time = timer::delta(context).as_secs_f32();

		let mut rotate_system = RotateSystem { delta_time };

		rotate_system.run_now(&self.world);

		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, self.background_color);

		let mut draw_system = DrawSystem {
			context,
			baton_mesh: &self.baton_mesh,
		};

		draw_system.run_now(&self.world);

		graphics::present(context)
	}
}

#[derive(Component)]
struct Location(Vector2<f32>);

impl Location {
	pub fn new(x: f32, y: f32) -> Location {
		Location(Vector2::new(x, y))
	}
}

#[derive(Component)]
struct Angle(f32);

#[derive(Component)]
struct AngularVelocity(f32);

#[derive(Component)]
struct AngularAcceleration(f32);

#[derive(Component)]
struct AngularVelocityMax(f32);

#[derive(Component)]
struct Scale(f32);

struct DrawSystem<'a> {
	context: &'a mut Context,
	baton_mesh: &'a graphics::Mesh,
}

impl<'a> System<'a> for DrawSystem<'a> {
	type SystemData = (
		ReadStorage<'a, Location>,
		ReadStorage<'a, Angle>,
		ReadStorage<'a, AngularVelocity>,
		ReadStorage<'a, Scale>,
	);

	fn run(&mut self, (location, angle, angular_velocity, scale): Self::SystemData) {
		use specs::Join;

		for (location, angle, angular_velocity, scale) in
			(&location, &angle, &angular_velocity, &scale).join()
		{
			graphics::draw(
				self.context,
				self.baton_mesh,
				graphics::DrawParam::new()
					.dest(Point2::from(location.0))
					.scale([scale.0, scale.0])
					.rotation(angle.0),
			)
			.unwrap();

			// let angular_velocity_text =
			// 	graphics::Text::new(format!("angular velocity: {}", angular_velocity.0));

			// graphics::draw(
			// 	self.context,
			// 	&angular_velocity_text,
			// 	graphics::DrawParam::new()
			// 		.dest(Point2::new(5.0, 5.0))
			// 		.color(graphics::BLACK),
			// )
			// .unwrap();
		}
	}
}

struct RotateSystem {
	delta_time: f32,
}

impl<'a> System<'a> for RotateSystem {
	type SystemData = (
		WriteStorage<'a, Angle>,
		WriteStorage<'a, AngularVelocity>,
		ReadStorage<'a, AngularAcceleration>,
		ReadStorage<'a, AngularVelocityMax>,
	);

	fn run(&mut self, (mut angle, mut velocity, acceleration, velocity_max): Self::SystemData) {
		use specs::Join;

		for (angle, velocity, acceleration, velocity_max) in
			(&mut angle, &mut velocity, &acceleration, &velocity_max).join()
		{
			velocity.0 += acceleration.0 * self.delta_time;
			if velocity.0 > velocity_max.0 {
				velocity.0 = velocity_max.0;
			}
			angle.0 += velocity.0 * self.delta_time;
		}
	}
}
