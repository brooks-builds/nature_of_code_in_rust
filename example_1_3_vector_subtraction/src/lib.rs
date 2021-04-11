use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawParam, Mesh, MeshBuilder, BLACK, WHITE};
use ggez::{input, Context, GameResult};
use utilities::vector2::Vector2;

mod utilities;

pub struct MainState {
    background_color: Color,
    mesh: Option<Mesh>,
}

impl MainState {
    pub fn new(_context: &mut Context) -> GameResult<Self> {
        let background_color = BLACK;
        let mesh = None;

        Ok(Self {
            background_color,
            mesh,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let mouse_position = input::mouse::position(context);
        let mut mouse_location = Vector2::new(mouse_position.x, mouse_position.y);
        let (width, height) = graphics::drawable_size(context);
        let center_location = Vector2::new(width / 2.0, height / 2.0);

        mouse_location -= center_location;
        let points = [[0.0, 0.0], mouse_location.to_array()];

        self.mesh = Some(
            MeshBuilder::new()
                .line(&points, 2.0, WHITE)?
                .build(context)?,
        );
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);

        let (width, height) = graphics::drawable_size(context);
        let transform = DrawParam::new()
            .dest([width / 2.0, height / 2.0])
            .to_matrix();
        graphics::set_transform(context, transform);
        if let Some(mesh) = &self.mesh {
            graphics::draw(context, mesh, DrawParam::new())?;
        }
        graphics::apply_transformations(context)?;

        graphics::present(context)
    }
}
