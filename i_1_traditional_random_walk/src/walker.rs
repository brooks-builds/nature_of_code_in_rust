use ggez::graphics::{self, DrawMode, DrawParam, Mesh, MeshBuilder, WHITE};
use ggez::{Context, GameResult};
use rand::random;

pub struct Walker {
    x: f32,
    y: f32,
    mesh: Mesh,
}

impl Walker {
    pub fn new(x: f32, y: f32, context: &mut Context) -> GameResult<Self> {
        let mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), [0.0, 0.0], 1.0, 0.1, WHITE)
            .build(context)?;
        Ok(Self { x, y, mesh })
    }

    pub fn display(&self, context: &mut Context) -> GameResult {
        graphics::draw(context, &self.mesh, DrawParam::new().dest([self.x, self.y]))
    }

    pub fn step(&mut self) {
        let choice = (random::<f32>() * 4.0).floor() as i32;
        match choice {
            0 => self.x += 1.0,
            1 => self.x -= 1.0,
            2 => self.y += 1.0,
            3 => self.y -= 1.0,
            _ => unreachable!(),
        }
    }
}
