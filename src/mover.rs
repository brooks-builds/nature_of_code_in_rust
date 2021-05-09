use ggez::graphics::{Color, DrawMode, MeshBuilder, WHITE};

use crate::utilities::vector2::Vector2;

pub struct Mover {
    pub location: Vector2,
    radius: f32,
    color: Color,
    acceleration: Vector2,
    pub velocity: Vector2,
}

impl Mover {
    pub fn new(location: Vector2, radius: f32) -> Self {
        let color = WHITE;
        let velocity = Vector2::default();
        let acceleration = Vector2::default();

        Self {
            location,
            radius,
            color,
            velocity,
            acceleration,
        }
    }

    pub fn draw(&self, mesh_builder: &mut MeshBuilder) {
        mesh_builder.circle(
            DrawMode::fill(),
            self.location.as_mint_point2(),
            self.radius,
            0.1,
            self.color,
        );
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.location += self.velocity;
        self.acceleration.reset();
    }

    pub fn apply_force(&mut self, force: Vector2) {
        self.acceleration += force;
    }

    pub fn check_edges(&mut self, screen_width: f32) {
        if self.location.x > screen_width {
            self.location.x = 0.0;
        }
    }
}
