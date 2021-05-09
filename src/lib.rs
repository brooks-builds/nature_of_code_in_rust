use ggez::event::EventHandler;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect, BLACK, WHITE};
use ggez::{Context, GameResult};
use utilities::input::mouse_location;
use utilities::vector2::Vector2;

mod utilities;

pub struct MainState {
    background_color: Color,
    length_mesh: Option<Mesh>,
    line_mesh: Option<Mesh>,
    center: Vector2<f32>,
    length_color: Color,
}

impl MainState {
    pub fn new(context: &mut Context) -> GameResult<Self> {
        let background_color = BLACK;

        let length_mesh = None;
        let line_mesh = None;
        let (width, height) = graphics::drawable_size(context);
        let center = Vector2::new(width / 2.0, height / 2.0);
        let length_color = Color::new(0.0, 1.0, 0.0, 1.0);

        Ok(Self {
            background_color,
            length_mesh,
            line_mesh,
            center,
            length_color,
        })
    }
}

impl EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        let mouse = mouse_location(context);
        let distance_to_mouse = mouse - self.center;
        let distance = distance_to_mouse.magnitude();
        let length_rect = Rect::new(0.0, 0.0, distance, 10.0);

        self.length_mesh = Some(
            MeshBuilder::new()
                .rectangle(DrawMode::fill(), length_rect, self.length_color)
                .build(context)?,
        );

        let points = [self.center.to_array(), mouse.to_array()];
        self.line_mesh = Some(
            MeshBuilder::new()
                .line(&points, 2.0, WHITE)?
                .build(context)?,
        );
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        graphics::clear(context, self.background_color);

        if let Some(mesh) = &self.length_mesh {
            graphics::draw(context, mesh, DrawParam::default())?;
        }

        if let Some(mesh) = &self.line_mesh {
            graphics::draw(context, mesh, DrawParam::default())?;
        }

        graphics::present(context)
    }
}
