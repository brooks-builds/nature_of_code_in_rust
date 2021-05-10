use ggez::graphics::{Color, DrawMode, MeshBuilder, WHITE};

use crate::utilities::vector2::Vector2;
use crate::zone::Zone;

#[allow(dead_code)]
pub struct Mover {
    pub location: Vector2,
    radius: f32,
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
        let radius = mass;

        Self {
            location,
            radius,
            color,
            velocity,
            acceleration,
            mass,
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
        self.acceleration += force / self.mass;
    }

    pub fn check_edges(&mut self, (_screen_width, screen_height): (f32, f32)) {
        if self.location.y + self.radius > screen_height {
            self.location.y = screen_height - self.radius;
            self.velocity.y *= -1.0;
        }
    }

    pub fn is_inside_zone(&self, zone: &Zone) -> bool {
        self.location.y + self.radius > zone.y
    }

    pub fn apply_drag(&mut self, zone: &Zone) {
        let speed = self.velocity.magnitude();
        let drag_speed = speed.powi(2) * zone.coefficient_of_drag;
        let mut drag_force = self.velocity;
        drag_force.reverse();
        drag_force.normalize();
        drag_force *= drag_speed;
        self.apply_force(drag_force);
    }
}
