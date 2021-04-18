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
}

#[allow(dead_code)]
impl Mover {
    pub fn new(x: f32, y: f32, context: &mut Context) -> GameResult<Self> {
        let location = Vector2::new(x, y);
        let velocity = Vector2::new(0.0, 0.0);
        let acceleration = Vector2::new(0.0, 0.0);
        let mesh = Some(
            MeshBuilder::new()
                .circle(DrawMode::fill(), [0.0, 0.0], 25.0, 0.1, WHITE)
                .build(context)?,
        );
        let topspeed = 10.0;

        Ok(Self {
            location,
            velocity,
            acceleration,
            mesh,
            topspeed,
        })
    }

    pub fn update(&mut self) {
        self.acceleration = Vector2::random();
        self.acceleration.multiply_scalar(2.0);
        self.velocity += self.acceleration;
        self.velocity.limit(self.topspeed);
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

    pub fn check_edges(&mut self, context: &mut Context) {
        let (screen_width, screen_height) = graphics::drawable_size(context);

        if self.location.x < 0.0 {
            self.location.x = screen_width;
        } else if self.location.x > screen_width {
            self.location.x = 0.0;
        }

        if self.location.y < 0.0 {
            self.location.y = screen_height;
        } else if self.location.y > screen_height {
            self.location.y = 0.0;
        }
    }

    pub fn handle_input(&mut self, context: &mut Context) {
        if is_key_pressed(context, KeyCode::Up) {
            self.acceleration = Vector2::new(0.0, -0.01);
        } else if is_key_pressed(context, KeyCode::Down) {
            self.acceleration = Vector2::new(0.0, 0.01);
        }
    }
}
