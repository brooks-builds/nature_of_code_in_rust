use ggez::graphics::{DrawParam, Mesh};
use ggez::{graphics, Context, GameResult};

use crate::utilities::vector2::Vector2;

#[allow(dead_code)]
pub struct Mover {
    location: Vector2<f32>,
    velocity: Vector2<f32>,
    mesh: Option<Mesh>,
}

#[allow(dead_code)]
impl Mover {
    pub fn new(x: f32, y: f32) -> Self {
        let location = Vector2::new(x, y);
        let velocity = Vector2::new(0.0, 0.0);
        let mesh = None;

        Self {
            location,
            velocity,
            mesh,
        }
    }

    pub fn update(&mut self) {
        self.location += self.velocity;
    }

    pub fn display(&self, context: &mut Context) -> GameResult {
        if let Some(mesh) = &self.mesh {
            graphics::draw(
                context,
                mesh,
                DrawParam::default().dest(self.location.to_array()),
            )?;
        }

        Ok(())
    }
}
