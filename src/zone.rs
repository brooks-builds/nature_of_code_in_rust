use ggez::graphics::{Color, DrawMode, MeshBuilder, Rect};

#[allow(dead_code)]
pub struct Zone {
    x: f32,
    pub y: f32,
    width: f32,
    height: f32,
    color: Color,
    pub coefficient_of_drag: f32,
}

#[allow(dead_code)]
impl Zone {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
        coefficient_of_drag: f32,
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            coefficient_of_drag,
            color,
        }
    }

    pub fn draw(&self, mesh_builder: &mut MeshBuilder) {
        mesh_builder.rectangle(DrawMode::fill(), self.as_rect(), self.color);
    }

    fn as_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
}
