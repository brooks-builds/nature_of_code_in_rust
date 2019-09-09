mod ball;

use ggez::{Context, GameResult, graphics};
use ggez::event::EventHandler;
use ggez::nalgebra::Point2;
use ball::Ball;

pub struct Game {
	balls: Vec<Ball>,
	window_width: f32,
	window_height: f32
}

impl Game {
    pub fn new(context: &mut Context) -> Game {
		let (window_width, window_height) = graphics::drawable_size(context);
		let mut balls = vec![];

		for _ in 0..50 {
			balls.push(Ball::new(window_width, window_height));
		}

        Game {
			balls,
			window_width,
			window_height
		}
    }

	fn wrap_screen(&mut self) {
		for ball in &mut self.balls {
			if ball.location.y - ball.radius > self.window_height {
				ball.location.y = -ball.radius;
			} else if ball.location.y + ball.radius < 0.0 {
				ball.location.y = self.window_height + ball.radius;
			}

			if ball.location.x + ball.radius < 0.0 {
				ball.location.x = self.window_width + ball.radius;
			} else if ball.location.x - ball.radius > self.window_width {
				ball.location.x = -ball.radius;
			}
		}
	}
}

impl EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
		for ball in &mut self.balls {
			ball.update();
		}

		self.wrap_screen();

		Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);  

		let mut meshes = vec![];

		for ball in &self.balls {
			meshes.push(graphics::MeshBuilder::new()
				.circle(
					graphics::DrawMode::fill(), 
					ball.location, 
					ball.radius, 
					0.1, 
					ball.color
				)
				.build(context)?
			)
		}

		for mesh in meshes {
			graphics::draw(context, &mesh, (Point2::new(0.0, 0.0),))?;
		}

        graphics::present(context)
    }
}