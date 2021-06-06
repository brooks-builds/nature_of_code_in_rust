use ggez::graphics::Rect;
use ggez::graphics::{Color, DrawMode, MeshBuilder, WHITE};

use crate::utilities::vector2::Vector2;
use crate::zone::Zone;

#[allow(dead_code)]
pub struct Mover {
    pub location: Vector2,
    width: f32,
    height: f32,
    color: Color,
    acceleration: Vector2,
    pub velocity: Vector2,
    pub mass: f32,
}

#[allow(dead_code)]
impl Mover {
    pub fn new(location: Vector2, mass: f32) -> Self {
        let color = WHITE;
        let velocity = Vector2::default();
        let acceleration = Vector2::default();
        let width = mass * 2.0;
        let height = 10.0;

        Self {
            location,
            width,
            height,
            color,
            acceleration,
            velocity,
            mass,
        }
    }

    pub fn draw(&self, mesh_builder: &mut MeshBuilder) {
        let bounds = Rect::new(self.location.x, self.location.y, self.width, self.height);

        mesh_builder.rectangle(DrawMode::fill(), bounds, WHITE);
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.location += self.velocity;
        self.acceleration.reset();
    }

    pub fn apply_force(&mut self, force: Vector2) {
        self.acceleration += force / self.mass;
    }

    pub fn check_edges(&mut self, (_screen_width, screen_height): (f32, f32)) {
        if self.location.y + self.height > screen_height {
            self.location.y = screen_height - self.height;
            self.velocity.y *= -1.0;
        }
    }

    pub fn is_inside_zone(&self, zone: &Zone) -> bool {
        self.location.y + self.height > zone.y
    }

    pub fn apply_drag(&mut self, zone: &Zone) {
        let drag = self.velocity.magnitude().powi(2) * self.width * zone.coefficient_of_drag;
        let mut drag_force = self.velocity;
        drag_force.normalize();
        drag_force.reverse();
        drag_force *= drag;
        self.acceleration += drag_force;
    }
}
