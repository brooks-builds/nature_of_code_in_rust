use bbggez::ggez::nalgebra::Vector2;
use crate::foods::food::Food;

pub trait WalkerMovement {
	fn keep_in_arena(&mut self, (arena_width, arena_height): (f32, f32)) {
		let mut location = self.get_location();
		let health = self.get_health();

		if location.x - health > arena_width {
			location.x = -health;
		}  else if location.x + health < 0.0 {
			location.x = arena_width - health;
		} 

		if location.y - health > arena_height {
			location.y = -health;
		} else if location.y + health < 0.0 {
			location.y = arena_height - health;
		}

		if location != self.get_location() {
			self.set_location(location);
		}
	}

	fn set_location(&mut self, location: Vector2<f32>);

	fn get_location(&self) -> Vector2<f32>;

	fn get_health(&self) -> f32;

	fn eat(&mut self, foods: &mut Vec<Food>) {
		for (index, food) in foods.clone().iter().enumerate().rev() {
			let direction = food.location - self.get_location();
			let distance = direction.magnitude();
			let size = self.get_health();

			if distance <= size {
				self.set_size(size + food.calories);
				foods.remove(index);
			}

		}
	}

	fn set_size(&mut self, new_size: f32);

	fn get_distance(&self, other_location: Vector2<f32>) -> f32 {
		let location = self.get_location();
		let direction = location - other_location;

		direction.magnitude()
	}

	fn is_larger(&self, other_size: f32) -> bool {
		self.get_health() > other_size
	}
}