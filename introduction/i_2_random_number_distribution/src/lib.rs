use ggez::{Context, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::nalgebra::Point2;
use rand::prelude::*;
use rand::seq::SliceRandom;

pub struct Game {
	width: f32,
	heights: Vec<f32>,
	colors: Vec<graphics::Color>
}

impl Game {
    pub fn new(context: &mut Context) -> Game {
        // store a vector (20 length) with 0.0 in each
        // [1.0, 1.0, 2.0]
		let mut heights = vec![];
		let mut colors = vec![];
		let mut rng = rand::thread_rng();
		let rectangle_count = 100;
		let (window_width, _window_height) = graphics::drawable_size(context);

		for _ in 0..rectangle_count {
			heights.push(0.0);
			colors.push(
				graphics::Color::from_rgb(
					rng.gen_range(0, 255), 
					rng.gen_range(0, 255), 
					rng.gen_range(0, 255))
			);
		}

        Game {
			width: window_width / rectangle_count as f32,
			heights,
			colors
		}
    }

	fn reset(&mut self) {
		for index in 0..self.heights.len() {
			self.heights[index] = 0.0;
		}
	}
}

impl EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
		let mut rng = rand::thread_rng();
		let rectangle_height = self.heights.choose_mut(&mut rng);
		let (_window_width, window_height) = graphics::drawable_size(context);

		if let Some(height) = rectangle_height {
			*height = *height + 1.0;
		}

		for (index, height) in self.heights.iter().enumerate() {
			if height > &window_height {
				println!("{} won the race!", index);

				self.reset();
				break;
			}
		}

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);

		let (_window_width, window_height) = graphics::drawable_size(context);

		for (index, rectangle_height) in self.heights.iter().enumerate() {
			let rectangle_mesh = graphics::MeshBuilder::new()
				.rectangle(
					graphics::DrawMode::fill(), 
					graphics::Rect::new(0.0, 0.0, self.width, *rectangle_height), 
					graphics::WHITE)
				.build(context)?;

			graphics::draw(context, &rectangle_mesh, (
				Point2::new(self.width * index as f32, window_height - rectangle_height), 0.0, 
				self.colors[index]))?;
			
		}
         

        graphics::present(context)
    }
}