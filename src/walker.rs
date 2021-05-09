use eyre::Result;
use ggez::graphics::{DrawMode, DrawParam, Mesh, MeshBuilder, WHITE};
use ggez::{graphics, Context};
use noise::{NoiseFn, Perlin};

use crate::utilities::map;

pub struct Walker {
    x: f32,
    y: f32,
    noise_x: [f64; 2],
    noise_y: [f64; 2],
    mesh: Mesh,
}

impl Walker {
    pub fn new(context: &mut Context) -> Result<Self> {
        let x = 0.0;
        let y = 0.0;
        let noise_x = [0.0; 2];
        let noise_y = [0.0; 2];
        let mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), [0.0, 0.0], 25.0, 0.1, WHITE)
            .build(context)?;

        Ok(Self {
            x,
            y,
            noise_x,
            noise_y,
            mesh,
        })
    }

    pub fn step(&mut self, noise: &Perlin, screen_width: f32, screen_height: f32) {
        self.x = map(noise.get(self.noise_x) as f32, -1.0, 1.0, 0.0, screen_width);
        self.y = map(
            noise.get(self.noise_y) as f32,
            -1.0,
            1.0,
            0.0,
            screen_height,
        );
        self.noise_x[0] += 0.01;
        self.noise_y[1] += 0.01;
    }

    pub fn draw(&self, context: &mut Context) -> Result<()> {
        graphics::draw(context, &self.mesh, DrawParam::new().dest([self.x, self.y]))?;
        Ok(())
    }
}
