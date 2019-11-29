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
	Builder, Component, DenseVecStorage, Entities, Entity, NullStorage, Read, ReadStorage, RunNow,
	System, World, WorldExt, WriteStorage,
};

pub struct Game {
	world: World,
	rng: ThreadRng,
	walker_mesh: graphics::Mesh,
}

impl Game {
	pub fn new(arena_width: f32, arena_height: f32, context: &mut Context) -> Game {
		let mut world = World::new();
		let base_health = 2.0;
		let mut rng = rand::thread_rng();
		let walker_mesh = graphics::MeshBuilder::new()
			.circle(
				graphics::DrawMode::fill(),
				Point2::new(0.0, 0.0),
				1.0,
				0.0000000001,
				graphics::WHITE,
			)
			.build(context)
			.unwrap();

		world.insert(ArenaSize(arena_width, arena_height));
		world.insert(GrowRate(1.0));
		world.register::<Health>();
		world.register::<Location>();
		world.register::<Velocity>();
		world.register::<Acceleration>();
		world.register::<Color>();
		world.register::<Speed>();
		world.register::<GrowBy>();
		world.register::<RandomWalker>();
		world.register::<AttractionWalker>();
		world.register::<Target>();
		world.register::<Drag>();

		// add food
		for _ in 0..200 {
			world
				.create_entity()
				.with(Health(rng.gen_range(1.0, 5.0)))
				.with(Location(Vector2::new(
					rng.gen_range(10.0, arena_width - 10.0),
					rng.gen_range(10.0, arena_height - 10.0),
				)))
				.with(Color::new())
				.build();
		}

		// add random walkers
		for _ in 0..5 {
			world
				.create_entity()
				.with(Health(base_health))
				.with(Location(Vector2::new(
					rng.gen_range(10.0, arena_width - 10.0),
					rng.gen_range(10.0, arena_height - 10.0),
				)))
				.with(Color::new())
				.with(Velocity::new())
				.with(Acceleration::new())
				.with(Speed(5.0))
				.with(GrowBy(0.0))
				.with(RandomWalker)
				.build();
		}

		// add attraction walkers
		for _ in 0..5 {
			world
				.create_entity()
				.with(Health(base_health))
				.with(Location(Vector2::new(
					rng.gen_range(10.0, arena_width - 10.0),
					rng.gen_range(10.0, arena_height - 10.0),
				)))
				.with(Color::new())
				.with(Velocity::new())
				.with(Acceleration::new())
				.with(Speed(4.0))
				.with(GrowBy(0.0))
				.with(AttractionWalker)
				.with(Target(None))
				.with(Drag(0.05))
				.build();
		}

		Game {
			world,
			rng,
			walker_mesh,
		}
	}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		let mut random_move = RandomMoveSystem(self.rng);
		let delta = timer::delta(context);
		let mut physics = PhysicsSystem(delta.as_secs_f32());
		let mut eat = EatSystem;
		let mut grow = GrowSystem;
		let mut contain_to_arena = ContainToArenaSystem;
		let mut choose_target = ChooseTargetSystem(self.rng);
		let mut attraction_move = AttractionMoveSystem;
		let mut drag = DragSystem;
		let mut drop_target = DropTargetSystem;

		random_move.run_now(&self.world);
		// drop target
		drop_target.run_now(&self.world);
		// choose target
		choose_target.run_now(&self.world);
		// move towards target
		attraction_move.run_now(&self.world);
		drag.run_now(&self.world);
		physics.run_now(&self.world);
		eat.run_now(&self.world);
		grow.run_now(&self.world);
		contain_to_arena.run_now(&self.world);

		self.world.maintain();
		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);

		let mut draw_system = DrawSystem {
			context,
			walker_mesh: &self.walker_mesh,
		};

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

impl Velocity {
	pub fn new() -> Velocity {
		Velocity(Vector2::new(0.0, 0.0))
	}

	pub fn reset_x(&mut self) {
		self.0.x = 0.0;
	}

	pub fn reset_y(&mut self) {
		self.0.y = 0.0;
	}
}

#[derive(Component)]
struct Color(graphics::Color);

#[derive(Component)]
struct Speed(f32);

impl Color {
	pub fn new() -> Color {
		Color(bbggez::color::random_bright_color())
	}
}

#[derive(Component)]
struct Acceleration(Vector2<f32>);

impl Acceleration {
	pub fn new() -> Acceleration {
		Acceleration(Vector2::new(0.0, 0.0))
	}
}

#[derive(Default)]
struct GrowRate(f32);

#[derive(Component)]
struct GrowBy(f32);

#[derive(Component, Default)]
#[storage(NullStorage)]
struct RandomWalker;

#[derive(Component, Default)]
#[storage(NullStorage)]
struct AttractionWalker;

#[derive(Component)]
struct Target(Option<Entity>);

#[derive(Component)]
struct Drag(f32);

struct DrawSystem<'a> {
	context: &'a mut Context,
	walker_mesh: &'a graphics::Mesh,
}

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
			graphics::draw(
				&mut self.context,
				self.walker_mesh,
				graphics::DrawParam::default()
					.dest(Point2::from(location.0))
					.scale([health.0, health.0])
					.color(color.0),
			)
			.unwrap();
		}

		let border_mesh = graphics::MeshBuilder::new()
			.rectangle(
				graphics::DrawMode::stroke(2.0),
				graphics::Rect::new(0.0, 0.0, arena_size.0, arena_size.1),
				graphics::WHITE,
			)
			.build(&mut self.context)
			.unwrap();

		graphics::draw(
			&mut self.context,
			&border_mesh,
			graphics::DrawParam::default(),
		)
		.unwrap();

		let fps = graphics::Text::new(format!("fps: {}", timer::fps(&mut self.context)));
		graphics::draw(
			&mut self.context,
			&fps,
			graphics::DrawParam::default().dest(Point2::new(5.0, 5.0)),
		)
		.unwrap();
	}
}

