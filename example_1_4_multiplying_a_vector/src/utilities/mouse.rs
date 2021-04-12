use ggez::{input, Context};

use super::vector2::Vector2;

pub fn mouse_location(context: &mut Context) -> Vector2<f32> {
    let mouse_location = input::mouse::position(context);
    Vector2::new(mouse_location.x, mouse_location.y)
}
