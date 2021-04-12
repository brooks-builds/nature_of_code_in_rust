use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawParam, Mesh, MeshBuilder, BLACK, WHITE};
use ggez::{Context, GameResult};
use utilities::mouse::mouse_location;
use utilities::vector2::Vector2;

mod utilities;

pub struct MainState {
    background_color: Color,
    center: Vector2<f32>,
    scale: f32,
    mesh: Option<Mesh>,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = BLACK;
        let (width, height) = graphics::drawable_size(context);
        let center = Vector2::new(width / 2.0, height / 2.0);
        let scale = 0.5;
        let mesh = None;

        Ok(Self {
            background_color,
            center,
            scale,
            mesh,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let mut mouse = mouse_location(context);
        mouse -= self.center;

        mouse.multiply_scalar(self.scale);
        let points = [[0.0, 0.0], mouse.to_array()];

        self.mesh = Some(
            MeshBuilder::new()
                .line(&points, 2.0, WHITE)?
                .build(context)?,
        );
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);
        graphics::set_transform(
            context,
            DrawParam::new().dest(self.center.to_array()).to_matrix(),
        );
        if let Some(mesh) = &self.mesh {
            graphics::draw(context, mesh, DrawParam::new())?;
        }
        graphics::apply_transformations(context)?;
        graphics::present(context)
    }
}
