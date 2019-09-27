use example_2_2_forces_acting_on_many_objects::*;
use ggez::{ContextBuilder, event};

fn main() {
    let window_mode = ggez::conf::WindowMode {
        width: 925.0,
        height: 900.0,
        maximized: false,
        fullscreen_type: ggez::conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 925.0,
        max_width: 0.0,
        min_height: 900.0,
        max_height: 0.0,
        resizable: false,
    };
    let (mut context, mut event_loop) = ContextBuilder::new("Forces acting on many objects", "Brookzerker")
        .window_mode(window_mode)
        .build()
        .expect("Game context was not able to be created");

    let mut game = Game::new(&mut context);

    match event::run(&mut context, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly"),
        Err(error) => println!("Error occured: {}", error),
    };
}