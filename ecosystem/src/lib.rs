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
	Builder, Component, DenseVecStorage, Entities, Read, ReadStorage, RunNow, System, World,
	WorldExt, WriteStorage,
};

pub struct Game {
	world: World,
}

impl Game {
	pub fn new(arena_width: f32, arena_height: f32) -> Game {
		let mut world = World::new();
		let food_calories = 2.0;
		let mut rng = rand::thread_rng();

		world.insert(ArenaSize(arena_width, arena_height));
		world.register::<Health>();
		world.register::<Location>();
		world.register::<Velocity>();
		world.register::<Acceleration>();
		world.register::<Color>();
		world.register::<Speed>();
		world.register::<GrowBy>();

		// add food
		for _ in 0..200 {
			world
				.create_entity()
				.with(Health(food_calories))
				.with(Location(Vector2::new(
					rng.gen_range(10.0, arena_width - 10.0),
					rng.gen_range(10.0, arena_height - 10.0),
				)))
				.with(Color::new())
				.build();
		}

		// add random walkers
		for _ in 0..25 {
			world
				.create_entity()
				.with(Health(food_calories))
				.with(Location(Vector2::new(
					rng.gen_range(10.0, arena_width - 10.0),
					rng.gen_range(10.0, arena_height - 10.0),
				)))
				.with(Color::new())
				.with(Velocity::new())
				.with(Acceleration::new())
				.with(Speed(10.0))
				.with(GrowBy(0.0))
				.build();
		}

		Game { world }
	}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		let mut random_move = RandomMoveSystem;
		let delta = timer::delta(context);
		let mut physics = PhysicsSystem(delta.as_secs_f32());
		let mut eat = EatSystem;
		let mut grow = GrowSystem;
		let mut contain_to_arena = ContainToArenaSystem;

		random_move.run_now(&self.world);
		physics.run_now(&self.world);
		eat.run_now(&self.world);
		grow.run_now(&self.world);
		contain_to_arena.run_now(&self.world);

		self.world.maintain();
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
struct Velocity(Vector2<f32>);

#[derive(Component)]
struct Color(graphics::Color);

#[derive(Component)]
struct Speed(f32);

impl Color {
	pub fn new() -> Color {
		Color(bbggez::color::random_bright_color())
	}
}

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

#[derive(Component)]
struct GrowBy(f32);

struct DrawSystem<'a>(&'a mut Context);

impl<'a> System<'a> for DrawSystem<'a> {
	type SystemData = (
		Read<'a, ArenaSize>,
		ReadStorage<'a, Location>,
		ReadStorage<'a, Health>,
		ReadStorage<'a, Color>,
	);

	fn run(&mut self, (arena_size, location, health, color): Self::SystemData) {
		use specs::Join;

		for (location, health, color) in (&location, &health, &color).join() {
			let mesh = graphics::MeshBuilder::new()
				.circle(
					graphics::DrawMode::fill(),
					Point2::from(location.0),
					health.0,
					0.000001,
					color.0,
				)
				.build(self.0)
				.unwrap();

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

struct RandomMoveSystem;

impl<'a> System<'a> for RandomMoveSystem {
	type SystemData = (ReadStorage<'a, Speed>, WriteStorage<'a, Acceleration>);

	fn run(&mut self, (speed, mut acceleration): Self::SystemData) {
		use specs::Join;

		let mut rng = rand::thread_rng();

		for (speed, acceleration) in (&speed, &mut acceleration).join() {
			let force = Vector2::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0));

			acceleration.0 += force * speed.0;
		}
	}
}

struct PhysicsSystem(f32);

impl<'a> System<'a> for PhysicsSystem {
	type SystemData = (
		WriteStorage<'a, Acceleration>,
		WriteStorage<'a, Velocity>,
		WriteStorage<'a, Location>,
	);

	fn run(&mut self, (mut acceleration, mut velocity, mut location): Self::SystemData) {
		use specs::Join;

		for (acceleration, velocity, location) in
			(&mut acceleration, &mut velocity, &mut location).join()
		{
			velocity.0 += acceleration.0;
			location.0 += velocity.0 * self.0;
			acceleration.0 *= 0.0;
		}
	}
}

struct EatSystem;

impl<'a> System<'a> for EatSystem {
	type SystemData = (
		ReadStorage<'a, Location>,
		ReadStorage<'a, Health>,
		Entities<'a>,
		WriteStorage<'a, GrowBy>,
	);

	fn run(&mut self, (location, health, entities, mut grow_by): Self::SystemData) {
		use specs::Join;

		for (my_location, my_health, myself, grow_by) in
			(&location, &health, &entities, &mut grow_by).join()
		{
			for (other_location, other_health, other) in (&location, &health, &entities).join() {
				if myself != other {
					let distance = my_location.0 - other_location.0;
					let distance = distance.magnitude();

					if distance < my_health.0 {
						grow_by.0 += other_health.0;
						entities.delete(other).unwrap();
					}
				}
			}
		}
	}
}

struct GrowSystem;

impl<'a> System<'a> for GrowSystem {
	type SystemData = (WriteStorage<'a, Health>, WriteStorage<'a, GrowBy>);

	fn run(&mut self, (mut health, mut grow_by): Self::SystemData) {
		use specs::Join;

		for (health, grow_by) in (&mut health, &mut grow_by).join() {
			health.0 += grow_by.0;
			grow_by.0 = 0.0;
		}
	}
}

struct ContainToArenaSystem;

impl<'a> System<'a> for ContainToArenaSystem {
	type SystemData = (
		Read<'a, ArenaSize>,
		WriteStorage<'a, Location>,
		WriteStorage<'a, Velocity>,
		ReadStorage<'a, Health>,
	);

	fn run(&mut self, (arena_size, mut location, mut velocity, health): Self::SystemData) {
		use specs::Join;

		for (location, velocity, health) in (&mut location, &mut velocity, &health).join() {
			if location.0.x + health.0 > arena_size.0 {
				location.0.x = arena_size.0 - health.0;
				velocity.0.x *= -1.0;
			} else if location.0.x - health.0 < 0.0 {
				location.0.x = health.0;
				velocity.0.x *= -1.0;
			}

			if location.0.y + health.0 > arena_size.1 {
				location.0.y = arena_size.1 - health.0;
				velocity.0.y *= -1.0
			} else if location.0.y - health.0 < 0.0 {
				location.0.y = health.0;
				velocity.0.y *= -1.0
			}
		}
	}
}
