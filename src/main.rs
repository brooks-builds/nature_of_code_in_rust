use example_i_5::MainState;
use eyre::Result;
use ggez::conf::WindowMode;
use ggez::{event, ContextBuilder};

fn main() -> Result<()> {
    let window_mode = WindowMode::default().dimensions(1920.0, 1080.0);
    let (mut context, mut events_loop) = ContextBuilder::new(
        "Nature of Code in Rust: Example I.5 - Perlin Noise Walker",
        "Brookzerker",
    )
    .window_mode(window_mode)
    .build()?;
    let mut main_state = MainState::new(&mut context)?;
    event::run(&mut context, &mut events_loop, &mut main_state)?;
    Ok(())
}
