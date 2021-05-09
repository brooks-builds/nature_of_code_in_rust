use ggez::graphics::{Color, DrawMode, MeshBuilder, Rect};

use crate::mover::Mover;

pub struct Zone {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
    coefficient_of_friction: f32,
}

impl Zone {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
        coefficient_of_friction: f32,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            coefficient_of_friction,
            color,
        }
    }

    pub fn draw(&self, mesh_builder: &mut MeshBuilder) {
        mesh_builder.rectangle(DrawMode::fill(), self.as_rect(), self.color);
    }

    pub fn apply_friction_to_mover(&self, mover: &mut Mover) {
        let mut friction = mover.velocity;
        if mover.location.x < self.x
            || mover.location.x > self.x + self.width
            || friction.magnitude() == 0.0
        {
            return;
        }
        friction.reverse();
        friction.normalize();
        friction *= self.coefficient_of_friction;
        mover.apply_force(friction);
    }

    fn as_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
}
