use eyre::Result;
use ggez::graphics::{DrawMode, DrawParam, Mesh, MeshBuilder, WHITE};
use ggez::{graphics, Context};
use noise::{NoiseFn, Perlin};

pub struct Walker {
    x: f32,
    y: f32,
    mesh: Mesh,
}

impl Walker {
    pub fn new(context: &mut Context) -> Result<Self> {
        let x = 0.0;
        let y = 0.0;
        let mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), [0.0, 0.0], 25.0, 0.1, WHITE)
            .build(context)?;

        Ok(Self { x, y, mesh })
    }

    pub fn step(&mut self, noise: &Perlin, screen_width: f32, screen_height: f32) {}

    pub fn draw(&self, context: &mut Context) -> Result<()> {
        graphics::draw(context, &self.mesh, DrawParam::new().dest([self.x, self.y]))?;
        Ok(())
    }
}
