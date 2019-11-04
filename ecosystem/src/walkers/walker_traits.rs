use bbggez::ggez::nalgebra::Vector2;

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
			println!("got here");
			self.set_location(location);
		}
	}

	fn set_location(&mut self, location: Vector2<f32>);

	fn get_location(&self) -> Vector2<f32>;

	fn get_health(&self) -> f32;
}