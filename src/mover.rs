use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::graphics::Mesh;
use ggez::graphics::Rect;
use ggez::graphics::{Color, DrawMode, MeshBuilder, WHITE};
use ggez::Context;
use ggez::GameResult;

use crate::utilities::vector2::Vector2;
use crate::zone::Zone;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Mover {
    pub location: Vector2,
    width: f32,
    height: f32,
    color: Color,
    acceleration: Vector2,
    pub velocity: Vector2,
    pub mass: f32,
    rotation: f32,
    rotate_amount: f32,
    mesh: Mesh,
    crashed: bool,
    fly_force: Vector2,
}

#[allow(dead_code)]
impl Mover {
    pub fn new(
        location: Vector2,
        mass: f32,
        context: &mut Context,
        speed: f32,
    ) -> GameResult<Self> {
        let color = WHITE;
        let velocity = Vector2::default();
        let acceleration = Vector2::default();
        let width = mass * 100.0;
        let height = 10.0;
        let rotation = 0.0;
        let rotate_amount = 0.01;
        let mesh = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                Rect::new(-width / 2.0, -height / 2.0, width, height),
                WHITE,
            )
            .build(context)?;
        let crashed = false;
        let fly_force = Vector2::new(speed, 0.0);

        Ok(Self {
            location,
            width,
            height,
            color,
            acceleration,
            velocity,
            mass,
            rotation,
            rotate_amount,
            mesh,
            crashed,
            fly_force,
        })
    }

    pub fn draw(&self, context: &mut Context) -> GameResult {
        graphics::draw(
            context,
            &self.mesh,
            DrawParam::new()
                .rotation(self.rotation)
                .dest(self.location.as_mint_point2()),
        )
    }

    pub fn update(&mut self) {
        if !self.crashed {
            self.velocity += self.acceleration;
            self.location += self.velocity;
            self.acceleration.reset();
        }
    }

    pub fn apply_force(&mut self, force: Vector2) {
        self.acceleration += force / self.mass;
    }

    pub fn check_edges(&mut self, (_screen_width, screen_height): (f32, f32)) {
        if self.location.y + self.height > screen_height {
            self.crashed = true;
        }
    }

    pub fn is_inside_zone(&self, zone: &Zone) -> bool {
        self.location.y + self.height > zone.y
    }

    pub fn apply_drag(&mut self, zone: &Zone) {
        let drag = self.velocity.magnitude().powi(2) * self.width * zone.drag;
        let mut drag_force = self.velocity;
        drag_force.normalize();
        drag_force.reverse();
        drag_force *= drag;
        self.acceleration += drag_force;
    }

    pub fn apply_lift(&mut self, zone: &Zone) {
        if self.rotation <= 0.0 {
            let lift = self.velocity.magnitude().powi(2) * zone.drag;
            let mut lift_force = self.velocity.perpendicular_left();
            lift_force.normalize();
            lift_force *= lift;
            self.acceleration += lift_force;
        }
    }

    pub fn handle_input(&mut self, context: &mut Context) {
        let keys = ggez::input::keyboard::pressed_keys(context);
        for key_code in keys {
            match key_code {
                ggez::event::KeyCode::Left => self.rotation -= self.rotate_amount,
                ggez::event::KeyCode::Right => self.rotation += self.rotate_amount,
                _ => {}
            }
        }
    }

    pub fn fly(&mut self) {
        self.apply_force(self.fly_force);
    }
}
