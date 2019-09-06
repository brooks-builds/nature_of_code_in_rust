use ggez::{Context, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::nalgebra::Point2;
use rand::prelude::*;

pub struct Game {
	random_points: Vec<Point2<f32>>,
	current_x: f32,
	window_width: f32,
	window_height: f32,
	rng: ThreadRng
}

impl Game {
    pub fn new(context: &mut Context) -> Game {
		let (window_width, window_height) = graphics::drawable_size(context);
		let current_x = window_width;
		let rng = rand::thread_rng();

        Game {
			random_points: vec!{Point2::new(current_x, window_width / 2.0)},
			current_x,
			window_height,
			window_width,
			rng
		}
    }
}

impl EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
		let random_y = self.rng.gen_range(0.0, self.window_height);

		for random_point in self.random_points.iter_mut() {
			random_point.x = random_point.x - 1.0;
		}

		let new_point = Point2::new(self.current_x, random_y);

		self.random_points.push(new_point);

		if self.random_points.len() > self.window_width as usize {
			self.random_points.remove(0);
		}

		Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);

		let line_width = 1.0;
		let color = graphics::WHITE;

		let line = graphics::MeshBuilder::new()
			.line(&self.random_points, line_width, color)?
			.build(context)?;

		graphics::draw(context, &line, (Point2::new(0.0, 0.0), 0.0, graphics::WHITE))?;

        graphics::present(context)
    }
}