struct RandomMoveSystem(ThreadRng);

impl<'a> System<'a> for RandomMoveSystem {
	type SystemData = (
		ReadStorage<'a, Speed>,
		WriteStorage<'a, Acceleration>,
		ReadStorage<'a, RandomWalker>,
	);

	fn run(&mut self, (speed, mut acceleration, random_walker): Self::SystemData) {
		use specs::Join;

		for (speed, acceleration, _) in (&speed, &mut acceleration, &random_walker).join() {
			let force = Vector2::new(self.0.gen_range(-10.0, 10.0), self.0.gen_range(-10.0, 10.0));

			acceleration.0 += force * speed.0;
		}
	}
}

struct AttractionMoveSystem;

impl<'a> System<'a> for AttractionMoveSystem {
	type SystemData = (
		ReadStorage<'a, AttractionWalker>,
		WriteStorage<'a, Acceleration>,
		ReadStorage<'a, Speed>,
		WriteStorage<'a, Target>,
		ReadStorage<'a, Location>,
	);

	fn run(
		&mut self,
		(attraction_walker, mut acceleration, speed, mut target, location): Self::SystemData,
	) {
		use specs::Join;

		for (_, acceleration, speed, target, my_location) in (
			&attraction_walker,
			&mut acceleration,
			&speed,
			&mut target,
			&location,
		)
			.join()
		{
			if let Some(target) = target.0 {
				if let Some(target_location) = location.get(target) {
					let mut direction = target_location.0 - my_location.0;

					direction = direction.normalize();

					let force = direction * speed.0;

					acceleration.0 += force;
				}
			} else {
				target.0 = None;
			}
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
		Read<'a, GrowRate>,
	);

	fn run(&mut self, (location, health, entities, mut grow_by, grow_rate): Self::SystemData) {
		use specs::Join;

		for (my_location, my_health, myself, grow_by) in
			(&location, &health, &entities, &mut grow_by).join()
		{
			for (other_location, other_health, other) in (&location, &health, &entities).join() {
				if myself != other {
					let distance = my_location.0 - other_location.0;
					let distance = distance.magnitude();

					if distance < my_health.0 && my_health.0 > other_health.0 {
						grow_by.0 += other_health.0 * grow_rate.0;
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
				velocity.reset_x();
			} else if location.0.x - health.0 < 0.0 {
				location.0.x = health.0;
				velocity.reset_x();
			}

			if location.0.y + health.0 > arena_size.1 {
				location.0.y = arena_size.1 - health.0;
				velocity.reset_y();
			} else if location.0.y - health.0 < 0.0 {
				location.0.y = health.0;
				velocity.reset_y();
			}
		}
	}
}

struct ChooseTargetSystem(ThreadRng);

impl<'a> System<'a> for ChooseTargetSystem {
	type SystemData = (
		ReadStorage<'a, AttractionWalker>,
		Entities<'a>,
		WriteStorage<'a, Target>,
		ReadStorage<'a, Health>,
	);

	fn run(&mut self, (attraction_walker, entities, mut target, health): Self::SystemData) {
		use specs::Join;

		for (_, myself, target, my_health) in
			(&attraction_walker, &entities, &mut target, &health).join()
		{
			match target.0 {
				None => {
					let mut potential_targets = vec![];
					for (other, other_health) in (&entities, &health).join() {
						if myself != other && other_health.0 < my_health.0 {
							potential_targets.push(other);
						}
					}
					if potential_targets.len() > 0 {
						let index = self.0.gen_range(0, potential_targets.len());
						target.0 = Some(potential_targets[index]);
					}
				}
				_ => continue,
			}
		}
	}
}

struct DragSystem;

impl<'a> System<'a> for DragSystem {
	type SystemData = (
		ReadStorage<'a, Velocity>,
		WriteStorage<'a, Acceleration>,
		ReadStorage<'a, Drag>,
	);

	fn run(&mut self, (velocity, mut acceleration, drag): Self::SystemData) {
		use specs::Join;

		for (velocity, acceleration, drag) in (&velocity, &mut acceleration, &drag).join() {
			let opposite_direction = velocity.0 * -1.0;
			let drag_force = opposite_direction * drag.0;
			acceleration.0 += drag_force;
		}
	}
}

struct DropTargetSystem;

impl<'a> System<'a> for DropTargetSystem {
	type SystemData = (ReadStorage<'a, Health>, WriteStorage<'a, Target>);

	fn run(&mut self, (health, mut target): Self::SystemData) {
		use specs::Join;

		for (my_health, target) in (&health, &mut target).join() {
			// we don't have a target
			// we have a target
			// target is now bigger than us
			// target is still good
			// target is dead
			match target.0 {
				None => continue,
				Some(other_entity) => {
					if let Some(other_health) = health.get(other_entity) {
						if my_health.0 < other_health.0 {
							target.0 = None;
						}
					} else {
						target.0 = None;
					}
				}
			}
		}
	}
}
