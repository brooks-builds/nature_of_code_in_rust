use exercise_1_1::MainState;
use ggez::{event, ContextBuilder, GameResult};

fn main() -> GameResult {
    let (context, event_loop) = &mut ContextBuilder::new("Exercise 1.1", "Brooks Builds")
        .build()
        .unwrap();

    let mut main_state = MainState::new(context)?;

    event::run(context, event_loop, &mut main_state)
}
