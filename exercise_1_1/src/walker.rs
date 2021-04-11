use ggez::graphics::{self, DrawMode, DrawParam, Mesh, MeshBuilder, WHITE};
use ggez::{Context, GameResult};
use nalgebra::Vector2;
use rand::random;

pub struct Walker {
    location: Vector2<f32>,
    mesh: Mesh,
}

impl Walker {
    pub fn new(x: f32, y: f32, context: &mut Context) -> GameResult<Self> {
        let mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), [0.0, 0.0], 1.0, 0.1, WHITE)
            .build(context)?;
        let location = Vector2::new(x, y);
        Ok(Self { location, mesh })
    }

    pub fn display(&self, context: &mut Context) -> GameResult {
        graphics::draw(
            context,
            &self.mesh,
            DrawParam::new().dest([self.location.x, self.location.y]),
        )
    }

    pub fn step(&mut self) {
        let choice = (random::<f32>() * 4.0).floor() as i32;
        match choice {
            0 => self.location.x += 1.0,
            1 => self.location.x -= 1.0,
            2 => self.location.y += 1.0,
            3 => self.location.y -= 1.0,
            _ => unreachable!(),
        }
    }
}
