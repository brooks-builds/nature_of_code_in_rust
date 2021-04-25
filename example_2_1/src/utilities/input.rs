use ggez::{input, Context};

use super::vector2::Vector2;

#[allow(dead_code)]
pub fn mouse_location(context: &mut Context) -> Vector2 {
    let mouse_location = input::mouse::position(context);
    Vector2::new(mouse_location.x, mouse_location.y)
}
