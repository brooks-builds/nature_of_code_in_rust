use ggez::{
    graphics::{self, Color, DrawMode, DrawParam, MeshBuilder, Rect},
    Context, GameResult,
};

#[allow(dead_code)]
pub struct Zone {
    x: f32,
    pub y: f32,
    width: f32,
    height: f32,
    color: Color,
    pub drag: f32,
}

#[allow(dead_code)]
impl Zone {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color, drag: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
            drag,
        }
    }

    pub fn draw(&self, context: &mut Context) -> GameResult {
        let mesh = MeshBuilder::new()
            .rectangle(DrawMode::fill(), self.as_rect(), self.color)
            .build(context)?;
        graphics::draw(context, &mesh, DrawParam::new())
    }

    fn as_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
}
