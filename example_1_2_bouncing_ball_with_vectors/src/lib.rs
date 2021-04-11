use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder, BLACK};
use ggez::{Context, GameResult};
use utilities::Vector2;

mod utilities;

pub struct MainState {
    background_color: Color,
    mesh: Mesh,
    location: Vector2,
    velocity: Vector2,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = BLACK;

        let size = 16.0;
        let color = Color::from_rgb(175, 175, 175);
        let mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), [0.0, 0.0], size, 0.1, color)
            .build(context)?;
        let location = Vector2::new(100.0, 100.0);
        let velocity = Vector2::new(1.0, 3.3);

        Ok(Self {
            background_color,
            mesh,
            location,
            velocity,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        self.location += self.velocity;

        let (width, height) = graphics::drawable_size(context);

        if self.location.x < 0.0 {
            self.location.x = 0.0;
            self.velocity.x *= -1.0;
        } else if self.location.x > width {
            self.location.x = width;
            self.velocity.x *= -1.0;
        }

        if self.location.y < 0.0 {
            self.location.y = 0.0;
            self.velocity.y *= -1.0;
        } else if self.location.y > height {
            self.location.y = height;
            self.velocity.y *= -1.0;
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);
        graphics::draw(
            context,
            &self.mesh,
            DrawParam::new().dest(self.location.to_array()),
        )?;
        graphics::present(context)
    }
}
