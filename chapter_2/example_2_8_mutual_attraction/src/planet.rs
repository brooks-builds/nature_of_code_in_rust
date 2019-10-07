use bbggez::{
	Utility,
	ggez::{
		nalgebra::Vector2,
		graphics::Mesh,
		Context,
	},
};
use rand::prelude::*;

#[derive(Clone)]
pub struct Planet {
	pub location: Vector2<f32>,
	pub mass: f32,
	velocity: Vector2<f32>,
	pub acceleration: Vector2<f32>,
	pub mesh: Mesh,
	id: usize,
}

impl Planet {
	pub fn new(location: Vector2<f32>, context: &mut Context, utility: &mut Utility, rng: &mut ThreadRng, id: usize) -> Planet {
		let mass = rng.gen_range(3.0, 35.0);
		let color = utility.random_bright_color();

		Planet {
			location,
			mass,
			velocity: Vector2::new(0.0, 0.0),
			acceleration: Vector2::new(0.0, 0.0),
			mesh: utility.create_circle(0.0, 0.0, mass, color, context),
			id,
		}
	}

	pub fn attract(&self, planets: &mut Vec<Planet>, gravitational_constant: f32) {
		for other_planet in planets {
			if self.id != other_planet.id {
				let mut force = self.location - other_planet.location;
				let distance = force.magnitude();
				let distance = self.constrain_distance(distance);
				let strength = (gravitational_constant * self.mass * other_planet.mass) / (distance * distance);
				
				force = force.normalize();
				force = force * strength;
				other_planet.apply_force(force);
			}
		}
	}

	fn constrain_distance(&self, distance: f32) -> f32 {
		if distance < 5.0 {
			return 5.0;
		} else if distance > 25.0 {
			return 25.0;
		}

		distance
	}

	pub fn apply_force(&mut self, force: Vector2<f32>) {
		self.acceleration = self.acceleration + force;
	}

	pub fn update(&mut self) {
		self.velocity = self.velocity + self.acceleration;
		self.location = self.location + self.velocity;
		self.acceleration = self.acceleration * 0.0;
	}
}
