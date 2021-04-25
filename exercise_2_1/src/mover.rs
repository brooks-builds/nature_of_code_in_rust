use ggez::event::KeyCode;
use ggez::graphics::{DrawMode, DrawParam, Mesh, MeshBuilder, WHITE};
use ggez::input::keyboard::is_key_pressed;
use ggez::{graphics, Context, GameResult};

use crate::utilities::vector2::Vector2;

#[allow(dead_code)]
pub struct Mover {
    location: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    mesh: Option<Mesh>,
    topspeed: f32,
    scale: f32,
    radius: f32,
}

#[allow(dead_code)]
impl Mover {
    pub fn new(x: f32, y: f32, context: &mut Context) -> GameResult<Self> {
        let location = Vector2::new(x, y);
        let velocity = Vector2::new(0.0, 0.0);
        let acceleration = Vector2::new(0.0, 0.0);
        let radius = 25.0;
        let mesh = Some(
            MeshBuilder::new()
                .circle(DrawMode::fill(), [0.0, 0.0], radius, 0.1, WHITE)
                .build(context)?,
        );
        let topspeed = 10.0;
        let scale = 0.5;

        Ok(Self {
            location,
            velocity,
            acceleration,
            mesh,
            topspeed,
            scale,
            radius,
        })
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.velocity.limit(self.topspeed);
        self.location += self.velocity;
        self.acceleration.multiply_scalar(0.0);
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

    pub fn check_edges(&mut self, context: &mut Context) {
        if self.location.y - self.radius < 0.0 {
            self.location.y = self.radius;
            let mut bounce_force = self.velocity;
            bounce_force.y *= -1.0;
            bounce_force.multiply_scalar(1.75);
            self.apply_force(bounce_force);
        }

        let (screen_width, _screen_height) = graphics::drawable_size(context);

        if self.location.x - self.radius < 0.0 {
            self.location.x = self.radius;
            self.velocity.x = 0.0;
        } else if self.location.x + self.radius > screen_width {
            self.location.x = screen_width - self.radius;
            self.velocity.x = 0.0;
        }
    }

    pub fn handle_input(&mut self, context: &mut Context) {
        if is_key_pressed(context, KeyCode::Up) {
            self.acceleration = Vector2::new(0.0, -0.01);
        } else if is_key_pressed(context, KeyCode::Down) {
            self.acceleration = Vector2::new(0.0, 0.01);
        }
    }

    pub fn apply_force(&mut self, force: Vector2) {
        self.acceleration += force;
    }
}
