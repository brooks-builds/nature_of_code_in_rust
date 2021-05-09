use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder, BLACK};
use ggez::{Context, GameResult};

mod utilities;

pub struct MainState {
    background_color: Color,
    size: f32,
    mesh: Mesh,
    x: f32,
    y: f32,
    speed_x: f32,
    speed_y: f32,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = BLACK;

        let size = 16.0;
        let color = Color::from_rgb(175, 175, 175);
        let mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), [0.0, 0.0], size, 0.1, color)
            .build(context)?;
        let x = 100.0;
        let y = 100.0;
        let speed_x = 1.0;
        let speed_y = 3.3;

        Ok(Self {
            background_color,
            size,
            mesh,
            x,
            y,
            speed_x,
            speed_y,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        self.x += self.speed_x;
        self.y += self.speed_y;

        let (width, height) = graphics::drawable_size(context);

        if self.x < 0.0 {
            self.x = 0.0;
            self.speed_x *= -1.0;
        } else if self.x > width {
            self.x = width;
            self.speed_x *= -1.0;
        }

        if self.y < 0.0 {
            self.y = 0.0;
            self.speed_y *= -1.0;
        } else if self.y > height {
            self.y = height;
            self.speed_y *= -1.0;
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);
        graphics::draw(context, &self.mesh, DrawParam::new().dest([self.x, self.y]))?;
        graphics::present(context)
    }
}
