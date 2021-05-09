use eyre::Result;
use ggez::graphics::{self, DrawParam, Mesh, MeshBuilder, WHITE};
use ggez::Context;
use noise::{NoiseFn, Perlin};
use rand::random;

use crate::utilities::map;

pub struct Walker {
    points: Vec<[f32; 2]>,
    mesh: Option<Mesh>,
    noise: Perlin,
    noise_step: f64,
    noise_location: [f64; 2],
}

impl Walker {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            points: vec![[x, y]],
            mesh: None,
            noise: Perlin::new(),
            noise_step: 0.01,
            noise_location: [0.0, 10_000.0],
        }
    }

    pub fn update(&mut self, context: &mut Context) -> Result<()> {
        let mut next_point = self.create_next_point();
        self.constrain_to_drawable_size(&mut next_point, graphics::drawable_size(context));
        self.points.push(next_point);

        self.noise_location[0] += self.noise_step;
        self.noise_location[1] += self.noise_step;

        self.update_mesh(context)?;

        Ok(())
    }

    pub fn draw(&self, context: &mut Context) -> Result<()> {
        if let Some(mesh) = &self.mesh {
            graphics::draw(context, mesh, DrawParam::new())?;
        }
        Ok(())
    }

    fn get_step_size(&self) -> f32 {
        map(
            self.noise.get(self.noise_location) as f32,
            -1.0,
            1.0,
            0.0,
            25.0,
        )
    }

    fn update_mesh(&mut self, context: &mut Context) -> Result<()> {
        self.mesh = Some(
            MeshBuilder::new()
                .line(&self.points, 2.0, WHITE)?
                .build(context)?,
        );
        Ok(())
    }

    fn create_next_point(&self) -> [f32; 2] {
        let direction: f32 = random();
        let mut next_point = *self.points.last().unwrap();

        if direction < 0.25 {
            next_point[1] -= self.get_step_size();
        } else if direction < 0.5 {
            next_point[0] += self.get_step_size();
        } else if direction < 0.75 {
            next_point[1] += self.get_step_size();
        } else {
            next_point[0] -= self.get_step_size();
        }

        next_point
    }

    fn constrain_to_drawable_size(&self, point: &mut [f32; 2], (width, height): (f32, f32)) {
        if point[0] < 0.0 {
            point[0] = 0.0;
        } else if point[0] > width {
            point[0] = width;
        }

        if point[1] < 0.0 {
            point[1] = 0.0;
        } else if point[1] > height {
            point[1] = height;
        }
    }
}
