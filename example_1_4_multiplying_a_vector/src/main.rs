use example_1_4::MainState;
use eyre::Result;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{event, ContextBuilder};

const TITLE: &str = "Nature of Code in Rust: Example 1.4 - Multiplying a vector";

fn main() -> Result<()> {
    let window_mode = WindowMode::default().dimensions(1920.0, 1080.0);
    let window_setup = WindowSetup::default().title(TITLE);
    let (mut context, mut events_loop) = ContextBuilder::new(TITLE, "Brookzerker")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()?;
    let mut main_state = MainState::new(&mut context)?;
    event::run(&mut context, &mut events_loop, &mut main_state)?;
    Ok(())
}
