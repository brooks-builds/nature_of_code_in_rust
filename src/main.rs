use exercise_i_6::MainState;
use ggez::conf::WindowMode;
use ggez::{event, ContextBuilder, GameResult};

fn main() -> GameResult {
    let window_mode = WindowMode::default().dimensions(1920.0, 1080.0);
    let (mut context, mut events_loop) =
        ContextBuilder::new("Nature of Code in Rust: Exercise I.6", "Brookzerker")
            .window_mode(window_mode)
            .build()?;
    let mut main_state = MainState::new(&mut context)?;
    event::run(&mut context, &mut events_loop, &mut main_state)
}
