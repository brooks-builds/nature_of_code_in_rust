use ggez::graphics::{self, DrawMode, DrawParam, Mesh, MeshBuilder, WHITE};
use ggez::{Context, GameError, GameResult};
use rand::{thread_rng, Rng};

use crate::utilities::vector2::Vector2;

pub struct Mover {
    location: Vector2<f32>,
    velocity: Vector2<f32>,
    mesh: Mesh,
}

impl Mover {
    pub fn new(x: f32, y: f32, context: &mut Context) -> Result<Self, GameError> {
        let mut rng = thread_rng();
        let velocity = Vector2::new(rng.gen_range(-2.0..2.0), rng.gen_range(-2.0..2.0));
        let mesh = MeshBuilder::new()
            .circle(DrawMode::fill(), [0.0, 0.0], 25.0, 0.1, WHITE)
            .build(context)?;
        Ok(Self {
            location: Vector2::new(x, y),
            velocity,
            mesh,
        })
    }

    pub fn display(&self, context: &mut Context) -> GameResult {
        graphics::draw(
            context,
            &self.mesh,
            DrawParam::default().dest(self.location.to_array()),
        )
    }

    pub fn update(&mut self) {
        self.location += self.velocity;
    }

    pub fn check_edges(&mut self, context: &mut Context) {
        let (screen_width, screen_heigth) = graphics::drawable_size(context);
        if self.location.x < 0.0 {
            self.location.x = screen_width;
        } else if self.location.x > screen_width {
            self.location.x = 0.0;
        }

        if self.location.y < 0.0 {
            self.location.y = screen_heigth;
        } else if self.location.y > screen_heigth {
            self.location.y = 0.0;
        }
    }
}
