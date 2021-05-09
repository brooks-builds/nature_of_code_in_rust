use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawParam, Mesh, MeshBuilder, BLACK, WHITE};
use ggez::{Context, GameResult};
use utilities::input::mouse_location;
use utilities::vector2::Vector2;

mod utilities;

pub struct MainState {
    background_color: Color,
    mesh: Option<Mesh>,
    center: Vector2<f32>,
    scale: f32,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = BLACK;
        let mesh = None;
        let (width, height) = graphics::drawable_size(context);
        let center = Vector2::new(width / 2.0, height / 2.0);
        let scale = 150.0;

        Ok(Self {
            background_color,
            mesh,
            center,
            scale,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let mouse = mouse_location(context);
        let mut direction = mouse - self.center;
        direction.normalize();
        direction.multiply_scalar(self.scale);
        direction += self.center;
        let points = [self.center.to_array(), direction.to_array()];
        self.mesh = Some(
            MeshBuilder::new()
                .line(&points, 2.0, WHITE)?
                .build(context)?,
        );

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);
        if let Some(mesh) = &self.mesh {
            graphics::draw(context, mesh, DrawParam::default())?;
        }
        graphics::present(context)
    }
}
