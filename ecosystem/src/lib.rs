use bbggez::{
	ggez::{event::EventHandler, graphics, nalgebra::Vector2, Context, GameResult},
	rand,
	rand::prelude::*,
};
use specs::{
	Builder, Component, DenseVecStorage, Read, ReadStorage, RunNow, System, World, WorldExt,
	WriteStorage,
};

pub struct Game {
	world: World,
}

impl Game {
	pub fn new(arena_width: f32, arena_height: f32) -> Game {
		let mut world = World::new();
		let food_calories = 1.0;
		let mut rng = rand::thread_rng();

		world.insert(ArenaSize(arena_width, arena_height));
		world.register::<Health>();
		world.register::<Location>();
		world.register::<Radius>();
		world.register::<Velocity>();
		world.register::<Acceleration>();

		for _ in 0..25 {
			world
				.create_entity()
				.with(Health(food_calories))
				.with(Location(Vector2::new(
					rng.gen_range(10.0, arena_width - 10.0),
					rng.gen_range(10.0, arena_height - 10.0),
				)))
				.with(Radius(3.0))
				.with(Velocity::new())
				.with(Acceleration::new())
				.build();
		}

		Game { world }
	}
}

impl EventHandler for Game {
	fn update(&mut self, _context: &mut Context) -> GameResult<()> {
		let mut update_velocity = UpdateVelocitySystem;
		let mut update_acceleration = UpdateAccelerationSystem;
		let mut update_location = UpdateLocationSystem;
		let mut reset_acceleration = ResetAccelerationSystem;

		update_velocity.run_now(&self.world);
		update_acceleration.run_now(&self.world);
		update_location.run_now(&self.world);
		reset_acceleration.run_now(&self.world);
		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);

		let mut draw_system = DrawSystem(context);

		draw_system.run_now(&self.world);

		graphics::present(context)
	}
}

#[derive(Default)]
struct ArenaSize(f32, f32);

#[derive(Component)]
struct Health(f32);

#[derive(Component)]
struct Location(Vector2<f32>);

#[derive(Component)]
struct Radius(f32);

#[derive(Component)]
struct Velocity(Vector2<f32>);

impl Velocity {
	pub fn new() -> Velocity {
		Velocity(Vector2::new(0.0, 0.0))
	}
}

#[derive(Component)]
struct Acceleration(Vector2<f32>);

impl Acceleration {
	pub fn new() -> Acceleration {
		Acceleration(Vector2::new(0.0, 0.0))
	}
}

struct DrawSystem<'a>(&'a mut Context);

impl<'a> System<'a> for DrawSystem<'a> {
	type SystemData = (
		Read<'a, ArenaSize>,
		ReadStorage<'a, Location>,
		ReadStorage<'a, Radius>,
	);

	fn run(&mut self, (arena_size, location, radius): Self::SystemData) {
		use specs::Join;

		for (location, radius) in (&location, &radius).join() {
			let mesh = bbggez::mesh::create_circle(
				location.0.x,
				location.0.y,
				radius.0,
				graphics::Color::new(0.0, 1.0, 0.0, 1.0),
				self.0,
			);

			graphics::draw(self.0, &mesh, graphics::DrawParam::default()).unwrap();
		}

		let border_mesh = graphics::MeshBuilder::new()
			.rectangle(
				graphics::DrawMode::stroke(2.0),
				graphics::Rect::new(0.0, 0.0, arena_size.0, arena_size.1),
				graphics::WHITE,
			)
			.build(self.0)
			.unwrap();

		graphics::draw(self.0, &border_mesh, graphics::DrawParam::default()).unwrap();
	}
}

struct UpdateVelocitySystem;

impl<'a> System<'a> for UpdateVelocitySystem {
	type SystemData = (WriteStorage<'a, Velocity>);

	fn run(&mut self, mut velocity: Self::SystemData) {
		use specs::Join;

		let mut rng = rand::thread_rng();

		for velocity in (&mut velocity).join() {
			velocity.0.x = rng.gen_range(-1.0, 1.0);
			velocity.0.y = rng.gen_range(-1.0, 1.0);

			velocity.0 *= 10.0;
		}
	}
}

struct UpdateAccelerationSystem;

impl<'a> System<'a> for UpdateAccelerationSystem {
	type SystemData = (WriteStorage<'a, Acceleration>, ReadStorage<'a, Velocity>);

	fn run(&mut self, (mut acceleration, velocity): Self::SystemData) {
		use specs::Join;

		for (acceleration, velocity) in (&mut acceleration, &velocity).join() {
			acceleration.0 += velocity.0;
		}
	}
}

struct UpdateLocationSystem;

impl<'a> System<'a> for UpdateLocationSystem {
	type SystemData = (WriteStorage<'a, Location>, ReadStorage<'a, Acceleration>);

	fn run(&mut self, (mut location, acceleration): Self::SystemData) {
		use specs::Join;

		for (location, acceleration) in (&mut location, &acceleration).join() {
			location.0 += acceleration.0;
		}
	}
}

struct ResetAccelerationSystem;

impl<'a> System<'a> for ResetAccelerationSystem {
	type SystemData = (WriteStorage<'a, Acceleration>);

	fn run(&mut self, mut acceleration: Self::SystemData) {
		use specs::Join;

		for acceleration in (&mut acceleration).join() {
			acceleration.0 *= 0.0;
		}
	}
}
